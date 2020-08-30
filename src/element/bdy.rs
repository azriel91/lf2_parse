/// Hittable body of the object.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bdy {
    /// Only used in criminal (type 5) objects.
    ///
    /// If you use `kind: 1050` (1000 + Frame number) and the bdy is hit by some
    /// `itr`s, the object switches to frame 50.
    pub kind: u32,
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Width.
    pub w: u32,
    /// Height.
    pub h: u32,
}
