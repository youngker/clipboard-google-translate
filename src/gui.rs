use glutin::WindowedContext;
use imgui::*;
use imgui_glow_renderer::AutoRenderer;
type Window = WindowedContext<glutin::PossiblyCurrent>;
use imgui_winit_support::WinitPlatform;

pub struct Popup {
    pub imgui_context: imgui::Context,
    pub winit_platform: WinitPlatform,
    text: String,
    pub width: f32,
    pub height: f32,
}

impl Popup {
    pub fn new(window: &Window, w: u32, hidpi: f32) -> Self {
        let size = 25.0 * hidpi;
        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        imgui_context.fonts().add_font(&[FontSource::TtfData {
            data: include_bytes!("../assets/NotoSansCJK-Regular.ttc"),
            size_pixels: size,
            config: Some(FontConfig {
                rasterizer_multiply: 1.75,
                glyph_ranges: FontGlyphRanges::korean(),
                ..FontConfig::default()
            }),
        }]);

        let mut winit_platform = WinitPlatform::init(&mut imgui_context);
        winit_platform.attach_window(
            imgui_context.io_mut(),
            window.window(),
            imgui_winit_support::HiDpiMode::Rounded,
        );

        imgui_context.fonts().build_rgba32_texture();
        let text = String::new();
        let width = w as f32;
        let height = 1024.0;
        Popup {
            imgui_context,
            winit_platform,
            text,
            width,
            height,
        }
    }

    pub fn set_measure_text(&mut self, hidpi: f32) {
        let mut position: f32 = 0.0;
        let ui = self.imgui_context.frame();
        let text = self.text.clone();
        ui.window("contents")
            .position([0.0, 0.0], Condition::Always)
            .size([self.width * hidpi, self.height * hidpi], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .build(|| {
                ui.text_wrapped(text);
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

    pub fn get_renderer(&mut self, window: &mut Window) -> AutoRenderer {
        let gl =
            unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) };
        imgui_glow_renderer::AutoRenderer::initialize(gl, &mut self.imgui_context)
            .expect("failed to create renderer")
    }

    pub fn frame(&mut self, hidpi: f32, renderer: &AutoRenderer) {
        let io = self.imgui_context.io_mut();
        io.display_size = [self.width * hidpi, self.height * hidpi];
        let ui = self.imgui_context.frame();
        let text = self.text.clone();
        ui.show_demo_window(&mut true);
        // println!("width {}", self.width);
        // println!("height {}", self.height);
        // println!("hidpi {}", hidpi);
        // ui.window("contents")
        //     .position([0.0, 0.0], Condition::FirstUseEver)
        //     .size([self.width * hidpi, self.height * hidpi], Condition::Always)
        //     .title_bar(true)
        //     .resizable(false)
        //     .movable(false)
        //     .build(|| {
        //         ui.text_wrapped("Hello world!!!!!");
        //     });

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        //        renderer.render(&mut self.imgui_context);
//        window.swap_buffers();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
