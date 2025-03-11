use goats;
use std::fs;
use std::io;

#[test]
fn test_should_generate_types_from_golang_code() {
    let expect = fs::read_to_string("tests/testdata/want_simple.d.ts").expect("failed to read want_simple_types.d.ts");
    let golang = fs::read_to_string("tests/testdata/in_simple.go").expect("failed to read simple.go");

    let actual = goats::generate(io::Cursor::new(golang)).unwrap();
    assert_eq!(actual, expect);
}
