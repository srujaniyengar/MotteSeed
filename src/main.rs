mod core;
mod util;

use core::torrent::torrent::TorrentFile;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let torrent_file = TorrentFile::from_file(&Path::new(&file_path)).unwrap();
    println!("{:?}", torrent_file);
}
