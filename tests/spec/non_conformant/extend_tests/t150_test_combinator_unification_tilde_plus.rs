//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/150_test_combinator_unification_tilde_plus.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".a ~ x {a: b}\
            \n.b + y {@extend x}\
            \n"
        )
        .unwrap(),
        ".a ~ x, .a ~ .b + y, .b.a + y {\
        \n  a: b;\
        \n}\
        \n"
    );
}
