fn main() {
    println!("{}", print_hello());
}

fn print_hello() -> &'static str {
    return "Hello, world!";
}

#[test]
fn check_print_hello() {
    assert_eq!(print_hello(), "Hello, world!");
}