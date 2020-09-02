/// Whether the same / opposite of parent, or always to the right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPointFacingDir {
    /// Face the same direction as the parent.
    ParentSame,
    /// Face the opposite direction to the parent.
    ParentOpposite,
    /// Always face to the right.
    Right,
}
