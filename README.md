# Rust BitTorrent Client

A BitTorrent client implementation in Rust, focusing on performance, correctness, and modularity.

## Features

* Parsing `.torrent` files.
* Tracker communication.
* Peer connection and communication.
* Piece management and verification.
* Disk I/O for file storage.

## Getting Started

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/pushkar-gr/MotteSeed.git
    cd MotteSeed
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Run the client:**

    ```bash
    cargo run --release -- <torrent_file_path>
    ```

## Contributing

Contributions are welcome! Please submit pull requests or open issues for bugs and feature requests.
