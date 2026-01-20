use std::ops::Range;

use crate::{actions::*, config::CONFIG};
use gpui::*;

pub struct HelpWindow {
    focus_handle: FocusHandle,
}

impl Render for HelpWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let keymap = cx.key_bindings();
        let keymap = keymap.borrow();

        div()
            .track_focus(&self.focus_handle)
            .on_action(|_: &CloseWindow, window, _cx| window.remove_window())
            .bg(CONFIG.get().unwrap().theme.background.into_rgba())
            .child(
                div()
                    .text_color(CONFIG.get().unwrap().theme.primary.into_rgba())
                    .child(t!("actions.help").to_string())
                    .text_center()
                    .text_xl(),
            )
            .id("help_root")
            .size_full()
            .overflow_scroll()
            .child(
                uniform_list(
                    "key_bindings",
                    keymap.bindings().count(),
                    cx.processor(|_this, range: Range<usize>, _window, cx| {
                        let keymap = cx.key_bindings();
                        let keymap = keymap.borrow();
                        let key_bindings: Vec<_> = keymap.bindings().collect();

                        (range.start..range.end)
                            .map(|idx| {
                                let key_binding = key_bindings[idx].clone();

                                cx.new(|_| KeyBindingItem { key_binding })
                            })
                            .collect()
                    }),
                )
                .size_full(),
            )
    }
}

impl HelpWindow {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        window.set_window_title(&t!("actions.help"));

        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        Self { focus_handle }
    }
}

struct KeyBindingItem {
    key_binding: KeyBinding,
}

impl Render for KeyBindingItem {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let binding = &self.key_binding;

        div()
            .size_full()
            .flex()
            .p_1()
            .gap_1()
            .items_center()
            .text_color(CONFIG.get().unwrap().theme.foreground.into_rgba())
            .child(
                div()
                    .p_1()
                    .bg(CONFIG.get().unwrap().theme.surface.into_rgba())
                    .rounded_md()
                    .child(
                        binding
                            .keystrokes()
                            .iter()
                            .map(|keystroke| keystroke.key())
                            .collect::<Vec<_>>()
                            .join("-"),
                    ),
            )
            .child(
                t!(format!(
                    "actions.{}",
                    binding.action().name().to_lowercase()
                ))
                .to_string(),
            )
    }
}
