use crate::{
    reader::{
        keymap::{self, KeyMap, Map},
        scroll::Scroll,
        window::Canvas,
    },
    FPS,
};

#[derive(Debug)]
pub struct Once {}

impl Once {
    pub fn start(tmp: &mut Scroll, canvas: &mut Canvas, keymaps: &[KeyMap]) {
        let mut time_start = std::time::Instant::now();
        let mut sleep = FPS;

        let buffer_max = tmp.buffer_max;
        let y_step = tmp.y_step;
        let _end = tmp.end();
        let mut rng = 0;
        let page = tmp.page_list.get_mut(0);
        let size = page.size();

        let mut buffer = if size > buffer_max {
            vec![0; size]
        } else {
            vec![0; buffer_max]
        };

        'l1: while canvas.window.is_open() {
            match keymap::match_event(canvas.window.get_keys().iter().as_slice(), keymaps) {
                Map::Down => {
                    // scrolling
                    if rng + y_step <= buffer.len() - buffer_max {
                        rng += y_step;
                    } else {
                        rng = buffer.len() - buffer_max;
                    };
                }

                Map::Up => {
                    if rng >= y_step {
                        rng -= y_step;
                    } else {
                        // if (rng >= 0)
                        rng -= rng;
                    };
                }

                Map::Exit => {
                    println!("EXIT");

                    // BUG: Miss Key::Escape
                    break 'l1;
                }

                _ => {}
            }

            for (idx, data) in page.data().iter().enumerate() {
                buffer[idx] = *data;
            }

            canvas.flush(&buffer[rng..rng + buffer_max]);
            page.to_next_frame();

            let now = std::time::Instant::now();
            let count = (now - time_start).as_millis() as u64;

            time_start = now;
            sleep = FPS.checked_sub(count / 6).unwrap_or(10);

            std::thread::sleep(std::time::Duration::from_millis(sleep));
        }
    }
}
