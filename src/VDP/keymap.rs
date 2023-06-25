use sdl2::keyboard::{Scancode, Mod};
pub trait KeyboardLayout {
    fn sdl_scancode_to_fbgl_virtual_key(&self, scancode: &Scancode, keymod: &Mod) -> FabGlVirtualKey;
}

pub struct KeyboardLayoutUS;

impl KeyboardLayout for KeyboardLayoutUS {
    fn sdl_scancode_to_fbgl_virtual_key(&self, scancode: &Scancode, keymod: &Mod) -> FabGlVirtualKey {
        return sdl_scancode_to_fbgl_virtual_key(scancode, keymod);
    }
}
pub struct KeyboardLayoutDE;

impl KeyboardLayout for KeyboardLayoutDE {
    fn sdl_scancode_to_fbgl_virtual_key(&self, scancode: &Scancode, keymod: &Mod) -> FabGlVirtualKey {
        let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
        let caps_lock = keymod.intersects(Mod::CAPSMOD);
        let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);

        use FabGlVirtualKey as FVK;

        match scancode {
            Scancode::Grave /* 0x0E */ => if shift {FVK::VK_DEGREE} else {FVK::VK_CARET},
            Scancode::Minus /* 0x4E */ => if alt {FVK::VK_BACKSLASH} else {if shift {FVK::VK_QUESTION} else {FVK::VK_ESZETT}},
            Scancode::Equals /* 0x55 */ => FVK::VK_ACUTEACCENT,
            Scancode::LeftBracket /* 0x54 */ => FVK::VK_UMLAUT_u,
            Scancode::RightBracket /* 0x5B */ => if alt {FVK::VK_TILDE} else {if shift {FVK::VK_ASTERISK} else {FVK::VK_PLUS}},
            Scancode::Semicolon /* 0x4C */ => FVK::VK_UMLAUT_o,
            Scancode::Apostrophe /* 0x52 */ => FVK::VK_UMLAUT_a,
            Scancode::Backslash /* 0x5D */ => if shift {FVK::VK_QUOTE} else {FVK::VK_HASH},
            Scancode::NonUsBackslash /* 0x61 */ => if alt {FVK::VK_VERTICALBAR} else {if shift {FVK::VK_GREATER} else {FVK::VK_LESS}},
            Scancode::Slash /* 0x4A */ => if shift {FVK::VK_UNDERSCORE} else {FVK::VK_MINUS},
            Scancode::Period => if shift {FVK::VK_COLON} else {FVK::VK_PERIOD},
            Scancode::Comma => if shift {FVK::VK_SEMICOLON} else {FVK::VK_COMMA},
            Scancode::Y /* 0x35 */ => if shift | caps_lock {FVK::VK_Z} else {FVK::VK_z},
            Scancode::Z /* 0x1A */ => if shift | caps_lock {FVK::VK_Y} else {FVK::VK_y},
            Scancode::E => if alt {FVK::VK_EURO} else {if shift | caps_lock {FVK::VK_E} else {FVK::VK_e}},
            Scancode::Q => if alt {FVK::VK_AT} else {if shift | caps_lock {FVK::VK_Q} else {FVK::VK_q}},

            Scancode::Num2 => if shift {FVK::VK_QUOTEDBL} else {FVK::VK_2},
            Scancode::Num3 => if shift {FVK::VK_SECTION} else {FVK::VK_3},
            Scancode::Num6 => if shift {FVK::VK_AMPERSAND} else {FVK::VK_6},
            Scancode::Num7 => if alt {FVK::VK_LEFTBRACE} else {if shift {FVK::VK_SLASH} else {FVK::VK_7}},
            Scancode::Num8 => if alt {FVK::VK_LEFTBRACKET} else {if shift {FVK::VK_LEFTPAREN} else {FVK::VK_8}},
            Scancode::Num9 => if alt {FVK::VK_RIGHTBRACKET} else {if shift {FVK::VK_RIGHTPAREN} else {FVK::VK_9}},
            Scancode::Num0 => if alt {FVK::VK_RIGHTBRACE} else {if shift {FVK::VK_EQUALS} else {FVK::VK_0}},

            _ => return sdl_scancode_to_fbgl_virtual_key(scancode, keymod),
        }
    }
}

pub fn fabgl_virtual_key_to_ascii(fabgl_vk: &FabGlVirtualKey) -> u8 {
    match fabgl_vk {
        FabGlVirtualKey::VK_SPACE => ' ' as u8,
        FabGlVirtualKey::VK_0 => '0' as u8,
        FabGlVirtualKey::VK_1 => '1' as u8,
        FabGlVirtualKey::VK_2 => '2' as u8,
        FabGlVirtualKey::VK_3 => '3' as u8,
        FabGlVirtualKey::VK_4 => '4' as u8,
        FabGlVirtualKey::VK_5 => '5' as u8,
        FabGlVirtualKey::VK_6 => '6' as u8,
        FabGlVirtualKey::VK_7 => '7' as u8,
        FabGlVirtualKey::VK_8 => '8' as u8,
        FabGlVirtualKey::VK_9 => '9' as u8,
        FabGlVirtualKey::VK_KP_0 => '0' as u8,
        FabGlVirtualKey::VK_KP_1 => '1' as u8,
        FabGlVirtualKey::VK_KP_2 => '2' as u8,
        FabGlVirtualKey::VK_KP_3 => '3' as u8,
        FabGlVirtualKey::VK_KP_4 => '4' as u8,
        FabGlVirtualKey::VK_KP_5 => '5' as u8,
        FabGlVirtualKey::VK_KP_6 => '6' as u8,
        FabGlVirtualKey::VK_KP_7 => '7' as u8,
        FabGlVirtualKey::VK_KP_8 => '8' as u8,
        FabGlVirtualKey::VK_KP_9 => '9' as u8,
        FabGlVirtualKey::VK_KP_PERIOD => '.' as u8,
        FabGlVirtualKey::VK_KP_DIVIDE => '/' as u8,
        FabGlVirtualKey::VK_KP_MULTIPLY => '*' as u8,
        FabGlVirtualKey::VK_KP_MINUS => '-' as u8,
        FabGlVirtualKey::VK_KP_PLUS => '+' as u8,
        FabGlVirtualKey::VK_KP_ENTER => 0x0D as u8,

        FabGlVirtualKey::VK_a => 'a' as u8,
        FabGlVirtualKey::VK_b => 'b' as u8,
        FabGlVirtualKey::VK_c => 'c' as u8,
        FabGlVirtualKey::VK_d => 'd' as u8,
        FabGlVirtualKey::VK_e => 'e' as u8,
        FabGlVirtualKey::VK_f => 'f' as u8,
        FabGlVirtualKey::VK_g => 'g' as u8,
        FabGlVirtualKey::VK_h => 'h' as u8,
        FabGlVirtualKey::VK_i => 'i' as u8,
        FabGlVirtualKey::VK_j => 'j' as u8,
        FabGlVirtualKey::VK_k => 'k' as u8,
        FabGlVirtualKey::VK_l => 'l' as u8,
        FabGlVirtualKey::VK_m => 'm' as u8,
        FabGlVirtualKey::VK_n => 'n' as u8,
        FabGlVirtualKey::VK_o => 'o' as u8,
        FabGlVirtualKey::VK_p => 'p' as u8,
        FabGlVirtualKey::VK_q => 'q' as u8,
        FabGlVirtualKey::VK_r => 'r' as u8,
        FabGlVirtualKey::VK_s => 's' as u8,
        FabGlVirtualKey::VK_t => 't' as u8,
        FabGlVirtualKey::VK_u => 'u' as u8,
        FabGlVirtualKey::VK_v => 'v' as u8,
        FabGlVirtualKey::VK_w => 'w' as u8,
        FabGlVirtualKey::VK_x => 'x' as u8,
        FabGlVirtualKey::VK_y => 'y' as u8,
        FabGlVirtualKey::VK_z => 'z' as u8,
        FabGlVirtualKey::VK_A => 'A' as u8,
        FabGlVirtualKey::VK_B => 'B' as u8,
        FabGlVirtualKey::VK_C => 'C' as u8,
        FabGlVirtualKey::VK_D => 'D' as u8,
        FabGlVirtualKey::VK_E => 'E' as u8,
        FabGlVirtualKey::VK_F => 'F' as u8,
        FabGlVirtualKey::VK_G => 'G' as u8,
        FabGlVirtualKey::VK_H => 'H' as u8,
        FabGlVirtualKey::VK_I => 'I' as u8,
        FabGlVirtualKey::VK_J => 'J' as u8,
        FabGlVirtualKey::VK_K => 'K' as u8,
        FabGlVirtualKey::VK_L => 'L' as u8,
        FabGlVirtualKey::VK_M => 'M' as u8,
        FabGlVirtualKey::VK_N => 'N' as u8,
        FabGlVirtualKey::VK_O => 'O' as u8,
        FabGlVirtualKey::VK_P => 'P' as u8,
        FabGlVirtualKey::VK_Q => 'Q' as u8,
        FabGlVirtualKey::VK_R => 'R' as u8,
        FabGlVirtualKey::VK_S => 'S' as u8,
        FabGlVirtualKey::VK_T => 'T' as u8,
        FabGlVirtualKey::VK_U => 'U' as u8,
        FabGlVirtualKey::VK_V => 'V' as u8,
        FabGlVirtualKey::VK_W => 'W' as u8,
        FabGlVirtualKey::VK_X => 'X' as u8,
        FabGlVirtualKey::VK_Y => 'Y' as u8,
        FabGlVirtualKey::VK_Z => 'Z' as u8,

        FabGlVirtualKey::VK_QUESTION => '?' as u8,
        FabGlVirtualKey::VK_EXCLAIM => '!' as u8,
        FabGlVirtualKey::VK_QUOTE => '\'' as u8,
        FabGlVirtualKey::VK_COLON => ':' as u8,
        FabGlVirtualKey::VK_SEMICOLON => ';' as u8,
        FabGlVirtualKey::VK_COMMA => ',' as u8,
        FabGlVirtualKey::VK_PERIOD => '.' as u8,
        FabGlVirtualKey::VK_SLASH => '/' as u8,
        FabGlVirtualKey::VK_BACKSLASH => '\\' as u8,
        FabGlVirtualKey::VK_UNDERSCORE => '_' as u8,
        FabGlVirtualKey::VK_MINUS => '-' as u8,
        FabGlVirtualKey::VK_PLUS => '+' as u8,
        FabGlVirtualKey::VK_EQUALS => '=' as u8,
        FabGlVirtualKey::VK_LEFTBRACKET => '[' as u8,
        FabGlVirtualKey::VK_RIGHTBRACKET => ']' as u8,
        FabGlVirtualKey::VK_LEFTPAREN => '(' as u8,
        FabGlVirtualKey::VK_RIGHTPAREN => ')' as u8,
        FabGlVirtualKey::VK_LEFTBRACE => '{' as u8,
        FabGlVirtualKey::VK_RIGHTBRACE => '}' as u8,
        FabGlVirtualKey::VK_LESS => '<' as u8,
        FabGlVirtualKey::VK_GREATER => '>' as u8,
        FabGlVirtualKey::VK_ASTERISK => '*' as u8,
        FabGlVirtualKey::VK_CARET => '^' as u8,
        FabGlVirtualKey::VK_PERCENT => '%' as u8,
        FabGlVirtualKey::VK_DOLLAR => '$' as u8,
        FabGlVirtualKey::VK_POUND => '£' as u8,
        FabGlVirtualKey::VK_EURO => '€' as u8,
        FabGlVirtualKey::VK_AT => '@' as u8,
        FabGlVirtualKey::VK_HASH => '#' as u8,
        FabGlVirtualKey::VK_AMPERSAND => '&' as u8,
        FabGlVirtualKey::VK_QUOTEDBL => '"' as u8,
        FabGlVirtualKey::VK_TILDE => '~' as u8,
        FabGlVirtualKey::VK_VERTICALBAR => '|' as u8,
        FabGlVirtualKey::VK_GRAVEACCENT => '`' as u8,

        FabGlVirtualKey::VK_RETURN => 0x0D as u8,
        FabGlVirtualKey::VK_ESCAPE => 0x1B as u8,

        FabGlVirtualKey::VK_LEFT => 0x08 as u8,
        FabGlVirtualKey::VK_TAB => 0x09 as u8,
        FabGlVirtualKey::VK_RIGHT => 0x15 as u8,
        FabGlVirtualKey::VK_DOWN => 0x0A as u8,
        FabGlVirtualKey::VK_UP => 0x0B as u8,
        FabGlVirtualKey::VK_BACKSPACE => 0x7F as u8,
        _ => 0,
    }
}

pub fn sdl_scancode_to_fbgl_virtual_key(scancode: &Scancode, keymod: &Mod) -> FabGlVirtualKey {
    log::info!("scancode: {}, keymod: {:?}", scancode, keymod);
    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
    let caps_lock = keymod.intersects(Mod::CAPSMOD);
    match scancode {
        Scancode::Space => FabGlVirtualKey::VK_SPACE,

        Scancode::Grave => if shift {FabGlVirtualKey::VK_TILDE} else {FabGlVirtualKey::VK_GRAVEACCENT},
        Scancode::Num1 => if shift {FabGlVirtualKey::VK_EXCLAIM} else {FabGlVirtualKey::VK_1},
        Scancode::Num2 => if shift {FabGlVirtualKey::VK_AT} else {FabGlVirtualKey::VK_2},
        Scancode::Num3 => if shift {FabGlVirtualKey::VK_HASH} else {FabGlVirtualKey::VK_3},
        Scancode::Num4 => if shift {FabGlVirtualKey::VK_DOLLAR} else {FabGlVirtualKey::VK_4},
        Scancode::Num5 => if shift {FabGlVirtualKey::VK_PERCENT} else {FabGlVirtualKey::VK_5},
        Scancode::Num6 => if shift {FabGlVirtualKey::VK_CARET} else {FabGlVirtualKey::VK_6},
        Scancode::Num7 => if shift {FabGlVirtualKey::VK_AMPERSAND} else {FabGlVirtualKey::VK_7},
        Scancode::Num8 => if shift {FabGlVirtualKey::VK_ASTERISK} else {FabGlVirtualKey::VK_8},
        Scancode::Num9 => if shift {FabGlVirtualKey::VK_LEFTPAREN} else {FabGlVirtualKey::VK_9},
        Scancode::Num0 => if shift {FabGlVirtualKey::VK_RIGHTPAREN} else {FabGlVirtualKey::VK_0},
        Scancode::Minus => if shift {FabGlVirtualKey::VK_UNDERSCORE} else {FabGlVirtualKey::VK_MINUS},
        Scancode::Equals => if shift {FabGlVirtualKey::VK_PLUS} else {FabGlVirtualKey::VK_EQUALS},
        Scancode::LeftBracket => if shift {FabGlVirtualKey::VK_LEFTBRACE} else {FabGlVirtualKey::VK_LEFTBRACKET},
        Scancode::RightBracket => if shift {FabGlVirtualKey::VK_RIGHTBRACE} else {FabGlVirtualKey::VK_RIGHTBRACKET},
        Scancode::Semicolon => if shift {FabGlVirtualKey::VK_COLON} else {FabGlVirtualKey::VK_SEMICOLON},
        Scancode::Apostrophe => if shift {FabGlVirtualKey::VK_QUOTEDBL} else {FabGlVirtualKey::VK_QUOTE},
        Scancode::Backslash => if shift {FabGlVirtualKey::VK_VERTICALBAR} else {FabGlVirtualKey::VK_BACKSLASH},
        Scancode::NonUsBackslash => if shift {FabGlVirtualKey::VK_VERTICALBAR} else {FabGlVirtualKey::VK_BACKSLASH},
        Scancode::Comma => if shift {FabGlVirtualKey::VK_LESS} else {FabGlVirtualKey::VK_COMMA},
        Scancode::Period => if shift {FabGlVirtualKey::VK_GREATER} else {FabGlVirtualKey::VK_PERIOD},
        Scancode::Slash => if shift {FabGlVirtualKey::VK_QUESTION} else {FabGlVirtualKey::VK_SLASH},

        Scancode::Kp0 => FabGlVirtualKey::VK_KP_0,
        Scancode::Kp1 => FabGlVirtualKey::VK_KP_1,
        Scancode::Kp2 => FabGlVirtualKey::VK_KP_2,
        Scancode::Kp3 => FabGlVirtualKey::VK_KP_3,
        Scancode::Kp4 => FabGlVirtualKey::VK_KP_4,
        Scancode::Kp5 => FabGlVirtualKey::VK_KP_5,
        Scancode::Kp6 => FabGlVirtualKey::VK_KP_6,
        Scancode::Kp7 => FabGlVirtualKey::VK_KP_7,
        Scancode::Kp8 => FabGlVirtualKey::VK_KP_8,
        Scancode::Kp9 => FabGlVirtualKey::VK_KP_9,
        Scancode::KpDivide => FabGlVirtualKey::VK_KP_DIVIDE,
        Scancode::KpMultiply => FabGlVirtualKey::VK_KP_MULTIPLY,
        Scancode::KpMinus => FabGlVirtualKey::VK_KP_MINUS,
        Scancode::KpPlus => FabGlVirtualKey::VK_KP_PLUS,
        Scancode::KpEnter => FabGlVirtualKey::VK_KP_ENTER,
        Scancode::KpPeriod => FabGlVirtualKey::VK_KP_PERIOD,
        Scancode::A => if shift | caps_lock {FabGlVirtualKey::VK_A} else {FabGlVirtualKey::VK_a},
        Scancode::B => if shift | caps_lock {FabGlVirtualKey::VK_B} else {FabGlVirtualKey::VK_b},
        Scancode::C => if shift | caps_lock {FabGlVirtualKey::VK_C} else {FabGlVirtualKey::VK_c},
        Scancode::D => if shift | caps_lock {FabGlVirtualKey::VK_D} else {FabGlVirtualKey::VK_d},
        Scancode::E => if shift | caps_lock {FabGlVirtualKey::VK_E} else {FabGlVirtualKey::VK_e},
        Scancode::F => if shift | caps_lock {FabGlVirtualKey::VK_F} else {FabGlVirtualKey::VK_f},
        Scancode::G => if shift | caps_lock {FabGlVirtualKey::VK_G} else {FabGlVirtualKey::VK_g},
        Scancode::H => if shift | caps_lock {FabGlVirtualKey::VK_H} else {FabGlVirtualKey::VK_h},
        Scancode::I => if shift | caps_lock {FabGlVirtualKey::VK_I} else {FabGlVirtualKey::VK_i},
        Scancode::J => if shift | caps_lock {FabGlVirtualKey::VK_J} else {FabGlVirtualKey::VK_j},
        Scancode::K => if shift | caps_lock {FabGlVirtualKey::VK_K} else {FabGlVirtualKey::VK_k},
        Scancode::L => if shift | caps_lock {FabGlVirtualKey::VK_L} else {FabGlVirtualKey::VK_l},
        Scancode::M => if shift | caps_lock {FabGlVirtualKey::VK_M} else {FabGlVirtualKey::VK_m},
        Scancode::N => if shift | caps_lock {FabGlVirtualKey::VK_N} else {FabGlVirtualKey::VK_n},
        Scancode::O => if shift | caps_lock {FabGlVirtualKey::VK_O} else {FabGlVirtualKey::VK_o},
        Scancode::P => if shift | caps_lock {FabGlVirtualKey::VK_P} else {FabGlVirtualKey::VK_p},
        Scancode::Q => if shift | caps_lock {FabGlVirtualKey::VK_Q} else {FabGlVirtualKey::VK_q},
        Scancode::R => if shift | caps_lock {FabGlVirtualKey::VK_R} else {FabGlVirtualKey::VK_r},
        Scancode::S => if shift | caps_lock {FabGlVirtualKey::VK_S} else {FabGlVirtualKey::VK_s},
        Scancode::T => if shift | caps_lock {FabGlVirtualKey::VK_T} else {FabGlVirtualKey::VK_t},
        Scancode::U => if shift | caps_lock {FabGlVirtualKey::VK_U} else {FabGlVirtualKey::VK_u},
        Scancode::V => if shift | caps_lock {FabGlVirtualKey::VK_V} else {FabGlVirtualKey::VK_v},
        Scancode::W => if shift | caps_lock {FabGlVirtualKey::VK_W} else {FabGlVirtualKey::VK_w},
        Scancode::X => if shift | caps_lock {FabGlVirtualKey::VK_X} else {FabGlVirtualKey::VK_x},
        Scancode::Y => if shift | caps_lock {FabGlVirtualKey::VK_Y} else {FabGlVirtualKey::VK_y},
        Scancode::Z => if shift | caps_lock {FabGlVirtualKey::VK_Z} else {FabGlVirtualKey::VK_z},
        Scancode::F1 => FabGlVirtualKey::VK_F1,
        Scancode::F2 => FabGlVirtualKey::VK_F2,
        Scancode::F3 => FabGlVirtualKey::VK_F3,
        Scancode::F4 => FabGlVirtualKey::VK_F4,
        Scancode::F5 => FabGlVirtualKey::VK_F5,
        Scancode::F6 => FabGlVirtualKey::VK_F6,
        Scancode::F7 => FabGlVirtualKey::VK_F7,
        Scancode::F8 => FabGlVirtualKey::VK_F8,
        Scancode::F9 => FabGlVirtualKey::VK_F9,
        Scancode::F10 => FabGlVirtualKey::VK_F10,
        Scancode::F11 => FabGlVirtualKey::VK_F11,
        Scancode::F12 => FabGlVirtualKey::VK_F12,
        Scancode::Escape => FabGlVirtualKey::VK_ESCAPE,
        Scancode::Return => FabGlVirtualKey::VK_RETURN,
        Scancode::Backspace => FabGlVirtualKey::VK_BACKSPACE,
        Scancode::Tab => FabGlVirtualKey::VK_TAB,
        Scancode::LShift => FabGlVirtualKey::VK_LSHIFT,
        Scancode::RShift => FabGlVirtualKey::VK_RSHIFT,
        Scancode::LCtrl => FabGlVirtualKey::VK_LCTRL,
        Scancode::RCtrl => FabGlVirtualKey::VK_RCTRL,
        Scancode::LAlt => FabGlVirtualKey::VK_LALT,
        Scancode::RAlt => FabGlVirtualKey::VK_RALT,
        Scancode::Up => FabGlVirtualKey::VK_UP,
        Scancode::Down => FabGlVirtualKey::VK_DOWN,
        Scancode::Left => FabGlVirtualKey::VK_LEFT,
        Scancode::Right => FabGlVirtualKey::VK_RIGHT,
        Scancode::Home => FabGlVirtualKey::VK_HOME,
        Scancode::End => FabGlVirtualKey::VK_END,
        Scancode::PageUp => FabGlVirtualKey::VK_PAGEUP,
        Scancode::PageDown => FabGlVirtualKey::VK_PAGEDOWN,
        Scancode::Insert => FabGlVirtualKey::VK_INSERT,
        Scancode::Delete => FabGlVirtualKey::VK_DELETE,


        Scancode::CapsLock => FabGlVirtualKey::VK_CAPSLOCK,

        _ => FabGlVirtualKey::VK_NONE,
    }
}

pub enum FabGlVirtualKey {
    VK_NONE,            /**< No character (marks the first virtual key) */

    VK_SPACE,           /**< Space */
    
    VK_0,               /**< Number 0 */
    VK_1,               /**< Number 1 */
    VK_2,               /**< Number 2 */
    VK_3,               /**< Number 3 */
    VK_4,               /**< Number 4 */
    VK_5,               /**< Number 5 */
    VK_6,               /**< Number 6 */
    VK_7,               /**< Number 7 */
    VK_8,               /**< Number 8 */
    VK_9,               /**< Number 9 */
    VK_KP_0,            /**< Keypad number 0 */
    VK_KP_1,            /**< Keypad number 1 */
    VK_KP_2,            /**< Keypad number 2 */
    VK_KP_3,            /**< Keypad number 3 */
    VK_KP_4,            /**< Keypad number 4 */
    VK_KP_5,            /**< Keypad number 5 */
    VK_KP_6,            /**< Keypad number 6 */
    VK_KP_7,            /**< Keypad number 7 */
    VK_KP_8,            /**< Keypad number 8 */
    VK_KP_9,            /**< Keypad number 9 */

    VK_a,               /**< Lower case letter 'a' */
    VK_b,               /**< Lower case letter 'b' */
    VK_c,               /**< Lower case letter 'c' */
    VK_d,               /**< Lower case letter 'd' */
    VK_e,               /**< Lower case letter 'e' */
    VK_f,               /**< Lower case letter 'f' */
    VK_g,               /**< Lower case letter 'g' */
    VK_h,               /**< Lower case letter 'h' */
    VK_i,               /**< Lower case letter 'i' */
    VK_j,               /**< Lower case letter 'j' */
    VK_k,               /**< Lower case letter 'k' */
    VK_l,               /**< Lower case letter 'l' */
    VK_m,               /**< Lower case letter 'm' */
    VK_n,               /**< Lower case letter 'n' */
    VK_o,               /**< Lower case letter 'o' */
    VK_p,               /**< Lower case letter 'p' */
    VK_q,               /**< Lower case letter 'q' */
    VK_r,               /**< Lower case letter 'r' */
    VK_s,               /**< Lower case letter 's' */
    VK_t,               /**< Lower case letter 't' */
    VK_u,               /**< Lower case letter 'u' */
    VK_v,               /**< Lower case letter 'v' */
    VK_w,               /**< Lower case letter 'w' */
    VK_x,               /**< Lower case letter 'x' */
    VK_y,               /**< Lower case letter 'y' */
    VK_z,               /**< Lower case letter 'z' */
    VK_A,               /**< Upper case letter 'A' */
    VK_B,               /**< Upper case letter 'B' */
    VK_C,               /**< Upper case letter 'C' */
    VK_D,               /**< Upper case letter 'D' */
    VK_E,               /**< Upper case letter 'E' */
    VK_F,               /**< Upper case letter 'F' */
    VK_G,               /**< Upper case letter 'G' */
    VK_H,               /**< Upper case letter 'H' */
    VK_I,               /**< Upper case letter 'I' */
    VK_J,               /**< Upper case letter 'J' */
    VK_K,               /**< Upper case letter 'K' */
    VK_L,               /**< Upper case letter 'L' */
    VK_M,               /**< Upper case letter 'M' */
    VK_N,               /**< Upper case letter 'N' */
    VK_O,               /**< Upper case letter 'O' */
    VK_P,               /**< Upper case letter 'P' */
    VK_Q,               /**< Upper case letter 'Q' */
    VK_R,               /**< Upper case letter 'R' */
    VK_S,               /**< Upper case letter 'S' */
    VK_T,               /**< Upper case letter 'T' */
    VK_U,               /**< Upper case letter 'U' */
    VK_V,               /**< Upper case letter 'V' */
    VK_W,               /**< Upper case letter 'W' */
    VK_X,               /**< Upper case letter 'X' */
    VK_Y,               /**< Upper case letter 'Y' */
    VK_Z,               /**< Upper case letter 'Z' */

    VK_GRAVEACCENT,     /**< Grave accent: ` */
    VK_ACUTEACCENT,     /**< Acute accent: ´ */
    VK_QUOTE,           /**< Quote: ' */
    VK_QUOTEDBL,        /**< Double quote: " */
    VK_EQUALS,          /**< Equals: = */
    VK_MINUS,           /**< Minus: - */
    VK_KP_MINUS,        /**< Keypad minus: - */
    VK_PLUS,            /**< Plus: + */
    VK_KP_PLUS,         /**< Keypad plus: + */
    VK_KP_MULTIPLY,     /**< Keypad multiply: * */
    VK_ASTERISK,        /**< Asterisk: * */
    VK_BACKSLASH,       /**< Backslash: \ */
    VK_KP_DIVIDE,       /**< Keypad divide: / */
    VK_SLASH,           /**< Slash: / */
    VK_KP_PERIOD,       /**< Keypad period: . */
    VK_PERIOD,          /**< Period: . */
    VK_COLON,           /**< Colon: : */
    VK_COMMA,           /**< Comma: , */
    VK_SEMICOLON,       /**< Semicolon: ; */
    VK_AMPERSAND,       /**< Ampersand: & */
    VK_VERTICALBAR,     /**< Vertical bar: | */
    VK_HASH,            /**< Hash: # */
    VK_AT,              /**< At: @ */
    VK_CARET,           /**< Caret: ^ */
    VK_DOLLAR,          /**< Dollar: $ */
    VK_POUND,           /**< Pound: £ */
    VK_EURO,            /**< Euro: € */
    VK_PERCENT,         /**< Percent: % */
    VK_EXCLAIM,         /**< Exclamation mark: ! */
    VK_QUESTION,        /**< Question mark: ? */
    VK_LEFTBRACE,       /**< Left brace: { */
    VK_RIGHTBRACE,      /**< Right brace: } */
    VK_LEFTBRACKET,     /**< Left bracket: [ */
    VK_RIGHTBRACKET,    /**< Right bracket: ] */
    VK_LEFTPAREN,       /**< Left parenthesis: ( */
    VK_RIGHTPAREN,      /**< Right parenthesis: ) */
    VK_LESS,            /**< Less: < */
    VK_GREATER,         /**< Greater: > */
    VK_UNDERSCORE,      /**< Underscore: _ */
    VK_DEGREE,          /**< Degree: ° */
    VK_SECTION,         /**< Section: § */
    VK_TILDE,           /**< Tilde: ~ */
    VK_NEGATION,        /**< Negation: ¬ */

    VK_LSHIFT,          /**< Left SHIFT */
    VK_RSHIFT,          /**< Right SHIFT */
    VK_LALT,            /**< Left ALT */
    VK_RALT,            /**< Right ALT */
    VK_LCTRL,           /**< Left CTRL */
    VK_RCTRL,           /**< Right CTRL */
    VK_LGUI,            /**< Left GUI */
    VK_RGUI,            /**< Right GUI */

    VK_ESCAPE,          /**< ESC */

    VK_PRINTSCREEN,     /**< PRINTSCREEN */
    VK_SYSREQ,          /**< SYSREQ */

    VK_INSERT,          /**< INS */
    VK_KP_INSERT,       /**< Keypad INS */
    VK_DELETE,          /**< DEL */
    VK_KP_DELETE,       /**< Keypad DEL */
    VK_BACKSPACE,       /**< Backspace */
    VK_HOME,            /**< HOME */
    VK_KP_HOME,         /**< Keypad HOME */
    VK_END,             /**< END */
    VK_KP_END,          /**< Keypad END */
    VK_PAUSE,           /**< PAUSE */
    VK_BREAK,           /**< CTRL + PAUSE */
    VK_SCROLLLOCK,      /**< SCROLLLOCK */
    VK_NUMLOCK,         /**< NUMLOCK */
    VK_CAPSLOCK,        /**< CAPSLOCK */
    VK_TAB,             /**< TAB */
    VK_RETURN,          /**< RETURN */
    VK_KP_ENTER,        /**< Keypad ENTER */
    VK_APPLICATION,     /**< APPLICATION / MENU key */
    VK_PAGEUP,          /**< PAGEUP */
    VK_KP_PAGEUP,       /**< Keypad PAGEUP */
    VK_PAGEDOWN,        /**< PAGEDOWN */
    VK_KP_PAGEDOWN,     /**< Keypad PAGEDOWN */
    VK_UP,              /**< Cursor UP */
    VK_KP_UP,           /**< Keypad cursor UP  */
    VK_DOWN,            /**< Cursor DOWN */
    VK_KP_DOWN,         /**< Keypad cursor DOWN */
    VK_LEFT,            /**< Cursor LEFT */
    VK_KP_LEFT,         /**< Keypad cursor LEFT */
    VK_RIGHT,           /**< Cursor RIGHT */
    VK_KP_RIGHT,        /**< Keypad cursor RIGHT */
    VK_KP_CENTER,       /**< Keypad CENTER key */

    VK_F1,              /**< F1 function key */
    VK_F2,              /**< F2 function key */
    VK_F3,              /**< F3 function key */
    VK_F4,              /**< F4 function key */
    VK_F5,              /**< F5 function key */
    VK_F6,              /**< F6 function key */
    VK_F7,              /**< F7 function key */
    VK_F8,              /**< F8 function key */
    VK_F9,              /**< F9 function key */
    VK_F10,             /**< F10 function key */
    VK_F11,             /**< F11 function key */
    VK_F12,             /**< F12 function key */
    
    VK_GRAVE_a,         /**< Grave a: à */
    VK_GRAVE_e,         /**< Grave e: è */
    VK_GRAVE_i,         /**< Grave i: ì */
    VK_GRAVE_o,         /**< Grave o: ò */
    VK_GRAVE_u,         /**< Grave u: ù */
    VK_GRAVE_y,         /**< Grave y: ỳ */

    VK_ACUTE_a,         /**< Acute a: á */
    VK_ACUTE_e,         /**< Acute e: é */
    VK_ACUTE_i,         /**< Acute i: í */
    VK_ACUTE_o,         /**< Acute o: ó */
    VK_ACUTE_u,         /**< Acute u: ú */
    VK_ACUTE_y,         /**< Acute y: ý */

    VK_GRAVE_A,		      /**< Grave A: À */
    VK_GRAVE_E,		      /**< Grave E: È */
    VK_GRAVE_I,		      /**< Grave I: Ì */
    VK_GRAVE_O,		      /**< Grave O: Ò */
    VK_GRAVE_U,		      /**< Grave U: Ù */
    VK_GRAVE_Y,         /**< Grave Y: Ỳ */

    VK_ACUTE_A,		      /**< Acute A: Á */
    VK_ACUTE_E,		      /**< Acute E: É */
    VK_ACUTE_I,		      /**< Acute I: Í */
    VK_ACUTE_O,		      /**< Acute O: Ó */
    VK_ACUTE_U,		      /**< Acute U: Ú */
    VK_ACUTE_Y,         /**< Acute Y: Ý */

    VK_UMLAUT_a,        /**< Diaeresis a: ä */
    VK_UMLAUT_e,        /**< Diaeresis e: ë */
    VK_UMLAUT_i,        /**< Diaeresis i: ï */
    VK_UMLAUT_o,        /**< Diaeresis o: ö */
    VK_UMLAUT_u,        /**< Diaeresis u: ü */
    VK_UMLAUT_y,        /**< Diaeresis y: ÿ */

    VK_UMLAUT_A,        /**< Diaeresis A: Ä */
    VK_UMLAUT_E,        /**< Diaeresis E: Ë */
    VK_UMLAUT_I,        /**< Diaeresis I: Ï */
    VK_UMLAUT_O,        /**< Diaeresis O: Ö */
    VK_UMLAUT_U,        /**< Diaeresis U: Ü */
    VK_UMLAUT_Y,        /**< Diaeresis Y: Ÿ */

    VK_CARET_a,		      /**< Caret a: â */
    VK_CARET_e,		      /**< Caret e: ê */
    VK_CARET_i,		      /**< Caret i: î */
    VK_CARET_o,		      /**< Caret o: ô */
    VK_CARET_u,		      /**< Caret u: û */
    VK_CARET_y,         /**< Caret y: ŷ */

    VK_CARET_A,		      /**< Caret A: Â */
    VK_CARET_E,		      /**< Caret E: Ê */
    VK_CARET_I,		      /**< Caret I: Î */
    VK_CARET_O,		      /**< Caret O: Ô */
    VK_CARET_U,		      /**< Caret U: Û */
    VK_CARET_Y,         /**< Caret Y: Ŷ */

    VK_CEDILLA_c,       /**< Cedilla c: ç */
    VK_CEDILLA_C,       /**< Cedilla C: Ç */
    
    VK_TILDE_a,         /**< Lower case tilde a: ã */
    VK_TILDE_o,         /**< Lower case tilde o: õ */
    VK_TILDE_n,		      /**< Lower case tilde n: ñ */

    VK_TILDE_A,         /**< Upper case tilde A: Ã */
    VK_TILDE_O,         /**< Upper case tilde O: Õ */
    VK_TILDE_N,		      /**< Upper case tilde N: Ñ */

    VK_UPPER_a,		      /**< primera: a */
    VK_ESZETT,          /**< Eszett: ß */
    VK_EXCLAIM_INV,     /**< Inverted exclamation mark: ! */
    VK_QUESTION_INV,    /**< Inverted question mark : ? */
    VK_INTERPUNCT,	    /**< Interpunct : · */
    VK_DIAERESIS,	  	  /**< Diaeresis  : ¨ */
    VK_SQUARE,          /**< Square     : ² */
    VK_CURRENCY,        /**< Currency   : ¤ */
    VK_MU,              /**< Mu         : µ */
    
    VK_aelig,           /** Lower case aelig  : æ */
    VK_oslash,          /** Lower case oslash : ø */
    VK_aring,           /** Lower case aring  : å */

    VK_AELIG,           /** Upper case aelig  : Æ */
    VK_OSLASH,          /** Upper case oslash : Ø */
    VK_ARING,           /** Upper case aring  : Å */
    
    // Japanese layout support
    VK_YEN,
    VK_MUHENKAN,
    VK_HENKAN,
    VK_KATAKANA_HIRAGANA_ROMAJI,
    VK_HANKAKU_ZENKAKU_KANJI,
    VK_SHIFT_0,

    VK_ASCII,           /**< Specifies an ASCII code - used when virtual key is embedded in VirtualKeyItem structure and VirtualKeyItem.ASCII is valid */
    VK_LAST,            // marks the last virtual key

}
