use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "s", long, default_value = "en")]
    pub src: String,
    #[structopt(short = "d", long, default_value = "ko")]
    pub dst: String,
    #[structopt(short = "w", long, default_value = "1024")]
    pub width: u32,
}

fn main() {
    let args = Args::from_args();

    if let Err(e) = ::clipboard_google_translate::init(args.src, args.dst, args.width) {
        fatal(format!("initialization error: {}", e));
    }
}

fn fatal<S: AsRef<str>>(msg: S) -> ! {
    eprintln!("rx: {}", msg.as_ref());
    process::exit(1);
}
