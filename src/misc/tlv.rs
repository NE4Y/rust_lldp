use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub type TlvType = u8;
pub type TlvLength = u16;
pub type TlvValue = [u8];

pub trait TLV : Debug + Display {
    fn get_type(&self) -> TlvType;
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
        write!(f, "{}", 5)
    }
}

impl<'a> TLV for ChassisIdTLV<'a> {
    fn get_type(&self) -> TlvType {
        self.tlv_type
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
            tlv_type: 1,
            tlv_length: length,
            tlv_value: value
        }
    }
}
