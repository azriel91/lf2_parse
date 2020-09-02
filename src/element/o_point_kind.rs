/// Object spawning variants.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/178-opoint-object-point
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPointKind {
    /// Spawns an object on the same team.
    ///
    /// Note that when spawning type: 0 objects (characters), ID 5 (Rudolf) and
    /// ID 52 (Julian) are spawned with 10 HP, and all other IDs are spawned
    /// with 500 HP.
    Spawn = 1,
    /// Object is spawned and held as a light weapon.
    ///
    /// Ensure the spawned object has `WPoint` kind: `2` in its spawned frame.
    HoldLightWeapon = 2,
}
