use bevy::prelude::*;
use crate::game::game_mode::GameMode;

pub(super) fn build_main_menu(app: &mut App) {
    app.add_systems(OnEnter(GameMode::MainMenu), show_main_menu)
        .add_systems(Update, increment_atlas_index);
}

#[derive(Component)]
struct PapirusAppearanceAnimation {
    pub timer: Timer,
    pub frames_count: usize,
}

fn show_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let text_style = TextStyle {
        color: Color::ANTIQUE_WHITE,
        font_size: 20.,
        ..default()
    };

    let menu_handle = asset_server.load("paid/ui/Papirus Long 4.png");
    const SPRITESHEET_COLUMNS: usize = 5;
    const SPRITESHEET_ROWS: usize = 4;
    let texture_atlas = TextureAtlas::from_grid(menu_handle,
                                                Vec2::new(457., 548.),
                                                SPRITESHEET_COLUMNS, SPRITESHEET_ROWS,
                                                None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::End,
                row_gap: Val::Px(text_style.font_size * 2.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                PapirusAppearanceAnimation {
                    timer: Timer::from_seconds(1. / 30., TimerMode::Repeating),
                    frames_count: SPRITESHEET_COLUMNS * SPRITESHEET_ROWS,
                },
                AtlasImageBundle {
                    texture_atlas: texture_atlas_handle,
                    texture_atlas_image: UiTextureAtlasImage::default(),
                    ..default()
                }));
        });
}

fn increment_atlas_index(
    mut atlas_images: Query<(&mut UiTextureAtlasImage, &mut PapirusAppearanceAnimation)>,
    time: Res<Time>,
) {
    for (mut atlas_image, mut animation) in &mut atlas_images {
        if atlas_image.index >= animation.frames_count -1 {
            continue;
        }

        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            atlas_image.index = atlas_image.index + 1;
        }
    }
}