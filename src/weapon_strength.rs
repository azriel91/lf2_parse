/// Attack strength of a light weapon.
///
/// This is used when the `attacking` tag on a `WPoint` `kind: 1` is non-zero.
///
/// See https://lf-empire.de/lf2-empire/data-changing/types/168-type-1-light-weapons
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeaponStrength {
    /// Acceleration to place on the hit object in the X axis.
    pub d_vx: i64,
    /// Acceleration to place on the hit object in the Y axis.
    pub d_vy: i64,
    /// Delay before another hit may happen, restricts this `Itr` to one object.
    pub arest: u32,
    /// Delay before another hit may happen, allows multiple objects to be hit.
    pub vrest: u32,
    /// How much a character is "off balance".
    ///
    /// The `fall` value determines how an attacked character will react to this
    /// itr by flinching, getting into the stunned frames, or `falling`. If no
    /// value is specified, the default of `20` will be used.
    ///
    /// * If a character accumulates `20` `fall` points, he switches to
    ///   `injured1` (`220`).
    /// * If a character accumulates `40` `fall` points, he switches to
    ///   `injured2` (`222`) or `injured2back` (`224`) depending on the
    ///   direction he is hit, and will fall if he is in mid-air.
    /// * If a character accumulates `60` `fall` points, he switches into the
    ///   `stunned` (`226`) frames where he can be grabbed or hit by
    ///   `super_punch`.
    ///
    /// Attacks with `fall: 41` or more can hit `falling` characters.
    ///
    /// Here are a few values as a rule of thumb for various `fall` values:
    ///
    /// | `fall` | Description                                              |
    /// | -----: | :------------------------------------------------------- |
    /// |     -1 | Does not go into injured frames and harder to knockdown. |
    /// |      1 | Never stun, never fall (Davis DvA shrafe)                |
    /// |     20 | 3 hit stun, 4 hit fall                                   |
    /// |     25 | 2 hit stun, 3 hit fall (Dennis normal kick)              |
    /// |     40 | Does not stun, 2 hit fall (baseball bat normal swing)    |
    /// |     60 | 1 hit stun, 2 hit fall (Henry's arrow)                   |
    /// |     70 | 1 hit fall                                               |
    ///
    /// Every 1 TU, a `fall` point is deducted.
    pub fall: i32,
    /// Broken-defence points.
    ///
    /// `bdefend` points determine if a character is able to block an attack by
    /// defending or if he will go to the broken-defense-frames. As long as he
    /// has 30 or less Bdefend-points, he will be able to block the attack, if
    /// it's 31 or higher, he goes to the broken-defense-frames. If an itr hits
    /// the character while he is not in the defend-frames, his `bdefend`
    /// counter will automatically increase to 45. If you have, for example,
    /// accumulated 31 points and get hit during your defense (assuming you have
    /// specified a positive `bdefend` value in the hitting itr), the character
    /// will go to the broken-defense-frames.
    ///
    /// Here are some common values for various `bdefend` values:
    ///
    /// | `bdefend` | Description                                       |
    /// | --------: | :------------------------------------------------ |
    /// |         0 | never breaks defense (ex: John's D>J shield)      |
    /// |        12 | 4 hit break                                       |
    /// |        16 | 3 hit break                                       |
    /// |        30 | 2 hit break                                       |
    /// |        60 | 1 hit break                                       |
    /// |       100 | ignores defense, sets `bdefend` counter to `45`,\
    ///               and instantly destroys weapons.                   |
    ///
    /// Every 1 TU, a `bdefend` point is deducted, so he will be able to recover
    /// his defense.
    ///
    /// Armor will function as long as a character has not accumulated more
    /// `bdefend` points than the specific armor points of Louis(1), Knight or
    /// Julian(15) at the time of attack. For example, Julian can resist a dash
    /// attack(bdefend 60) even though he only has 15 armor points, but he will
    /// be left completely vulnerable for the next 45 TU until he regains his
    /// 1st armor point.
    pub b_defend: i32,
    /// Amount of damage to inflict on the target object.
    pub injury: i32,
}
