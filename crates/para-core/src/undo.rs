use crate::command::InverseCommand;

/// Undo/redo stack. Each applied command pushes its inverse.
/// New edits clear the redo stack (standard branching behavior).
#[derive(Debug)]
pub struct UndoStack {
    undo: Vec<InverseCommand>,
    redo: Vec<InverseCommand>,
    max_depth: usize,
}

impl UndoStack {
    /// Create a new undo stack with a maximum depth.
    pub fn new(max_depth: usize) -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
            max_depth,
        }
    }

    /// Push an inverse command for undo, clearing the redo stack.
    pub fn push_undo(&mut self, inverse: InverseCommand) {
        self.redo.clear();
        self.undo.push(inverse);
        if self.undo.len() > self.max_depth {
            self.undo.remove(0);
        }
    }

    /// Pop an inverse command from the undo stack.
    pub fn pop_undo(&mut self) -> Option<InverseCommand> {
        self.undo.pop()
    }

    /// Push an inverse command for redo.
    pub fn push_redo(&mut self, inverse: InverseCommand) {
        self.redo.push(inverse);
    }

    /// Pop an inverse command from the redo stack.
    pub fn pop_redo(&mut self) -> Option<InverseCommand> {
        self.redo.pop()
    }

    /// Number of items in the undo stack.
    pub fn undo_depth(&self) -> usize {
        self.undo.len()
    }

    /// Number of items in the redo stack.
    pub fn redo_depth(&self) -> usize {
        self.redo.len()
    }

    /// Clear both undo and redo stacks.
    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}

impl Default for UndoStack {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;

    #[test]
    fn push_and_pop_undo() {
        let mut u = UndoStack::new(10);
        let cmd = Command::InsertText { pos: 0, text: "a".into() };
        u.push_undo(cmd.clone());
        assert_eq!(u.undo_depth(), 1);
        assert_eq!(u.pop_undo(), Some(cmd));
        assert_eq!(u.undo_depth(), 0);
    }

    #[test]
    fn push_undo_clears_redo() {
        let mut u = UndoStack::new(10);
        let cmd = Command::InsertText { pos: 0, text: "a".into() };
        u.push_redo(cmd.clone());
        assert_eq!(u.redo_depth(), 1);
        u.push_undo(cmd);
        assert_eq!(u.redo_depth(), 0);
    }

    #[test]
    fn max_depth_enforced() {
        let mut u = UndoStack::new(3);
        for i in 0..5 {
            u.push_undo(Command::InsertText { pos: i, text: "a".into() });
        }
        assert_eq!(u.undo_depth(), 3);
        // The first 2 were removed, so popping should give pos 4, then 3, then 2.
        assert_eq!(u.pop_undo(), Some(Command::InsertText { pos: 4, text: "a".into() }));
    }

    #[test]
    fn undo_redo_cycle() {
        let mut u = UndoStack::new(10);
        let cmd = Command::InsertText { pos: 0, text: "a".into() };
        u.push_undo(cmd.clone());
        
        let popped = u.pop_undo().unwrap();
        u.push_redo(popped);
        
        assert_eq!(u.undo_depth(), 0);
        assert_eq!(u.redo_depth(), 1);
        
        let redone = u.pop_redo().unwrap();
        u.push_undo(redone);
        
        assert_eq!(u.undo_depth(), 1);
        assert_eq!(u.redo_depth(), 0);
    }

    #[test]
    fn clear_both_stacks() {
        let mut u = UndoStack::new(10);
        let cmd = Command::InsertText { pos: 0, text: "a".into() };
        u.push_undo(cmd.clone());
        u.push_redo(cmd.clone());
        u.clear();
        assert_eq!(u.undo_depth(), 0);
        assert_eq!(u.redo_depth(), 0);
    }

    #[test]
    fn default_max_depth() {
        let u = UndoStack::default();
        assert_eq!(u.max_depth, 1000);
    }

    #[test]
    fn pop_empty_returns_none() {
        let mut u = UndoStack::new(10);
        assert!(u.pop_undo().is_none());
        assert!(u.pop_redo().is_none());
    }
}
