use crate::cursor::Cursor;

/// Identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub usize);

/// Scroll state of a view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ScrollState {
    /// The topmost visible line.
    pub top_line: usize,
    /// The leftmost visible column.
    pub left_col: usize,
}

/// Viewport dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    /// Width of the viewport.
    pub width: u16,
    /// Height of the viewport.
    pub height: u16,
}

/// A view into a buffer, including cursor, scroll, and viewport dimensions.
#[derive(Debug, Clone)]
pub struct View {
    /// The buffer being viewed.
    pub buffer_id: BufferId,
    /// The cursor state within this view.
    pub cursor: Cursor,
    /// The scroll state.
    pub scroll: ScrollState,
    /// The viewport dimensions.
    pub viewport: Viewport,
}

impl View {
    /// Create a new view for a buffer with specified dimensions.
    #[must_use]
    pub fn new(buffer_id: BufferId, width: u16, height: u16) -> Self {
        Self {
            buffer_id,
            cursor: Cursor::new(0),
            scroll: ScrollState::default(),
            viewport: Viewport { width, height },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_view_defaults() {
        let v = View::new(BufferId(1), 80, 24);
        assert_eq!(v.buffer_id, BufferId(1));
        assert_eq!(v.cursor.offset(), 0);
        assert_eq!(v.scroll.top_line, 0);
        assert_eq!(v.scroll.left_col, 0);
        assert_eq!(v.viewport.width, 80);
        assert_eq!(v.viewport.height, 24);
    }

    #[test]
    fn viewport_dimensions() {
        let vp = Viewport { width: 100, height: 50 };
        assert_eq!(vp.width, 100);
        assert_eq!(vp.height, 50);
    }

    #[test]
    fn scroll_state_default() {
        let s = ScrollState::default();
        assert_eq!(s.top_line, 0);
        assert_eq!(s.left_col, 0);
    }
}
