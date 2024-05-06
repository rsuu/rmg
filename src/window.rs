#[cfg(target_arch = "x86_64")]
pub mod desktop;

#[cfg(target_arch = "wasm32")]
pub mod web;

// event_loop.run(move |event, _target, control_flow| {
//     let start_time = Instant::now();
//     match event {
//         Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
//             *control_flow = ControlFlow::Exit;
//         },
//         ...
//     /*
//      * Process events here
//      */
//     }
//     match *control_flow {
//         ControlFlow::Exit => (),
//         _ => {
//             /*
//              * Grab window handle from the display (untested - based on API)
//              */
//             display.gl_window().window().request_redraw();
//             /*
//              * Below logic to attempt hitting TARGET_FPS.
//              * Basically, sleep for the rest of our milliseconds
//              */
//             let elapsed_time = Instant::now().duration_since(start_time).as_millis() as u64;
//
//             let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
//                 true => 1000 / TARGET_FPS - elapsed_time,
//                 false => 0
//             };
//             let new_inst = start_time + std::time::Duration::from_millis(wait_millis);
//             *control_flow = ControlFlow::WaitUntil(new_inst);
//         }
//     }
// });
