mod structs; use structs::*;


pub trait StringExt {
    /// Swap `n`th character and `m`th character (0-origin) in `string`
    /// 
    /// - Panics if `m` or `n` is equal to or larger than the number of characters in `string`
    fn swap_chars(&mut self, m: usize, n: usize);
} impl StringExt for String {
    fn swap_chars(&mut self, m: usize, n: usize) {
        let (pos_m, char_m) = self.char_indices().nth(m).unwrap();
        let (pos_n, char_n) = self.char_indices().nth(n).unwrap();
        let (len_m, len_n) = (char_m.len_utf8(), char_n.len_utf8());
        
        let bytes = unsafe {self.as_mut_vec()};

        for i in 0..len_m.min(len_n) {
            bytes.swap(pos_m+i, pos_n+i)
        }

        if len_m > len_n {
            bytes[(pos_m+len_n)..(pos_n+len_n)].rotate_left(len_m-len_n)
        } else if len_m < len_n {
            bytes[(pos_m+len_m)..(pos_n+len_n)].rotate_right(len_n-len_m)
        }
    }
}

pub trait CharExt {
    fn bytes(self) -> CharBytes;
} impl CharExt for char {
    fn bytes(self) -> CharBytes {
        CharBytes::from(self)
    }
}
