extern crate pcap;
mod layer2;
mod misc;
pub struct NetParser {}

impl NetParser {
    pub fn new() -> Self {
        NetParser {}
    }

    pub fn start(&self) {
        let device = pcap::Device::lookup().expect("device lookup failed");
        println!("Using device {}", device.name);

        // Setup Capture
        let mut cap = pcap::Capture::from_device(device)
            .unwrap()
            .immediate_mode(true)
            .open()
            .unwrap();

        // only look for lldp
        cap.filter("ether dst 01:80:C2:00:00:0E", true).expect("LLDP filter failed.");

        while let Ok(packet) = cap.next() {
            let ether = layer2::ethernet::Ethernet::new(packet.data);
            println!("{} \r\n ------ \r\n", ether);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
