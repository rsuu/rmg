// WARN: WIP

use crate::{ArchiveFmt, Canvas, Config, DataType, FileList, Page, Path, PathBuf, Size, State};

use std::{
    cell::RefCell, collections::HashMap, fs::File, io::Read, num::NonZeroU32, panic, rc::Rc,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use {
    console_error_panic_hook,
    wasm_bindgen::{prelude::*, JsCast},
    web_sys::console,
    winit::platform::web::{EventLoopExtWebSys, WindowExtWebSys},
};

// #[macro_export]
// macro_rules! dbg {
//     ($s: expr) => {
//         crate::web::log($s)
//     };
// }

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    init_log();

    let config = Config::new().unwrap();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let win_size = config.canvas_size();
    let mut size = LogicalSize::new(win_size.width(), win_size.height());
    let window = Rc::new(
        WindowBuilder::new()
            .with_inner_size(size)
            .with_max_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap(),
    );
    dbg!("INFO: winit");

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&window.canvas().unwrap())
        .unwrap();
    dbg!("INFO: web-sys");

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    dbg!("INFO: softbuffer");

    // let mut canvas = {
    //     // let file = DataType::new(PathBuf::from("./assets/test.zip").as_path()).unwrap();
    //     let file = DataType::SingleImg {
    //         path: PathBuf::from(""),
    //     };
    //     let mut canvas = Canvas::new(config, file, win_size.width(), win_size.height()).unwrap();
    //
    //     // let mut offset_y = 0.0;
    //     // let mut file_index = 1;
    //     // let cw = win_size.width();
    //     // let filter = fir::FilterType::Lanczos3;
    //     //
    //     // for _ in 0..4 {
    //     //     let (w, h) = (win_size.width(), win_size.height());
    //     //     let buf = vec![255_u8; (w as usize * h as usize) * 4];
    //     //     let (nw, nh) = (cw, 600);
    //     //     let img = Page::new_buf(
    //     //         buf,
    //     //         Size::new(w, h),
    //     //         Size::new(nw, nh),
    //     //         filter,
    //     //         &mut offset_y,
    //     //         &mut file_index,
    //     //     );
    //     //     canvas.add_page(img.unwrap());
    //     //
    //     //     // padding-buttom
    //     //     offset_y += 50.0;
    //     // }
    //
    //     canvas
    // };
    // dbg!(&format!("pages.len(): {}", canvas.pages.len()));
    //
    // event_loop.spawn(move |event, elwt| {
    //     elwt.set_control_flow(ControlFlow::Wait);
    //
    //     match event {
    //         Event::WindowEvent {
    //             window_id,
    //             event: WindowEvent::RedrawRequested,
    //         } => {
    //             // if window_id != window.id() {
    //             //     unimplemented!()
    //             // }
    //
    //             let (width, height) = {
    //                 let size = window.inner_size();
    //                 (size.width, size.height)
    //             };
    //             //dbg!(&format!("{width}x{height}"));
    //
    //             let (width, height) = (win_size.width(), win_size.height());
    //             surface
    //                 .resize(
    //                     NonZeroU32::new(width).unwrap(),
    //                     NonZeroU32::new(height).unwrap(),
    //                 )
    //                 .unwrap();
    //             let mut buffer = surface.buffer_mut().unwrap();
    //
    //             // dbg!(&format!("buf   : {}", buffer.len()));
    //             // dbg!(&format!("canvas: {}", canvas.buf.data.len()));
    //             // dbg!(&format!("buffer: {}", canvas.bg_vec.len()));
    //
    //             canvas.render().unwrap();
    //             buffer.swap_with_slice(&mut canvas.buffer.data);
    //             // buffer.copy_from_slice(&canvas.buf.data);
    //             buffer.present().unwrap();
    //
    //             // if canvas.pages[0].state == State::Unneeded {
    //             //     dbg!("free");
    //             // }
    //
    //             // dbg!("flush");
    //         }
    //
    //         Event::WindowEvent { event, .. } => match event {
    //             WindowEvent::KeyboardInput {
    //                 device_id,
    //                 event:
    //                     KeyEvent {
    //                         physical_key,
    //                         logical_key,
    //                         text,
    //                         location,
    //                         state,
    //                         repeat,
    //                         ..
    //                     },
    //                 is_synthetic,
    //             } => match physical_key {
    //                 PhysicalKey::Code(KeyCode::KeyJ) | PhysicalKey::Code(KeyCode::ArrowDown) => {
    //                     // dbg!("J");
    //                     canvas.move_down();
    //                 }
    //                 PhysicalKey::Code(KeyCode::KeyK) | PhysicalKey::Code(KeyCode::ArrowUp) => {
    //                     // dbg!("K");
    //                     canvas.move_up();
    //                 }
    //                 PhysicalKey::Code(KeyCode::KeyH) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
    //                     // dbg!("H");
    //                     canvas.move_left();
    //                 }
    //                 PhysicalKey::Code(KeyCode::KeyL) | PhysicalKey::Code(KeyCode::ArrowRight) => {
    //                     // dbg!("L");
    //                     canvas.move_right();
    //                 }
    //                 PhysicalKey::Code(KeyCode::KeyQ) | PhysicalKey::Code(KeyCode::Escape) => {
    //                     dbg!("exit");
    //                     elwt.exit();
    //                 }
    //                 _ => {}
    //             },
    //             WindowEvent::ModifiersChanged(new) => {}
    //             WindowEvent::Resized(new_size) => {
    //                 size = new_size.to_logical(1.0);
    //             }
    //             WindowEvent::CloseRequested => {
    //                 elwt.exit();
    //             }
    //             _ => (),
    //         },
    //         _ => {}
    //     }
    //
    //     window.request_redraw();
    // });
}

// fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//     window()
//         .request_animation_frame(f.as_ref().unchecked_ref())
//         .expect("should register `requestAnimationFrame` OK");
// }

// fn window() -> web_sys::Window {
//     web_sys::window().expect("no global `window` exists")
// }

fn init_log() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u8);

}
