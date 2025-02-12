#![warn(missing_docs)]

//! Utilities module provides set of commonly used algorithms.

pub mod astar;
pub mod behavior;
pub mod component;
pub mod lightmap;
pub mod navmesh;
pub mod raw_mesh;
pub mod uvgen;

use crate::{
    core::algebra::{Vector2, Vector3},
    event::{ElementState, MouseScrollDelta, WindowEvent},
    gui::{
        draw, message,
        message::{ButtonState, KeyboardModifiers, OsEvent},
    },
    keyboard::{KeyCode, ModifiersState},
    resource::texture::TextureResource,
};
use fyrox_ui::message::CursorIcon;
use half::f16;
use std::{any::Any, hash::Hasher, sync::Arc};

/// Translated key code to fyrox-ui key code.
pub fn translate_key(key: KeyCode) -> message::KeyCode {
    match key {
        KeyCode::Unidentified(_) => message::KeyCode::Unknown,
        KeyCode::Backquote => message::KeyCode::Backquote,
        KeyCode::Backslash => message::KeyCode::Backslash,
        KeyCode::BracketLeft => message::KeyCode::BracketLeft,
        KeyCode::BracketRight => message::KeyCode::BracketRight,
        KeyCode::Comma => message::KeyCode::Comma,
        KeyCode::Digit0 => message::KeyCode::Digit0,
        KeyCode::Digit1 => message::KeyCode::Digit1,
        KeyCode::Digit2 => message::KeyCode::Digit2,
        KeyCode::Digit3 => message::KeyCode::Digit3,
        KeyCode::Digit4 => message::KeyCode::Digit4,
        KeyCode::Digit5 => message::KeyCode::Digit5,
        KeyCode::Digit6 => message::KeyCode::Digit6,
        KeyCode::Digit7 => message::KeyCode::Digit7,
        KeyCode::Digit8 => message::KeyCode::Digit8,
        KeyCode::Digit9 => message::KeyCode::Digit9,
        KeyCode::Equal => message::KeyCode::Equal,
        KeyCode::IntlBackslash => message::KeyCode::IntlBackslash,
        KeyCode::IntlRo => message::KeyCode::IntlRo,
        KeyCode::IntlYen => message::KeyCode::IntlYen,
        KeyCode::KeyA => message::KeyCode::KeyA,
        KeyCode::KeyB => message::KeyCode::KeyB,
        KeyCode::KeyC => message::KeyCode::KeyC,
        KeyCode::KeyD => message::KeyCode::KeyD,
        KeyCode::KeyE => message::KeyCode::KeyE,
        KeyCode::KeyF => message::KeyCode::KeyF,
        KeyCode::KeyG => message::KeyCode::KeyG,
        KeyCode::KeyH => message::KeyCode::KeyH,
        KeyCode::KeyI => message::KeyCode::KeyI,
        KeyCode::KeyJ => message::KeyCode::KeyJ,
        KeyCode::KeyK => message::KeyCode::KeyK,
        KeyCode::KeyL => message::KeyCode::KeyL,
        KeyCode::KeyM => message::KeyCode::KeyM,
        KeyCode::KeyN => message::KeyCode::KeyN,
        KeyCode::KeyO => message::KeyCode::KeyO,
        KeyCode::KeyP => message::KeyCode::KeyP,
        KeyCode::KeyQ => message::KeyCode::KeyQ,
        KeyCode::KeyR => message::KeyCode::KeyR,
        KeyCode::KeyS => message::KeyCode::KeyS,
        KeyCode::KeyT => message::KeyCode::KeyT,
        KeyCode::KeyU => message::KeyCode::KeyU,
        KeyCode::KeyV => message::KeyCode::KeyV,
        KeyCode::KeyW => message::KeyCode::KeyW,
        KeyCode::KeyX => message::KeyCode::KeyX,
        KeyCode::KeyY => message::KeyCode::KeyY,
        KeyCode::KeyZ => message::KeyCode::KeyZ,
        KeyCode::Minus => message::KeyCode::Minus,
        KeyCode::Period => message::KeyCode::Period,
        KeyCode::Quote => message::KeyCode::Quote,
        KeyCode::Semicolon => message::KeyCode::Semicolon,
        KeyCode::Slash => message::KeyCode::Slash,
        KeyCode::AltLeft => message::KeyCode::AltLeft,
        KeyCode::AltRight => message::KeyCode::AltRight,
        KeyCode::Backspace => message::KeyCode::Backspace,
        KeyCode::CapsLock => message::KeyCode::CapsLock,
        KeyCode::ContextMenu => message::KeyCode::ContextMenu,
        KeyCode::ControlLeft => message::KeyCode::ControlLeft,
        KeyCode::ControlRight => message::KeyCode::ControlRight,
        KeyCode::Enter => message::KeyCode::Enter,
        KeyCode::SuperLeft => message::KeyCode::SuperLeft,
        KeyCode::SuperRight => message::KeyCode::SuperRight,
        KeyCode::ShiftLeft => message::KeyCode::ShiftLeft,
        KeyCode::ShiftRight => message::KeyCode::ShiftRight,
        KeyCode::Space => message::KeyCode::Space,
        KeyCode::Tab => message::KeyCode::Tab,
        KeyCode::Convert => message::KeyCode::Convert,
        KeyCode::KanaMode => message::KeyCode::KanaMode,
        KeyCode::Lang1 => message::KeyCode::Lang1,
        KeyCode::Lang2 => message::KeyCode::Lang2,
        KeyCode::Lang3 => message::KeyCode::Lang3,
        KeyCode::Lang4 => message::KeyCode::Lang4,
        KeyCode::Lang5 => message::KeyCode::Lang5,
        KeyCode::NonConvert => message::KeyCode::NonConvert,
        KeyCode::Delete => message::KeyCode::Delete,
        KeyCode::End => message::KeyCode::End,
        KeyCode::Help => message::KeyCode::Help,
        KeyCode::Home => message::KeyCode::Home,
        KeyCode::Insert => message::KeyCode::Insert,
        KeyCode::PageDown => message::KeyCode::PageDown,
        KeyCode::PageUp => message::KeyCode::PageUp,
        KeyCode::ArrowDown => message::KeyCode::ArrowDown,
        KeyCode::ArrowLeft => message::KeyCode::ArrowLeft,
        KeyCode::ArrowRight => message::KeyCode::ArrowRight,
        KeyCode::ArrowUp => message::KeyCode::ArrowUp,
        KeyCode::NumLock => message::KeyCode::NumLock,
        KeyCode::Numpad0 => message::KeyCode::Numpad0,
        KeyCode::Numpad1 => message::KeyCode::Numpad1,
        KeyCode::Numpad2 => message::KeyCode::Numpad2,
        KeyCode::Numpad3 => message::KeyCode::Numpad3,
        KeyCode::Numpad4 => message::KeyCode::Numpad4,
        KeyCode::Numpad5 => message::KeyCode::Numpad5,
        KeyCode::Numpad6 => message::KeyCode::Numpad6,
        KeyCode::Numpad7 => message::KeyCode::Numpad7,
        KeyCode::Numpad8 => message::KeyCode::Numpad8,
        KeyCode::Numpad9 => message::KeyCode::Numpad9,
        KeyCode::NumpadAdd => message::KeyCode::NumpadAdd,
        KeyCode::NumpadBackspace => message::KeyCode::NumpadBackspace,
        KeyCode::NumpadClear => message::KeyCode::NumpadClear,
        KeyCode::NumpadClearEntry => message::KeyCode::NumpadClearEntry,
        KeyCode::NumpadComma => message::KeyCode::NumpadComma,
        KeyCode::NumpadDecimal => message::KeyCode::NumpadDecimal,
        KeyCode::NumpadDivide => message::KeyCode::NumpadDivide,
        KeyCode::NumpadEnter => message::KeyCode::NumpadEnter,
        KeyCode::NumpadEqual => message::KeyCode::NumpadEqual,
        KeyCode::NumpadHash => message::KeyCode::NumpadHash,
        KeyCode::NumpadMemoryAdd => message::KeyCode::NumpadMemoryAdd,
        KeyCode::NumpadMemoryClear => message::KeyCode::NumpadMemoryClear,
        KeyCode::NumpadMemoryRecall => message::KeyCode::NumpadMemoryRecall,
        KeyCode::NumpadMemoryStore => message::KeyCode::NumpadMemoryStore,
        KeyCode::NumpadMemorySubtract => message::KeyCode::NumpadMemorySubtract,
        KeyCode::NumpadMultiply => message::KeyCode::NumpadMultiply,
        KeyCode::NumpadParenLeft => message::KeyCode::NumpadParenLeft,
        KeyCode::NumpadParenRight => message::KeyCode::NumpadParenRight,
        KeyCode::NumpadStar => message::KeyCode::NumpadStar,
        KeyCode::NumpadSubtract => message::KeyCode::NumpadSubtract,
        KeyCode::Escape => message::KeyCode::Escape,
        KeyCode::Fn => message::KeyCode::Fn,
        KeyCode::FnLock => message::KeyCode::FnLock,
        KeyCode::PrintScreen => message::KeyCode::PrintScreen,
        KeyCode::ScrollLock => message::KeyCode::ScrollLock,
        KeyCode::Pause => message::KeyCode::Pause,
        KeyCode::BrowserBack => message::KeyCode::BrowserBack,
        KeyCode::BrowserFavorites => message::KeyCode::BrowserFavorites,
        KeyCode::BrowserForward => message::KeyCode::BrowserForward,
        KeyCode::BrowserHome => message::KeyCode::BrowserHome,
        KeyCode::BrowserRefresh => message::KeyCode::BrowserRefresh,
        KeyCode::BrowserSearch => message::KeyCode::BrowserSearch,
        KeyCode::BrowserStop => message::KeyCode::BrowserStop,
        KeyCode::Eject => message::KeyCode::Eject,
        KeyCode::LaunchApp1 => message::KeyCode::LaunchApp1,
        KeyCode::LaunchApp2 => message::KeyCode::LaunchApp2,
        KeyCode::LaunchMail => message::KeyCode::LaunchMail,
        KeyCode::MediaPlayPause => message::KeyCode::MediaPlayPause,
        KeyCode::MediaSelect => message::KeyCode::MediaSelect,
        KeyCode::MediaStop => message::KeyCode::MediaStop,
        KeyCode::MediaTrackNext => message::KeyCode::MediaTrackNext,
        KeyCode::MediaTrackPrevious => message::KeyCode::MediaTrackPrevious,
        KeyCode::Power => message::KeyCode::Power,
        KeyCode::Sleep => message::KeyCode::Sleep,
        KeyCode::AudioVolumeDown => message::KeyCode::AudioVolumeDown,
        KeyCode::AudioVolumeMute => message::KeyCode::AudioVolumeMute,
        KeyCode::AudioVolumeUp => message::KeyCode::AudioVolumeUp,
        KeyCode::WakeUp => message::KeyCode::WakeUp,
        KeyCode::Meta => message::KeyCode::Meta,
        KeyCode::Hyper => message::KeyCode::Hyper,
        KeyCode::Turbo => message::KeyCode::Turbo,
        KeyCode::Abort => message::KeyCode::Abort,
        KeyCode::Resume => message::KeyCode::Resume,
        KeyCode::Suspend => message::KeyCode::Suspend,
        KeyCode::Again => message::KeyCode::Again,
        KeyCode::Copy => message::KeyCode::Copy,
        KeyCode::Cut => message::KeyCode::Cut,
        KeyCode::Find => message::KeyCode::Find,
        KeyCode::Open => message::KeyCode::Open,
        KeyCode::Paste => message::KeyCode::Paste,
        KeyCode::Props => message::KeyCode::Props,
        KeyCode::Select => message::KeyCode::Select,
        KeyCode::Undo => message::KeyCode::Undo,
        KeyCode::Hiragana => message::KeyCode::Hiragana,
        KeyCode::Katakana => message::KeyCode::Katakana,
        KeyCode::F1 => message::KeyCode::F1,
        KeyCode::F2 => message::KeyCode::F2,
        KeyCode::F3 => message::KeyCode::F3,
        KeyCode::F4 => message::KeyCode::F4,
        KeyCode::F5 => message::KeyCode::F5,
        KeyCode::F6 => message::KeyCode::F6,
        KeyCode::F7 => message::KeyCode::F7,
        KeyCode::F8 => message::KeyCode::F8,
        KeyCode::F9 => message::KeyCode::F9,
        KeyCode::F10 => message::KeyCode::F10,
        KeyCode::F11 => message::KeyCode::F11,
        KeyCode::F12 => message::KeyCode::F12,
        KeyCode::F13 => message::KeyCode::F13,
        KeyCode::F14 => message::KeyCode::F14,
        KeyCode::F15 => message::KeyCode::F15,
        KeyCode::F16 => message::KeyCode::F16,
        KeyCode::F17 => message::KeyCode::F17,
        KeyCode::F18 => message::KeyCode::F18,
        KeyCode::F19 => message::KeyCode::F19,
        KeyCode::F20 => message::KeyCode::F20,
        KeyCode::F21 => message::KeyCode::F21,
        KeyCode::F22 => message::KeyCode::F22,
        KeyCode::F23 => message::KeyCode::F23,
        KeyCode::F24 => message::KeyCode::F24,
        KeyCode::F25 => message::KeyCode::F25,
        KeyCode::F26 => message::KeyCode::F26,
        KeyCode::F27 => message::KeyCode::F27,
        KeyCode::F28 => message::KeyCode::F28,
        KeyCode::F29 => message::KeyCode::F29,
        KeyCode::F30 => message::KeyCode::F30,
        KeyCode::F31 => message::KeyCode::F31,
        KeyCode::F32 => message::KeyCode::F32,
        KeyCode::F33 => message::KeyCode::F33,
        KeyCode::F34 => message::KeyCode::F34,
        KeyCode::F35 => message::KeyCode::F35,
        _ => message::KeyCode::Unknown,
    }
}

/// Translates cursor icon from fyrox-ui library to glutin format.
pub fn translate_cursor_icon(icon: CursorIcon) -> crate::window::CursorIcon {
    match icon {
        CursorIcon::Default => crate::window::CursorIcon::Default,
        CursorIcon::Crosshair => crate::window::CursorIcon::Crosshair,
        CursorIcon::Move => crate::window::CursorIcon::Move,
        CursorIcon::Text => crate::window::CursorIcon::Text,
        CursorIcon::Wait => crate::window::CursorIcon::Wait,
        CursorIcon::Help => crate::window::CursorIcon::Help,
        CursorIcon::Progress => crate::window::CursorIcon::Progress,
        CursorIcon::NotAllowed => crate::window::CursorIcon::NotAllowed,
        CursorIcon::ContextMenu => crate::window::CursorIcon::ContextMenu,
        CursorIcon::Cell => crate::window::CursorIcon::Cell,
        CursorIcon::VerticalText => crate::window::CursorIcon::VerticalText,
        CursorIcon::Alias => crate::window::CursorIcon::Alias,
        CursorIcon::Copy => crate::window::CursorIcon::Copy,
        CursorIcon::NoDrop => crate::window::CursorIcon::NoDrop,
        CursorIcon::Grab => crate::window::CursorIcon::Grab,
        CursorIcon::Grabbing => crate::window::CursorIcon::Grabbing,
        CursorIcon::AllScroll => crate::window::CursorIcon::AllScroll,
        CursorIcon::ZoomIn => crate::window::CursorIcon::ZoomIn,
        CursorIcon::ZoomOut => crate::window::CursorIcon::ZoomOut,
        CursorIcon::EResize => crate::window::CursorIcon::EResize,
        CursorIcon::NResize => crate::window::CursorIcon::NResize,
        CursorIcon::NeResize => crate::window::CursorIcon::NeResize,
        CursorIcon::NwResize => crate::window::CursorIcon::NwResize,
        CursorIcon::SResize => crate::window::CursorIcon::SResize,
        CursorIcon::SeResize => crate::window::CursorIcon::SeResize,
        CursorIcon::SwResize => crate::window::CursorIcon::SwResize,
        CursorIcon::WResize => crate::window::CursorIcon::WResize,
        CursorIcon::EwResize => crate::window::CursorIcon::EwResize,
        CursorIcon::NsResize => crate::window::CursorIcon::NsResize,
        CursorIcon::NeswResize => crate::window::CursorIcon::NeswResize,
        CursorIcon::NwseResize => crate::window::CursorIcon::NwseResize,
        CursorIcon::ColResize => crate::window::CursorIcon::ColResize,
        CursorIcon::RowResize => crate::window::CursorIcon::RowResize,
        CursorIcon::Pointer => crate::window::CursorIcon::Pointer,
    }
}

/// Translates window mouse button into fyrox-ui mouse button.
pub fn translate_button(button: crate::event::MouseButton) -> crate::gui::message::MouseButton {
    match button {
        crate::event::MouseButton::Left => crate::gui::message::MouseButton::Left,
        crate::event::MouseButton::Right => crate::gui::message::MouseButton::Right,
        crate::event::MouseButton::Middle => crate::gui::message::MouseButton::Middle,
        crate::event::MouseButton::Forward => crate::gui::message::MouseButton::Forward,
        crate::event::MouseButton::Back => crate::gui::message::MouseButton::Back,
        crate::event::MouseButton::Other(i) => crate::gui::message::MouseButton::Other(i),
    }
}

/// Translates library button state into fyrox-ui button state.
pub fn translate_state(state: ElementState) -> ButtonState {
    match state {
        ElementState::Pressed => ButtonState::Pressed,
        ElementState::Released => ButtonState::Released,
    }
}

/// Translates window event to fyrox-ui event.
pub fn translate_event(event: &WindowEvent) -> Option<OsEvent> {
    match event {
        WindowEvent::KeyboardInput { event, .. } => Some(OsEvent::KeyboardInput {
            button: translate_key(event.physical_key),
            state: translate_state(event.state),
            text: event
                .text
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default(),
        }),
        WindowEvent::CursorMoved { position, .. } => Some(OsEvent::CursorMoved {
            position: Vector2::new(position.x as f32, position.y as f32),
        }),
        WindowEvent::MouseWheel { delta, .. } => match delta {
            MouseScrollDelta::LineDelta(x, y) => Some(OsEvent::MouseWheel(*x, *y)),
            MouseScrollDelta::PixelDelta(pos) => {
                Some(OsEvent::MouseWheel(pos.x as f32, pos.y as f32))
            }
        },
        WindowEvent::MouseInput { state, button, .. } => Some(OsEvent::MouseInput {
            button: translate_button(*button),
            state: translate_state(*state),
        }),
        &WindowEvent::ModifiersChanged(modifiers) => Some(OsEvent::KeyboardModifiers(
            translate_keyboard_modifiers(modifiers.state()),
        )),
        _ => None,
    }
}

/// Translates keyboard modifiers to fyrox-ui keyboard modifiers.
pub fn translate_keyboard_modifiers(modifiers: ModifiersState) -> KeyboardModifiers {
    KeyboardModifiers {
        alt: modifiers.alt_key(),
        shift: modifiers.shift_key(),
        control: modifiers.control_key(),
        system: modifiers.super_key(),
    }
}

/// Maps key code to its name. Can be useful if you making adjustable key bindings in your
/// game and you need quickly map key code to its name.
pub fn virtual_key_code_name(code: KeyCode) -> &'static str {
    match code {
        KeyCode::Unidentified(_) => "Unidentified",
        KeyCode::Backquote => "Backquote",
        KeyCode::Backslash => "Backslash",
        KeyCode::BracketLeft => "BracketLeft",
        KeyCode::BracketRight => "BracketRight",
        KeyCode::Comma => "Comma",
        KeyCode::Digit0 => "0",
        KeyCode::Digit1 => "1",
        KeyCode::Digit2 => "2",
        KeyCode::Digit3 => "3",
        KeyCode::Digit4 => "4",
        KeyCode::Digit5 => "5",
        KeyCode::Digit6 => "6",
        KeyCode::Digit7 => "7",
        KeyCode::Digit8 => "8",
        KeyCode::Digit9 => "9",
        KeyCode::Equal => "Equal",
        KeyCode::IntlBackslash => "IntlBackslash",
        KeyCode::IntlRo => "IntlRo",
        KeyCode::IntlYen => "IntlYen",
        KeyCode::KeyA => "A",
        KeyCode::KeyB => "B",
        KeyCode::KeyC => "C",
        KeyCode::KeyD => "D",
        KeyCode::KeyE => "E",
        KeyCode::KeyF => "F",
        KeyCode::KeyG => "G",
        KeyCode::KeyH => "H",
        KeyCode::KeyI => "I",
        KeyCode::KeyJ => "J",
        KeyCode::KeyK => "K",
        KeyCode::KeyL => "L",
        KeyCode::KeyM => "M",
        KeyCode::KeyN => "N",
        KeyCode::KeyO => "O",
        KeyCode::KeyP => "P",
        KeyCode::KeyQ => "Q",
        KeyCode::KeyR => "R",
        KeyCode::KeyS => "S",
        KeyCode::KeyT => "T",
        KeyCode::KeyU => "U",
        KeyCode::KeyV => "V",
        KeyCode::KeyW => "W",
        KeyCode::KeyX => "X",
        KeyCode::KeyY => "Y",
        KeyCode::KeyZ => "Z",
        KeyCode::Minus => "Minus",
        KeyCode::Period => "Period",
        KeyCode::Quote => "Quote",
        KeyCode::Semicolon => "Semicolon",
        KeyCode::Slash => "Slash",
        KeyCode::AltLeft => "AltLeft",
        KeyCode::AltRight => "AltRight",
        KeyCode::Backspace => "Backspace",
        KeyCode::CapsLock => "CapsLock",
        KeyCode::ContextMenu => "ContextMenu",
        KeyCode::ControlLeft => "ControlLeft",
        KeyCode::ControlRight => "ControlRight",
        KeyCode::Enter => "Enter",
        KeyCode::SuperLeft => "SuperLeft",
        KeyCode::SuperRight => "SuperRight",
        KeyCode::ShiftLeft => "ShiftLeft",
        KeyCode::ShiftRight => "ShiftRight",
        KeyCode::Space => "Space",
        KeyCode::Tab => "Tab",
        KeyCode::Convert => "Convert",
        KeyCode::KanaMode => "KanaMode",
        KeyCode::Lang1 => "Lang1",
        KeyCode::Lang2 => "Lang2",
        KeyCode::Lang3 => "Lang3",
        KeyCode::Lang4 => "Lang4",
        KeyCode::Lang5 => "Lang5",
        KeyCode::NonConvert => "NonConvert",
        KeyCode::Delete => "Delete",
        KeyCode::End => "End",
        KeyCode::Help => "Help",
        KeyCode::Home => "Home",
        KeyCode::Insert => "Insert",
        KeyCode::PageDown => "PageDown",
        KeyCode::PageUp => "PageUp",
        KeyCode::ArrowDown => "ArrowDown",
        KeyCode::ArrowLeft => "ArrowLeft",
        KeyCode::ArrowRight => "ArrowRight",
        KeyCode::ArrowUp => "ArrowUp",
        KeyCode::NumLock => "NumLock",
        KeyCode::Numpad0 => "Numpad0",
        KeyCode::Numpad1 => "Numpad1",
        KeyCode::Numpad2 => "Numpad2",
        KeyCode::Numpad3 => "Numpad3",
        KeyCode::Numpad4 => "Numpad4",
        KeyCode::Numpad5 => "Numpad5",
        KeyCode::Numpad6 => "Numpad6",
        KeyCode::Numpad7 => "Numpad7",
        KeyCode::Numpad8 => "Numpad8",
        KeyCode::Numpad9 => "Numpad9",
        KeyCode::NumpadAdd => "NumpadAdd",
        KeyCode::NumpadBackspace => "NumpadBackspace",
        KeyCode::NumpadClear => "NumpadClear",
        KeyCode::NumpadClearEntry => "NumpadClearEntry",
        KeyCode::NumpadComma => "NumpadComma",
        KeyCode::NumpadDecimal => "NumpadDecimal",
        KeyCode::NumpadDivide => "NumpadDivide",
        KeyCode::NumpadEnter => "NumpadEnter",
        KeyCode::NumpadEqual => "NumpadEqual",
        KeyCode::NumpadHash => "NumpadHash",
        KeyCode::NumpadMemoryAdd => "NumpadMemoryAdd",
        KeyCode::NumpadMemoryClear => "NumpadMemoryClear",
        KeyCode::NumpadMemoryRecall => "NumpadMemoryRecall",
        KeyCode::NumpadMemoryStore => "NumpadMemoryStore",
        KeyCode::NumpadMemorySubtract => "NumpadMemorySubtract",
        KeyCode::NumpadMultiply => "NumpadMultiply",
        KeyCode::NumpadParenLeft => "NumpadParenLeft",
        KeyCode::NumpadParenRight => "NumpadParenRight",
        KeyCode::NumpadStar => "NumpadStar",
        KeyCode::NumpadSubtract => "NumpadSubtract",
        KeyCode::Escape => "Escape",
        KeyCode::Fn => "Fn",
        KeyCode::FnLock => "FnLock",
        KeyCode::PrintScreen => "PrintScreen",
        KeyCode::ScrollLock => "ScrollLock",
        KeyCode::Pause => "Pause",
        KeyCode::BrowserBack => "BrowserBack",
        KeyCode::BrowserFavorites => "BrowserFavorites",
        KeyCode::BrowserForward => "BrowserForward",
        KeyCode::BrowserHome => "BrowserHome",
        KeyCode::BrowserRefresh => "BrowserRefresh",
        KeyCode::BrowserSearch => "BrowserSearch",
        KeyCode::BrowserStop => "BrowserStop",
        KeyCode::Eject => "Eject",
        KeyCode::LaunchApp1 => "LaunchApp1",
        KeyCode::LaunchApp2 => "LaunchApp2",
        KeyCode::LaunchMail => "LaunchMail",
        KeyCode::MediaPlayPause => "MediaPlayPause",
        KeyCode::MediaSelect => "MediaSelect",
        KeyCode::MediaStop => "MediaStop",
        KeyCode::MediaTrackNext => "MediaTrackNext",
        KeyCode::MediaTrackPrevious => "MediaTrackPrevious",
        KeyCode::Power => "Power",
        KeyCode::Sleep => "Sleep",
        KeyCode::AudioVolumeDown => "AudioVolumeDown",
        KeyCode::AudioVolumeMute => "AudioVolumeMute",
        KeyCode::AudioVolumeUp => "AudioVolumeUp",
        KeyCode::WakeUp => "WakeUp",
        KeyCode::Meta => "Meta",
        KeyCode::Hyper => "Hyper",
        KeyCode::Turbo => "Turbo",
        KeyCode::Abort => "Abort",
        KeyCode::Resume => "Resume",
        KeyCode::Suspend => "Suspend",
        KeyCode::Again => "Again",
        KeyCode::Copy => "Copy",
        KeyCode::Cut => "Cut",
        KeyCode::Find => "Find",
        KeyCode::Open => "Open",
        KeyCode::Paste => "Paste",
        KeyCode::Props => "Props",
        KeyCode::Select => "Select",
        KeyCode::Undo => "Undo",
        KeyCode::Hiragana => "Hiragana",
        KeyCode::Katakana => "Katakana",
        KeyCode::F1 => "F1",
        KeyCode::F2 => "F2",
        KeyCode::F3 => "F3",
        KeyCode::F4 => "F4",
        KeyCode::F5 => "F5",
        KeyCode::F6 => "F6",
        KeyCode::F7 => "F7",
        KeyCode::F8 => "F8",
        KeyCode::F9 => "F9",
        KeyCode::F10 => "F10",
        KeyCode::F11 => "F11",
        KeyCode::F12 => "F12",
        KeyCode::F13 => "F13",
        KeyCode::F14 => "F14",
        KeyCode::F15 => "F15",
        KeyCode::F16 => "F16",
        KeyCode::F17 => "F17",
        KeyCode::F18 => "F18",
        KeyCode::F19 => "F19",
        KeyCode::F20 => "F20",
        KeyCode::F21 => "F21",
        KeyCode::F22 => "F22",
        KeyCode::F23 => "F23",
        KeyCode::F24 => "F24",
        KeyCode::F25 => "F25",
        KeyCode::F26 => "F26",
        KeyCode::F27 => "F27",
        KeyCode::F28 => "F28",
        KeyCode::F29 => "F29",
        KeyCode::F30 => "F30",
        KeyCode::F31 => "F31",
        KeyCode::F32 => "F32",
        KeyCode::F33 => "F33",
        KeyCode::F34 => "F34",
        KeyCode::F35 => "F35",
        _ => "Unknown",
    }
}

/// Helper function to convert `Option<Arc<T>>` to `Option<Arc<dyn Any>>`.
#[allow(clippy::manual_map)]
pub fn into_any_arc<T: Any + Send + Sync>(
    opt: Option<Arc<T>>,
) -> Option<Arc<dyn Any + Send + Sync>> {
    match opt {
        Some(r) => Some(r),
        None => None,
    }
}

/// Converts engine's optional texture "pointer" to fyrox-ui's.
pub fn into_gui_texture(this: TextureResource) -> draw::SharedTexture {
    draw::SharedTexture(this.into_untyped().0)
}

/// "Transmutes" array of any sized type to a slice of bytes.
pub fn array_as_u8_slice<T: Sized>(v: &[T]) -> &'_ [u8] {
    // SAFETY: It is safe to reinterpret data to read it.
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}

/// "Transmutes" value of any sized type to a slice of bytes.
pub fn value_as_u8_slice<T: Sized>(v: &T) -> &'_ [u8] {
    // SAFETY: It is safe to reinterpret data to read it.
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, std::mem::size_of::<T>()) }
}

/// Takes a vector of trivially-copyable values and turns it into a vector of bytes.
pub fn transmute_vec_as_bytes<T: Copy>(vec: Vec<T>) -> Vec<u8> {
    unsafe {
        let mut vec = std::mem::ManuallyDrop::new(vec);
        Vec::from_raw_parts(
            vec.as_mut_ptr() as *mut u8,
            vec.len() * std::mem::size_of::<T>(),
            vec.capacity() * std::mem::size_of::<T>(),
        )
    }
}

/// Performs hashing of a sized value by interpreting it as raw memory.
pub fn hash_as_bytes<T: Sized, H: Hasher>(value: &T, hasher: &mut H) {
    hasher.write(value_as_u8_slice(value))
}

/// A trait for entities that have name.
pub trait NameProvider {
    /// Returns a reference to the name of the entity.
    fn name(&self) -> &str;
}

/// Tries to find an entity by its name in a series of entities produced by an iterator.
pub fn find_by_name_ref<'a, T, I, S, K>(mut iter: I, name: S) -> Option<(K, &'a T)>
where
    T: NameProvider,
    I: Iterator<Item = (K, &'a T)>,
    S: AsRef<str>,
{
    iter.find(|(_, value)| value.name() == name.as_ref())
}

/// Tries to find an entity by its name in a series of entities produced by an iterator.
pub fn find_by_name_mut<'a, T, I, S, K>(mut iter: I, name: S) -> Option<(K, &'a mut T)>
where
    T: NameProvider,
    I: Iterator<Item = (K, &'a mut T)>,
    S: AsRef<str>,
{
    iter.find(|(_, value)| value.name() == name.as_ref())
}

/// Converts `Vector3<f32>` -> `Vector3<f16>`.
pub fn vec3_f16_from_f32(v: Vector3<f32>) -> Vector3<f16> {
    v.map(f16::from_f32)
}

/// Converts `Vector3<f16>` -> `Vector3<f32>`.
pub fn vec3_f32_from_f16(v: Vector3<f16>) -> Vector3<f32> {
    v.map(|v| v.to_f32())
}
