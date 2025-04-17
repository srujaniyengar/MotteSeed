#[derive(Debug)]
pub struct Peer<'a> {
    peer_id: &'a [u8; 20],
}
