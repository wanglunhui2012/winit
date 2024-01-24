#![allow(clippy::single_match)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use winit::window::WindowLevel;

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut to_back_time = None::<Duration>;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        if let Some(v) = to_back_time {
            let now = SystemTime::now();
            let epoch = now
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            if epoch.as_secs() - v.as_secs() > 3 { // 3s 后才操作
                window.to_front();

                to_back_time = None;
            }
        }

        match event {
            Event::WindowEvent {
                event,
                ..
            } => {
                match event {
                    WindowEvent::CloseRequested => {
                        let now = SystemTime::now();
                        let epoch = now
                            .duration_since(UNIX_EPOCH)
                            .expect("Time went backwards");
                        to_back_time = Some(epoch);

                        window.to_back();
                    }
                    _ => {
                    }
                }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => (),
        }
    });
}
