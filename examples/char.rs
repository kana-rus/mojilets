use mojilets::CharExt;

fn main() {
    let char_bytes: _   = 'a'.bytes();
    let as_bytes: &[u8] = char_bytes.as_bytes();

    assert_eq!(char_bytes,   "a".as_bytes());
    assert_eq!(as_bytes,     "a".as_bytes());
    assert_eq!('の'.bytes(), "の".as_bytes());
}
