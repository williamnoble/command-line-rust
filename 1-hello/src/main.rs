fn main() {
    println!("{}", say_hello());
}


fn say_hello() -> String {
    "Hello, William!".to_string()
}

#[test]
fn test_hello() {
    assert_eq!(say_hello(), "Hello, William!")
}

