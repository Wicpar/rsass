//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/083_test_pseudoelement_goes_lefter_than_pseudoclass.hrx"

#[test]
#[ignore] // unexepected error
fn test() {
    assert_eq!(
        crate::rsass(
            ".foo::bar {a: b}\
            \n.baz:bang {@extend .foo}\
            \n"
        )
        .unwrap(),
        ".foo::bar, .baz:bang::bar {\
        \n  a: b;\
        \n}\
        \n"
    );
}
