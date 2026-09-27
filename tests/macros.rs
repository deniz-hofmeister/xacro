//! Ports of upstream macro tests within the minimal feature set.
//! Not ported: `test_dynamic_macro_names`, `test_dynamic_macro_name_clash`,
//! `test_dynamic_macro_undefined` (`xacro:call`), `test_xacro_element`, `test_xacro_attribute`,
//! `test_evaluate_macro_params_before_body` (lists), `test_property_forwarding` (`^` params),
//! `test_macro_name_with_colon` (warning output), `test_unicode_macro` (Python `u''` literals),
//! `test_include_from_macro` (`$(cwd)`, `xacro.abs_filename`),
//! `test_error_reporting`, `test_print_location` (macro call stack output).
mod common;

use common::{assert_matches, assert_xacro_err, xacro};

#[test]
fn test_macro_undefined() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
                          <xacro:undefined><foo/><bar/></xacro:undefined></a>"#,
    );
}

#[test]
fn test_should_replace_before_macroexpand() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="inner" params="*the_block">
  <in_the_inner><xacro:insert_block name="the_block" /></in_the_inner>
</xacro:macro>
<xacro:macro name="outer" params="*the_block">
  <in_the_outer><xacro:inner><xacro:insert_block name="the_block" /></xacro:inner></in_the_outer>
</xacro:macro>
<xacro:outer><woot /></xacro:outer></a>"#;
    let expected = r#"<a><in_the_outer><in_the_inner><woot /></in_the_inner></in_the_outer></a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_macro_params_escaped_string() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
    <xacro:macro name="foo" params="a='1 -2' c=3"><bar a="${a}" c="${c}"/></xacro:macro>
    <xacro:foo/></a>"#;
    assert_matches(&xacro(src), r#"<a><bar a="1 -2" c="3"/></a>"#);
}

#[test]
fn test_multiple_insert_blocks() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="foo" params="*block">
  <xacro:insert_block name="block" />
  <xacro:insert_block name="block" />
</xacro:macro>
<xacro:foo>
  <a_block />
</xacro:foo>
</a>"#;
    let expected = r#"<a>
  <a_block />
  <a_block />
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_multiple_blocks() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="foo" params="*block{A} *block{B}">
  <xacro:insert_block name="block1" />
  <xacro:insert_block name="block2" />
</xacro:macro>
<xacro:foo>
  <block1/>
  <block2/>
</xacro:foo>
</a>"#;
    let expected = r#"<a>
<block{A}/>
<block{B}/>
</a>"#;
    // test both, reversal and non-reversal of block order
    for (a, b) in [("1", "2"), ("2", "1")] {
        let fill = |s: &str| s.replace("{A}", a).replace("{B}", b);
        assert_matches(&xacro(&fill(src)), &fill(expected));
    }
}

#[test]
fn test_integer_stays_integer() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="m" params="num">
  <test number="${num}" />
</xacro:macro>
<xacro:m num="100" />
</a>"#;
    let expected = r#"
<a>
  <test number="100" />
</a>"#;
    assert_matches(&xacro(src), expected);
}

// Upstream defines the expected result but never asserts it
#[test]
fn test_macro_has_new_scope() {
    let src = r#"<root xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="prop" value="outer"/>
  <xacro:macro name="foo">
    <outer prop="${prop}"/>
    <xacro:property name="prop" value="inner"/>
    <xacro:macro name="foo">
      <inner prop="${prop}"/>
    </xacro:macro>
    <xacro:foo/>
  </xacro:macro>
  <xacro:foo/>
  <xacro:foo/>
</root>"#;
    let expected = r#"<root>
  <outer prop="outer"/>
  <inner prop="inner"/>
  <outer prop="outer"/>
  <inner prop="inner"/>
</root>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_consider_non_elements_block() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="foo" params="*block">
  <!-- comment -->
  foo
  <xacro:insert_block name="block" />
</xacro:macro>
<xacro:foo>
  <!-- ignored comment -->
  ignored text
  <a_block />
</xacro:foo>
</a>"#;
    let expected = r#"
<a>
  <!-- comment -->
  foo
  <a_block />
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_default_param() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:macro name="fixed_link" params="parent_link:=base_link child_link *joint_pose">
    <link name="${child_link}"/>
    <joint name="${child_link}_joint" type="fixed">
      <xacro:insert_block name="joint_pose" />
      <parent link="${parent_link}"/>
      <child link="${child_link}" />
    </joint>
  </xacro:macro>
  <xacro:fixed_link child_link="foo">
    <origin xyz="0 0 0" rpy="0 0 0" />
  </xacro:fixed_link >
</robot>"#;
    let expected = r#"
<robot>
  <link name="foo"/>
  <joint name="foo_joint" type="fixed">
    <origin rpy="0 0 0" xyz="0 0 0"/>
    <parent link="base_link"/>
    <child link="foo"/>
  </joint>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_default_param_override() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:macro name="fixed_link" params="parent_link:=base_link child_link *joint_pose">
    <link name="${child_link}"/>
    <joint name="${child_link}_joint" type="fixed">
      <xacro:insert_block name="joint_pose" />
      <parent link="${parent_link}"/>
      <child link="${child_link}" />
    </joint>
  </xacro:macro>
  <xacro:fixed_link child_link="foo" parent_link="bar">
    <origin xyz="0 0 0" rpy="0 0 0" />
  </xacro:fixed_link >
</robot>"#;
    let expected = r#"
<robot>
  <link name="foo"/>
  <joint name="foo_joint" type="fixed">
    <origin rpy="0 0 0" xyz="0 0 0"/>
    <parent link="bar"/>
    <child link="foo"/>
  </joint>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_param_missing() {
    assert_xacro_err(
        r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:macro name="fixed_link" params="parent_link child_link *joint_pose">
    <link name="${child_link}"/>
    <joint name="${child_link}_joint" type="fixed">
      <xacro:insert_block name="joint_pose" />
      <parent link="${parent_link}"/>
      <child link="${child_link}" />
    </joint>
  </xacro:macro>
  <xacro:fixed_link child_link="foo">
    <origin xyz="0 0 0" rpy="0 0 0" />
  </xacro:fixed_link >
</robot>"#,
    );
}

#[test]
fn test_no_double_evaluation() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/xacro">
  <xacro:macro name="foo" params="a b:=${a} c:=$${a}"> a=${a} b=${b} c=${c} </xacro:macro>
  <xacro:property name="a" value="1"/>
  <xacro:property name="d" value="$${a}"/>
  <d d="${d}"><xacro:foo a="2"/></d>
</a>"#;
    assert_matches(&xacro(src), r#"<a><d d="${a}"> a=2 b=1 c=${a} </d></a>"#);
}

#[test]
fn test_macro_default_param_evaluation_order() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="foo" params="arg:=${2*foo}">
    <xacro:property name="foo" value="-"/>
    <f val="${arg}"/>
</xacro:macro>
<xacro:property name="foo" value="${3*7}"/>
<xacro:foo/>
<xacro:property name="foo" value="*"/>
<xacro:foo/>
</a>"#;
    let expected = r#"<a>
<f val="42"/><f val="**"/></a>"#;
    assert_matches(&xacro(src), expected);
}
