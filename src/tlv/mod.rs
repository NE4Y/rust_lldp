pub mod port_id;
pub mod ttl;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use crate::layer2::ethernet::MacAddress;

pub trait Packet : Display {
    fn get_size(&self) -> u32;
}

pub type TlvLength = u16;
pub type TlvValue = [u8];

#[derive(Debug)]
pub enum TlvType {
    ChassisID,
    PortID,
    PortDescription,
    TimeToLive,
    ManagementAddress,
    SystemName,
    SystemDescription,
    Unknown(u8),
}

impl TlvType {
    fn get_type(t: u8) -> TlvType {
        match t {
            1 => TlvType::ChassisID,
            2 => TlvType::PortID,
            3 => TlvType::TimeToLive,
            4 => TlvType::PortDescription,
            5 => TlvType::SystemName,
            6 => TlvType::SystemDescription,
            8 => TlvType::ManagementAddress,
            _ => TlvType::Unknown(t)
        }
    }
}

impl Display for TlvType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChassisID => write!(f, "Chassis ID"),
            Self::PortID => write!(f, "Port ID"),
            Self::PortDescription => write!(f, "Port Description"),
            Self::TimeToLive => write!(f, "Time to Live"),
            Self::SystemName => write!(f, "System Name"),
            Self::SystemDescription => write!(f, "System Description"),
            Self::ManagementAddress => write!(f, "Management Adress"),
            Self::Unknown(t) => write!(f, "Unknown TLV (type {})", t)
        }
    }
}

pub trait TLV: Debug + Display {
    fn get_type(&self) -> &TlvType;
    fn get_length(&self) -> TlvLength;
    fn get_value(&self) -> &TlvValue;
}

/*
ChassisID TLV
 */
#[derive(Debug)]
pub struct ChassisIdTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
}

impl<'a> Display for ChassisIdTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {})", self.tlv_type, self.tlv_length)
    }
}

impl<'a> TLV for ChassisIdTLV<'a> {
    fn get_type(&self) -> &TlvType {
        &self.tlv_type
    }

    fn get_length(&self) -> TlvLength {
        self.tlv_length
    }

    fn get_value(&self) -> &TlvValue {
        &self.tlv_value
    }
}

impl<'a> ChassisIdTLV<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> ChassisIdTLV<'a> {
        ChassisIdTLV {
            tlv_type: TlvType::ChassisID,
            tlv_length: length,
            tlv_value: value,
        }
    }
}

/*
Port TLV
 */


/*
Port Description
*/
#[derive(Debug)]
pub struct PortDescriptionTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
}

impl<'a> PortDescriptionTLV<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> PortDescriptionTLV<'a> {
        PortDescriptionTLV {
            tlv_type: TlvType::PortDescription,
            tlv_length: length,
            tlv_value: value
        }
    }
}


impl<'a> Display for PortDescriptionTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Description: {}", self.tlv_type, self.tlv_length, std::str::from_utf8(&self.tlv_value).unwrap().to_string())
    }
}

impl<'a> TLV for PortDescriptionTLV<'a> {
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

/*
Time to Live TLV
 */

/*
Unknown TLV
 */

#[derive(Debug)]
pub struct UnknownTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
}

impl<'a> UnknownTLV<'a> {
    pub fn new(tlv_type: u8, length: TlvLength, value: &'a TlvValue) -> UnknownTLV<'a> {
        UnknownTLV {
            tlv_type: TlvType::Unknown(tlv_type),
            tlv_length: length,
            tlv_value: value,
        }
    }
}

impl<'a> Display for UnknownTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {})", self.tlv_type, self.tlv_length)
    }
}

impl<'a> TLV for UnknownTLV<'a> {
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


/*
Management Address TLV
 */

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

impl<'a> ManagementAddressTLV<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> ManagementAddressTLV<'a> {
        // length of management address string
        let string_length = value[0] - 1;

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

/*
System Name TLV
 */
#[derive(Debug)]
pub struct SystemName<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
}

impl<'a> SystemName<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> SystemName<'a> {
        SystemName {
            tlv_type: TlvType::SystemName,
            tlv_length: length,
            tlv_value: value
        }
    }
}


impl<'a> Display for SystemName<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Name: {}", self.tlv_type, self.tlv_length, std::str::from_utf8(&self.tlv_value).unwrap().to_string())
    }
}

impl<'a> TLV for SystemName<'a> {
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

/*
System Description
 */
#[derive(Debug)]
pub struct SystemDescription<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
}

impl<'a> SystemDescription<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> SystemDescription<'a> {
        SystemDescription {
            tlv_type: TlvType::SystemDescription,
            tlv_length: length,
            tlv_value: value
        }
    }
}


impl<'a> Display for SystemDescription<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Description: {}", self.tlv_type, self.tlv_length, std::str::from_utf8(&self.tlv_value).unwrap().to_string())
    }
}

impl<'a> TLV for SystemDescription<'a> {
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