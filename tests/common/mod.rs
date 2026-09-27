//! Test harness for the ports of the official ROS 2 xacro test suite
//! (https://github.com/ros/xacro, `test/test_xacro.py`, BSD-3-Clause, see
//! `tests/fixtures/ros_xacro/LICENSE`).
//!
//! `xml_matches` mirrors upstream's comparison: whitespace is normalized, whitespace-only text is
//! ignored, numbers match up to 1e-9, and namespace declarations count as attributes.
#![allow(dead_code)]

use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};
use xacro::{XacroError, XacroOptions};
use xml::reader::{EventReader, ParserConfig, XmlEvent};

/// Guards against processor bugs that never terminate, so one hang does not stall the suite
const TIMEOUT: Duration = Duration::from_secs(10);

/// Root of the `xacro` package as resolved by `$(find xacro)`
pub fn package_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ros_xacro")
}

/// Directory upstream runs its tests from, and thus the base for relative includes
pub fn test_dir() -> PathBuf {
    package_dir().join("test")
}

pub fn default_options() -> XacroOptions {
    let mut options = XacroOptions::default();
    options.packages.insert("xacro".into(), package_dir());
    options
}

/// Builds options from upstream-style command line mappings such as `foo:=4`
pub fn cli(mappings: &[&str]) -> XacroOptions {
    let mut options = default_options();
    for mapping in mappings {
        let (name, value) = mapping
            .split_once(":=")
            .unwrap_or_else(|| panic!("invalid mapping: {mapping}"));
        options.args.insert(name.into(), value.into());
    }
    options
}

pub fn try_xacro_with(
    src: &str,
    options: XacroOptions,
) -> Result<String, XacroError> {
    let src = src.to_owned();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(xacro::process_str(&src, test_dir(), &options));
    });
    match rx.recv_timeout(TIMEOUT) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => panic!("xacro did not finish within {TIMEOUT:?}"),
        Err(RecvTimeoutError::Disconnected) => panic!("xacro panicked"),
    }
}

pub fn xacro_with(
    src: &str,
    options: XacroOptions,
) -> String {
    try_xacro_with(src, options).unwrap_or_else(|e| panic!("xacro failed: {e}"))
}

pub fn xacro(src: &str) -> String {
    xacro_with(src, default_options())
}

/// Equivalent of upstream's `assertRaises(XacroException, self.quick_xacro, src)`
pub fn assert_xacro_err_with(
    src: &str,
    options: XacroOptions,
) {
    if let Ok(output) = try_xacro_with(src, options) {
        panic!("expected xacro to fail, but it produced:\n{output}");
    }
}

pub fn assert_xacro_err(src: &str) {
    assert_xacro_err_with(src, default_options());
}

#[track_caller]
pub fn assert_matches(
    actual: &str,
    expected: &str,
) {
    assert!(
        xml_matches(actual, expected, false),
        "XML mismatch\n--- actual:\n{actual}\n--- expected:\n{expected}"
    );
}

#[track_caller]
pub fn assert_matches_ignoring_comments(
    actual: &str,
    expected: &str,
) {
    assert!(
        xml_matches(actual, expected, true),
        "XML mismatch (ignoring comments)\n--- actual:\n{actual}\n--- expected:\n{expected}"
    );
}

#[derive(Debug)]
enum Node {
    Element {
        name: String,
        attributes: BTreeMap<String, String>,
        children: Vec<Node>,
    },
    Text(String),
    Comment(String),
}

pub fn xml_matches(
    a: &str,
    b: &str,
    ignore_comments: bool,
) -> bool {
    match (parse(a), parse(b)) {
        (Ok(a), Ok(b)) => nodes_match(&a, &b, ignore_comments),
        (a, b) => {
            eprintln!("Cannot compare documents: {:?} / {:?}", a.err(), b.err());
            false
        }
    }
}

fn parse(src: &str) -> Result<Node, String> {
    let config = ParserConfig::new()
        .trim_whitespace(false)
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .coalesce_characters(true)
        .ignore_comments(false);
    let mut stack: Vec<Node> = Vec::new();
    let mut scopes: Vec<BTreeMap<String, String>> = vec![BTreeMap::new()];
    for event in EventReader::new_with_config(src.as_bytes(), config) {
        match event.map_err(|e| e.to_string())? {
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                let mut attrs: BTreeMap<String, String> = attributes
                    .into_iter()
                    .map(|a| {
                        (
                            qualified(a.name.prefix.as_deref(), &a.name.local_name),
                            a.value,
                        )
                    })
                    .collect();
                // xml-rs reports all namespaces in scope; declarations are the ones new here
                let parent = scopes.last().unwrap();
                for (prefix, uri) in &namespace {
                    let declared = matches!(prefix, "xml" | "xmlns") || uri.is_empty();
                    if !declared && parent.get(prefix).map(String::as_str) != Some(uri) {
                        let key = if prefix.is_empty() {
                            "xmlns".into()
                        } else {
                            format!("xmlns:{prefix}")
                        };
                        attrs.insert(key, uri.to_owned());
                    }
                }
                scopes.push(namespace.0);
                stack.push(Node::Element {
                    name: qualified(name.prefix.as_deref(), &name.local_name),
                    attributes: attrs,
                    children: Vec::new(),
                });
            }
            XmlEvent::EndElement { .. } => {
                scopes.pop();
                let node = stack.pop().unwrap();
                match stack.last_mut() {
                    Some(Node::Element { children, .. }) => children.push(node),
                    _ => return Ok(node),
                }
            }
            XmlEvent::Characters(text) => push_child(&mut stack, Node::Text(text)),
            XmlEvent::Comment(text) => push_child(&mut stack, Node::Comment(text)),
            _ => {}
        }
    }
    Err("document has no root element".into())
}

fn push_child(
    stack: &mut [Node],
    node: Node,
) {
    // Nodes outside the root element are not compared, as upstream only compares documentElement
    if let Some(Node::Element { children, .. }) = stack.last_mut() {
        children.push(node);
    }
}

fn qualified(
    prefix: Option<&str>,
    local_name: &str,
) -> String {
    match prefix {
        Some(prefix) => format!("{prefix}:{local_name}"),
        None => local_name.to_owned(),
    }
}

fn nodes_match(
    a: &Node,
    b: &Node,
    ignore_comments: bool,
) -> bool {
    match (a, b) {
        (Node::Text(a), Node::Text(b)) | (Node::Comment(a), Node::Comment(b)) => text_matches(a, b),
        (
            Node::Element {
                name: a_name,
                attributes: a_attrs,
                children: a_children,
            },
            Node::Element {
                name: b_name,
                attributes: b_attrs,
                children: b_children,
            },
        ) => {
            if a_name != b_name {
                eprintln!("Different element names: {a_name} and {b_name}");
                return false;
            }
            if a_attrs.len() != b_attrs.len() {
                eprintln!("Different attributes on {a_name}: {a_attrs:?} and {b_attrs:?}");
                return false;
            }
            for ((a_key, a_value), (b_key, b_value)) in a_attrs.iter().zip(b_attrs) {
                if a_key != b_key {
                    eprintln!("Different attribute names: {a_key} and {b_key}");
                    return false;
                }
                if !text_matches(a_value, b_value) {
                    return false;
                }
            }
            let a_children = significant(a_children, ignore_comments);
            let b_children = significant(b_children, ignore_comments);
            if a_children.len() != b_children.len() {
                eprintln!("Different number of children in {a_name}");
                return false;
            }
            a_children
                .iter()
                .zip(&b_children)
                .all(|(a, b)| nodes_match(a, b, ignore_comments))
        }
        _ => {
            eprintln!("Different node types: {a:?} and {b:?}");
            false
        }
    }
}

fn significant(
    children: &[Node],
    ignore_comments: bool,
) -> Vec<&Node> {
    children
        .iter()
        .filter(|node| match node {
            Node::Text(text) => !text.trim().is_empty(),
            Node::Comment(_) => !ignore_comments,
            Node::Element { .. } => true,
        })
        .collect()
}

pub fn text_matches(
    a: &str,
    b: &str,
) -> bool {
    if text_values_match(a, b) {
        return true;
    }
    eprintln!("Different text values: '{a}' and '{b}'");
    false
}

fn text_values_match(
    a: &str,
    b: &str,
) -> bool {
    let a: Vec<&str> = a.split_whitespace().collect();
    let b: Vec<&str> = b.split_whitespace().collect();
    // Upstream also matches dict literals regardless of key order; dicts are out of scope here
    a.len() == b.len()
        && a.iter().zip(&b).all(|(a, b)| {
            a == b
                || matches!((a.parse::<f64>(), b.parse::<f64>()), (Ok(a), Ok(b)) if (a - b).abs() <= 1e-9)
        })
}
