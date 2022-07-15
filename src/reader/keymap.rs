use sdl2::{event::Event, keyboard::Keycode, mouse::MouseWheelDirection};

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
    DisplayMeta,
    Reset,

    Exit,
    Stop,
}

#[inline(always)]
pub fn match_event(event: &sdl2::event::Event, keymaps: &[KeyMap]) -> Map {
    match event {
            // Mouse
        Event::MouseWheel {
            direction: MouseWheelDirection::Normal,
            ..
        } => {
            if let Event::MouseWheel { y, .. } = event {
                if *y < 0 {
                    return Map::Down;
                } else if *y > 0 {
                    return Map::Up;
                };
            } else {
            }

            Map::Stop
        }

        Event::KeyDown {
            keycode: Option::Some(key),
            repeat: true,
            ..
        } => {
            KeyMap::by_key(key.as_char(), keymaps)
        }

        Event::KeyUp {
            keycode: Option::Some(key),
            repeat: false,
            ..
        } => {
            KeyMap::by_key(key.as_char(), keymaps)
        }

        _ => Map::Stop,
    }
}

impl KeyMap {
    pub fn new() -> Vec<Self> {
        vec![
            KeyMap::add('f', Map::FullScreen),
            KeyMap::add('h', Map::Left),
            KeyMap::add('j', Map::Down),
            KeyMap::add('k', Map::Up),
            KeyMap::add('l', Map::Right),
            KeyMap::add('p', Map::DisplayMeta),
            KeyMap::add('q', Map::Exit),
            KeyMap::add('r', Map::Reset),
        ]
    }

    pub fn add(key: char, map: Map) -> Self {
        Self { key, map }
    }

    pub fn by_key(key: char, keymaps: &[Self]) -> Map {
        for keymap in keymaps.iter() {
            if key == keymap.key {
                return keymap.map;
            }
        }

        Map::Stop
    }

    pub fn by_map(map: Map, keymaps: &[Self]) -> char {
        for keymap in keymaps.iter() {
            if map == keymap.map {
                return keymap.key;
            }
        }

        '\0'
    }
}

impl TKeycode for sdl2::keyboard::Keycode {
    fn as_char(&self) -> char {
        match self {
            Keycode::A => 'a',
            Keycode::B => 'b',
            Keycode::C => 'c',
            Keycode::D => 'd',
            Keycode::E => 'e',
            Keycode::F => 'f',
            Keycode::G => 'g',
            Keycode::H => 'h',
            Keycode::I => 'i',
            Keycode::J => 'j',
            Keycode::K => 'k',
            Keycode::L => 'l',
            Keycode::M => 'm',
            Keycode::N => 'n',
            Keycode::O => 'o',
            Keycode::P => 'p',
            Keycode::Q => 'q',
            Keycode::R => 'r',
            Keycode::S => 's',
            Keycode::T => 't',
            Keycode::U => 'u',
            Keycode::V => 'v',
            Keycode::W => 'w',
            Keycode::X => 'x',
            Keycode::Y => 'y',
            Keycode::Z => 'z',
            _ => '\0',
        }
    }
}
