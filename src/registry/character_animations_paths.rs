pub struct CharacterAnimationsPaths {
    pub idle_up: String,
    pub idle_down: String,
    pub idle_side: String,
    pub run_up: String,
    pub run_down: String,
    pub run_side: String,
    pub hurt_up: String,
    pub hurt_down: String,
    pub hurt_side: String,
    pub die_up: String,
    pub die_down: String,
    pub die_side: String,
}


impl CharacterAnimationsPaths {
    pub fn young_hero() -> Self { create("young_hero") }
}

fn create(folder: &str) -> CharacterAnimationsPaths {
    CharacterAnimationsPaths {
        idle_up: format_path(folder, "Weapon Idle Up Sheet001.png"),
        idle_down: format_path(folder, "Weapon Idle Down Sheet001.png"),
        idle_side: format_path(folder, "Weapon Idle Side Sheet001.png"),
        run_up: format_path(folder, "Weapon Run Up Sheet001.png"),
        run_down: format_path(folder, "Weapon Run Down Sheet001.png"),
        run_side: format_path(folder, "Weapon Run Side Sheet001.png"),
        hurt_up: format_path(folder, "Hurt Up Sheet001.png"),
        hurt_down: format_path(folder, "Hurt Down Sheet001.png"),
        hurt_side: format_path(folder, "Hurt Side Sheet001.png"),
        die_up: format_path(folder, "Die Up Sheet001.png"),
        die_down: format_path(folder, "Die Down Sheet001.png"),
        die_side: format_path(folder, "Die Side Sheet001.png"),
    }
}

fn format_path(folder: &str, file: &str) -> String {
    format!("paid/spritesheets/{folder}/{file}")
}
