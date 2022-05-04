use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use byteorder::{BigEndian, ByteOrder};
use crate::layer2::ethernet::EtherType::Unknown;
use crate::misc::tlv::TLVType::{ChassisID, PortID};

pub type TlvType = TLVType;
pub type TlvLength = u16;
pub type TlvValue = [u8];

#[derive(Debug)]
enum TLVType {
    ChassisID,
    PortID,
    TimeToLive,
    Unknown(u8)
}

impl TLVType {
    fn get_type(t: u8) -> TLVType {
        match t {
            1 => TLVType::ChassisID,
            2 => TLVType::PortID,
            3 => TLVType::TimeToLive,
            _ => TLVType::Unknown(t)
        }
    }
}

impl Display for TLVType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChassisID => write!(f, "Chassis ID"),
            Self::PortID => write!(f, "Port ID"),
            Self::TimeToLive => write!(f, "Time to Live"),
            Self::Unknown(t) => write!(f, "Unknown TLV (type {})", t)
        }

    }
}

pub trait TLV : Debug + Display {
    fn get_type(&self) -> &TlvType;
    fn get_length(&self) -> TlvLength;
    fn get_value(&self) -> &TlvValue;
}

#[derive(Debug)]
pub struct ChassisIdTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
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
            tlv_type: TLVType::ChassisID,
            tlv_length: length,
            tlv_value: value
        }
    }
}

#[derive(Debug)]
pub struct PortTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
}

impl<'a> PortTLV<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> PortTLV<'a> {
        PortTLV {
            tlv_type: TLVType::PortID,
            tlv_length: length,
            tlv_value: value
        }
    }
}


impl<'a> Display for PortTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {})", self.tlv_type, self.tlv_length)
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
pub struct TimeToLiveTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
    seconds: u32
}

impl<'a> TimeToLiveTLV<'a> {
    pub fn new(length: TlvLength, value: &'a TlvValue) -> TimeToLiveTLV<'a> {
        let mut s = 0;

        // parse seconds
        for (i, v) in value.iter().rev().enumerate() {
            s += (*v as u32) << i;
        }

        TimeToLiveTLV {
            tlv_type: TLVType::TimeToLive,
            tlv_length: length,
            tlv_value: value,
            seconds: s
        }
    }
}


impl<'a> Display for TimeToLiveTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Seconds: {}", self.tlv_type, self.tlv_length, self.seconds)
    }
}

impl<'a> TLV for TimeToLiveTLV<'a> {
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
pub struct UnknownTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
}

impl<'a> UnknownTLV<'a> {
    pub fn new(tlv_type: u8, length: TlvLength, value: &'a TlvValue) -> PortTLV<'a> {
        PortTLV {
            tlv_type: TLVType::Unknown(tlv_type),
            tlv_length: length,
            tlv_value: value
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