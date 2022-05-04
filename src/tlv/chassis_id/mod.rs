use std::fmt;
use std::fmt::{Display, Formatter};
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

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

pub fn new(length: TlvLength, value: & TlvValue) -> ChassisIdTLV {
    ChassisIdTLV {
        tlv_type: TlvType::ChassisID,
        tlv_length: length,
        tlv_value: value,
    }
}