mod gui;
mod window;

use env_logger;
use gui::Popup;
use window::Window;

pub fn init(_src: String, _dst: String, width: u32) -> std::io::Result<()> {
    let mut logger = env_logger::Builder::new();
    logger.init();

    let mut window = Window::new((width, 200)).unwrap();
    let mut popup = Popup::new(&mut window.window, width, window.hidpi);
    let mut renderer = popup.get_renderer(&mut window.window);
    window.run(&mut popup, &mut renderer);

    Ok(())
}
