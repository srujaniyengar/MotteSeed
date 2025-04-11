use crate::core::torrent::torrent_error::{BStreamingError, ReadTorrentError};

use bencode::util::ByteString;
use bencode::{Bencode, from_buffer};
use sha1::{Digest, Sha1};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

trait BencodeDecodable: Sized {
    //decode Bencode and return Self
    fn decode(b: &Bencode) -> Result<Self, ReadTorrentError>;

    //get u64 value from Bencode
    fn get_u64(b: &Bencode) -> Result<u64, ReadTorrentError> {
        match b {
            Bencode::Number(num) => Ok((*num)
                .try_into()
                .map_err(|_| ReadTorrentError::LogicError("Expected a Number".into()))?),
            _ => Err(ReadTorrentError::LogicError("Expected a Number".into())),
        }
    }

    //get raw bytes from Bencode
    fn get_str<'a>(b: &'a Bencode) -> Result<&'a [u8], ReadTorrentError> {
        match b {
            Bencode::ByteString(bytes) => Ok(bytes),
            _ => Err(ReadTorrentError::LogicError("Expected a ByteString".into())),
        }
    }

    //get string from Bencode
    fn get_string(b: &Bencode) -> Result<Cow<'_, str>, ReadTorrentError> {
        let bytes = Self::get_str(b)?;
        Ok(String::from_utf8_lossy(bytes))
    }

    //get dict_map from Bencode
    fn get_struct(b: &Bencode) -> Result<&BTreeMap<ByteString, Bencode>, ReadTorrentError> {
        match b {
            Bencode::Dict(dict_map) => Ok(dict_map),
            _ => Err(ReadTorrentError::LogicError("Expected a dictionary".into())),
        }
    }

    //get value from BTreeMap
    fn get_struct_value<'a>(
        key: &str,
        dict_map: &'a BTreeMap<ByteString, Bencode>,
    ) -> Result<&'a Bencode, ReadTorrentError> {
        dict_map
            .get(&ByteString::from_str(key))
            .ok_or_else(|| ReadTorrentError::LogicError(format!("Key '{}' not found", key)))
    }

    //get list from Bencode
    fn get_list(b: &Bencode) -> Result<&Vec<Bencode>, ReadTorrentError> {
        match b {
            Bencode::List(list) => Ok(list),
            _ => Err(ReadTorrentError::LogicError("Expected a list".into())),
        }
    }
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: String,    //tracker URL
    pub info: Info,          //main metadata
    pub info_hash: [u8; 20], //SHA1 encoding of bencode value of info
}

impl BencodeDecodable for Torrent {
    fn decode(b: &Bencode) -> Result<Self, ReadTorrentError> {
        let dict = Self::get_struct(b)?;
        let announce = Self::get_string(Self::get_struct_value("announce", dict)?)?.to_string();
        let info_dict = Self::get_struct_value("info", dict)?;
        let info = Info::decode(info_dict)?;
        let info_bytes = info_dict.to_bytes()?;

        //calculate sha1 of info
        let mut hasher = Sha1::new();
        hasher.update(&info_bytes);
        let info_hash = hasher.finalize();

        Ok(Self {
            announce,
            info,
            info_hash: info_hash.into(),
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

impl BencodeDecodable for Info {
    fn decode(b: &Bencode) -> Result<Self, ReadTorrentError> {
        let dict = Self::get_struct(b)?;
        let name = Self::get_string(Self::get_struct_value("name", dict)?)?.to_string();
        let piece_length = Self::get_u64(Self::get_struct_value("piece length", dict)?)?;

        let raw_pieces = Self::get_str(Self::get_struct_value("pieces", dict)?)?;

        if raw_pieces.len() % 20 != 0 {
            return Err(ReadTorrentError::LogicError("Invalid pieces length".into()));
        }

        let mut pieces = Vec::with_capacity(raw_pieces.len() / 20);
        for chunk in raw_pieces.chunks_exact(20) {
            let mut hash = [0u8; 20];
            hash.copy_from_slice(chunk);
            pieces.push(hash);
        }

        let file_details = match Self::get_struct_value("length", dict) {
            Ok(b) => FileDetails::SingleFile {
                length: Self::get_u64(b)?,
            },
            _ => FileDetails::MultiFile {
                files: {
                    let file_list = Self::get_list(Self::get_struct_value("files", dict)?)?;

                    let mut files = Vec::with_capacity(file_list.len());
                    for file_item in file_list {
                        files.push(FileEntry::decode(file_item)?)
                    }

                    files
                },
            },
        };

        Ok(Self {
            name,
            piece_length,
            pieces,
            file_details,
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

impl BencodeDecodable for FileEntry {
    fn decode(b: &Bencode) -> Result<Self, ReadTorrentError> {
        let dict = Self::get_struct(b)?;
        let length = Self::get_u64(Self::get_struct_value("length", dict)?)?;
        let path_list = Self::get_list(Self::get_struct_value("path", dict)?)?;

        let mut path = Vec::with_capacity(path_list.len());
        for path_item in path_list {
            path.push(Self::get_string(path_item)?.to_string());
        }

        Ok(Self { length, path })
    }
}

impl Torrent {
    //create Torrent from bytes
    //input: bytes
    //output: Torrent struct or error if any
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ReadTorrentError> {
        //convert bytes to Torrent struct
        let bencode = from_buffer(bytes).map_err(BStreamingError::from)?;
        let torrent = Torrent::decode(&bencode)?;

        Ok(torrent)
    }

    //Create Torrent from file
    //input: file path
    //output: Torrent struct or error if any
    pub fn from_file(file: &Path) -> Result<Self, ReadTorrentError> {
        let content = fs::read(file).map_err(|e| ReadTorrentError::IOError(e))?;
        Self::from_bytes(&content)
    }
}
