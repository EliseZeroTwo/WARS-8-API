pub mod gfx {
    use std::ffi::CString;
    use std::os::raw::c_char;

    #[derive(Copy, Clone, Debug)]
    pub enum ColorPallete {
        Black,
        DarkBlue,
        DarkPurple,
        DarkGreen,
        Brown,
        DarkGray,
        LightGray,
        White,
        Red,
        Orange,
        Yellow,
        Green,
        Blue,
        Indigo,
        Pink,
        Peach,
    }

    impl From<i32> for ColorPallete {
        fn from(color: i32) -> Self {
            match color {
                0 => ColorPallete::Black,
                1 => ColorPallete::DarkBlue,
                2 => ColorPallete::DarkPurple,
                3 => ColorPallete::DarkGreen,
                4 => ColorPallete::Brown,
                5 => ColorPallete::DarkGray,
                6 => ColorPallete::LightGray,
                7 => ColorPallete::White,
                8 => ColorPallete::Red,
                9 => ColorPallete::Orange,
                10 => ColorPallete::Yellow,
                11 => ColorPallete::Green,
                12 => ColorPallete::Blue,
                13 => ColorPallete::Indigo,
                14 => ColorPallete::Pink,
                15 => ColorPallete::Peach,
                _ => panic!("Invalid color {}", color),
            }
        }
    }

    impl From<ColorPallete> for i32 {
        fn from(color: ColorPallete) -> Self {
            match color {
                ColorPallete::Black => 0,
                ColorPallete::DarkBlue => 1,
                ColorPallete::DarkPurple => 2,
                ColorPallete::DarkGreen => 3,
                ColorPallete::Brown => 4,
                ColorPallete::DarkGray => 5,
                ColorPallete::LightGray => 6,
                ColorPallete::White => 7,
                ColorPallete::Red => 8,
                ColorPallete::Orange => 9,
                ColorPallete::Yellow => 10,
                ColorPallete::Green => 11,
                ColorPallete::Blue => 12,
                ColorPallete::Indigo => 13,
                ColorPallete::Pink => 14,
                ColorPallete::Peach => 15,
            }
        }
    }

    extern "C" {
        #[link_name="cls"]
        fn _cls(color: i32);
        #[link_name="rectfill"]
        fn _rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: i32);
        #[link_name="rect"]
        fn _rect(x0: i32, y0: i32, x1: i32, y1: i32, color: i32);
        #[link_name="pset"]
        fn _pset(x: i32, y: i32, c: i32);
        #[link_name="pget"]
        fn _pget(x: i32, y: i32) -> i32;
        #[link_name="print"]
        fn _print(string: *const c_char, x: i32, y: i32, col: i32);
        #[link_name="printh"]
        fn _printh(string: *const c_char);
    }

    pub fn cls(color: ColorPallete) {
        unsafe {
            _cls(i32::from(color));
        }
    }
    
    pub fn rectfill(x0: i32, y0: i32, x1: i32, y1: i32, color: ColorPallete) {
        unsafe {
            _rectfill(x0, y0, x1, y1, i32::from(color));
        }
    }

    pub fn rect(x0: i32, y0: i32, x1: i32, y1: i32, color: ColorPallete) {
        unsafe {
            _rect(x0, y0, x1, y1, i32::from(color));
        }
    }

    pub fn pset(x: i32, y: i32, color: ColorPallete) {
        unsafe {
            _pset(x, y, i32::from(color));
        }
    }

    pub fn pget(x: i32, y: i32) -> ColorPallete {
        unsafe {
            ColorPallete::from(_pget(x, y))
        }
    }

    pub fn print(string: String, x: i32, y: i32, col: ColorPallete) {
        let cstring = CString::new(string).unwrap();
        unsafe {
            let ptr = cstring.into_raw();
            _print(ptr, x, y, i32::from(col));
            CString::from_raw(ptr);
        }
    }

    pub fn printh(string: String) {
        let cstring = CString::new(string).unwrap();
        unsafe {
            let ptr = cstring.into_raw();
            _printh(ptr);
            CString::from_raw(ptr);
        }
    }
}

pub mod input {
    use std::convert::From;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Scancode {
        None,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        ONE,
        TWO,
        THREE,
        FOUR,
        FIVE,
        SIX,
        SEVEN,
        EIGHT,
        NINE,
        ZERO,
        RETURN,
        ESCAPE,
        BACKSPACE,
        TAB,
        SPACE,
        MINUS,
        EQUALS,
        LEFT_BRACKET,
        RIGHT_BRACKET,
        BACKSLASH,
        SEMICOLON,
        APOSTROPHE,
        GRAVE,
        COMMA,
        PERIOD,
        SLASH,
        CAPSLOCK,
    }

    impl From<Scancode> for i32 {
        fn from(scancode: Scancode) -> Self {
            match scancode {
                Scancode::None => 0,
                Scancode::A => 4,
                Scancode::B => 5,
                Scancode::C => 6,
                Scancode::D => 7,
                Scancode::E => 8,
                Scancode::F => 9,
                Scancode::G => 10,
                Scancode::H => 11,
                Scancode::I => 12,
                Scancode::J => 13,
                Scancode::K => 14,
                Scancode::L => 15,
                Scancode::M => 16,
                Scancode::N => 17,
                Scancode::O => 18,
                Scancode::P => 19,
                Scancode::Q => 20,
                Scancode::R => 21,
                Scancode::S => 22,
                Scancode::T => 23,
                Scancode::U => 24,
                Scancode::V => 25,
                Scancode::W => 26,
                Scancode::X => 27,
                Scancode::Y => 28,
                Scancode::Z => 29,
                Scancode::ONE => 30,
                Scancode::TWO => 31,
                Scancode::THREE => 32,
                Scancode::FOUR => 33,
                Scancode::FIVE => 34,
                Scancode::SIX => 35,
                Scancode::SEVEN => 36,
                Scancode::EIGHT => 37,
                Scancode::NINE => 38,
                Scancode::ZERO => 39,
                Scancode::RETURN => 40,
                Scancode::ESCAPE => 41,
                Scancode::BACKSPACE => 42,
                Scancode::TAB => 43,
                Scancode::SPACE => 44,
                Scancode::MINUS => 45,
                Scancode::EQUALS => 46,
                Scancode::LEFT_BRACKET => 47,
                Scancode::RIGHT_BRACKET => 48,
                Scancode::BACKSLASH => 49,
                Scancode::SEMICOLON => 51,
                Scancode::APOSTROPHE => 52,
                Scancode::GRAVE => 53,
                Scancode::COMMA => 54,
                Scancode::PERIOD => 55,
                Scancode::SLASH => 56,
                Scancode::CAPSLOCK => 57,
            }
        }
    }

    impl From<Scancode> for char {
        fn from(scancode: Scancode) -> Self {
            match scancode {
                Scancode::A => 'A',
                Scancode::B => 'B',
                Scancode::C => 'C',
                Scancode::D => 'D',
                Scancode::E => 'E',
                Scancode::F => 'F',
                Scancode::G => 'G',
                Scancode::H => 'H',
                Scancode::I => 'I',
                Scancode::J => 'J',
                Scancode::K => 'K',
                Scancode::L => 'L',
                Scancode::M => 'M',
                Scancode::N => 'N',
                Scancode::O => 'O',
                Scancode::P => 'P',
                Scancode::Q => 'Q',
                Scancode::R => 'R',
                Scancode::S => 'S',
                Scancode::T => 'T',
                Scancode::U => 'U',
                Scancode::V => 'V',
                Scancode::W => 'W',
                Scancode::X => 'X',
                Scancode::Y => 'Y',
                Scancode::Z => 'Z',
                Scancode::ONE => '1',
                Scancode::TWO => '2',
                Scancode::THREE => '3',
                Scancode::FOUR => '4',
                Scancode::FIVE => '5',
                Scancode::SIX => '6',
                Scancode::SEVEN => '7',
                Scancode::EIGHT => '8',
                Scancode::NINE => '9',
                Scancode::ZERO => '0',
                Scancode::SPACE => ' ',
                Scancode::MINUS => '-',
                Scancode::EQUALS => '=',
                Scancode::LEFT_BRACKET => '(',
                Scancode::RIGHT_BRACKET => ')',
                Scancode::BACKSLASH => '\\',
                Scancode::SEMICOLON => ';',
                Scancode::APOSTROPHE => '\'',
                Scancode::GRAVE => '`',
                Scancode::COMMA => ',',
                Scancode::PERIOD => '.',
                Scancode::SLASH => '/',
                _ => ' ',
            }
        }
    }

    impl From<i32> for Scancode {
        fn from(scancode: i32) -> Self {
            match scancode { 
                4 => Scancode::A,
                5 => Scancode::B,
                6 => Scancode::C,
                7 => Scancode::D,
                8 => Scancode::E,
                9 => Scancode::F,
                10 => Scancode::G,
                11 => Scancode::H,
                12 => Scancode::I,
                13 => Scancode::J,
                14 => Scancode::K,
                15 => Scancode::L,
                16 => Scancode::M,
                17 => Scancode::N,
                18 => Scancode::O,
                19 => Scancode::P,
                20 => Scancode::Q,
                21 => Scancode::R,
                22 => Scancode::S,
                23 => Scancode::T,
                24 => Scancode::U,
                25 => Scancode::V,
                26 => Scancode::W,
                27 => Scancode::X,
                28 => Scancode::Y,
                29 => Scancode::Z,
                30 => Scancode::ONE,
                31 => Scancode::TWO,
                32 => Scancode::THREE,
                33 => Scancode::FOUR,
                34 => Scancode::FIVE,
                35 => Scancode::SIX,
                36 => Scancode::SEVEN,
                37 => Scancode::EIGHT,
                38 => Scancode::NINE,
                39 => Scancode::ZERO,
                40 => Scancode::RETURN,
                41 => Scancode::ESCAPE,
                42 => Scancode::BACKSPACE,
                43 => Scancode::TAB,
                44 => Scancode::SPACE,
                45 => Scancode::MINUS,
                46 => Scancode::EQUALS,
                47 => Scancode::LEFT_BRACKET,
                48 => Scancode::RIGHT_BRACKET,
                49 => Scancode::BACKSLASH,
                51 => Scancode::SEMICOLON,
                52 => Scancode::APOSTROPHE,
                53 => Scancode::GRAVE,
                54 => Scancode::COMMA,
                55 => Scancode::PERIOD,
                56 => Scancode::SLASH,
                57 => Scancode::CAPSLOCK,
                _ => Scancode::None,
            }
        }
    }


    #[derive(Copy, Clone, Debug)]
    pub enum Button {
        Left,
        Right,
        Up,
        Down,
        O,
        X,
    }

    impl From<Button> for i32 {
        fn from(player: Button) -> Self {
            match player {
                Button::Left => 0,
                Button::Right => 1,
                Button::Up => 2,
                Button::Down => 3,
                Button::O => 4,
                Button::X => 5,
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Player {
        One,
        Two,
    }

    impl From<Player> for i32 {
        fn from(player: Player) -> Self {
            match player {
                Player::One => 1,
                Player::Two => 2,
            }
        }
    }

    extern "C" {
        #[link_name="btn"]
        fn _btn(i: i32, p: i32) -> i32;
        #[link_name="btnp"]
        fn _btnp(i: i32, p: i32) -> i32;
        #[link_name="key"]
        fn _key() -> i32;
    }

    pub fn btn(button: Button, player: Player) -> bool {
        unsafe {
            _btn(i32::from(button), i32::from(player)) == 1
        }
    }

    pub fn btnp(button: Button, player: Player) -> bool {
        unsafe {
            _btnp(i32::from(button), i32::from(player)) == 1
        }
    }

    pub fn key() -> Scancode {
        unsafe {
            Scancode::from(_key())
        }
    }
}

pub mod math {
    extern "C" {
        #[link_name="abs"]
        fn _abs(x: f32) -> i32;
        #[link_name="atan2"]
        fn _atan2(dx: f32, dy: f32) -> f32;
        #[link_name="band"]
        fn _band(x: i32, y: i32) -> i32;
        #[link_name="bnot"]
        fn _bnot(x: i32) -> i32;
        #[link_name="bor"]
        fn _bor(x: i32, y: i32) -> i32;
        #[link_name="bxor"]
        fn _bxor(x: i32, y: i32) -> i32;
        #[link_name="cos"]
        fn _cos(x: f32) -> f32;
        #[link_name="flr"]
        fn _flr(x: f32) -> f32;
        #[link_name="min"]
        fn _min(x: i32, y: i32) -> i32;
        #[link_name="minf"]
        fn _minf(x: f32, y: f32) -> f32;
        #[link_name="max"]
        fn _max(x: i32, y: i32) -> i32;
        #[link_name="maxf"]
        fn _maxf(x: f32, y: f32) -> f32;
        #[link_name="mid"]
        fn _mid(x: i32, y: i32, z: i32) -> i32;
        #[link_name="rnd"]
        fn _rnd(x: i32) -> i32;
        #[link_name="srand"]
        fn _srand(x: i32);
        #[link_name="sgn"]
        fn _sgn(x: i32) -> i32;
        #[link_name="shl"]
        fn _shl(x: i32, y: i32) -> i32;
        #[link_name="shr"]
        fn _shr(x: i32, y: i32) -> i32;
        #[link_name="sin"]
        fn _sin(x: f32) -> f32;
        #[link_name="sqrt"]
        fn _sqrt(x: i32) -> f32;
        #[link_name="sqrtf"]
        fn _sqrtf(x: f32) -> f32;
    }

    pub fn abs(x: f32) -> i32 {
        unsafe {
            _abs(x)
        }
    }

    pub fn atan2(dx: f32, dy: f32) -> f32 {
        unsafe {
            _atan2(dx, dy)
        }
    }

    pub fn band(x: i32, y: i32) -> i32 {
        unsafe {
            _band(x, y)
        }
    }

    pub fn bnot(x: i32) -> i32 {
        unsafe {
            _bnot(x)
        }
    }

    pub fn bor(x: i32, y: i32) -> i32 {
        unsafe {
            _bor(x, y)
        }
    }

    pub fn bxor(x: i32, y: i32) -> i32 {
        unsafe {
            _bxor(x, y)
        }
    }

    pub fn cos(x: f32) -> f32 {
        unsafe {
            _cos(x)
        }
    }

    pub fn flr(x: f32) -> f32 {
        unsafe {
            _flr(x)
        }
    }

    pub fn min(x: i32, y: i32) -> i32 {
        unsafe {
            _min(x, y)
        }
    }

    pub fn minf(x: f32, y: f32) -> f32  {
        unsafe {
            _minf(x, y)
        }
    }

    pub fn max(x: i32, y: i32) -> i32 {
        unsafe {
            _max(x, y)
        }
    }

    pub fn maxf(x: f32, y: f32) -> f32 {
        unsafe {
            _maxf(x, y)
        }
    }

    pub fn mid(x: i32, y: i32, z: i32) -> i32 {
        unsafe {
            _mid(x, y, z)
        }
    }

    pub fn rnd(x: i32) -> i32 {
        unsafe {
            _rnd(x)
        }
    }

    pub fn srand(x: i32) {
        unsafe {
            _srand(x);
        }
    }

    pub fn sgn(x: i32) -> i32 {
        unsafe {
            _sgn(x)
        }
    }

    pub fn shl(x: i32, y: i32) -> i32 {
        unsafe {
            _shl(x, y)
        }
    }

    pub fn shr(x: i32, y: i32) -> i32 {
        unsafe {
            _shr(x, y)
        }
    }

    pub fn sin(x: f32) -> f32 {
        unsafe {
            _sin(x)
        }
    }

    pub fn sqrt(x: i32) -> f32 {
        unsafe {
            _sqrt(x)
        }
    }

    pub fn sqrtf(x: f32) -> f32 {
        unsafe {
            _sqrtf(x)
        }
    }

}

pub mod misc {
    use std::{ffi::CString, os::raw::c_char};

    extern "C" {
        #[link_name="exit"]
        fn _exit() -> !;
        #[link_name="save"]
        fn _save() -> i32;
        #[link_name="unload"]
        fn _unload();
        #[link_name="load"]
        fn _load(string: *const c_char);
    }

    pub fn exit() -> ! {
        unsafe {
            _exit();
        }
    }

    pub fn save() -> bool {
        unsafe {
            _save() != 0
        }
    }

    pub fn unload() {
        unsafe {
            _unload();
        }
    }

    pub fn load(local_cart_path: String) {
        let cstring = CString::new(local_cart_path).unwrap();
        unsafe {
            let ptr = cstring.into_raw();
            _load(ptr);
            CString::from_raw(ptr);
        }
    }
}