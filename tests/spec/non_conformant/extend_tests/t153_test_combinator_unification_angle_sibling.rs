//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/153_test_combinator_unification_angle_sibling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("153_test_combinator_unification_angle_sibling")
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a > x {a: b}\
             \n.b + y {@extend x}\n"),
        ".a > x, .a > .b + y {\
         \n  a: b;\
         \n}\n"
    );
}
