pub struct Torrent {
    pub announce: String, //tracker URL
    pub info: Info,       //main metadata
}

pub struct Info {
    pub name: String,              //torrent name/file name
    pub piece_length: u64,         //size of each piece in bytes
    pub pieces: Vec<[u8; 20]>,     //list of SHA-1 piece hashes
    pub file_details: FileDetails, //single/multi file torrent
}

pub enum FileDetails {
    SingleFile { length: u64 }, //file length in bytes for single file torrent
    MultiFile { files: Vec<FileEntry> }, //list of files for multi file torrent
}

pub struct FileEntry {
    pub length: u64,       //file length in bytes
    pub path: Vec<String>, //path components
}
