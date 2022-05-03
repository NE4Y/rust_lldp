extern crate rust_lldp;

use rust_lldp::NetworkManager;

mod layer2;

fn main() {
    let nm = NetworkManager::new();

    nm.start();

    let b = vec![0x01, 0x80, 0xc2, 0x00, 0x00, 0x0e, 0xdc, 0x39,
                 0x6f, 0x56, 0x6f, 0x56, 0x88, 0xcc, 0x33];
    let p = layer2::ethernet::Ethernet::new(&b);


}