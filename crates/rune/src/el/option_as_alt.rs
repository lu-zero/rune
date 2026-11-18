//! Which Option keys are Alt while a terminal's caret has the input method (`El::ime_option_as_alt`).

/// The Option keys that send Alt instead of the character the layout makes with them, as
/// iTerm2's per-key Option setting and winit's `OptionAsAlt` choose. Only macOS reads it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum OptionAsAlt {
    #[default]
    None,
    Left,
    Right,
    Both,
}

impl From<bool> for OptionAsAlt {
    fn from(on: bool) -> Self {
        if on { Self::Both } else { Self::None }
    }
}
