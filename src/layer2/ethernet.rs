use std::fmt;
use std::fmt::Formatter;
use std::path::Display;

use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub enum EtherType {
    LLDP,
    Unknown(u16),
}

impl EtherType {
    fn get_type(t: u16) -> EtherType {
        match t {
            0x88cc => EtherType::LLDP,
            _ => EtherType::Unknown(t)
        }
    }
}

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Ethernet<'a> {
    mac_dst: MacAddress<'a>,
    mac_src: MacAddress<'a>,
    vlan: Option<Vlan>,
    ether_type: EtherType,
    payload: &'a [u8],
}

impl<'a> Ethernet<'a> {
    pub fn new(bytes: &[u8]) -> Ethernet {
        Ethernet {
            mac_dst: MacAddress(&bytes[..6]),
            mac_src: MacAddress(&bytes[6..13]),
            vlan: None,
            ether_type: EtherType::get_type(BigEndian::read_u16(&bytes[12..14])),
            payload: &bytes[14..],
        }
    }

    pub fn get_ether_type(&self) -> &EtherType {
        &self.ether_type
    }

    pub fn get_payload(&self)-> &[u8] {
        &self.payload
    }
}

impl<'a> fmt::Display for Ethernet<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Ethernet Frame \r\nSource: {} \r\nDestination: {} \r\nEthertype: {}",
               self.mac_src,
               self.mac_dst,
               self.ether_type)
    }
}

#[derive(Debug)]
pub struct MacAddress<'a>(pub &'a [u8]);

impl<'a> fmt::Display for MacAddress<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}-{}-{}-{}",
               format!("{:x}", self.0[0]),
               format!("{:x}", self.0[1]),
               format!("{:x}", self.0[2]),
               format!("{:x}", self.0[3]),
               format!("{:x}", self.0[4]),
               format!("{:x}", self.0[5]))
    }
}

#[derive(Debug)]
struct Vlan {}