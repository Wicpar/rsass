//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/05_comma_list_quoted/06_escape_interpolation.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"alpha\", \'beta\';\
            \n.result {\
            \n  output: \"[\\#{\"alpha\", \'beta\'}]\";\
            \n  output: \"\\#{\"alpha\", \'beta\'}\";\
            \n  output: \'\\#{\"alpha\", \'beta\'}\';\
            \n  output: \"[\'\\#{\"alpha\", \'beta\'}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[#{\" alpha \", \'beta\'}]\";\
        \n  output: \"#{\" alpha \", \'beta\'}\";\
        \n  output: \'#{\"alpha\", \' beta \"}\";\
        \n  output: \"[\'#{\" alpha \", \'beta\'}\']\";\
        \n}\
        \n"
    );
}
