//?rewrite in minifb-web

use js_sys;
use std::{
    cell::RefCell,
    io::{Cursor, Read, Seek, SeekFrom, Write},
    rc::Rc,
};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Blob, HtmlElement, HtmlImageElement, MessageEvent, Url, Window, Worker};
use zip::ZipArchive;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u8);

}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let img_list = document
        .query_selector("#img_list")
        .expect("should find #img_list")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .expect("#img_list should be a div element");

    let btn_get = document
        .query_selector("#btn_get")
        .expect("should find #btn_get")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .expect("#btn_get should be a button element");

    let btn_get_url = document
        .query_selector("#btn_get_url")
        .expect("should find #btn_get_url")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .expect("#btn_get_url should be an input element");

    let mut last = 0;
    //let arc_last_number = Arc::new(last_page.text_content().unwrap().parse::<usize>().unwrap());

    Ok(())
}

//pub struct MySlice<T> {
//    phantom: std::marker::PhantomData<T>,
//    _ptr: u32,
//    _len: u32,
//}
//
//impl<T: wasm_bindgen::describe::WasmDescribe> wasm_bindgen::describe::WasmDescribe for MySlice<T> {
//    fn describe() {
//        wasm_bindgen::describe::inform(wasm_bindgen::describe::REF);
//        wasm_bindgen::describe::inform(wasm_bindgen::describe::SLICE);
//        T::describe();
//    }
//}
//
//impl<T: wasm_bindgen::describe::WasmDescribe> wasm_bindgen::convert::IntoWasmAbi for MySlice<T> {
//    type Abi = wasm_bindgen::convert::WasmSlice;
//
//    #[inline]
//    fn into_abi(self) -> wasm_bindgen::convert::WasmSlice {
//        log(&format!("{}", &self._len));
//        wasm_bindgen::convert::WasmSlice {
//            ptr: self._ptr,
//            len: self._len,
//        }
//    }
//}
//
//impl<T> std::convert::From<&Vec<T>> for MySlice<T> {
//    fn from(vec: &Vec<T>) -> Self {
//        let _ptr = vec.as_ptr() as u32;
//        let _len = vec.len() as u32;
//
//        Self {
//            phantom: std::marker::PhantomData,
//            _ptr,
//            _len,
//        }
//    }
//}

/// Run entry point for the main thread.
#[wasm_bindgen]
pub fn startup() {
    log("startup()");

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let window = web_sys::window().unwrap();
    let document = window.document().expect("should have a document on window");

    let pref = document
        .query_selector("#pref")
        .expect("should find #btn_get_url")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .expect("#btn_get_url should be an input element");
    let last_page = document
        .query_selector("#last")
        .expect("should find #last")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .expect("#last should be a paragraph element");

    if let Some(perf) = window.performance() {
        let start_time = perf.now();

        *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            //log("hi");
            pref.set_text_content(Some(last_page.text_content().unwrap().as_str()));

            window
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("failed requesting animation frame");
        }) as Box<dyn FnMut()>));

        let window = web_sys::window().unwrap();
        window
            .request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }
}

#[wasm_bindgen]
pub struct ZipExtract {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    len: usize,
    buf: Vec<u8>,
}

#[wasm_bindgen]
pub struct Img {
    name: String,
    page_number: usize,
    idx: usize,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl Img {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn page_number(&self) -> usize {
        self.page_number
    }

    pub fn idx(&self) -> usize {
        self.idx
    }
}

#[wasm_bindgen]
impl ZipExtract {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Self {
        log("ZipExtract::new()");

        let mut c = Cursor::new(Vec::new());
        c.write_all(bytes).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();
        let archive = zip::ZipArchive::new(c).unwrap();
        let len = archive.len();

        ZipExtract {
            archive,
            len,
            buf: Vec::new(),
        }
    }

    pub fn get_slice(&mut self, idx: usize) -> js_sys::Uint8Array {
        let mut file = self.archive.by_index(idx).unwrap();
        self.buf.clear();
        file.read_to_end(&mut self.buf).unwrap();

        js_sys::Uint8Array::from(self.buf.as_slice())
    }

    pub fn get_list(&mut self) -> Vec<Img> {
        let mut res = vec![];

        for idx in 0..self.len {
            let mut file = self.archive.by_index(idx).unwrap();
            if file.is_dir() {
                continue;
            }

            let name = file.name().to_string();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();

            res.push(Img {
                name,
                data,
                page_number: 0,
                idx,
            });
        }

        res.sort_by(|a, b| a.name.as_str().partial_cmp(b.name.as_str()).unwrap());

        for (i, f) in res.iter_mut().enumerate() {
            // start with 1
            f.page_number = i + 1;
        }

        res
    }
}
