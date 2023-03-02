use core::iter;

use usbd_hid::descriptor::KeyboardReport;

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Key {
    ErrorRollOver = 0x01,
    POSTFail = 0x02,
    ErrorUndefined = 0x03,

    /// Key a/A
    KeyA = 0x04,

    /// Key b/B
    KeyB = 0x05,

    /// Key c/C
    KeyC = 0x06,

    /// Key d/D
    KeyD = 0x07,

    /// Key e/E
    KeyE = 0x08,

    /// Key f/F
    KeyF = 0x09,

    /// Key g/G
    KeyG = 0x0A,

    /// Key h/H
    KeyH = 0x0B,

    /// Key i/I
    KeyI = 0x0C,

    /// Key j/J
    KeyJ = 0x0D,

    /// Key k/K
    KeyK = 0x0E,

    /// Key l/L
    KeyL = 0x0F,

    /// Key m/M
    KeyM = 0x10,

    /// Key n/N
    KeyN = 0x11,

    /// Key o/O
    KeyO = 0x12,

    /// Key p/P
    KeyP = 0x13,

    /// Key q/Q
    KeyQ = 0x14,

    /// Key r/R
    KeyR = 0x15,

    /// Key s/S
    KeyS = 0x16,

    /// Key t/T
    KeyT = 0x17,

    /// Key u/U
    KeyU = 0x18,

    /// Key v/V
    KeyV = 0x19,

    /// Key w/W
    KeyW = 0x1A,

    /// Key x/X
    KeyX = 0x1B,

    /// Key y/Y
    KeyY = 0x1C,

    /// Key z/Z
    KeyZ = 0x1D,

    /// Key 1/!
    Key1 = 0x1E,

    /// Key 2/"
    Key2 = 0x1F,

    /// Key 3/£
    Key3 = 0x20,

    /// Key 4/$
    Key4 = 0x21,

    /// Key 5/%
    Key5 = 0x22,

    /// Key 6/^
    Key6 = 0x23,

    /// Key 7/&
    Key7 = 0x24,

    /// Key 8/*
    Key8 = 0x25,

    /// Key 9/(
    Key9 = 0x26,

    /// Key 0/)
    Key0 = 0x27,

    /// Return key
    Return = 0x28,

    /// Escape key
    Esc = 0x29,

    /// Delete key
    Del = 0x2A,

    /// Tab key
    Tab = 0x2B,

    /// Spacebar
    Spacebar = 0x2C,

    /// Key -/_
    Minus = 0x2D,

    /// Key =/+
    Equals = 0x2E,

    /// Key [/{
    LeftSquare = 0x2F,

    /// Key ]/}
    RightSquare = 0x30,

    /// Key \/|
    Backslash = 0x31,

    /// Key #/~
    Hash = 0x32,

    /// Key ;/:
    SemiColon = 0x33,

    /// Key '/@
    Quote = 0x34,

    /// Key ,/<
    Comma = 0x36,

    /// Key ./>
    Dot = 0x37,
}
impl Key {
    pub fn try_from_char(c: char) -> Option<Self> {
        use Key::*;

        Some(match c.to_ascii_lowercase() {
            'a' => KeyA,
            'b' => KeyB,
            'c' => KeyC,
            'd' => KeyD,
            'e' => KeyE,
            'f' => KeyF,
            'g' => KeyG,
            'h' => KeyH,
            'i' => KeyI,
            'j' => KeyJ,
            'k' => KeyK,
            'l' => KeyL,
            'm' => KeyM,
            'n' => KeyN,
            'o' => KeyO,
            'p' => KeyP,
            'q' => KeyQ,
            'r' => KeyR,
            's' => KeyS,
            't' => KeyT,
            'u' => KeyU,
            'v' => KeyV,
            'w' => KeyW,
            'x' => KeyX,
            'y' => KeyY,
            'z' => KeyZ,
            '1' | '!' => Key1,
            '2' | '"' => Key2,
            '3' | '£' => Key3,
            '4' | '$' => Key4,
            '5' | '%' => Key5,
            '6' | '^' => Key6,
            '7' | '&' => Key7,
            '8' | '*' => Key8,
            '9' | '(' => Key9,
            '0' | ')' => Key0,
            '\n' => Return,
            '\t' => Tab,
            ' ' => Spacebar,
            '-' | '_' => Minus,
            '=' | '+' => Equals,
            '[' | '{' => LeftSquare,
            ']' | '}' => RightSquare,
            '\\' | '|' => Backslash,
            '#' | '~' => Hash,
            ';' | ':' => SemiColon,
            '\'' | '@' => Quote,
            ',' | '<' => Comma,
            '.' | '>' => Dot,
            _ => return None,
        })
    }

    pub fn get_modifier(c: char) -> Option<u8> {
        if c.is_ascii_uppercase() {
            return Some(0x02);
        }

        match c {
            '!' | '"' | '£' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '_' | '+' | '{' | '}'
            | '|' | '~' | ':' | '@' | '<' | '>' => Some(0x02),
            _ => None,
        }
    }
}

const RESET_KEYBOARD_REPORT: KeyboardReport = KeyboardReport {
    modifier: 0x00,
    reserved: 0x00,
    leds: 0x00,
    keycodes: [0x00; 6],
};

pub fn print_reports(s: &str) -> impl Iterator<Item = KeyboardReport> + '_ {
    s.chars()
        .filter_map(|c| Some((Key::try_from_char(c)?, Key::get_modifier(c))))
        .map(|(c, m)| KeyboardReport {
            modifier: m.unwrap_or(0x00),
            reserved: 0x0,
            leds: 0x0,
            keycodes: [c as u8, 0, 0, 0, 0, 0],
        })
        // Iterator::intersperse is unstable
        // (issue #79524 https://github.com/rust-lang/rust/issues/79524)
        .intersperse(RESET_KEYBOARD_REPORT)
        .chain(iter::once(RESET_KEYBOARD_REPORT))
}
