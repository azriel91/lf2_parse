/// Variants of `CPoint`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPointKind {
    /// The object that is holding the character.
    Catcher = 1,
    /// The held character.
    Caught = 2,
}
