//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/14_escapes_literal_numbers/03_inline_double.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            ".result {\
            \n  output: #{#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}};\
            \n  output: #{\"[#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\"};\
            \n  output: #{\"#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\"};\
            \n  output: #{\'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\'};\
            \n  output: #{\"[\'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\"};\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
        \n  output: [\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ];\
        \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
        \n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\
        \n  output: [\'\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 \'];\
        \n}\
        \n"
    );
}
