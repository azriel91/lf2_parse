use std::str::FromStr;

pub use self::effect_parse_error::EffectParseError;

mod effect_parse_error;

/// Itr `effect` variants.
///
/// See https://lf-empire.de/en/lf2-empire/data-changing/reference-pages/181-effects
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect {
    /// Normal, weapons fly away.
    ///
    /// * **Sound:** 001/006.wav.
    /// * **Effect:** Normal, weapons fly away.
    /// * **Examples:** Regular attacks.
    /// * **itrs **without any effect act like this.
    Normal = 0,
    /// Blood, weapons fly away.
    ///
    /// * **Sound:** 032/033.wav.
    /// * **Effect:** Blood, weapons fly away.
    /// * **Examples:** Deep's sword attacks.
    Blood = 1,
    /// Fire, can hit burning characters, weapons fly away.
    ///
    /// * **Sound:** 070/071.wav.
    /// * **Effect:** Fire, can hit burning characters, weapons fly away.
    /// * **Examples:** Firen's Firerun.
    Fire = 2,
    /// Ice, can hit frozen characters, weapons fly away.
    ///
    /// * **Sound:** 065/066.wav.
    /// * **Effect:** Ice, can hit frozen characters, weapons fly away.
    /// * **Examples:** Ice Ball.
    Ice = 3,
    /// Reflect all flying attacks with `state: 3000`, weapons fly away, has no
    /// influence on other characters.
    ///
    /// * **Sound:** weapon_hit_sound.
    /// * **Effect:** Reflect all flying attacks with `state: 3000`, weapons fly
    ///   away, has no influence on other characters.
    /// * **Examples:** "Shrafe" Attacks.
    Reflect = 4,
    /// Reflects all flying attacks with `state: 3000`, weapons fly away,
    /// characters are hit without sound.
    ///
    /// * **Sound:** [no Sound].
    /// * **Effect:** Reflects all flying attacks with `state: 3000`, weapons.
    ///   fly away, characters are hit without sound.
    /// * **Examples:** [Never used].
    Reflects = 5,
    /// Fire, burning characters are immune to `effect: 20`/`21`, weapons don't
    /// fly away.
    ///
    /// * **Sound:** 070/071.wav.
    /// * **Effect:** Fire, burning characters are immune to effect 20/21,
    ///   weapons don't fly away.
    /// * **Examples:** "Fire" frames, Firen's Ground-Fire.
    FireGround = 20,
    /// Fire, burning characters are immune to effect 20/21, weapons fly away,
    /// will not hit teammates when combined with `state: 18`.
    ///
    /// * **Sound:** 070/071.wav.
    /// * **Effect:** Fire, burning characters are immune to effect 20/21,
    ///   weapons fly away, will not hit teammates when combined with `state:
    ///   18`.
    /// * **Examples:** Firen's Inferno.
    FireBreath = 21,
    /// Fire, can hit burning characters, weapons fly away, a positive dvx goes
    /// to the middle, will not hit teammates when combined with `state: 18`.
    ///
    /// * **Sound:** 070/071.wav.
    /// * **Effect:** Fire, can hit burning characters, weapons fly away, a.
    ///   positive dvx goes to the middle, will not hit teammates when combined
    ///   with `state: 18`
    /// * **Examples:** Firen's Explosion.
    FireExplode = 22,
    /// Normal, character is hurtable, weapons fly away, a positive dvx goes to
    /// the middle.
    ///
    /// * **Sound:** 070/071.wav.
    /// * **Effect:** Normal, character is hurtable, weapons fly away, a.
    ///   positive dvx goes to the middle
    /// * **Examples:** Julian's Explosion.
    PowerExplode = 23,
    /// Ice, frozen characters are immune to `effect: 30`, weapons fly away.
    ///
    /// * **Sound:** 065.wav.
    /// * **Effect:** Ice, frozen characters are immune to `effect: 30`,
    ///   weapons. fly away.
    /// * **Examples:** Freeze Icicle.
    Icicle = 30,
}

impl FromStr for Effect {
    type Err = EffectParseError;

    fn from_str(s: &str) -> Result<Effect, EffectParseError> {
        s.parse::<u32>()
            .map_err(EffectParseError::ParseIntError)
            .and_then(|value| match value {
                0 => Ok(Effect::Normal),
                1 => Ok(Effect::Blood),
                2 => Ok(Effect::Fire),
                3 => Ok(Effect::Ice),
                4 => Ok(Effect::Reflect),
                5 => Ok(Effect::Reflects),
                20 => Ok(Effect::FireGround),
                21 => Ok(Effect::FireBreath),
                22 => Ok(Effect::FireExplode),
                23 => Ok(Effect::PowerExplode),
                30 => Ok(Effect::Icicle),
                value => Err(EffectParseError::InvalidValue(value)),
            })
    }
}
