//! Ports of upstream `xacro:if` / `xacro:unless` tests within the minimal feature set.
//! Not ported: `test_default_property` (the only `unless` test, needs property `default`),
//! `test_unicode_conditional` (Python `u''` literals).
mod common;

use common::{assert_matches, assert_xacro_err, xacro};

#[test]
fn test_boolean_if_statement() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:if value="false">
    <a />
  </xacro:if>
  <xacro:if value="true">
    <b />
  </xacro:if>
</robot>"#;
    let expected = r#"
<robot>
    <b />
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_invalid_if_statement() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
                          <xacro:if value="nonsense"><foo/></xacro:if></a>"#,
    );
}

#[test]
fn test_integer_if_statement() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:if value="${0*42}">
    <a />
  </xacro:if>
  <xacro:if value="0">
    <b />
  </xacro:if>
  <xacro:if value="${0}">
    <c />
  </xacro:if>
  <xacro:if value="${1*2+3}">
    <d />
  </xacro:if>
</robot>"#;
    let expected = r#"
<robot>
    <d />
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_float_if_statement() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:if value="${3*0.0}">
    <a />
  </xacro:if>
  <xacro:if value="${3*0.1}">
    <b />
  </xacro:if>
</robot>"#;
    let expected = r#"
<robot>
    <b />
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_property_if_statement() {
    let src = r#"
<robot xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="condT" value="${True}"/>
  <xacro:property name="condF" value="${False}"/>
  <xacro:if value="${condF}"><a /></xacro:if>
  <xacro:if value="${condT}"><b /></xacro:if>
  <xacro:if value="${True}"><c /></xacro:if>
</robot>"#;
    let expected = r#"
<robot>
    <b /><c />
</robot>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_consecutive_if() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:if value="1"><xacro:if value="0"><a>bar</a></xacro:if></xacro:if>
</a>"#;
    assert_matches(&xacro(src), "<a/>");
}

#[test]
fn test_equality_expression_in_if_statement() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:property name="var" value="useit"/>
  <xacro:if value="${var == 'useit'}"><foo>bar</foo></xacro:if>
  <xacro:if value="${'use' in var}"><bar>foo</bar></xacro:if>
</a>"#;
    let expected = r#"
<a>
  <foo>bar</foo>
  <bar>foo</bar>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_consider_non_elements_if() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <xacro:if value="1"><!-- comment --> text <b>bar</b></xacro:if>
</a>"#;
    assert_matches(&xacro(src), "<a><!-- comment --> text <b>bar</b></a>");
}
