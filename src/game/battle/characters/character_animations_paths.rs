use once_cell::sync::Lazy;

pub const CHARACTER_ANIMATIONS_FPS: usize = 60;

pub static FEM_RED: Lazy<CharacterAnimationsPaths> = Lazy::new(|| create("fem_red"));
pub static FEM_KNIFE: Lazy<CharacterAnimationsPaths> = Lazy::new(|| create("fem_knife"));
pub static FEM_CANDY: Lazy<CharacterAnimationsPaths> = Lazy::new(|| create("fem_candy"));
pub static FEM_ROSE: Lazy<CharacterAnimationsPaths> = Lazy::new(|| create("fem_rose"));

pub struct CharacterAnimationsPaths {
    pub idle_up: String,
    pub idle_down: String,
    pub idle_side: String,
    pub walk_up: String,
    pub walk_down: String,
    pub walk_side: String,
    pub run_up: String,
    pub run_down: String,
    pub run_side: String,
    pub hurt_up: String,
    pub hurt_down: String,
    pub hurt_side: String,
    pub die_up: String,
    pub die_down: String,
    pub die_side: String,
    pub spell_up: String,
    pub spell_down: String,
    pub spell_side: String,
    pub stab_up: String,
    pub stab_down: String,
    pub stab_side: String,
    pub swing_up: String,
    pub swing_down: String,
    pub swing_side: String,
}

fn create(folder: &str) -> CharacterAnimationsPaths {
    CharacterAnimationsPaths {
        idle_up: format_path(folder, "Weapon Idle Up Sheet001.png"),
        idle_down: format_path(folder, "Weapon Idle Down Sheet001.png"),
        idle_side: format_path(folder, "Weapon Idle Side Sheet001.png"),
        walk_up: format_path(folder, "Weapon Walk Up Sheet001.png"),
        walk_down: format_path(folder, "Weapon Walk Down Sheet001.png"),
        walk_side: format_path(folder, "Weapon Walk Side Sheet001.png"),
        run_up: format_path(folder, "Weapon Run Up Sheet001.png"),
        run_down: format_path(folder, "Weapon Run Down Sheet001.png"),
        run_side: format_path(folder, "Weapon Run Side Sheet001.png"),
        hurt_up: format_path(folder, "Hurt Up Sheet001.png"),
        hurt_down: format_path(folder, "Hurt Down Sheet001.png"),
        hurt_side: format_path(folder, "Hurt Side Sheet001.png"),
        die_up: format_path(folder, "Die Up Sheet001.png"),
        die_down: format_path(folder, "Die Down Sheet001.png"),
        die_side: format_path(folder, "Die Side Sheet001.png"),
        spell_up: format_path(folder, "Spell Up Sheet001.png"),
        spell_down: format_path(folder, "Spell Down Sheet001.png"),
        spell_side: format_path(folder, "Spell Side Sheet001.png"),
        stab_up: format_path(folder, "Stab Up Sheet001.png"),
        stab_down: format_path(folder, "Stab Down Sheet001.png"),
        stab_side: format_path(folder, "Stab Side Sheet001.png"),
        swing_up: format_path(folder, "Swing Up Sheet001.png"),
        swing_down: format_path(folder, "Swing Down Sheet001.png"),
        swing_side: format_path(folder, "Swing Side Sheet001.png"),
    }
}

fn format_path(folder: &str, file: &str) -> String {
    format!("paid/spritesheets/{folder}/{file}")
}
