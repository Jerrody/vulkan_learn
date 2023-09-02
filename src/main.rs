#![feature(offset_of)]
#![feature(const_cstr_from_ptr)]
#![deny(unsafe_op_in_unsafe_fn)]

mod fps_counter;
mod no_engine;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop,
};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let event_loop = event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Hello winit")
        // TODO: Later remove
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut no_engine = no_engine::NoEngine::new(&window);
    let mut fps_counter = fps_counter::FPSCounter::new();

    let mut does_show_fps = false;
    let mut next_time_to_show = std::time::Instant::now() + std::time::Duration::from_secs(1);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => control_flow.set_exit(),
            WindowEvent::DroppedFile(path) => {
                no_engine.load_file(path);
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.set_exit(),
            _ => (),
        },
        Event::MainEventsCleared => {
            window.request_redraw();

            if std::time::Instant::now() >= next_time_to_show {
                does_show_fps = !does_show_fps;
            }
        }
        Event::RedrawRequested(_) => {
            no_engine.draw();
            fps_counter.frame();

            if does_show_fps {
                window.set_title(&format!("FPS: {}", fps_counter.fps()));
                does_show_fps = !does_show_fps;
                next_time_to_show = std::time::Instant::now() + std::time::Duration::from_secs(1);
            }
        }
        Event::RedrawEventsCleared => {
            no_engine.update();
        }
        _ => (),
    });
}
