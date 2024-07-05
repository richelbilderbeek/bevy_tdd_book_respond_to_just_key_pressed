use bevy::input::keyboard::*;
use bevy::input::InputPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    if cfg!(test) {
        app.add_plugins(InputPlugin);
    }

    app.add_systems(Startup, add_player);
    app.add_systems(Update, respond_to_keyboard);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player {},
    ));
}

fn respond_to_keyboard(
    mut query: Query<&mut Transform, With<Player>>,
    maybe_input: Option<Res<ButtonInput<KeyCode>>>,
) {
    if maybe_input.is_none() {
        return;
    }
    let input = maybe_input.unwrap();
    let mut player_position = query.single_mut();
    if input.pressed(KeyCode::Space) {
        // This I can trigger in a test :-)
        player_position.translation.x += 16.0;
    }
    if input.just_pressed(KeyCode::Enter) {
        // How to trigger this in a test?
        player_position.translation.y += 16.0;
    }
}

#[cfg(test)]
pub fn count_n_players(app: &App) -> usize {
    let mut n = 0;
    for c in app.world.components().iter() {
        if c.name().contains("::Player") {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec3 {
    // Do 'app.update()' before calling this function,
    // else this assert goes off.
    assert_eq!(count_n_players(app), 1);
    let mut query = app.world.query::<(&Transform, &Player)>();
    let (transform, _) = query.single(&app.world);
    transform.translation
}

#[cfg(test)]
fn print_all_components_names(app: &App) {
    for c in app.world.components().iter() {
        println!("{}", c.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_players() {
        let app = App::new();
        assert_eq!(count_n_players(&app), 0);
    }

    #[test]
    fn test_add_player_adds_a_player() {
        let mut app = App::new();
        assert_eq!(count_n_players(&app), 0);
        app.add_systems(Startup, add_player);
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_key_press() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));

        // Press the right arrow button, thanks Periwinkle
        app.world
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Space);

        app.update();

        // Position must have changed now
        assert_ne!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));
    }

    #[test]
    fn test_player_responds_to_just_key_press() {
        //use bevy::input::keyboard::KeyboardInput;
        //use bevy::input::ButtonState;

        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));

        // Press the Enter button, thanks kristoff3r
        app.world.send_event(bevy::input::keyboard::KeyboardInput {
            key_code: KeyCode::Enter,
            logical_key: Key::Enter,
            state: bevy::input::ButtonState::Pressed,
            window: Entity::PLACEHOLDER,
        });

        app.update();
        app.update();

        // The position should have changed now
        assert_ne!(Vec3::new(0.0, 0.0, 0.0), get_player_position(&mut app));
    }

    #[test]
    fn test_print_all_components_names() {
        let mut app = create_app();
        app.update();
        print_all_components_names(&app);
    }
}
