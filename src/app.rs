pub mod buffer;
pub mod canvas;
pub mod draw;
pub mod gesture;
pub mod layout;
pub mod page;
pub mod state;
pub mod task;
pub mod view;
pub mod window;

#[cfg(target_arch = "wasm32")]
mod web;

use crate::{Element, ElementArgs, Frame, Page, State, Style};

pub type Elem = Box<dyn Element<Res = eyre::Result<()>>>;
pub type Elems = Vec<Elem>;

pub struct World {
    elems: Elems,
    // TODO: center by default
    // global value
    // offset: Vec2,
}

impl World {
    pub fn new(elems: Vec<Page>) -> Self {
        let elems = elems
            .into_iter()
            .map(|elem| Box::new(elem) as Box<dyn Element<Res = eyre::Result<()>>>)
            .collect();

        Self { elems }
    }

    pub fn render<'a>(&mut self, args: &'a mut ElementArgs) -> eyre::Result<()> {
        for elem in self.elems.iter() {
            elem.as_ref().draw(args)?;
        }

        Ok(())
    }
}
