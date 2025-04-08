use crate::core::torrent::torrent_error::{BDecoderError, BStreamingError, ReadTorrentError};

use bencode::from_buffer;
use rustc_serialize::{Decodable, Decoder};

#[derive(Debug)]
pub struct Torrent {
    pub announce: String, //tracker URL
    pub info: Info,       //main metadata
}

impl Decodable for Torrent {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        //read root dictionary
        d.read_struct("Torrent", 1, |d| {
            //read torrent url
            let announce = d.read_struct_field("announce", 0, |d| d.read_str())?;

            //read torrent info
            let info = d.read_struct_field("info", 1, |d| Info::decode(d))?;

            Ok(Self { announce, info })
        })
    }
}

#[derive(Debug)]
pub struct Info {
    pub name: String,              //torrent name/file name
    pub piece_length: u64,         //size of each piece in bytes
    pub pieces: Vec<[u8; 20]>,     //list of SHA-1 piece hashes
    pub file_details: FileDetails, //single/multi file torrent
}

impl Decodable for Info {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        //read info dictionary
        d.read_struct("info", 3, |d| {
            //read file name
            let name = d.read_struct_field("name", 0, |d| d.read_str())?;

            //read piece length
            let piece_length: u64 = d.read_struct_field("piece length", 1, |d| d.read_u64())?;

            //read SHA-1 pieces
            let pieces = d.read_struct_field("pieces", 2, |d| {
                let pieces_str = d.read_str()?;
                //convert string to bytes
                let pieces_bytes = pieces_str.as_bytes();
                let len = pieces_str.len();

                //validate that the length of the pieces is a multiple of 20
                if len % 20 != 0 {
                    Err(d.error("Invalid 'pieces' length: not a multiple of 20"))
                } else {
                    //convert bytes list into Vec<[u8; 20]>
                    let pieces = pieces_bytes
                        .chunks_exact(20)
                        .map(|chunk| chunk.try_into().unwrap())
                        .collect::<Vec<[u8; 20]>>();
                    Ok(pieces)
                }
            })?;

            //read file details
            let file_details =
                //read length for single file torrent
                if let Ok(length) = d.read_struct_field("length", 3, |d| d.read_u64()) {
                    FileDetails::SingleFile {
                        length,
                    }
                } else {
                    //read file entries for multi file torrent
                    d.read_struct_field("files", 3, |d| {
                        d.read_seq(|d, len| {
                            let mut files = Vec::new();
                            for i in 0..len {
                                let file = d.read_seq_elt(i, |d| FileEntry::decode(d))?;
                                files.push(file);
                            }
                            Ok(FileDetails::MultiFile { files })
                        })
                    })?
                };

            Ok(Self {
                name,
                piece_length,
                pieces,
                file_details,
            })
        })
    }
}

#[derive(Debug)]
pub enum FileDetails {
    SingleFile { length: u64 }, //file length in bytes for single file torrent
    MultiFile { files: Vec<FileEntry> }, //list of files for multi file torrent
}

#[derive(Debug)]
pub struct FileEntry {
    pub length: u64,       //file length in bytes
    pub path: Vec<String>, //path components
}

impl Decodable for FileEntry {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        //read length and file structure
        d.read_struct("files", 2, |d| {
            //read length
            let length: u64 = d.read_struct_field("length", 0, |d| d.read_u64())?;

            //read path
            let path = d.read_struct_field("path", 1, |d| {
                d.read_seq(|d, len| {
                    let mut path_vec = Vec::new();
                    for i in 0..len {
                        let file = d.read_seq_elt(i, |d| d.read_str())?;
                        path_vec.push(file);
                    }
                    Ok(path_vec)
                })
            })?;

            Ok(Self { length, path })
        })
    }
}

impl Torrent {
    //create Torrent from bytes
    //input: bytes
    //output: Torrent struct or error if any
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ReadTorrentError> {
        let bencode = from_buffer(bytes).map_err(BStreamingError::from)?;
        let mut decoder = bencode::Decoder::new(&bencode);
        Ok(Decodable::decode(&mut decoder).map_err(BDecoderError::from)?)
    }
}
