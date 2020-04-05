use glfw::Context;
use imgui::*;
use imgui_opengl_renderer::Renderer;

pub struct Popup {
    pub context: imgui::Context,
    text: String,
    pub width: f32,
    pub height: f32,
}

impl Popup {
    pub fn new(w: u32, hidpi: f32) -> Self {
        let size = 25.0 * hidpi;
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);
        context.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../assets/NotoSansCJK-Regular.ttc"),
            size_pixels: size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::korean(),
                ..FontConfig::default()
            }),
        }]);
        context.fonts().build_rgba32_texture();
        let text = String::new();
        let width = w as f32;
        let height = 1024.0;
        Popup {
            context,
            text,
            width,
            height,
        }
    }

    pub fn set_measure_text(&mut self, hidpi: f32) {
        let mut position: f32 = 0.0;
        let ui = self.context.frame();
        let text = self.text.clone();
        Window::new(im_str!("contents"))
            .position([0.0, 0.0], Condition::Always)
            .size([self.width * hidpi, self.height * hidpi], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .build(&ui, || {
                ui.text_wrapped(&im_str!("{}", text));
                position = ui.cursor_pos()[1];
            });
        if position > 250.0 {
            self.height = (250.0 + 8.0) / hidpi;
        } else {
            self.height = (position + 8.0) / hidpi;
        }

        println!("hidpi: {}, position: {}", hidpi, position);
        println!("width: {}, height: {}", self.width, self.height);
    }

    pub fn get_renderer(&mut self, window: &mut glfw::Window) -> Renderer {
        Renderer::new(&mut self.context, |s| window.get_proc_address(s) as _)
    }

    pub fn frame(&mut self, window: &mut glfw::Window, hidpi: f32, renderer: &Renderer) {
        let io = self.context.io_mut();
        io.display_size = [self.width * hidpi, self.height * hidpi];
        let ui = self.context.frame();
        let text = self.text.clone();

        Window::new(im_str!("contents"))
            .position([0.0, 0.0], Condition::FirstUseEver)
            .size([self.width * hidpi, self.height * hidpi], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .build(&ui, || {
                ui.text_wrapped(&im_str!("{}", text));
            });

        renderer.render(ui);
        window.swap_buffers();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
