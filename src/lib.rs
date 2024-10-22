use env_logger;

#[path = "popup.rs"]
mod popup;
mod clipboard;

pub fn init(_src: String, _dst: String, width: u32) -> std::io::Result<()> {
    let mut logger = env_logger::Builder::new();
    logger.init();
    popup::start();
    Ok(())
}
