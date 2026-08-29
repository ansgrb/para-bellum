/// A cursor position within a buffer, tracked as a character offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    /// Offset in characters.
    pub offset: usize,
}

/// Cursor state: position, optional selection, and sticky column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    /// Current cursor position.
    pub position: CursorPosition,
    /// Optional selection range.
    pub selection: Option<Selection>,
    /// Desired column for vertical movement through lines of different lengths.
    pub sticky_col: Option<usize>,
}

/// A selection range: anchor is where it started, head is where it extends to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Anchor position of the selection.
    pub anchor: usize,
    /// Head position of the selection.
    pub head: usize,
}

impl Selection {
    /// Get the normalized (min, max) range regardless of direction.
    pub fn range(&self) -> (usize, usize) {
        if self.anchor < self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    /// Number of characters selected.
    pub fn len(&self) -> usize {
        let (min, max) = self.range();
        max - min
    }

    /// Returns true if the selection extends forwards from the anchor.
    pub fn is_forward(&self) -> bool {
        self.head >= self.anchor
    }
}

impl Cursor {
    /// Create a new cursor at the given offset.
    pub fn new(offset: usize) -> Self {
        Self {
            position: CursorPosition { offset },
            selection: None,
            sticky_col: None,
        }
    }

    /// Get the current offset of the cursor.
    pub fn offset(&self) -> usize {
        self.position.offset
    }

    /// Move the cursor to a new offset, clearing any selection and sticky column.
    pub fn move_to(&mut self, offset: usize) {
        self.position.offset = offset;
        self.selection = None;
        self.sticky_col = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cursor_at_zero() {
        let c = Cursor::new(0);
        assert_eq!(c.offset(), 0);
        assert!(c.selection.is_none());
        assert!(c.sticky_col.is_none());
    }

    #[test]
    fn move_to_changes_offset() {
        let mut c = Cursor::new(0);
        c.move_to(5);
        assert_eq!(c.offset(), 5);
    }

    #[test]
    fn move_clears_selection() {
        let mut c = Cursor::new(0);
        c.selection = Some(Selection { anchor: 0, head: 5 });
        c.move_to(10);
        assert!(c.selection.is_none());
    }

    #[test]
    fn move_clears_sticky_col() {
        let mut c = Cursor::new(0);
        c.sticky_col = Some(42);
        c.move_to(10);
        assert!(c.sticky_col.is_none());
    }

    #[test]
    fn selection_range_forward() {
        let s = Selection { anchor: 5, head: 10 };
        assert_eq!(s.range(), (5, 10));
        assert!(s.is_forward());
    }

    #[test]
    fn selection_range_backward() {
        let s = Selection { anchor: 10, head: 5 };
        assert_eq!(s.range(), (5, 10));
        assert!(!s.is_forward());
    }

    #[test]
    fn selection_len() {
        let s1 = Selection { anchor: 5, head: 10 };
        assert_eq!(s1.len(), 5);
        
        let s2 = Selection { anchor: 10, head: 5 };
        assert_eq!(s2.len(), 5);
    }
}
