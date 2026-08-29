use ropey::{Rope, RopeSlice};
use std::io;

/// A text buffer backed by a rope data structure.
///
/// Provides O(log n) insert/delete operations suitable for editing
/// multi-gigabyte files without full-buffer copies.
///
/// `Buffer` knows nothing about cursors, viewports, or UI concerns.
/// Those are handled by [`Cursor`](crate::cursor::Cursor) and
/// [`View`](crate::view::View) respectively.
#[derive(Debug, Clone)]
pub struct Buffer {
    rope: Rope,
    modified: bool,
}

impl Buffer {
    /// Create a new, empty buffer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            modified: false,
        }
    }

    /// Create a buffer from a string.
    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            modified: false,
        }
    }

    /// Create a buffer from a reader.
    ///
    /// # Errors
    ///
    /// Returns an error if reading from the reader fails.
    pub fn from_reader<R: io::Read>(reader: R) -> io::Result<Self> {
        Ok(Self {
            rope: Rope::from_reader(reader)?,
            modified: false,
        })
    }

    /// Insert text at the specified character index.
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        self.rope.insert(char_idx, text);
        self.modified = true;
    }

    /// Delete text in the range [start, end) and return the deleted text.
    pub fn delete(&mut self, start: usize, end: usize) -> String {
        let deleted = self.rope.slice(start..end).to_string();
        self.rope.remove(start..end);
        self.modified = true;
        deleted
    }

    /// Get the character at the given index.
    #[must_use]
    pub fn char_at(&self, char_idx: usize) -> char {
        self.rope.char(char_idx)
    }

    /// Get a slice representing the specified line (0-indexed).
    #[must_use]
    pub fn line(&self, line_idx: usize) -> RopeSlice<'_> {
        self.rope.line(line_idx)
    }

    /// Get a slice of characters in the range [start, end).
    #[must_use]
    pub fn slice(&self, start: usize, end: usize) -> RopeSlice<'_> {
        self.rope.slice(start..end)
    }

    /// Returns the length of the buffer in characters.
    #[must_use]
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Returns the length of the buffer in lines.
    #[must_use]
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns the length of the buffer in bytes.
    #[must_use]
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Returns true if the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    /// Returns true if the buffer has been modified.
    #[must_use]
    pub const fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark the buffer as saved, clearing the modified flag.
    pub const fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Convert a character index to a line index.
    #[must_use]
    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.rope.char_to_line(char_idx)
    }

    /// Convert a line index to the character index at the start of the line.
    #[must_use]
    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    /// Write the buffer content to a writer.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer fails.
    pub fn write_to<W: io::Write>(&self, writer: W) -> io::Result<()> {
        self.rope.write_to(writer)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor as IoCursor;

    #[test]
    fn new_buffer_is_empty() {
        let b = Buffer::new();
        assert!(b.is_empty());
        assert_eq!(b.len_chars(), 0);
        assert!(!b.is_modified());
    }

    #[test]
    fn from_str_creates_correct_buffer() {
        let b = Buffer::from_str("hello\nworld");
        assert_eq!(b.len_chars(), 11);
        assert_eq!(b.len_lines(), 2);
        assert!(!b.is_modified());
    }

    #[test]
    fn insert_at_beginning() {
        let mut b = Buffer::from_str("world");
        b.insert(0, "hello ");
        assert_eq!(b.to_string(), "hello world");
        assert!(b.is_modified());
    }

    #[test]
    fn insert_at_end() {
        let mut b = Buffer::from_str("hello");
        b.insert(5, " world");
        assert_eq!(b.to_string(), "hello world");
    }

    #[test]
    fn insert_in_middle() {
        let mut b = Buffer::from_str("helloworld");
        b.insert(5, " ");
        assert_eq!(b.to_string(), "hello world");
    }

    #[test]
    fn delete_range() {
        let mut b = Buffer::from_str("hello world");
        let deleted = b.delete(5, 11);
        assert_eq!(deleted, " world");
        assert_eq!(b.to_string(), "hello");
        assert!(b.is_modified());
    }

    #[test]
    fn delete_at_beginning() {
        let mut b = Buffer::from_str("hello world");
        let deleted = b.delete(0, 6);
        assert_eq!(deleted, "hello ");
        assert_eq!(b.to_string(), "world");
    }

    #[test]
    fn char_at_various_positions() {
        let b = Buffer::from_str("abc");
        assert_eq!(b.char_at(0), 'a');
        assert_eq!(b.char_at(1), 'b');
        assert_eq!(b.char_at(2), 'c');
    }

    #[test]
    fn line_access() {
        let b = Buffer::from_str("line1\nline2\nline3");
        assert_eq!(b.line(0).as_str(), Some("line1\n"));
        assert_eq!(b.line(1).as_str(), Some("line2\n"));
        assert_eq!(b.line(2).as_str(), Some("line3"));
        assert_eq!(b.len_lines(), 3);
    }

    #[test]
    fn coordinate_conversion() {
        let b = Buffer::from_str("line1\nline2");
        assert_eq!(b.char_to_line(0), 0); // 'l'
        assert_eq!(b.char_to_line(5), 0); // '\n'
        assert_eq!(b.char_to_line(6), 1); // 'l' in line2
        assert_eq!(b.line_to_char(0), 0);
        assert_eq!(b.line_to_char(1), 6);
    }

    #[test]
    fn mark_saved_clears_modified_flag() {
        let mut b = Buffer::new();
        b.insert(0, "a");
        assert!(b.is_modified());
        b.mark_saved();
        assert!(!b.is_modified());
    }

    #[test]
    fn slice_access() {
        let b = Buffer::from_str("hello world");
        assert_eq!(b.slice(0, 5).as_str(), Some("hello"));
    }

    #[test]
    fn unicode_support() {
        let b = Buffer::from_str("héllo 🌍");
        assert_eq!(b.len_chars(), 7); // h, é, l, l, o, ' ', 🌍
        assert_eq!(b.char_at(1), 'é');
        assert_eq!(b.char_at(6), '🌍');
    }

    #[test]
    fn empty_delete() {
        let mut b = Buffer::from_str("hello");
        let deleted = b.delete(2, 2);
        assert_eq!(deleted, "");
        assert_eq!(b.to_string(), "hello");
    }

    #[test]
    fn multiline_insert() {
        let mut b = Buffer::new();
        b.insert(0, "a\nb\nc");
        assert_eq!(b.len_lines(), 3);
    }

    #[test]
    #[should_panic]
    fn insert_out_of_bounds_panics() {
        let mut b = Buffer::new();
        b.insert(1, "a");
    }

    #[test]
    #[should_panic]
    fn delete_out_of_bounds_panics() {
        let mut b = Buffer::new();
        b.delete(0, 1);
    }

    #[test]
    fn write_to_vec_roundtrip() {
        let b = Buffer::from_str("hello");
        let mut vec = Vec::new();
        b.write_to(&mut vec).unwrap();
        assert_eq!(String::from_utf8(vec).unwrap(), "hello");
    }

    #[test]
    fn from_reader_roundtrip() {
        let cursor = IoCursor::new(b"hello world");
        let b = Buffer::from_reader(cursor).unwrap();
        assert_eq!(b.to_string(), "hello world");
    }

    #[test]
    fn default_is_empty() {
        let b = Buffer::default();
        assert!(b.is_empty());
    }

    #[test]
    fn display_trait() {
        let b = Buffer::from_str("test display");
        assert_eq!(format!("{}", b), "test display");
    }
}
