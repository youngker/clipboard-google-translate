#[path = "clipboard.rs"]
mod clipboard;

use crate::gui::Popup;
use imgui_glow_renderer::AutoRenderer;
use std::io;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use glow::HasContext;
use glutin::{event_loop::EventLoop, WindowedContext};
use std::time::Instant;
const TITLE: &str = "Hello, imgui-rs!";

pub struct Window {
    pub window: WindowedContext<glutin::PossiblyCurrent>,
    pub event_loop: EventLoop<()>,
    pub hidpi: f32,
}

impl Window {
    pub fn new(size: (u32, u32)) -> io::Result<Window> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_title(TITLE)
            .with_transparent(true)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window, &event_loop)
            .expect("could not create window");
        let window = unsafe {
            window
                .make_current()
                .expect("could not make window context current")
        };
        let hidpi = 2.0;
        Ok(Window {
            window,
            event_loop,
            hidpi,
        })
    }

    pub fn run(&self, popup: &mut Popup, renderer: &mut AutoRenderer) {
        let Window {
            window,
            event_loop,
            hidpi,
            ..
        } = self;
        let (tx, rx) = channel();
        let mut pressed = false;
        let mut cx: f32 = 0.0;
        let mut cy: f32 = 0.0;
        thread::spawn(move || {
            let mut clipboard = clipboard::ClipboardThread::new();
            let receiver = clipboard.start();
            loop {
                if let Ok(received) = receiver.try_recv() {
                    if clipboard.text != received {
                        if received.trim() == "" {
                            println!("this is a null");
                            clipboard.set_text(received.clone());
                        } else {
                            println!("got: {}", received);
                            clipboard.set_text(received.clone());
                            tx.send(clipboard.request()).unwrap();
                        }
                    }
                }
                thread::sleep(Duration::from_millis(500));
            }
        });

        popup.frame(self.hidpi, renderer);

        let mut last_frame = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    popup.imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                glutin::event::Event::MainEventsCleared => {
                    popup
                        .winit_platform
                        .prepare_frame(popup.imgui_context.io_mut(), self.window.window())
                        .unwrap();
                    self.window.window().request_redraw();
                }
                glutin::event::Event::RedrawRequested(_) => {
                    // The renderer assumes you'll be clearing the buffer yourself
                    unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

                    let ui = popup.imgui_context.frame();
                    ui.show_demo_window(&mut true);

                    popup
                        .winit_platform
                        .prepare_render(ui, self.window.window());
                    let draw_data = popup.imgui_context.render();

                    // This is the only extra render step to add
                    renderer.render(draw_data).expect("error rendering imgui");

                    self.window.swap_buffers().unwrap();
                }
                glutin::event::Event::WindowEvent {
                    event: glutin::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                event => {
                    popup.winit_platform.handle_event(
                        popup.imgui_context.io_mut(),
                        self.window.window(),
                        &event,
                    );
                }
            }
        });
    }

    pub fn resize(&mut self, popup: &mut Popup) {
        println!("resize");
        // popup.set_measure_text(self.hidpi);
        // self.handle.set_size(
        //     (popup.width * self.hidpi) as i32,
        //     (popup.height * self.hidpi) as i32,
        // );
    }
}
