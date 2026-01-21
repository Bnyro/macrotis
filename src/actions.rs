use gpui::{Action, KeyBinding, actions};

use crate::config::{self, CONFIG};

// All of these must be translated in the `locales` folder!
//
// Additionally, all of them have to be added to `build_key_bindings_from_config`!
actions!([Help, ToggleFullscreen, CloseWindow]);
actions!([NextImage, PreviousImage, OpenFiles]);
actions!([ZoomIn, ZoomOut]);
actions!([MoveUp, MoveDown, MoveLeft, MoveRight]);
actions!([ToggleImageInfo]);

/// Build a vec of [gpui::KeyBinding] from the key bindings configured in [crate::config::CONFIG].
///
/// These should later be registered to the app using using [gpui::App::bind_keys].
pub fn build_key_bindings_from_config() -> Vec<KeyBinding> {
    let key_bindings = &CONFIG.get().unwrap().keybindings;

    [
        create_key_bindings(Help, key_bindings),
        create_key_bindings(ToggleFullscreen, key_bindings),
        create_key_bindings(CloseWindow, key_bindings),
        create_key_bindings(NextImage, key_bindings),
        create_key_bindings(PreviousImage, key_bindings),
        create_key_bindings(OpenFiles, key_bindings),
        create_key_bindings(ZoomIn, key_bindings),
        create_key_bindings(ZoomOut, key_bindings),
        create_key_bindings(MoveUp, key_bindings),
        create_key_bindings(MoveDown, key_bindings),
        create_key_bindings(MoveLeft, key_bindings),
        create_key_bindings(MoveRight, key_bindings),
        create_key_bindings(ToggleImageInfo, key_bindings),
    ]
    .iter()
    .flatten()
    .cloned()
    .collect()
}

fn create_key_bindings<T: Action + Clone>(
    action: T,
    key_bindings: &[config::KeyBinding],
) -> Vec<KeyBinding> {
    let action_name = action.name();

    let bindings = key_bindings
        .iter()
        .filter(|binding| action_name.eq_ignore_ascii_case(&binding.action));

    bindings
        .map(|binding| KeyBinding::new(&binding.key, action.clone(), None))
        .collect()
}
