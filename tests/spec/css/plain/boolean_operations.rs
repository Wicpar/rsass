//! Tests auto-converted from "sass-spec/spec/css/plain/boolean_operations.hrx"

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        crate::rsass(
            "@import \"plain\";\
            \n"
        )
        .unwrap(),
        "a {\
        \n  and: true and false;\
        \n  or: true or false;\
        \n  not: not true;\
        \n}\
        \n"
    );
}
