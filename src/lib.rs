mod structs; use structs::*;

/// Swap `n`th character and `m`th character (0-origin) in `string`
/// 
/// - Panics if `m` or `n` is equal to or larger than the number of characters in `string`
pub fn swap_chars(string: &mut String, m: usize, n: usize) {
    let (pos_m, char_m) = string.char_indices().nth(m).unwrap();
    let (pos_n, char_n) = string.char_indices().nth(n).unwrap();
    let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
    
    let bytes = unsafe {string.as_mut_vec()};

    for i in 0..len_m.min(len_n) {
        bytes.swap(pos_m+i, pos_n+i)
    }

    if len_m > len_n {
        bytes[(pos_m+len_n)..(pos_n+len_n)].rotate_left(len_m-len_n)
    } else if len_m < len_n {
        bytes[(pos_m+len_m)..(pos_n+len_n)].rotate_right(len_n-len_m)
    }
}

/// Get `CharBytes` from `ch`. `CharBytes` implments
/// 
/// - `AsRef<[u8]>`
/// - `Borrow<[u8]>`
/// - `Deref<[u8]>`
#[inline] pub fn char_bytes(ch: char) -> CharBytes {
    CharBytes::from(ch)
}
