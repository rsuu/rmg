use minifb::Key;

pub trait TKeycode {
    fn as_char(&self) -> char;
}

#[derive(Debug, Clone, Copy)]
pub struct KeyMap {
    pub key: char,
    pub map: Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Map {
    Up,
    Down,

    Left,
    Right,

    FullScreen, // and UnFullScreen
    Reset,

    Exit,
    Stop,
}

#[inline]
pub fn match_event(event: &[Key], keymaps: &[KeyMap]) -> Map {
    KeyMap::by_key(event.as_char(), keymaps)
}

impl KeyMap {
    pub fn new() -> Vec<Self> {
        vec![
            KeyMap::add('k', Map::Up),
            KeyMap::add('j', Map::Down),
            KeyMap::add('h', Map::Left),
            KeyMap::add('l', Map::Right),
            KeyMap::add('f', Map::FullScreen),
            KeyMap::add('q', Map::Exit),
            KeyMap::add('r', Map::Reset),
            KeyMap::add('U', Map::Up),
            KeyMap::add('D', Map::Down),
            KeyMap::add('L', Map::Left),
            KeyMap::add('R', Map::Right),
            KeyMap::add('E', Map::Exit),
        ]
    }

    pub fn add(key: char, map: Map) -> Self {
        Self { key, map }
    }

    #[inline]
    pub fn by_key(key: char, keymaps: &[Self]) -> Map {
        for keymap in keymaps.iter() {
            if key == keymap.key {
                return keymap.map;
            } else {
            }
        }

        Map::Stop
    }

    #[inline]
    pub fn by_map(map: Map, keymaps: &[Self]) -> char {
        for keymap in keymaps.iter() {
            if map == keymap.map {
                return keymap.key;
            }
        }

        '\0'
    }
}

impl TKeycode for &[Key] {
    fn as_char(&self) -> char {
        match **self {
            [Key::A] => 'a',
            [Key::B] => 'b',
            [Key::C] => 'c',
            [Key::D] => 'd',
            [Key::E] => 'e',
            [Key::F] => 'f',
            [Key::G] => 'g',
            [Key::H] => 'h',
            [Key::I] => 'i',
            [Key::J] => 'j',
            [Key::K] => 'k',
            [Key::L] => 'l',
            [Key::M] => 'm',
            [Key::N] => 'n',
            [Key::O] => 'o',
            [Key::P] => 'p',
            [Key::Q] => 'q',
            [Key::R] => 'r',
            [Key::S] => 's',
            [Key::T] => 't',
            [Key::U] => 'u',
            [Key::V] => 'v',
            [Key::W] => 'w',
            [Key::X] => 'x',
            [Key::Y] => 'y',
            [Key::Z] => 'z',
            [Key::Escape] => 'E',
            [Key::Up] => 'U',
            [Key::Down] => 'D',
            [Key::Left] => 'L',
            [Key::Right] => 'R',
            _ => '\0',
        }
    }
}

// impl TKeycode for sdl2::keyboard::Keycode {
//     fn as_char(&self) -> char {
//         match self {
//             Keycode::A => 'a',
//             Keycode::B => 'b',
//             Keycode::C => 'c',
//             Keycode::D => 'd',
//             Keycode::E => 'e',
//             Keycode::F => 'f',
//             Keycode::G => 'g',
//             Keycode::H => 'h',
//             Keycode::I => 'i',
//             Keycode::J => 'j',
//             Keycode::K => 'k',
//             Keycode::L => 'l',
//             Keycode::M => 'm',
//             Keycode::N => 'n',
//             Keycode::O => 'o',
//             Keycode::P => 'p',
//             Keycode::Q => 'q',
//             Keycode::R => 'r',
//             Keycode::S => 's',
//             Keycode::T => 't',
//             Keycode::U => 'u',
//             Keycode::V => 'v',
//             Keycode::W => 'w',
//             Keycode::X => 'x',
//             Keycode::Y => 'y',
//             Keycode::Z => 'z',
//             _ => '\0',
//         }
//     }
// }
//
// #[inline(always)]
// pub fn match_event(event: &sdl2::event::Event, keymaps: &[KeyMap]) -> Map {
//     match event {
//             // Mouse
//         Event::MouseWheel {
//             direction: MouseWheelDirection::Normal,
//             ..
//         } => {
//             if let Event::MouseWheel { y, .. } = event {
//                 if *y < 0 {
//                     return Map::Down;
//                 } else if *y > 0 {
//                     return Map::Up;
//                 };
//             } else {
//             }
//
//             Map::Stop
//         }
//
//         Event::KeyDown {
//             keycode: Option::Some(key),
//             repeat: true,
//             ..
//         } => {
//             KeyMap::by_key(key.as_char(), keymaps)
//         }
//
//         Event::KeyUp {
//             keycode: Option::Some(key),
//             repeat: false,
//             ..
//         } => {
//             KeyMap::by_key(key.as_char(), keymaps)
//         }
//
//         _ => Map::Stop,
//     }
// }
//
