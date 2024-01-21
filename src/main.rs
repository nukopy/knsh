fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    // test content: add(1, 2) == 3
    // given (prerequisites):
    let expected = 3;

    // when (operation):
    let actual = add(1, 2);

    // then (expected results):
    assert_eq!(actual, expected, "1 + 2 should be 3");
}
