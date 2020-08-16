/// States
///
/// Descriptions are adapted from:
/// https://lf-empire.de/lf2-empire/data-changing/reference-pages/182-states?showall=1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Character standing.
    ///
    /// Frames with this state automatically react to certain inputs:
    ///
    /// * `F`: walking (Frame 5)
    /// * `F` + `F`: running (Frame 9)
    /// * `A`: punch (Frame 60)
    /// * `D`: defend (Frame 110)
    /// * `J`: jump (Frame 210)
    ///
    /// If you use this state in the air, the character will automatically
    /// switch to frame 212.
    ///
    /// Some other states also have special reactions to key-inputs, so for
    /// those states, the following key is used to describe the inputs:
    ///
    /// * `F`: Directions (forward, backward, up, down)
    /// * `A`: Attack
    /// * `D`: Defend
    /// * `J`: Jump
    ///
    /// When `A` is pressed:
    ///
    /// * If the character's `hit_a` is **not** `0`, then it is used.
    /// * If the character is holding a light weapon, there is a 50% chance to
    ///   go to frame 20 and 50% chance to go to frame 25.
    /// * If the character is not holding a light weapon, there is a 50% chance
    ///   to go to frame 60 and 50% chance to go to frame 65.
    ///
    /// When `J` is pressed:
    ///
    /// * If a character's `hit_j` is **not** `0`, then it is used.
    /// * Otherwise the character switches to frame 210.
    ///
    /// When `D` is pressed:
    ///
    /// * If a character's `hit_d` is **not** `0`, then it is used.
    /// * Otherwise the character switches to frame 110.
    ///
    /// If direction key is pressed, character will go to one of frames 5-8, but
    /// the initial frame it goes to is neither random nor fixed.
    ///
    /// However, if both of these happen (Left and right are both or neither
    /// pressed) (up and down are both or neither pressed), the direction
    /// keys are cancelled.
    ///
    /// This can be represented by logic condition `(up XNOR down) & (left XNOR
    /// right)`.
    ///
    /// A character (pragmatically, any object including weapon and projectile)
    /// has a hidden "walking counter" that counts from 0 to 5 whenever a
    /// tick has begun when the character is in `state: 1`.
    ///
    /// The counter starts at 0 when the character is spawned.
    ///
    /// The specific frame when a character goes to when it moves from `state:
    /// 0` is:
    ///
    /// | Counter | Next frame |
    /// | ------: | ---------: |
    /// |       0 |          5 |
    /// |       1 |          6 |
    /// |       2 |          7 |
    /// |       3 |          8 |
    /// |       4 |          7 |
    /// |       5 |          6 |
    ///
    /// The period of a tick is equal to (walking_frame_rate+1) tu, but exe will
    /// crash if walking_frame_rate is set to `0`.
    ///
    /// There is bad code hidden in lf2 that causes a simultaneous key input of
    /// JA and DA to go to frame 210 or 110 respectively, listed by Yinyin
    /// [here](https://www.lf-empire.de/forum/showthread.php?tid=804).
    ///
    /// If no button is pressed, a hidden standing counter will be incremented.
    /// Whenever this character leaves a frame with `state: `0``, this counter
    /// is reset to 0. The counter similar to walking counter except its highest
    /// possible number is equal to total tu of walking frames.
    /// For example, if a character has standing frames with wait 3,4,5,9, the
    /// counter can go to maximum of (3+4+5+9-4) = 17tu. Which standing
    /// frame the character goes to is determined by the counter.
    Standing = 0,
    /// Character walking.
    ///
    /// In frames with `state: 1`, the character moves with the speed noted in
    /// `walking_speed` and `walking_speedz`. All the inputs for this state are
    /// the same as `state: 0`.
    ///
    /// The walking state, it acts very similar to `state: 0` (there are some
    /// misconception regarding this state though)
    ///
    /// It acts identically as `state: 0` regarding `A`, `J` and `D` buttons.
    ///
    /// * When a direction key is held, it will determine if walking should be
    ///   continued.
    /// * When a direction key is released, although the character will go to
    ///   standing, there is a hidden running initiation counter that records
    ///   the last left/right key pressed and its initial press time.
    /// * When the same direction key is pressed quickly after the last key
    ///   press, it triggers running.
    /// * A `hit_*` tag leading to running frame grants the character automatic
    ///   running.
    ///
    /// # Notes
    ///
    /// * Grabbing a stunned person is NOT related to `state: 1`.
    /// * Picking up a weapon is NOT related to `state: 1`.
    Walking = 1,
    /// Character running.
    ///
    /// When a player presses left or right twice quickly, the character runs.
    /// `state: 2` is used for the running frames, and `running_speed` and
    /// `running_speedz` set the speed.
    ///
    /// * When `J` is pressed, the character switches to frame 213 (`dash`).
    /// * When `A` is pressed, the character switches to frame 85
    ///   (`run_attack`).
    /// * When `D` is pressed, the character switches to frame 102 (`rowing`).
    ///
    /// The running state, this also has a hidden running timer which maximum
    /// number equals to 3. It is very similar to walking counter except with 4
    /// transitions:
    ///
    /// | Counter | Next frame |
    /// | ------: | ---------: |
    /// |       0 |          9 |
    /// |       1 |         10 |
    /// |       2 |         11 |
    /// |       3 |         10 |
    ///
    /// * If `hit_d` parameter is 0, character will go to frame 102 (rolling)
    ///   when D is pressed.
    /// * If `hit_j` parameter is 0, character will go to frame 213 (forward
    ///   dash) when J is pressed.
    ///
    /// When `A` is pressed:
    ///
    /// * If the character's `hit_a` is **not** `0`, then it is used.
    /// * If the character is holding a light weapon, it switches to frame 35
    ///   (`run_weapon_attack`).
    /// * If the character is not holding a light weapon, it switches to frame
    ///   85 (`run_attack`)
    ///
    /// When the direction key against the character's running direction is
    /// pressed, it switches to frame 218 (`stop_running`)
    ///
    /// `hit_*` inputs are disabled if a character is holding a heavy weapon via
    /// itr kind 2; `state: 2` itself doesn't disable `hit_*`. For details go
    /// the section for itr kind 2.
    Running = 2,
    /// Character attacking.
    ///
    /// If there's an itr: within a frame with `state: 3`, a computer player
    /// will attempt to defend.
    ///
    /// It is very similar to `state: 15`; `state: 15` does not invoke defense
    /// on computer characters.
    Attacking = 3,
    /// Character jumping.
    ///
    /// When the character is in the air, you can press right or left to change
    /// the direction it is facing. Pressing `A` switches the character to frame
    /// 80 (`jump_attack`).
    ///
    /// The "Jump state" is an aerial state that takes some well-known and some
    /// hidden properties.
    ///
    /// When A is pressed:
    ///
    /// * If the character's `hit_a` is **not** `0`, then it is used.
    /// * If the character is holding a light weapon, it switches to frame 30
    ///   (`jump_weapon_attack`).
    /// * If the character is not holding a light weapon, it switches to frame
    ///   80 (`jump_attack`)
    ///
    ///     This does not work when the character is on the ground.
    ///
    /// If a direction key opposite to the character's facing is pressed, it
    /// changes direction. This does not work when the character is on ground.
    ///
    /// There is no application encoded reaction to `J` and `D` inputs.
    ///
    /// When a character with `state: 4` is hit, it switches to a `falling`
    /// frame. This can be seen by the stuck Louis bug -- when Louis tries to
    /// jump but his armor takes a hit, he falls to the ground instead.
    ///
    /// Landing in a frame with `state: 4` does not mean a character will go to
    /// frame 215. This behaviour is determined by the character landing from
    /// frame 212.
    Jumping = 4,
    /// Character dashing.
    ///
    /// As long as `dvx` is 0, you can use the direction-keys to change the
    /// direction of the character, like `state: 4`. If you run to the right and
    /// press left you'll go to frame 214. If you press right again you'll go
    /// back to frame 213. Pressing `A` will take your character to frame 90
    /// (`dash_attack`).
    ///
    /// The dashing state uses y-axis-velocity, direction control and initial
    /// direction to interchange between 4 frames, namely 213, 214, 216 and 217.
    ///
    /// The 4 frames represent the following:
    ///
    /// | Frame | Description        |
    /// | ----: | :----------------- |
    /// |   213 | Forward dash 1     |
    /// |   214 | Forward dash 2     |
    /// |   216 | Back-facing dash 1 |
    /// |   217 | Back-facing dash 2 |
    ///
    /// * If a character is in frame 213, pressing opposite direction will go to
    ///   216, vice versa.
    /// * If it is in frame 214, pressing opposite direction will go to 217,
    ///   vice versa.
    /// * If it changes into frame 213 or 216, there's no hidden cooldown and
    ///   frame countdown is refreshed.
    ///
    /// When `A` is pressed:
    ///
    /// * If the character's `hit_a` is **not** `0`, then it is used.
    ///
    /// If the character is in frame 213 or 216, when `A` is pressed:
    ///
    /// * If the character is holding a weapon, it switches to frame 40
    ///   (`dash_weapon_attack`)
    /// * If the character is not holding a weapon, it switches to frame 90
    ///   (`dash_attack`)
    ///
    /// Henry and Hunter can use dash backward shoot because there is hit_a: 81
    /// written on frame 214 and 217.
    Dashing = 5,
    /// Character "dodging".
    ///
    /// If a character in `state: 6` lands on the ground, it switches to frame
    /// 215.
    Rowing = 6,
    /// Character defending.
    ///
    /// Using state: 7 protects your character from frontal attacks. He can
    /// block as long as he has enough `bdefend:` and `fall:` points to block
    /// the attack. If he loses all of his `bdefend:` and `fall:` points,
    /// his defense will break or he may fall down.
    ///
    /// If a character is in `state: 7` and frame 110, and takes a hit that
    /// doesn't break his defense, he will go to frame 111.
    ///
    /// Taking a blow that doesn't break defense outside frame 100 won't change
    /// frame.
    ///
    /// Breaking defense makes character go to 112, but if character is in air
    /// or grabbed into `state: 7` frame, his `state: 7` frame cannot be broken.
    ///
    /// Nonetheless an attack with `bdefend >60` can nullify `state: 7`.
    ///
    /// If a character in frame 110 (but not necessary in `state: 7`), he can
    /// change his direction if he presses opposite direction key as his current
    /// facing.
    ///
    /// This only affects frame 110.
    Defend = 7,
    /// Character's defence is broken.
    ///
    /// When a character is in `broken_defend` frames, an enemy character
    /// pressing `A` will go to frame 70 (`super_punch`), but it is not caused
    /// by `state: 8` but itr kind: 6 in the frames.
    BrokenDefence = 8,
    /// Character is holding another character.
    ///
    /// To ensure that all parts of `cpoint:` work properly, it is best to use
    /// `state: 9`.
    ///
    /// The catching properties is done by `cpoint` parameters.
    Catching = 9,
    /// Character is held by another object.
    ///
    /// `state: 10` is used in the caught frames together with `next: 0`. The
    /// caught character cannot act at all and he/she drops any weapons that
    /// he/she is carrying.
    ///
    /// Object from any team can hit an object in `state: 10`.
    Caught = 10,
    /// Character is hit.
    ///
    /// Knight and Julian lose innate armor in `state: 8`, `10`, `11`, `16`,
    /// Louis' armor is only effective in frames 0-19 or `state: 4` and `5`.
    Injured = 11,
    /// Character is falling after being hit.
    ///
    /// If a character is knocked into the air by an attack (for example, by
    /// Davis' Dragon Punch), he/she goes to these frames. When he/she hits the
    /// ground, he/she switches to the lying frames, `230` or `231`, depending
    /// on the direction he/she is facing. As the character lands, he/she'll
    /// also drop any weapons that he/she is holding.
    ///
    /// `state: 12` has a state machine when the object is also a `type: 0`. Any
    /// non-character with `state: 12` frames are not affected.
    ///
    /// The state machine is similar to dash system, using y-axis velocity and
    /// facing direction to display falling animation.
    ///
    /// Any characters sent to falling frames will either start in frame `180`
    /// or `186`. The character immediately switches to the frame the
    /// machine indicates after 1 tick. The frame which the character switches
    /// to is as follows:
    ///
    /// | Condition                     | Frame          |
    /// | :---------------------------- | -------------: |
    /// | vy -10 or lower               | `180` or `186` |
    /// | vy >-10                       | `181` or `187` |
    /// | vy >0 (can flip)              | `182` or `188` |
    /// | vy >6                         | `183` or `189` |
    /// | Apparently unused -- `rewlf2` | `184` or `190` |
    /// | Bounce                        | `185` or `191` |
    ///
    /// Bounce happens when vx `>10` or vy `>1`. The bounce itself has a cap on
    /// its speed.
    ///
    /// If someone has a large negative yspeed, he will stay in frame 180 for
    /// some time before vy exceeds `-10`.
    ///
    /// Putting a dvx tag in falling will not change facing direction of
    /// character, but dvy tag will change its frame because y-axis velocity is
    /// changed.
    ///
    /// Character hitting ground will be set to a fixed x and y-axis velocity
    /// which again is affected by gravity, and goes to frame `185` or `191`
    /// (forward or backward).
    ///
    /// An object with `state: 12` is immune to any attack that has less than
    /// `41` fall, and will automatically drop their weapon, like the effect of
    /// wpoint kind 3.
    ///
    /// Flipping function in frame `183` and `188` are independent of state and
    /// type. I once stumbled with a glitch that allowed a `type: 3` character
    /// to go to frame `108` when she's in frame `188`. It appears `183` and
    /// `188` are separately hardcoded to allow flipping.
    Falling = 12,
    /// Character frozen state.
    ///
    /// A character in a frame with state: 13 can be hit by his/her teammates.
    /// The character will fall down after getting punched just once, but
    /// landing on the floor will damage him/her by 10 hp. Also, when the ice
    /// breaks, the broken ice shards will appear.
    ///
    /// There is an (undetermined) threshold for ice characters to not break if
    /// he lands on ground with too small x-axis and y-axis velocity.
    Ice = 13,
    /// Character lying on ground.
    ///
    /// `state: 14` is used in the lying frames. With this state, you can
    /// determine whether a character is alive or dead. If the character is
    /// alive, the frame with this state is performs normally -- after the wait
    /// time is up, the character goes to the next frame.
    ///
    /// However, if the character is dead, the frame is repeated with a `wait:
    /// 1`. If you include an `opoint:` in the frame, it will activate, but be
    /// careful: it is activated every TU! That's the basic idea of the "move
    /// after death" technique. Another thing is that computer controlled
    /// enemies don't pay attention to the character while he's in this state
    /// and will try to avoid him if he is still alive.
    ///
    /// `state: 14` has a condition-based state machine which either grants the
    /// character blinking status after he leaves the frame, gives a retreating
    /// warning to computer characters, or loops the frame every TU if the
    /// character has no current health.
    ///
    /// Note that if a character has 0 health is revived by F7, the lying frame
    /// countdown will start at when F7 is pressed.
    Lying = 14,
    /// Character state with no notable special function.
    Other = 15,
    /// Character is stunned and can be grabbed.
    ///
    /// `state: 16` is used in the injured frames: 226-229. You can catch a
    /// character who is in a frame with `state: 16` by using an `itr: kind: 1`
    /// (normal grab-move), in walking frames.
    ///
    /// Only a character with `state: 16` can be caught with a hostile object's
    /// `itr kind: 1`. The hostile will perform a grab if hostile is pressing a
    /// direction key and itr collides with `state: 16` character's `bdy`.
    Stunned = 16,
    /// Character drinking.
    ///
    /// The id-numbers 122 (milk) and 123 (beer) have bonus functions - milk can
    /// add life and a bit energy, beer can only add energy. But these bonus
    /// functions aren't activated all the time - only, if a character holds one
    /// of the drinks with `state: 17` (drinking-frames) the effect works.
    ///
    /// Frame 55-58 responsible for drinking action does not have an encoded
    /// state machine, if a character presses D in these frames, they stop
    /// drinking because there is `hit_d: 999` in these frames.
    Drinking = 17,
    /// Character is burning.
    ///
    /// `state: 18` creates the "burning_smoke" from `broken_weapon.dat`. An
    /// `itr:` with this state can also hit teammates, and if you spawn another
    /// object with `state: 18` , you can have it hit yourself. However, be
    /// aware that some effects (namely, `effect: 22`) will disable the
    /// self-hit function of `state: 18`.
    ///
    /// The fire state uses y-axis-velocity but has no direction control.
    ///
    /// There are 4 frames representing the following:
    ///
    /// | Frame | Description     |
    /// | ----: | :-------------- |
    /// |   203 | Upward fire 1   |
    /// |   204 | Upward fire 2   |
    /// |   205 | Downward fire 1 |
    /// |   206 | Downward fire 2 |
    ///
    /// Changing frame 203-206 to use `state: 15` disables the state machine.
    ///
    /// When a type: 0 character is in `state: 18` and in frame 203-206, the
    /// next frame is determined by current y-axis speed.
    ///
    /// If character is going up, next frame will be 203/204, otherwise will be
    /// 205/206 The exact frames are not confirmed and it is unknown if
    /// non-type: 0 share this property.
    ///
    /// A `state: 18` character landing will always be directed to frame 185
    /// (forward bouncing)
    ///
    /// `state: 18` characters are immune to itr held by `state: 18`, `19`
    /// objects and `itr` with `effect: 20`.
    ///
    /// `state: 18` characters can hit projectiles (example: soul bomb's
    /// explosion and firen's inferno), `itr effect: 20` doesn't hit projectiles
    /// though.
    Burning = 18,
    /// Fire run.
    ///
    /// `state: 19` is used in Firen's fire run. You can move along the Z axis,
    /// but you can't hurt teammates. Like `state: 18`, the burning smoke
    /// sprites are created when you use this state.
    ///
    /// `state: 19` allows Z axis movement based on `running_speedz`, generates
    /// fire smoke and is immune to certain fire attacks.
    ///
    /// `state: 19` characters are immune to `itr` on `state: 18` objects and
    /// `itr`s with `effect: 20`.
    ///
    /// Characters in `state: 19` that hit `state: 3000` projectiles will cause
    /// them to switch to frame 20 (`hit`) instead of 30 (`rebounding`).
    FireRun = 19,
    /// Louis dash hit ground.
    ///
    /// This state is used in Louis' dash attack. When using `state: 100`, your
    /// character should be in the air, and the "wait" must be high enough so
    /// he'll have time to land on the ground during this frame; the `next:`
    /// value is not used in this case). When he lands back on floor, the
    /// character automatically goes to frame 94. You can customize this frame
    /// to make your character do an action when he lands.
    HitGround = 100,
    /// Allows the character to move along the Z axis while attacking.
    ///
    /// Alternatively, you can use `state: 3` instead of `state: 301` and
    /// include a special value for "dvz:" to allow movements along the
    /// z-axis.
    ///
    /// Used for Deep's strafe.
    ZMovement = 301,
    /// Moves this object 120 pixels behind the closest enemy character.
    ///
    /// Has no effect if there are no enemy characters.
    TeleportNearestEnemy = 400,
    /// Moves this object 60 pixels in front of the closest ally character.
    ///
    /// Has no effect if there are no ally characters.
    TeleportFurthestAlly = 401,
    /// Prevents this frame from executing if this object has not previously
    /// transformed into another.
    ///
    /// # Note
    ///
    /// Reverse transformation begins on frame 245.
    TransformCheck = 500,
    /// Changes this object into the last transformed object.
    ///
    /// # Note
    ///
    /// Reverse transformation begins on frame 245.
    Transform = 501,
    /// Character self-heal.
    ///
    /// With `state: 1700`, you can heal up to 100 health points, but it won't
    /// heal past the dark red bar. When the healing occurs, the health bar will
    /// flicker white. `itr: kind: 8` also has a similar heal effect, except you
    /// can set how much you want it to heal.
    ///
    /// HP normally regenerates 1 hp every 12 TU.
    ///
    /// Healing spells recover 8 hp every 8 TU over 100 TU; the first 8 hp is
    /// applied within 4 TU.
    ///
    /// John's heals (`state: 1700` or `itr kind 8`) do not stack with each
    /// other, stopping prematurely when Red HP = Dark HP.
    ///
    /// `state: 1700` only begins healing after leaving the frame, though the HP
    /// bar will start blinking when 1700 starts.
    ///
    /// Jan's heal (ball `hit_Fa: 4`) applies a "regeneration" status effect
    /// that lasts the full 100 TU, and stacks with `state: 1700` or `itr kind
    /// 8`.
    Heal = 1700,
    /// Light weapon is falling in mid air.
    ///
    /// Light weapons fall freely in sky in this state. Whether it will damage
    /// others is dependent on if this frame has an itr.
    ///
    /// Light weapons in original LF2 have no itr in any `state: 1000` frames.
    ///
    /// Baseball, milk and beer will not go to frame 0-15 (which have `state:
    /// 1000`) even if they are hit by itr with less than 60 fall.
    ///
    /// Others light weapons will go to frame 0-15, which frame it switches to
    /// is random.
    LightWeaponInSky = 1000,
    /// Light weapon held state.
    ///
    /// The weapon frame number and action is controlled by the object having
    /// the `wpoint`.
    LightWeaponInHand = 1001,
    /// Light weapon has been thrown.
    ///
    /// This typically follows a `wpoint` throw, but milk, beer and baseball
    /// will go to thrown frames when hit.
    LightWeaponBeingThrown = 1002,
    /// Light weapon just landed on ground.
    ///
    /// `state: 1003` does not have state engine, the ground collision is
    /// determined by type of object -- when a `type: 1` object lands on the
    /// ground, it switches to frame 60 or 70
    LightWeaponJustOnGround = 1003,
    /// Light weapon is on ground.
    ///
    /// Object in this state can have a bdy that interacts with `itr kind: 2` of
    /// the character, the character links the object via wpoint. `state: 1004`
    /// causes picking character to go to `picking_light` frame.
    LightWeaponOnGround = 1004,
    /// Heavy weapon is falling in mid air.
    ///
    /// Heavy weapons fall freely in sky in this state. Whether it damages
    /// others is dependent on if the frame has an itr.
    ///
    /// Heavy weapons in original LF2 have itr in `state: 2000` frames, thus the
    /// stones hitting unlucky characters.
    ///
    /// If a heavy weapon is hit by itr with `fall: 70+`, it will switch its
    /// frame to 0-5 randomly.
    HeavyWeaponInSky = 2000,
    /// Heavy weapon held state.
    ///
    /// The weapon frame number and action is controlled by the object having
    /// the `wpoint`.
    ///
    /// Characters will switch to `heavy_walking` frames (frame 12) when picking
    /// up a heavy weapon.
    HeavyWeaponInHand = 2001,
    /// Heavy weapon held state.
    ///
    /// The weapon frame number and action is controlled by the object having
    /// the `wpoint`.
    HeavyWeaponOnGround = 2004,
    /// Ball flying.
    ///
    /// If the ball hits other attacks with this state, it switches to the
    /// hitting frame `10`.  If it is hit by another ball or a character, it
    /// switches to the the hit frame `20` or rebounding frame `30`.
    BallFlying = 3000,
    /// Ball hitting another object.
    ///
    /// If the ball hits a character while it has `state: 3001`, then it won't
    /// go to the hitting frame `20`. This is the same for states `3001` through
    /// `3004`.
    BallFlyingHitting = 3001,
    /// Ball is hit by another object.
    ///
    /// If the ball hits a character while it has `state: 3002`, then it won't
    /// go to the hitting frame `20`. This is the same for states `3001` through
    /// `3004`.
    BallFlyingHit = 3002,
    /// Ball is hit by a character.
    ///
    /// If the ball hits a character while it has `state: 3003`, then it won't
    /// go to the hitting frame `20`. This is the same for states `3001` through
    /// `3004`.
    BallFlyingRebound = 3003,
    /// Ball is displaying its disappearing animation.
    ///
    /// If the ball hits a character while it has `state: 3004`, then it won't
    /// go to the hitting frame `20`. This is the same for states `3001` through
    /// `3004`.
    BallFlyingDisappear = 3004,
    /// Ball is flying, and has no shadow.
    ///
    /// If this is used in a ball's flying frames, it will destroy any other
    /// ball attack that it hits -- it is stronger than state `3000` and state
    /// `3006`.
    BallFlyingNoShadow = 3005,
    /// Ball flying (piercing).
    ///
    /// `state: 3006` is a stronger version of `state: 3000`.  It cannot be
    /// rebounded and `state: 3000` balls won't destroy it.  However, if a
    /// `state: 3006` ball is hit by a `state: 3005` attack or another `state:
    /// 3006` attack, it will be destroyed.
    ///
    /// This is used by Henry's piercing shot.
    BallFlyingPiercing = 3006,
    /// Transform into object with `id: 0`.
    ///
    /// With `state: 8000`, you can transform one character into another. There
    /// is another transform state, but it only works with id-numbers 6 and 50:
    /// the transformation of Louis to LouisEX (see state: 9995 in extra
    /// states). Here is some basic info about transforming:
    ///
    /// Use `state: 8000` + id-number of the object you want to transform into
    /// -- e.g. `state: 8030` to transform into `id: 30`.
    ///
    /// When you transform, the computer takes a frame's pic-number, adds 140 to
    /// it, and uses that pic instead. Because of this, you usually have to
    /// change the way the character's spritesheets are defined in the
    /// bmp_header at the beginning of each character. If you select the
    /// character from the menu, they'll use their normal sprites, but if you
    /// transform into him, they'll use the pic-number + 140 sprites.
    ///
    /// The computer calculates the number of pics using the product of the
    /// "row" and "col" parts of the file tag, so sometimes you'll have to
    /// "waste" pic-numbers to guarantee that the transformed character will use
    /// the proper sprites.
    ///
    /// In the bmp part, you have to remember that you are limited to 10 picture
    /// files!
    ///
    /// If a character has more than 140 pictures, you have to use pic 0 to 139
    /// for the first 140 pictures and 280 to 419 for the following pictures.
    ///
    /// This transformation will cause the character to go to frame 0 when id is
    /// changed.
    ///
    /// The character will try to use pic number with +140 offset. Transforming
    /// into Knight will try to use +140 offset sprites which normally will
    /// glitch display as the knight_b sprites are offset by +114.
    Transform00 = 8000,
    /// Transform into object with `id: 01`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform01 = 8001,
    /// Transform into object with `id: 02`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform02 = 8002,
    /// Transform into object with `id: 03`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform03 = 8003,
    /// Transform into object with `id: 04`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform04 = 8004,
    /// Transform into object with `id: 05`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform05 = 8005,
    /// Transform into object with `id: 06`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform06 = 8006,
    /// Transform into object with `id: 07`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform07 = 8007,
    /// Transform into object with `id: 08`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform08 = 8008,
    /// Transform into object with `id: 09`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform09 = 8009,
    /// Transform into object with `id: 10`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform10 = 8010,
    /// Transform into object with `id: 11`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform11 = 8011,
    /// Transform into object with `id: 12`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform12 = 8012,
    /// Transform into object with `id: 13`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform13 = 8013,
    /// Transform into object with `id: 14`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform14 = 8014,
    /// Transform into object with `id: 15`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform15 = 8015,
    /// Transform into object with `id: 16`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform16 = 8016,
    /// Transform into object with `id: 17`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform17 = 8017,
    /// Transform into object with `id: 18`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform18 = 8018,
    /// Transform into object with `id: 19`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform19 = 8019,
    /// Transform into object with `id: 20`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform20 = 8020,
    /// Transform into object with `id: 21`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform21 = 8021,
    /// Transform into object with `id: 22`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform22 = 8022,
    /// Transform into object with `id: 23`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform23 = 8023,
    /// Transform into object with `id: 24`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform24 = 8024,
    /// Transform into object with `id: 25`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform25 = 8025,
    /// Transform into object with `id: 26`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform26 = 8026,
    /// Transform into object with `id: 27`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform27 = 8027,
    /// Transform into object with `id: 28`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform28 = 8028,
    /// Transform into object with `id: 29`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform29 = 8029,
    /// Transform into object with `id: 30`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform30 = 8030,
    /// Transform into object with `id: 31`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform31 = 8031,
    /// Transform into object with `id: 32`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform32 = 8032,
    /// Transform into object with `id: 33`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform33 = 8033,
    /// Transform into object with `id: 34`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform34 = 8034,
    /// Transform into object with `id: 35`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform35 = 8035,
    /// Transform into object with `id: 36`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform36 = 8036,
    /// Transform into object with `id: 37`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform37 = 8037,
    /// Transform into object with `id: 38`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform38 = 8038,
    /// Transform into object with `id: 39`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform39 = 8039,
    /// Transform into object with `id: 40`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform40 = 8040,
    /// Transform into object with `id: 41`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform41 = 8041,
    /// Transform into object with `id: 42`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform42 = 8042,
    /// Transform into object with `id: 43`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform43 = 8043,
    /// Transform into object with `id: 44`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform44 = 8044,
    /// Transform into object with `id: 45`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform45 = 8045,
    /// Transform into object with `id: 46`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform46 = 8046,
    /// Transform into object with `id: 47`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform47 = 8047,
    /// Transform into object with `id: 48`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform48 = 8048,
    /// Transform into object with `id: 49`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform49 = 8049,
    /// Transform into object with `id: 50`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform50 = 8050,
    /// Transform into object with `id: 51`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform51 = 8051,
    /// Transform into object with `id: 52`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform52 = 8052,
    /// Transform into object with `id: 53`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform53 = 8053,
    /// Transform into object with `id: 54`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform54 = 8054,
    /// Transform into object with `id: 55`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform55 = 8055,
    /// Transform into object with `id: 56`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform56 = 8056,
    /// Transform into object with `id: 57`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform57 = 8057,
    /// Transform into object with `id: 58`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform58 = 8058,
    /// Transform into object with `id: 59`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform59 = 8059,
    /// Transform into object with `id: 60`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform60 = 8060,
    /// Transform into object with `id: 61`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform61 = 8061,
    /// Transform into object with `id: 62`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform62 = 8062,
    /// Transform into object with `id: 63`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform63 = 8063,
    /// Transform into object with `id: 64`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform64 = 8064,
    /// Transform into object with `id: 65`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform65 = 8065,
    /// Transform into object with `id: 66`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform66 = 8066,
    /// Transform into object with `id: 67`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform67 = 8067,
    /// Transform into object with `id: 68`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform68 = 8068,
    /// Transform into object with `id: 69`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform69 = 8069,
    /// Transform into object with `id: 70`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform70 = 8070,
    /// Transform into object with `id: 71`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform71 = 8071,
    /// Transform into object with `id: 72`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform72 = 8072,
    /// Transform into object with `id: 73`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform73 = 8073,
    /// Transform into object with `id: 74`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform74 = 8074,
    /// Transform into object with `id: 75`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform75 = 8075,
    /// Transform into object with `id: 76`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform76 = 8076,
    /// Transform into object with `id: 77`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform77 = 8077,
    /// Transform into object with `id: 78`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform78 = 8078,
    /// Transform into object with `id: 79`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform79 = 8079,
    /// Transform into object with `id: 80`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform80 = 8080,
    /// Transform into object with `id: 81`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform81 = 8081,
    /// Transform into object with `id: 82`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform82 = 8082,
    /// Transform into object with `id: 83`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform83 = 8083,
    /// Transform into object with `id: 84`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform84 = 8084,
    /// Transform into object with `id: 85`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform85 = 8085,
    /// Transform into object with `id: 86`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform86 = 8086,
    /// Transform into object with `id: 87`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform87 = 8087,
    /// Transform into object with `id: 88`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform88 = 8088,
    /// Transform into object with `id: 89`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform89 = 8089,
    /// Transform into object with `id: 90`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform90 = 8090,
    /// Transform into object with `id: 91`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform91 = 8091,
    /// Transform into object with `id: 92`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform92 = 8092,
    /// Transform into object with `id: 93`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform93 = 8093,
    /// Transform into object with `id: 94`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform94 = 8094,
    /// Transform into object with `id: 95`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform95 = 8095,
    /// Transform into object with `id: 96`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform96 = 8096,
    /// Transform into object with `id: 97`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform97 = 8097,
    /// Transform into object with `id: 98`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform98 = 8098,
    /// Transform into object with `id: 99`.
    ///
    /// See [`State::Transform00`] for more details.
    Transform99 = 8099,
    /// Louis transform into LouisEx.
    ///
    /// This state is used to transform Louis into LouisEX. Normally, `state:
    /// 8000` is used to transform, but because LouisEX has more than 140 single
    /// pictures and there isn't a need for two color themes for LouisEX, this
    /// state was created.
    ///
    /// If `state: 9995` is used in a character, the character transforms into
    /// the character with `id: 50` specified in `data.txt` and switches to
    /// frame 0.
    LouisTransform = 9995,
    /// Louis transform create armour.
    ///
    /// State 9996 is used in Louis' transform move. It creates the armour
    /// weapons that appear during the transformation.  Four copies of `id: 217`
    /// are created in a square around LouisEX and one `id: 218` is spawned in
    /// the center.
    LouisTransformSpawnArmour = 9996,
    /// State for a message bubble display.
    ///
    /// This state is used for messages (`etc.dat`).  A frame with this state
    /// has no shadow and the picture can be seen from anywhere in the arena
    /// But be careful: if the picture is larger than 80 pixels, only part
    /// of the picture will be shown on the right side of the screen.  If
    /// you create a message with an `opoint`, you need to put a `dvy: 550` in
    /// the frame, otherwise the object will fall.
    Message = 9997,
    /// Deletes the object.
    ///
    /// `state: 9998` has the same effect as `next: 1000`: The object gets
    /// deleted from the game.  If you're working with `cpoint` or weapons,
    /// `state: 9998` is a safer to use than `next: 1000`.
    DeleteObject = 9998,
    /// Weapon fragments.
    ///
    /// This state is used in `broken_weapon.dat`, but it functions the same as
    /// `state: 15` ([`State::Other`]).
    BrokenWeapon = 9999,
}
