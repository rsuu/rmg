pub mod history;

// TODO: esyn
// TODO: impl EsynDe for minifb::Key
pub mod rsconf;

struct KeyMap<T: char> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
    pub exit: T,
    pub fullscreen: T,
}

//fn get_config() {
//    let a = &EsynBuilder::new()
//        .set_val("a")
//        .get_once::<Test>(config)
//        .unwrap();
//}
