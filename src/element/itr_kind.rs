/// Interaction variants.
///
/// See https://lf-empire.de/lf2-empire/data-changing/frame-elements/174-itr-interaction?showall=1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItrKind {
    /// Hit another object's `bdy`.
    Normal = 0,
    /// Catch a character that is stunned / "dance of pain" (state 16).
    CatchStunned = 1,
    /// Picks up a light or heavy weapon.
    ///
    /// Character switches to the `picking_light` frame (`115`) for light
    /// weapons, and `picking_heavy` (`116`) for heavy weapons.
    WeaponPick = 2,
    /// Catch a character.
    ///
    /// `catchingact` and `caughtact` specify the frame numbers for where the
    /// catching and caught characters should switch to. They are specified
    /// twice because one is for catching from the front and and the other is
    /// for catching from the back.
    ///
    /// This is used in Louis' whirlwind throw.
    CatchForce = 3,
    /// Interaction on a falling character that only hits if he/she was thrown.
    Falling = 4,
    /// Interaction on a weapon whose damage depends on `weaponact`.
    ///
    /// The strength of a weapon can vary, so you can't note it directly in the
    /// frames. The wpoint in the characters using the `attacking` tag to
    /// activate an entry in the `weapon_strength_list`. The corresponding `itr`
    /// values are then used for the weapon's itr.
    WeaponStrength = 5,
    /// Interaction that enables characters to do a super punch.
    ///
    /// `vrest:` 1 is normally used because this should affect all characters.
    SuperPunch = 6,
    /// Picks up a light weapon without switching frame.
    ///
    /// This `itr` allows a character to pick up a light weapon without going to
    /// the `picking_light` frames (`115`). It's used in the "rowing"
    /// frames. If you roll over a weapon and press the attack-button, you
    /// pick it up.
    RollWeaponPick = 7,
    /// Heals a character, constrained by dark red HP.
    ///
    /// When the healing is activated, you'll see your healthbar flash. Some
    /// additional functions of `itr/kind: 8` are:
    ///
    /// * Aligns its centerx and centery to that of any `type: 0` object it
    ///   touches - independant of `wait` and `next`.
    /// * Reacts to both allies and enemies.
    /// * Only interacts with `type: 0` objects.
    /// * Doesn't have to be used with a special state like `CPoint`.
    /// * The character that `itr/kind: 8` "sticks" to is not influenced in any
    ///   way (besides healing).
    ///
    /// The `injury` tag doesn't do damage here. Instead, it sets the amount of
    /// life points that the character can regenerate. Normally it is `100`,
    /// because `state: 1700` and `hit_Fa: 4` heal `100` points (defined by
    /// source code), but you may also use other values -- set the value to `0`
    /// for no healing.
    ///
    /// The `dvx` tag is also repurposed -- if the `itr` hits a character, the
    /// object switches to the frame noted by `dvx`.
    HealBall = 8,
    /// John's reflective shield.
    ///
    /// An itr with `kind: 9` permits an object to reflect/destroy any sort of
    /// incoming projectiles. Additionally the itr can hit characters (type: 0
    /// objects), but doing so will reduce the attacker's health to zero.
    ///
    /// This is utilized in conjunction with `hit_a`/`hit_d` to create John's
    /// shield, which will disappear if a character runs into it.
    ///
    /// Using an `itr/kind:9` in a character (type: 0) would cause the character
    /// to die instantly once he/she hits another character with the itr.
    ReflectiveShield = 9,
    /// Henry's Sonata of Death.
    ///
    /// All characters and weapons are lifted up into the air.
    SonataOfDeath = 10,
    /// Henry's Sonata of Death.
    ///
    /// Similar to [`ItrKind::SonataOfDeath`].
    SonataOfDeath2 = 11,
    /// Impassable object.
    ///
    /// This kind doesn't do any damage, it just acts as a solid object that
    /// other objects cannot pass through. It's used in heavy weapons and
    /// Freeze's icicles, so you can't simply walk through these objects.
    Wall = 14,
    /// Freeze's whirlwind's vacuum.
    ///
    /// Objects are sucked in like a vacuum.
    WhirlwindWind = 15,
    /// Freeze's whirlwind freeze.
    ///
    /// Turns characters into ice without using the `effect` tag and lifts up
    /// only weapons.
    WhirlwindIce = 16,
}
