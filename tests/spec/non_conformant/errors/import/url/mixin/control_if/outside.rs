//! Tests auto-converted from "sass-spec/spec/non_conformant/errors/import/url/mixin/control-if/outside.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "@mixin do_import() {\r\
            \n  @import url(\"http://www.libsass.org\");\r\
            \n}\r\
            \n\r\
            \n@if (true) {\r\
            \n  @include do_import();\r\
            \n}"
        )
        .unwrap(),
        "@import url(\"http://www.libsass.org\");\
        \n"
    );
}
