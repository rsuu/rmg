use esyn::EsynDe;

#[derive(Debug, Default, Clone, Copy, EsynDe, PartialEq)]
pub enum Align {
    #[default]
    Center,

    Top,
    Right,
    Buttom,
    Left,
}
