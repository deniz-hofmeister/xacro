//! Ports of upstream `${...}` expression and escaping tests within the minimal feature set.
//! Not ported: `test_message_functions`, `test_dotify` (`xacro.*` functions),
//! `test_comments` (`xacro:eval-comments`).
mod common;

use common::{assert_matches, assert_xacro_err, xacro};

#[test]
fn test_math_ignores_spaces() {
    assert_matches(
        &xacro(r#"<a><f v="${0.9 / 2 - 0.2}" /></a>"#),
        r#"<a><f v="0.25" /></a>"#,
    );
}

#[test]
fn test_math_expressions() {
    let src = r#"
<a xmlns:xacro="http://www.ros.org/wiki/xacro">
  <foo function="${1. + sin(pi)}"/>
</a>"#;
    let expected = r#"
<a>
  <foo function="1.0"/>
</a>"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_restricted_builtins() {
    assert_xacro_err(r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">${__import__('math')}</a>"#);
}

#[test]
fn test_restricted_builtins_nested() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">
<xacro:macro name="foo" params="arg">
  <xacro:property name="prop" value="${arg}"/>
  ${__import__('math')}
</xacro:macro>
<xacro:foo/>
</a>"#,
    );
}

#[test]
fn test_safe_eval() {
    assert_xacro_err(
        r#"<a xmlns:xacro="http://www.ros.org/wiki/xacro">${"".__class__.__base__.__subclasses__()}</a>"#,
    );
}

#[test]
fn test_escaping_dollar_braces() {
    let src = r#"<a b="$${foo}" c="$$${foo}" d="text $${foo}" e="text $$${foo}" f="$$(pwd)" />"#;
    let expected = r#"<a b="${foo}" c="$${foo}" d="text ${foo}" e="text $${foo}" f="$(pwd)" />"#;
    assert_matches(&xacro(src), expected);
}

#[test]
fn test_just_a_dollar_sign() {
    let src = r#"<a b="$" c="text $" d="text $ text"/>"#;
    assert_matches(&xacro(src), src);
}

#[test]
fn test_invalid_syntax() {
    for src in [
        "<a>a${</a>",
        "<a>${b</a>",
        "<a>${{}}</a>",
        "<a>a$(</a>",
        "<a>$(b</a>",
    ] {
        assert_xacro_err(src);
    }
}
