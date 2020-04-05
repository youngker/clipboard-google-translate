use clipboard::{ClipboardContext, ClipboardProvider};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

#[path = "client.rs"]
mod client;

pub struct ClipboardThread {
    pub text: String,
}

impl ClipboardThread {
    pub fn new() -> ClipboardThread {
        let text = String::new();
        ClipboardThread { text }
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
    pub fn request(&self) -> String {
        let mut client = client::HttpClient::new();
        let send_txt: String = self
            .text
            .lines()
            .map(|line| match line {
                "" => "\n",
                _ => line,
            })
            .collect::<Vec<_>>()
            .join(" ");
        client.request(client.make_url(&send_txt[..]))
    }
    pub fn start(&mut self) -> Receiver<String> {
        let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
        self.set_text(ctx.get_contents().unwrap());
        let (tx, rx) = channel();
        thread::spawn(move || {
            loop {
                let text = ctx.get_contents().unwrap();
                tx.send(text).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });
        rx
    }
}
