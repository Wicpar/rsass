//! Tests auto-converted from "sass-spec/spec/core_functions/meta/inspect/list/single.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("single")
}

mod bracketed {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn comma() {
        assert_eq!(
            runner().ok("$result: inspect([1,]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: [1,];\
         \n  type: string;\
         \n}\n"
        );
    }
    #[test]
    fn undecided() {
        assert_eq!(
            runner().ok("$result: inspect([1]);\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
            "a {\
         \n  value: [1];\
         \n  type: string;\
         \n}\n"
        );
    }
}
#[test]
fn comma() {
    assert_eq!(
        runner().ok("$result: inspect((1,));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: (1,);\
         \n  type: string;\
         \n}\n"
    );
}
#[test]
fn slash() {
    assert_eq!(
        runner().ok("$result: inspect(append((), 1, slash));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: (1/);\
         \n  type: string;\
         \n}\n"
    );
}
#[test]
fn space() {
    assert_eq!(
        runner().ok("$result: inspect(append((), 1, space));\
             \na {\
             \n  value: $result;\
             \n  type: type-of($result);\
             \n}\n"),
        "a {\
         \n  value: 1;\
         \n  type: string;\
         \n}\n"
    );
}
