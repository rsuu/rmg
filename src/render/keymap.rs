use crate::config::rsconf::Config;
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

    FullScreen,
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

    pub fn update(list: &mut Vec<Self>, config: &Config) {
        //dbg!(&config);
        let new = config.keymap;

        for Self { key, map } in list.iter_mut() {
            match map {
                Map::Up => *key = new.up,
                Map::Down => *key = new.down,
                Map::Left => *key = new.left,
                Map::Right => *key = new.right,
                Map::Exit => *key = new.exit,

                _ => {}
            }
        }
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
