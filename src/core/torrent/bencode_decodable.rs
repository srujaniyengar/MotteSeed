use crate::core::torrent::torrent_error::ReadTorrentError;

use bencode::Bencode;
use bencode::util::ByteString;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::collections::BTreeMap;

//define cached keys
static LENGTH_KEY: Lazy<ByteString> = Lazy::new(|| ByteString::from_str("length"));
static PATH_KEY: Lazy<ByteString> = Lazy::new(|| ByteString::from_str("path"));

//a trait for decoding Bencode data into Rust types
pub trait BencodeDecodable<'a>: Sized {
    //decode Bencode into Self
    fn decode(b: &'a Bencode) -> Result<Self, ReadTorrentError>;

    //extract u64 value from a Bencode Number variant
    fn get_u64(b: &'a Bencode) -> Result<u64, ReadTorrentError> {
        match b {
            Bencode::Number(num) => Ok((*num)
                .try_into()
                .map_err(|_| ReadTorrentError::WrongType("Expected a Number".into()))?),
            _ => Err(ReadTorrentError::WrongType("Expected a Number".into())),
        }
    }

    //extract raw bytes from a Bencode ByteString variant
    fn get_str(b: &'a Bencode) -> Result<&'a [u8], ReadTorrentError> {
        match b {
            Bencode::ByteString(bytes) => Ok(bytes),
            _ => Err(ReadTorrentError::WrongType("Expected a ByteString".into())),
        }
    }

    //extract string from a Bencode ByteString variant
    fn get_string(b: &'a Bencode) -> Result<Cow<'a, str>, ReadTorrentError> {
        let bytes = Self::get_str(b)?;
        Ok(String::from_utf8_lossy(bytes))
    }

    //extract dictionary from a Bencode Dict variant
    fn get_struct(b: &'a Bencode) -> Result<&'a BTreeMap<ByteString, Bencode>, ReadTorrentError> {
        match b {
            Bencode::Dict(dict_map) => Ok(dict_map),
            _ => Err(ReadTorrentError::WrongType("Expected a dictionary".into())),
        }
    }

    //retrieve value from a Bencode dictionary by key
    fn get_struct_value_from_bytestring(
        key: &ByteString,
        dict_map: &'a BTreeMap<ByteString, Bencode>,
    ) -> Result<&'a Bencode, ReadTorrentError> {
        dict_map
            .get(key)
            .ok_or_else(|| ReadTorrentError::KeyNotFound(format!("Key '{}' not found", key)))
    }

    //retrieve value from a Bencode dictionary by key
    fn get_struct_value(
        key: &str,
        dict_map: &'a BTreeMap<ByteString, Bencode>,
    ) -> Result<&'a Bencode, ReadTorrentError> {
        Self::get_struct_value_from_bytestring(&ByteString::from_str(key), dict_map)
    }

    //extracts a list from a Bencode List variant
    fn get_list(b: &'a Bencode) -> Result<&'a Vec<Bencode>, ReadTorrentError> {
        match b {
            Bencode::List(list) => Ok(list),
            _ => Err(ReadTorrentError::WrongType("Expected a list".into())),
        }
    }

    //use cache keys to get values

    //retrieve length value from Bencode dictionary
    fn get_length(
        dict_map: &'a BTreeMap<ByteString, Bencode>,
    ) -> Result<&'a Bencode, ReadTorrentError> {
        Self::get_struct_value_from_bytestring(&LENGTH_KEY, dict_map)
    }

    //retrieve path value from Bencode dictionary
    fn get_path(
        dict_map: &'a BTreeMap<ByteString, Bencode>,
    ) -> Result<&'a Bencode, ReadTorrentError> {
        Self::get_struct_value_from_bytestring(&PATH_KEY, dict_map)
    }
}
