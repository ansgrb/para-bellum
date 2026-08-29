/// Every edit operation expressed as a command.
///
/// Commands are dispatched through the command bus — no direct buffer mutation
/// from UI code. Each command produces an inverse command for undo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Insert text at a character position.
    InsertText {
        /// Position to insert at.
        pos: usize,
        /// Text to insert.
        text: String,
    },
    /// Delete characters in range [start, end).
    DeleteRange {
        /// Start of the range.
        start: usize,
        /// End of the range.
        end: usize,
    },
    /// Move cursor to a character position.
    MoveCursor {
        /// Position to move to.
        to: usize,
    },
}

/// The inverse of a command — used for undo.
/// Structurally identical to `Command`.
pub type InverseCommand = Command;

/// Trait for types that can apply edit commands.
pub trait Editable {
    /// Apply a command and return its inverse (for undo).
    fn apply(&mut self, cmd: Command) -> InverseCommand;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_clone_eq_debug() {
        let cmd1 = Command::InsertText { pos: 0, text: "a".into() };
        let cmd2 = cmd1.clone();
        assert_eq!(cmd1, cmd2);
        assert_eq!(format!("{:?}", cmd1), "InsertText { pos: 0, text: \"a\" }");
    }
}
