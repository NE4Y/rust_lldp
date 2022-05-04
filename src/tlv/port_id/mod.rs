use std::fmt;
use std::fmt::{Display, Formatter};
use crate::layer2::ethernet::MacAddress;
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

#[derive(Debug)]
pub struct PortTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
    port_type: PortIDSubType,
    address: String,
}

pub fn new(length: TlvLength, value: &TlvValue) -> PortTLV {
    let port_type = match value[0] {
        3 => PortIDSubType::MACAddress,
        _ => PortIDSubType::Unknown(value[0])
    };

    let address = match port_type {
        PortIDSubType::MACAddress => MacAddress(&value[1..(length as usize)]).to_string(),
        PortIDSubType::Unknown(_) => String::from("Unknown")
    };

    PortTLV {
        tlv_type: TlvType::PortID,
        tlv_length: length,
        tlv_value: value,
        port_type,
        address,
    }
}

impl<'a> Display for PortTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Port type: {} - Port id: {}",
               self.tlv_type,
               self.tlv_length,
               self.port_type,
               self.address)
    }
}

impl<'a> TLV for PortTLV<'a> {
    fn get_type(&self) -> &TlvType {
        &self.tlv_type
    }

    fn get_length(&self) -> TlvLength {
        self.tlv_length
    }

    fn get_value(&self) -> &TlvValue {
        self.tlv_value
    }
}

#[derive(Debug)]
enum PortIDSubType {
    MACAddress,
    Unknown(u8),
}

impl Display for PortIDSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PortIDSubType::MACAddress => write!(f, "MAC address"),
            PortIDSubType::Unknown(t) => write!(f, "Unknown ({})", t)
        }
    }
}