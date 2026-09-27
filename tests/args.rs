//! Ports of upstream `xacro:arg` / `$(arg)` tests within the minimal feature set.
//! Not ported: `test_arg_function` (`xacro.arg()`), `test_dynamic_arg_default`,
//! `test_dynamic_arg_specified`, `test_expression_in_extension` (`${}` inside `$()`),
//! `test_extension_in_expression` (string repetition).
mod common;

use common::{assert_matches, assert_xacro_err, cli, xacro, xacro_with};

#[test]
fn test_substitution_args_arg() {
    let output = xacro_with(
        r#"<a><f v="$(arg sub_arg)" /></a>"#,
        cli(&["sub_arg:=my_arg"]),
    );
    assert_matches(&output, r#"<a><f v="my_arg" /></a>"#);
}

#[test]
fn test_default_arg() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:arg name="foo" default="2"/>
  <link name="my_link">
    <origin xyz="0 0 $(arg foo)"/>
  </link>
</robot>
"#;
    let expected = r#"
<robot>
  <link name="my_link">
    <origin xyz="0 0 2"/>
  </link>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_default_arg_override() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:arg name="foo" default="2"/>
  <link name="my_link">
    <origin xyz="0 0 $(arg foo)"/>
  </link>
</robot>
"#;
    let expected = r#"
<robot>
  <link name="my_link">
    <origin xyz="0 0 4"/>
  </link>
</robot>"#;
    assert_matches(&xacro_with(src, cli(&["foo:=4"])), expected);
}

#[test]
fn test_default_arg_missing() {
    assert_xacro_err(
        r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <a arg="$(arg foo)"/>
</a>
"#,
    );
}

#[test]
fn test_default_arg_empty() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:arg name="foo" default=""/>$(arg foo)</a>"#;
    assert_matches(&xacro(src), "<a/>");
}

#[test]
fn test_issue_68_numeric_arg() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:arg name="foo" default="0.5"/>
  <xacro:property name="prop" value="$(arg foo)" />
  <a prop="${prop-0.3}"/>
</a>
"#;
    let expected = r#"
<a>
  <a prop="0.2"/>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_transitive_arg_evaluation() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:arg name="foo" default="0.5"/>
  <xacro:arg name="bar" default="$(arg foo)"/>
  <xacro:property name="prop" value="$(arg bar)" />
  <a prop="${prop-0.3}"/>
</a>
"#;
    let expected = r#"
<a>
  <a prop="0.2"/>
</a>"#;
    assert_matches(&xacro(src), expected);
}
