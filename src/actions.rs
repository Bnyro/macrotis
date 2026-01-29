use gpui::{Action, KeyBinding, actions};

use crate::config::{self, CONFIG};

/// Given the action names and their default key bindings,
/// this macro does three things:
/// - Invoke [gpui::actions!] for each of the action names
/// - Create a method `default_key_bindings` containing a list of all default key bindings
/// - Create a method `convert_to_gpui_keybindings` which converts the [config::KeyBinding]s to [gpui::KeyBinding].
///
/// This is necessary to avoid duplicated code here and because actions created by the
/// [gpui::actions!] macro don't implement [Clone], making them hard to handle.
macro_rules! build_actions_with_key_bindings {
    ($(($action_name:ident, $default_key:literal)),*) => {
        // invoke gpui's actions macro for all $name(s)
        actions!([$($action_name),*]);

        pub fn default_key_bindings() -> Vec<config::KeyBinding> {
            vec![$((config::KeyBinding::new($default_key, &$action_name)),)*]
        }

        /// Convert a `Vec` of [config::KeyBinding] to a `Vec` of [gpui::KeyBinding].
        fn convert_to_gpui_keybindings(key_bindings: &[config::KeyBinding]) -> Vec<KeyBinding> {
            [
                $(create_key_bindings(&$action_name, key_bindings),)*
            ].iter().flatten().cloned().collect()
        }
    };
}

// This declares all actions the app supports.
// All of these must be translated in the `locales` folder, using the lowercased action name as key!
//
// E.g. for the action `Help`, one would need to create
// ```toml
// [actions]
// help = "Show the help dialog"
// ```
build_actions_with_key_bindings!(
    (Help, "?"),
    (ToggleFullscreen, "f"),
    (CloseWindow, "q"),
    (NextImage, "l"),
    (PreviousImage, "h"),
    (GotoFirstImage, "g"),
    (GotoLastImage, "shift-g"),
    (ZoomIn, "+"),
    (ZoomOut, "-"),
    (MoveUp, "up"),
    (MoveDown, "down"),
    (MoveLeft, "left"),
    (MoveRight, "right"),
    (OpenFiles, "o"),
    (ToggleImageInfo, "i")
);

/// Build a vec of [gpui::KeyBinding] from the [config::KeyBinding]s configured in [crate::config::CONFIG]
/// (or the default ones if none were provided).
///
/// These should later be registered to the app using using [gpui::App::bind_keys].
pub fn build_key_bindings_from_config() -> Vec<KeyBinding> {
    let configured_key_bindings = &CONFIG.get().unwrap().keybindings;

    convert_to_gpui_keybindings(configured_key_bindings)
}

/// Takes a [gpui::Action] and builds [gpui::KeyBinding]s for all of the provided
/// `key_bindings` that have the same name as the [gpui::Action].
///
/// Used by [convert_to_gpui_keybindings].
fn create_key_bindings<T: Action + Clone>(
    action: &T,
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
