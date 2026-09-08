use rmk_types::keycode::HidKeyCode;

use super::{ResolvedStroke, Stroke, en};
use crate::universal_symbols::{Platform, RussianLetter, Symbol};

pub(super) const fn letter_keycode(letter: RussianLetter) -> HidKeyCode {
    match letter {
        RussianLetter::Kha => HidKeyCode::LeftBracket,
        RussianLetter::Be => HidKeyCode::Comma,
        RussianLetter::Yu => HidKeyCode::Dot,
        RussianLetter::HardSign => HidKeyCode::RightBracket,
    }
}

// macOS's "Russian - PC" layout (the common choice for Russian Mac users,
// and the one this firmware targets) deliberately mirrors Windows/Linux
// punctuation placement, so `stroke` doesn't need a per-platform branch here.
pub(super) const fn stroke(_platform: Platform, symbol: Symbol) -> ResolvedStroke {
    let direct = match symbol {
        Symbol::Dot => Some(Stroke::plain(HidKeyCode::Slash)),
        Symbol::Comma => Some(Stroke::shifted(HidKeyCode::Slash)),
        Symbol::Semicolon => Some(Stroke::shifted(HidKeyCode::Kc4)),
        Symbol::Colon => Some(Stroke::shifted(HidKeyCode::Kc6)),
        Symbol::Exclamation => Some(Stroke::shifted(HidKeyCode::Kc1)),
        Symbol::Question => Some(Stroke::shifted(HidKeyCode::Kc7)),
        Symbol::Slash => Some(Stroke::shifted(HidKeyCode::Backslash)),
        Symbol::Quote => Some(Stroke::shifted(HidKeyCode::Kc2)),
        Symbol::LeftParenthesis => Some(Stroke::shifted(HidKeyCode::Kc9)),
        Symbol::RightParenthesis => Some(Stroke::shifted(HidKeyCode::Kc0)),
        Symbol::Minus => Some(Stroke::plain(HidKeyCode::Minus)),
        Symbol::Plus => Some(Stroke::shifted(HidKeyCode::Equal)),
        Symbol::Asterisk => Some(Stroke::shifted(HidKeyCode::Kc8)),
        Symbol::Equal => Some(Stroke::plain(HidKeyCode::Equal)),
        Symbol::Percent => Some(Stroke::shifted(HidKeyCode::Kc5)),
        Symbol::Underscore => Some(Stroke::shifted(HidKeyCode::Minus)),
        _ => None,
    };

    match direct {
        Some(stroke) => ResolvedStroke::current(stroke),
        None => ResolvedStroke::english(en::stroke(symbol)),
    }
}
