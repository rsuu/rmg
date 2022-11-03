#[derive(Debug, Default)]
pub enum ReaderMode {
    #[default]
    View,

    Command,
}

#[derive(Debug, Default)]
pub enum ViewMode {
    #[default]
    Scroll,

    Manga, // Left to Right
    Comic, // Right to Left
}

impl ViewMode {
    pub fn reader(mode: &ViewMode) {
        match mode {
            Scroll => {}
            Manga => {}
            Comic => {}
        }

        todo!()
    }
}
