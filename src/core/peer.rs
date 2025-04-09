use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Instant;

// struct: Represents a peer in the BitTorrent network
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
    pub peer_id: [u8; 20],
    pub am_choking: bool,       //is we choking
    pub am_interested: bool,    // is we want
    pub peer_choking: bool,     // is they choking
    pub peer_interested: bool,  // is they want
    pub bitfield: Vec<u8>,
    pub downloading: bool,
    pub uploading: bool,
    pub available_pieces: Vec<usize>,
    pub outstanding_requests: HashMap<u32, u32>,
    pub last_active: Instant,
}

// impl: Implementation block for the Peer struct
impl Peer {
    // func: creates a new Peer instance with default values
    pub fn new(ip: IpAddr, port: u16, peer_id: [u8; 20]) -> Peer {
        Peer {
            ip,
            port,
            peer_id,
            am_choking: true,
            am_interested: false,
            peer_choking: true,
            peer_interested: false,
            bitfield: Vec::new(),
            downloading: false,
            uploading: false,
            available_pieces: Vec::new(),
            outstanding_requests: HashMap::new(),
            last_active: Instant::now(),
        }
    }

    // func: sets the bitfield of the peer and updates the available pieces
    pub fn set_bitfield(&mut self, bitfield: Vec<u8>) {
        self.bitfield = bitfield;
        self.update_available_pieces();
    }

    // func: checks if the peer has a specific piece
    pub fn has_piece(&self, piece_index: usize) -> bool {
        if self.bitfield.is_empty() {
            return false;
        }
        let byte_index = piece_index / 8;
        let bit_index = 7 - (piece_index % 8);
        if byte_index >= self.bitfield.len() {
            return false;
        }
        (self.bitfield[byte_index] >> bit_index) & 1 == 1
    }

    // func: sets whether we are choking the peer
    pub fn set_am_choking(&mut self, choking: bool) {
        self.am_choking = choking;
    }

    // func: gets whether we are choking the peer
    pub fn get_am_choking(&self) -> bool {
        self.am_choking
    }

    // func: sets whether we are interested in the peer
    pub fn set_am_interested(&mut self, interested: bool) {
        self.am_interested = interested;
    }

    // func: gets whether we are interested in the peer
    pub fn get_am_interested(&self) -> bool {
        self.am_interested
    }

    // func: sets whether the peer is choking us
    pub fn set_peer_choking(&mut self, choking: bool) {
        self.peer_choking = choking;
    }

    // func: gets whether the peer is choking us
    pub fn get_peer_choking(&self) -> bool {
        self.peer_choking
    }

    // func: sets whether the peer is interested in us
    pub fn set_peer_interested(&mut self, interested: bool) {
        self.peer_interested = interested;
    }

    // func: gets whether the peer is interested in us
    pub fn get_peer_interested(&self) -> bool {
        self.peer_interested
    }

    // func: updates the list of available pieces based on the bitfield
    pub fn update_available_pieces(&mut self) {
        self.available_pieces.clear();
        for (i, byte) in self.bitfield.iter().enumerate() {
            for j in 0..8 {
                if (byte >> (7 - j)) & 1 == 1 {
                    self.available_pieces.push(i * 8 + j);
                }
            }
        }
    }

    // func: gets a reference to the list of available pieces
    pub fn get_available_pieces(&self) -> &Vec<usize> {
        &self.available_pieces
    }

    // func: adds a new outstanding request for a block
    pub fn add_outstanding_request(&mut self, begin: u32, length: u32) {
        self.outstanding_requests.insert(begin, length);
    }

    // func: removes an outstanding request for a block
    pub fn remove_outstanding_request(&mut self, begin: u32) {
        self.outstanding_requests.remove(&begin);
    }

    // func: gets a reference to the map of outstanding requests
    pub fn get_outstanding_requests(&self) -> &HashMap<u32, u32> {
        &self.outstanding_requests
    }

    // func: sets the downloading status of the peer
    pub fn set_downloading(&mut self, downloading: bool) {
        self.downloading = downloading;
    }

    // func: gets the downloading status of the peer
    pub fn get_downloading(&self) -> bool {
        self.downloading
    }

    // func: sets the uploading status of the peer
    pub fn set_uploading(&mut self, uploading: bool) {
        self.uploading = uploading;
    }

    // func: gets the uploading status of the peer
    pub fn get_uploading(&self) -> bool {
        self.uploading
    }

    // func: updates the last active timestamp of the peer
    pub fn update_last_active(&mut self) {
        self.last_active = Instant::now();
    }

    // func: gets a reference to the last active timestamp of the peer
    pub fn get_last_active(&self) -> &Instant {
        &self.last_active
    }
}
