mod gui;
mod window;

use env_logger;
use gui::Popup;
use window::Window;

pub fn init(
    src: String,
    dst: String,
    width: u32,
) -> std::io::Result<()> {
    let mut logger = env_logger::Builder::new();
    logger.init();

    let mut window = Window::new((width, 200)).unwrap();
    let mut popup = Popup::new(width, window.hidpi);
    let mut renderer = popup.get_renderer(&mut window.handle);
    window.run(&mut popup, &mut renderer);

    Ok(())
}
