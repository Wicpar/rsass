//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/174_test_combinator_unification_plus_space.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a x {a: b}\
            \n.a.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a x, .a .a.b + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
