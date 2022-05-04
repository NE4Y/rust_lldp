use std::fmt;
use std::fmt::Formatter;
use byteorder::{BigEndian, ByteOrder};
use crate::layer2::ethernet::EtherType::LLDP;
use crate::layer2::lldp::LLDPU;
use crate::tlv::Packet;

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

    fn get_value(t: EtherType) -> Result<u16, EtherType> {
        match t {
            EtherType::LLDP => Ok(0x88cc as u16),
            _ => Err(t)
        }
    }
}

impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Ethernet<'a> {
    mac_dst: MacAddress<'a>,
    mac_src: MacAddress<'a>,
    vlan: Option<Vlan>,
    ether_type: EtherType,
    payload: &'a [u8],
    next: Option<Box<dyn Packet + 'a>>
}

impl<'a> Ethernet<'a> {
    pub fn new(bytes: &[u8]) -> Ethernet {
        let ether_type = EtherType::get_type(BigEndian::read_u16(&bytes[12..14]));

        let next_packet: Option<Box<dyn Packet>> = match ether_type {
            EtherType::LLDP => Some(Box::new(LLDPU::new(&bytes[14..]))),
            _ => None
        };

        Ethernet {
            mac_dst: MacAddress(&bytes[..6]),
            mac_src: MacAddress(&bytes[6..13]),
            vlan: None,
            ether_type,
            payload: &bytes[14..],
            next: next_packet
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
        write!(f, "Ethernet Frame (size: {})\nSource: {} \nDestination: {} \nEthertype: {}\n",
               self.get_size(),
               self.mac_src,
               self.mac_dst,
               self.ether_type)?;

               match self.next.as_ref() {
                   None => write!(f, "{}", 5),
                   Some(p) => write!(f, "----- \r\n{}", p)
               }
    }
}

impl<'a> Packet for Ethernet<'a> {
    fn get_size(&self) -> u32 {
        5
    }
}


#[derive(Debug)]
pub struct MacAddress<'a>(pub &'a [u8]);

impl<'a> fmt::Display for MacAddress<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}:{}:{}:{}",
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