use std::borrow::Cow;
use std::rc::Rc;

use imgui::*;
use glium::{
    texture::{ClientFormat, RawImage2d},
    uniforms::{MagnifySamplerFilter, MinifySamplerFilter, SamplerBehavior},
    Texture2d,
};
use std::error::Error;
use imgui_glium_renderer::Texture;
use glium::backend::Facade;
use glium::Display;

pub struct Popup {
    text: String,
    pub width: f32,
    pub height: f32,
}

impl Popup {
    pub fn new(width: u32, height: u32) -> Self {
        let text = String::new();
        let width = width as f32;
        let height = height as f32;
        Popup {
            text,
            width,
            height,
        }
    }

    pub fn register_textures<F>(
        &mut self,
        gl_ctx: &F,
        textures: &mut Textures<Texture>,
    ) -> Result<(), Box<dyn Error>>
    where
        F: Facade,
    {
        const WIDTH: usize = 100;
        const HEIGHT: usize = 100;

            // Generate dummy texture
            let mut data = Vec::with_capacity(WIDTH * HEIGHT);
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    // Insert RGB values
                    data.push(i as u8);
                    data.push(j as u8);
                    data.push((i + j) as u8);
                }
            }

            let raw = RawImage2d {
                data: Cow::Owned(data),
                width: WIDTH as u32,
                height: HEIGHT as u32,
                format: ClientFormat::U8U8U8,
            };
            let gl_texture = Texture2d::new(gl_ctx, raw)?;
            let texture = Texture {
                texture: Rc::new(gl_texture),
                sampler: SamplerBehavior {
                    magnify_filter: MagnifySamplerFilter::Linear,
                    minify_filter: MinifySamplerFilter::Linear,
                    ..Default::default()
                },
            };
            let texture_id = textures.insert(texture);

//            self.my_texture_id = Some(texture_id);

        // if self.sipi.is_none() {
        //     self.sipi = Some(SipiPng::new(gl_ctx, textures)?);
        // }

        Ok(())
    }

    pub fn set_measure_text(&mut self, ui: &Ui) {
//        let mut position: f32 = 0.0;
        let text = self.text.clone();
        let text = "hello world 안녕하세요";
        ui.window("contents")
            .position([0.0, 0.0], Condition::Always)
            .size([self.width, self.height], Condition::Always)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .build(|| {
                ui.text_wrapped(text);
//                position = ui.cursor_pos()[1];
            });
        // if position > 250.0 {
        //     self.height = (250.0 + 8.0) / hidpi;
        // } else {
        //     self.height = (position + 8.0) / hidpi;
        // }

//        println!("hidpi: {}, position: {}", hidpi, position);
        println!("width: {}, height: {}", self.width, self.height);
    }

//     pub fn frame(&mut self, hidpi: f32, renderer: &AutoRenderer) {
//         let io = self.imgui_context.io_mut();
//         io.display_size = [self.width * hidpi, self.height * hidpi];
//         let ui = self.imgui_context.frame();
//         let text = self.text.clone();
//         ui.show_demo_window(&mut true);
//         // println!("width {}", self.width);
//         // println!("height {}", self.height);
//         // println!("hidpi {}", hidpi);
//         // ui.window("contents")
//         //     .position([0.0, 0.0], Condition::FirstUseEver)
//         //     .size([self.width * hidpi, self.height * hidpi], Condition::Always)
//         //     .title_bar(true)
//         //     .resizable(false)
//         //     .movable(false)
//         //     .build(|| {
//         //         ui.text_wrapped("Hello world!!!!!");
//         //     });

//         unsafe {
//             gl::ClearColor(0.2, 0.2, 0.2, 1.0);
//             gl::Clear(gl::COLOR_BUFFER_BIT);
//         }

//         //        renderer.render(&mut self.imgui_context);
// //        window.swap_buffers();
//     }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
