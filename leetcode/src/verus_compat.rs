//! Plain-Rust stand-ins for the few `vstd` methods some solutions call
//! (`str::unicode_len`, `str::get_char`, `Vec::set`), so they compile unchanged.

pub trait StrCompat {
    fn unicode_len(&self) -> usize;
    fn get_char(&self, i: usize) -> char;
}

impl StrCompat for str {
    fn unicode_len(&self) -> usize {
        self.chars().count()
    }

    fn get_char(&self, i: usize) -> char {
        self.chars().nth(i).unwrap()
    }
}

pub trait VecCompat<T> {
    fn set(&mut self, i: usize, value: T);
}

impl<T> VecCompat<T> for Vec<T> {
    fn set(&mut self, i: usize, value: T) {
        self[i] = value;
    }
}
