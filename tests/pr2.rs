//! Port of upstream `test_pr2`: the PR2 robot description snapshot must expand to the golden
//! output, ignoring comments.
mod common;

use common::{assert_matches_ignoring_comments, test_dir, xacro};
use std::fs;

#[test]
fn test_pr2() {
    let pr2_dir = test_dir().join("robots/pr2");
    let src = fs::read_to_string(pr2_dir.join("pr2.urdf.xacro")).unwrap();
    let golden = fs::read_to_string(pr2_dir.join("pr2_1.11.4.xml")).unwrap();
    assert_matches_ignoring_comments(&xacro(&src), &golden);
}
