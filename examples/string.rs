use mojilets::StringExt;

fn main() {
    let mut s = String::from("Hello, 文字列!");
    s.swap_chars(5, 9);
    assert_eq!(s, "Hello列 文字,!");
}