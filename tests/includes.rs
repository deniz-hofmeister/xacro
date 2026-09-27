//! Ports of upstream `xacro:include` and `$(find)` tests within the minimal feature set.
//! Not ported: `test_include_glob` (glob patterns), `test_include_with_namespace`,
//! `test_namespace_propagation`, `test_property_resolution_with_namespaced_include` (`ns`),
//! `test_xacro_exist_optional` (`optional`), `test_include_from_macro` (`$(cwd)`),
//! `test_xml_namespace_lifting`, `test_yaml_*` (`load_yaml`).
mod common;

use common::{assert_matches, assert_xacro_err, test_dir, xacro};
use std::{fs, path::Path};

#[test]
fn test_include() {
    let src =
        r#"<a xmlns:xacro="http://www.ros.org/xacro"><xacro:include filename="include1.xml"/></a>"#;
    assert_matches(&xacro(src), "<a><inc1/></a>");
}

#[test]
fn test_include_nonexistent() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/xacro">
                             <xacro:include filename="include-nada.xml" /></a>"#,
    );
}

// Upstream additionally checks that no deprecation warning is printed
#[test]
fn test_include_deprecated() {
    let src = r#"<a><include filename="nada"><tag/></include></a>"#;
    assert_matches(&xacro(src), src);
}

#[test]
fn test_include_from_variable() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/xacro">
        <xacro:property name="file" value="include1.xml"/>
        <xacro:include filename="${file}" /></a>"#;
    assert_matches(&xacro(src), "<a><inc1/></a>");
}

// The fixture drops upstream's third include, `$(cwd)/include1.xml`, and with it the trailing <inc1/>
#[test]
fn test_include_recursive() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/xacro">
    <xacro:include filename="include1.xml"/>
    <xacro:include filename="./include1.xml"/>
    <xacro:include filename="subdir/include-recursive.xacro"/>
</a>"#;
    let expected = r#"
<a><inc1/><inc1/><subdir_inc1/><subdir_inc1/></a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_include_lazy() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/xacro">
        <xacro:if value="false"><xacro:include filename="non-existent"/></xacro:if></a>"#;
    assert_matches(&xacro(src), "<a/>");
}

#[test]
fn test_issue_63_fixed_with_inorder_processing() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:arg name="has_stuff" default="false"/>
  <xacro:if value="$(arg has_stuff)">
    <xacro:include file="$(find nonexistent_package)/stuff.urdf" />
  </xacro:if>
</a>"#;
    assert_matches(&xacro(src), "<a/>");
}

// Upstream additionally checks the file stack recorded for error reporting
#[test]
fn test_broken_include_error_reporting() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
           <xacro:include filename="broken.xacro"/></a>"#,
    );
}

#[test]
fn test_xacro_exist_required() {
    assert_xacro_err(
        r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:include filename="non-existent.xacro"/>
</a>"#,
    );
}

#[test]
fn test_substitution_args_find() {
    let output = xacro("<a>$(find xacro)/test/include1.xml</a>");
    let resolved = xml::reader::EventReader::new(output.as_bytes())
        .into_iter()
        .find_map(|event| match event {
            Ok(xml::reader::XmlEvent::Characters(text)) => Some(text),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no text in output: {output}"));
    assert_eq!(
        fs::canonicalize(Path::new(resolved.trim())).unwrap(),
        fs::canonicalize(test_dir().join("include1.xml")).unwrap()
    );
}
