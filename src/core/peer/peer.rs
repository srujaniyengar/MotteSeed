use std::array::TryFromSliceError;

#[derive(Debug)]
pub struct Peer {
    peer_ip: [u8; 4], //ip address of peer
    peer_port: u16,   //connection port for peer
}

impl Peer {
    pub fn decode(bytes: &[u8; 6]) -> Result<Self, TryFromSliceError> {
        Ok(Self {
            peer_ip: bytes[0..4].try_into()?,
            peer_port: u16::from_be_bytes(bytes[4..6].try_into()?),
        })
    }
}
