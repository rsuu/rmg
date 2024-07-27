// TODO(threadpool): https://github.com/rustwasm/wasm-bindgen/tree/main/examples/raytrace-parallel

mod pool;

use crate::*;

use eyre::OptionExt;
use rayon::ThreadPoolBuilder;
use rgb::RGBA8;
use softbuffer::{Context, Surface};
use std::{
    collections::VecDeque,
    num::NonZeroU32,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{Event, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop, ActiveEventLoop},
    keyboard::{KeyCode, PhysicalKey},
    monitor::{MonitorHandle, VideoMode},
    window::Window,
};
use {
    console_error_panic_hook,
    js_sys::Uint8Array,
    wasm_bindgen::{prelude::*, JsCast, JsValue},
    wasm_bindgen_futures::JsFuture,
    web_sys::{console, Request, RequestInit, RequestMode, Response},
    winit::platform::web::{EventLoopExtWebSys, WindowExtWebSys},
};

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => (crate::app::web::log(&format_args!($($t)*).to_string()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let web_win = web_sys::window().unwrap();
    let web_dom = web_win.document().unwrap();

    // let (mut app, event_loop) = App::new(Config::new().unwrap()).unwrap();

    let event_loop = EventLoop::new().unwrap();
    let size = LogicalSize::new(100.0, 100.0);
    let attrs = Window::default_attributes().with_title("rmg")
     .with_inner_size(size);
     let window = event_loop.create_window(attrs).unwrap();        
    let window = Rc::new(window);
    
    web_dom
        .body()
        .unwrap()
        .append_child(&window.canvas().unwrap())
        .unwrap();
    let context = Context::new(window.clone()).unwrap();
    let mut surface = Surface::new(&context, window.clone()).unwrap();
    log!("INFO: web-sys");

    // TODO(web): ?canvas from url
    // REFS: https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
    let img = {
        let url = "/zoom.png";

        let req = web_win.fetch_with_str(url);
        let res: Response = JsFuture::from(req).await.unwrap().dyn_into().unwrap();
        let res = JsFuture::from(res.array_buffer().unwrap()).await.unwrap();

        Uint8Array::new(&res).to_vec()
    };
    log!("buffer.len: {}", img.len());

    let mut buffer = vec![];
    {
        let img = image::load_from_memory(&img).unwrap().into_rgba8().to_vec();

        for v in img.chunks(4) {
            let (r, g, b, a) = (v[0], v[1], v[2], v[3]);
            buffer.push(u32::from_be_bytes([a, r, g, b]));
        }
    }

    let loop_dur = Duration::from_millis(1000 / 90);

    event_loop.spawn(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::wait_duration(loop_dur));

        match event {
            Event::AboutToWait => {
                window.request_redraw();
            }

            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::RedrawRequested => {
                    surface
                        .resize(NonZeroU32::new(100).unwrap(), NonZeroU32::new(100).unwrap())
                        .unwrap();

                    let mut map = surface.buffer_mut().unwrap();

                    map.copy_from_slice(&buffer);
                    map.present().unwrap();
                }
                _ => {}
            },
            _ => {}
        }

        // app.event_loop(event, elwt).unwrap();
    });
}

#[wasm_bindgen]
pub struct Pool {}

#[wasm_bindgen]
impl Pool {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Pool, JsValue> {
        Ok(Self {})
    }

    pub fn render(pool: &pool::WorkerPool) {
        log!("render");

        let threads = 8;
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .spawn_handler(|thread| {
                pool.run(|| thread.run()).unwrap();
                Ok(())
            })
            .build()
            .unwrap();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}
