use std::fmt;
use std::fmt::{Display, Formatter};
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

#[derive(Debug)]
enum AddressSubType {
    IPv4,
    Unknown(u8),
}

impl Display for AddressSubType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AddressSubType::IPv4 => write!(f, "IPv4"),
            AddressSubType::Unknown(t) => write!(f, "Unknown ({})", t)
        }
    }
}

#[derive(Debug)]
pub struct ManagementAddressTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
    management_string_length: u8,
    address_subtype: AddressSubType,
    management_address: String,
}

pub fn new(length: TlvLength, value: &TlvValue) -> ManagementAddressTLV {
    let address_type = match value[1] {
        1 => AddressSubType::IPv4,
        _ => AddressSubType::Unknown(value[1])
    };

    //let s = std::str::from_utf8().unwrap();
    let s = match address_type {
        AddressSubType::IPv4 => std::net::Ipv4Addr::new(value[2], value[3], value[4], value[5]).to_string(),
        AddressSubType::Unknown(_) => String::from("Unknown")
    };

    ManagementAddressTLV {
        tlv_type: TlvType::ManagementAddress,
        tlv_length: length,
        tlv_value: value,
        management_string_length: value[0],
        address_subtype: address_type,
        management_address: s.to_string(),
    }
}

impl<'a> Display for ManagementAddressTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Address Type: {} - Address: {}", self.tlv_type, self.tlv_length, self.address_subtype, self.management_address)
    }
}

impl<'a> TLV for ManagementAddressTLV<'a> {
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
