//! Tests auto-converted from "sass-spec/spec/core_functions/string/slice/start.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("start")
}

mod negative {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn t1() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", -1)}\n"),
            "a {\
         \n  b: \"e\";\
         \n}\n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", -2)}\n"),
            "a {\
         \n  b: \"de\";\
         \n}\n"
        );
    }
    #[test]
    fn after_last() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", -100)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", -3)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
}
mod positive {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn t0() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", 0)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
    #[test]
    fn t1() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", 1)}\n"),
            "a {\
         \n  b: \"cde\";\
         \n}\n"
        );
    }
    #[test]
    fn t2() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", 2)}\n"),
            "a {\
         \n  b: \"de\";\
         \n}\n"
        );
    }
    #[test]
    fn after_end() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cdef\", 3, 2)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn after_last() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", 100)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
    #[test]
    fn last() {
        assert_eq!(
            runner().ok("a {b: str-slice(\"cde\", 4)}\n"),
            "a {\
         \n  b: \"\";\
         \n}\n"
        );
    }
}
