#![allow(non_camel_case_types)]

const TAG_TWO_B:   u8 = 0xC0;
const TAG_CONT:    u8 = 0x80;
const TAG_THREE_B: u8 = 0xE0;
const TAG_FOUR_B:  u8 = 0xF0;


pub struct CharBytes(UTF8CharBytes);
enum UTF8CharBytes {
    one   ([u8; 1]),
    two   ([u8; 2]),
    three ([u8; 3]),
    four  ([u8; 4]),
}
const _: () = {
    impl From<char> for CharBytes {
        /* ref: `encode_utf8_raw` in library/core/src/char/method.rs */
        #[inline] fn from(c: char) -> Self {
            let code = c as u32;
            match c.len_utf8() {
                1 => Self(UTF8CharBytes::one([
                    code as u8,
                ])),
                2 => Self(UTF8CharBytes::two([
                    (code >> 6 & 0x1F) as u8 | TAG_TWO_B,
                    (code & 0x3F) as u8 | TAG_CONT,
                ])),
                3 => Self(UTF8CharBytes::three([
                    (code >> 12 & 0x0F) as u8 | TAG_THREE_B,
                    (code >> 6 & 0x3F) as u8 | TAG_CONT,
                    (code & 0x3F) as u8 | TAG_CONT,
                ])),
                4 => Self(UTF8CharBytes::four([
                    (code >> 18 & 0x07) as u8 | TAG_FOUR_B,
                    (code >> 12 & 0x3F) as u8 | TAG_CONT,
                    (code >> 6 & 0x3F) as u8 | TAG_CONT,
                    (code & 0x3F) as u8 | TAG_CONT,
                ])),
                _ => unsafe {std::hint::unreachable_unchecked()}
            }
        }
    }
    impl AsRef<[u8]> for CharBytes {
        #[inline] fn as_ref(&self) -> &[u8] {
            match &self.0 {
                UTF8CharBytes::one   (bytes) => &bytes[..],
                UTF8CharBytes::two   (bytes) => &bytes[..],
                UTF8CharBytes::three (bytes) => &bytes[..],
                UTF8CharBytes::four  (bytes) => &bytes[..],
            }
        }
    }
    impl std::ops::Deref for CharBytes {
        type Target = [u8];
        #[inline] fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }
    impl std::borrow::Borrow<[u8]> for CharBytes {
        #[inline] fn borrow(&self) -> &[u8] {
            self.as_ref()
        }
    }
};
