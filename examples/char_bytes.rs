use mojilets::CharExt;

fn main() {
    assert_eq!('a'.bytes(),  "a".as_bytes());
    assert_eq!('の'.bytes(), "の".as_bytes());
}
