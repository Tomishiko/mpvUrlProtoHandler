use std::env;
use std::process::Command;
use std::{self, io::Read};
use urlencoding::decode;
fn main() {
    let args: Vec<String> = env::args().collect();

    let offset_start = args[1].find("url=").expect("wrong args format");
    let offset_end = args[1].find("&").expect("wrong args format");
    let url: &str;
    unsafe {
        url = args[1].slice_unchecked(offset_start + 4, offset_end);
    }
    let decoded = decode(url).unwrap();
    let mut comm = Command::new("mpv.com")
        .arg(&decoded.as_ref())
        .status()
        .expect("failed to start");
}
