pub mod style;
pub mod tags;

use crate::Body;

pub trait Draw {
    fn draw(&self, body: &mut Body);
}
