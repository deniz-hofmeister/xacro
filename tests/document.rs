//! Ports of upstream document-level tests (comments, namespaces, unicode, file API) within the
//! minimal feature set.
//! Not ported: `test_target_namespace`, `test_target_namespace_only_from_root`
//! (`xacro:targetNamespace`), `test_broken_input_doesnt_create_empty_output_file`,
//! `test_create_subdirs`, `test_set_root_directory`, `test_unicode_file` (command line tool).
mod common;

use common::{assert_matches, test_dir, xacro};
use std::fs;

#[test]
fn test_ignore_xacro_comments() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <!-- A -->

  <!-- ignore multiline comments before any xacro tag -->
  <!-- ignored -->
  <xacro:property name="foo" value="1"/>
  <!-- ignored -->
  <xacro:if value="1"><!-- B --></xacro:if>
  <!-- ignored -->
  <xacro:macro name="foo"><!-- C --></xacro:macro>
  <!-- ignored -->
  <xacro:foo/>
</a>"#;
    assert_matches(&xacro(src), "<a><!-- A --><!-- B --><!-- C --></a>");
}

// Upstream passes `xacro_ns=False`, which the ROS 2 processor ignores: tags without the
// `xacro:` prefix are never processed
#[test]
fn test_enforce_xacro_ns() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <arg name="foo" value="bar"/>
  <include filename="foo"/>
</a>"#;
    let expected = r#"
<a>
  <arg name="foo" value="bar"/>
  <include filename="foo"/>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_unicode_literal_parsing() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">🍔 </a>"#;
    assert_matches(&xacro(src), "<a>🍔 </a>");
}

// Covers upstream `test_process_return_value` and `test_process_file_types`. The input is
// copied to a scratch directory in case the processor writes files next to it.
#[test]
fn test_process_file() {
    let dir = std::env::temp_dir().join(format!("xacro-test-process-file-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let input = dir.join("emoji.xacro");
    fs::copy(test_dir().join("emoji.xacro"), &input).unwrap();

    let result = xacro::process_file(&input);
    fs::remove_dir_all(&dir).unwrap();

    assert_matches(&result.unwrap(), "<robot>🍔</robot>");
}
