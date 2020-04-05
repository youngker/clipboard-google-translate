#[path = "clipboard.rs"]
mod clipboard;

use crate::gui::Popup;
use gl;
use glfw::{Action, Context, MouseButton};
use imgui_opengl_renderer::Renderer;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::{io, sync};

pub struct Window {
    pub handle: glfw::Window,
    event: sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    pub hidpi: f32,
}

impl Window {
    pub fn new(size: (u32, u32)) -> io::Result<Window> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::CocoaRetinaFramebuffer(true));
        glfw.window_hint(glfw::WindowHint::ScaleToMonitor(true));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
        glfw.window_hint(glfw::WindowHint::Floating(true));
        glfw.window_hint(glfw::WindowHint::Visible(false));
        glfw.window_hint(glfw::WindowHint::Decorated(false));

        let (mut window, event) = glfw
            .create_window(size.0, size.1, "clipboard-google-translate", glfw::WindowMode::Windowed)
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                "glfw: error creating window",
            ))?;
        window.make_current();
        window.set_all_polling(true);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let hidpi = window.get_content_scale().0;
        Ok(Window {
            handle: window,
            event,
            glfw,
            hidpi,
        })
    }

    pub fn run(&mut self, popup: &mut Popup, renderer: &Renderer) {
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
        while !self.handle.should_close() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            if let Ok(received) = rx.try_recv() {
                popup.set_text(received);
                self.resize(popup);
                println!("=== {:?}", self.handle.get_pos());
                let (x, y) = self.handle.get_pos();
                self.handle.show();
                self.handle.set_pos(x, y);
            }

            popup.frame(&mut self.handle, self.hidpi, renderer);

            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.event) {
                match event {
                    glfw::WindowEvent::MouseButton(btn, action, _) => {
                        if btn == MouseButton::Button1 && action == Action::Press {
                            pressed = true;
                            cx = popup.context.io_mut().mouse_pos[0];
                            cy = popup.context.io_mut().mouse_pos[1];
                        } else {
                            pressed = false;
                        }
                        if btn == MouseButton::Button2 && action == Action::Press {
                            &self.handle.hide();
                        }
                    }
                    glfw::WindowEvent::CursorPos(x, y) => {
                        popup.context.io_mut().mouse_pos = [x as f32, y as f32];
                    }
                    glfw::WindowEvent::Scroll(_, v) => {
                        popup.context.io_mut().mouse_wheel = v as f32;
                    }
                    _ => {}
                }
            }
            if pressed {
                let (x, y) = self.handle.get_cursor_pos();
                let (a, b) = self.handle.get_pos();
                self.handle.set_pos(a + x as i32 - cx as i32, b + y as i32 - cy as i32);
            }
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn resize(&mut self, popup: &mut Popup) {
        popup.set_measure_text(self.hidpi);
        self.handle.set_size(
            (popup.width * self.hidpi) as i32,
            (popup.height * self.hidpi) as i32,
        );
    }
}
