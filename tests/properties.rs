//! Ports of upstream property tests within the minimal feature set.
//! Not ported: `test_double_underscore_property_name_raises` (guards Python dunder access),
//! `test_property_scope_parent`, `test_property_scope_global`, `test_property_scope_parent_namespaced`
//! (`scope` attribute), `test_property_in_comprehension`, `test_iterable_literals_eval`,
//! `test_greedy_property_evaluation` (`lazy_eval`, string methods), `test_default_property`,
//! `test_invalid_property_definitions`, `test_remove_property` (`default`/`remove` attributes),
//! `test_overwrite_globals`, `test_redefine_global_symbol` (warning output),
//! `test_unicode_conditional` (Python `u''` literals).
mod common;

use common::{assert_matches, assert_xacro_err, xacro};

#[test]
fn test_invalid_property_name() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
        <xacro:property name="invalid.name"/></a>"#,
    );
}

#[test]
fn test_property_replacement() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="foo" value="42" />
  <the_foo result="${foo}" />
</a>"#;
    assert_matches(&xacro(src), r#"<a><the_foo result="42"/></a>"#);
}

#[test]
fn test_inorder_processing() {
    let src = r#"
<xml xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="foo" value="1.0"/>
  <xacro:macro name="m" params="foo"><a foo="${foo}"/></xacro:macro>
  <xacro:m foo="1 ${foo}"/>
  <!-- now redefining the property and macro -->
  <xacro:property name="foo" value="2.0"/>
  <xacro:macro name="m" params="foo"><b bar="${foo}"/></xacro:macro>
  <xacro:m foo="2 ${foo}"/>
</xml>"#;
    let expected = r#"
<xml>
  <a foo="1 1.0"/>
  <b bar="2 2.0"/>
</xml>
"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_insert_block_property() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="bar">bar</xacro:macro>
<xacro:property name="val" value="2" />
<xacro:property name="some_block">
  <some_block attr="${val}"><xacro:bar/></some_block>
</xacro:property>
<foo>
  <xacro:insert_block name="some_block" />
</foo>
</a>"#;
    let expected = r#"
<a>
<foo><some_block attr="2">bar</some_block></foo>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_no_evaluation() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="xyz" value="5 -2"/>
  <foo>${xyz}</foo>
</a>"#;
    let expected = r#"
<a>
  <foo>5 -2</foo>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_recursive_evaluation() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a" value=" 42 "/>
  <xacro:property name="a2" value="${ 2 * a }"/>
  <a doubled="${a2}"/>
</robot>"#;
    let expected = r#"
<robot>
  <a doubled="84"/>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_recursive_evaluation_wrong_order() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a2" value="${2*a}"/>
  <xacro:property name="a" value="42"/>
  <a doubled="${a2}"/>
</robot>"#;
    let expected = r#"
<robot>
  <a doubled="84"/>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_recursive_definition() {
    assert_xacro_err(
        r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a" value="${a2}"/>
  <xacro:property name="a2" value="${2*a}"/>
  <a doubled="${a2}"/>
</robot>"#,
    );
}

#[test]
fn test_multiple_recursive_evaluation() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a" value="1"/>
  <xacro:property name="b" value="2"/>
  <xacro:property name="c" value="3"/>
  <xacro:property name="product" value="${a*b*c}"/>
  <answer product="${product}"/>
</robot>"#;
    let expected = r#"
<robot>
  <answer product="6"/>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_multiple_definition_and_evaluation() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a" value="42"/>
  <xacro:property name="b" value="${a}"/>
  <xacro:property name="b" value="${-a}"/>
  <xacro:property name="b" value="${a}"/>
  <answer b="${b} ${b} ${b}"/>
</robot>"#;
    let expected = r#"
<robot>
  <answer b="42 42 42"/>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_transitive_evaluation() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a" value="42"/>
  <xacro:property name="b" value="${a}"/>
  <xacro:property name="c" value="${b}"/>
  <xacro:property name="d" value="${c}"/>
  <answer d="${d}"/>
</robot>"#;
    let expected = r#"
<robot>
  <answer d="42"/>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_multi_tree_evaluation() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="a" value="42"/>
  <xacro:property name="b" value="2.1"/>
  <xacro:property name="c" value="${a}"/>
  <xacro:property name="d" value="${b}"/>
  <xacro:property name="f" value="${c*d}"/>
  <answer f="${f}"/>
</robot>"#;
    let expected = r#"
<robot>
  <answer f="88.2"/>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_from_issue() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="x" value="42"/>
  <xacro:property name="wheel_width" value="${x}"/>
  <link name="my_link">
    <origin xyz="0 0 ${wheel_width/2}"/>
  </link>
</robot>"#;
    let expected = r#"
<robot>
  <link name="my_link">
    <origin xyz="0 0 21.0"/>
  </link>
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_recursive_bad_math() {
    assert_xacro_err(
        r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="x" value="0"/>
  <tag badness="${1/x}"/>
</robot>"#,
    );
}

#[test]
fn test_iterable_literals_plain() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="l" value="[0, 1+1, 2]"/>
  <xacro:property name="t" value="(0,1+1,2)"/>
  <xacro:property name="d" value="{'a':0, 'b':1+1, 'c':2}"/>
  <a list="${l}" tuple="${t}" dict="${d}"/>
</a>"#;
    let expected = r#"
<a>
  <a list="[0, 1+1, 2]" tuple="(0,1+1,2)" dict="{'a':0, 'b':1+1, 'c':2}"/>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_literals_eval() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="f" value="1.23"/>
  <xacro:property name="i" value="123"/>
  <xacro:property name="s" value="1_2_3"/>
  float=${f+1} int=${i+1} string=${s}
</a>"#;
    let expected = r#"
<a>
  float=2.23 int=124 string=1_2_3
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_unicode_property() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:property name="burger" value="🍔"/>
${burger}</a>"#;
    assert_matches(&xacro(src), "<a>🍔</a>");
}

#[test]
fn test_unicode_property_attribute() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:property name="burger" value="🍔"/>
<b c="${burger}"/></a>"#;
    assert_matches(&xacro(src), r#"<a><b c="🍔"/></a>"#);
}

#[test]
fn test_unicode_property_block() {
    let src = r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:property name="burger">
🍔
</xacro:property>
<xacro:insert_block name="burger"/></a>"#;
    assert_matches(&xacro(src), "<a>🍔</a>");
}
