pub mod port_id;
pub mod ttl;
pub mod chassis_id;
pub mod port_description;
pub mod unknown;
pub mod management_address;
pub mod system_name;
pub mod system_description;
pub mod system_capabilities;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

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
    SystemCapabilities,
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
            7 => TlvType::SystemCapabilities,
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
            Self::SystemCapabilities => write!(f, "System Capabilities"),
            Self::ManagementAddress => write!(f, "Management Address"),
            Self::Unknown(t) => write!(f, "Unknown TLV (type {})", t)
        }
    }
}

pub trait TLV: Debug + Display {
    fn get_type(&self) -> &TlvType;
    fn get_length(&self) -> TlvLength;
    fn get_value(&self) -> &TlvValue;
}



