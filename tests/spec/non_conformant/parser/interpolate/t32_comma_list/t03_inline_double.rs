//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/32_comma_list/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\"[\"\',foo,   \'\"]\"}};\
            \n  output: #{\"[#{\"[\"\',foo,   \'\"]\"}]\"};\
            \n  output: #{\"#{\"[\"\',foo,   \'\"]\"}\"};\
            \n  output: #{\'#{\"[\"\',foo,   \'\"]\"}\'};\
            \n  output: #{\"[\'#{\"[\"\',foo,   \'\"]\"}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: [ ,foo,    ];\
        \n  output: [[ ,foo,    ]];\
        \n  output: [ ,foo,    ];\
        \n  output: [ ,foo,    ];\
        \n  output: [\'[ ,foo,    ]\'];\
        \n}\
        \n"
    );
}
