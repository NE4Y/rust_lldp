use std::fmt;
use std::fmt::{Display, Formatter};
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

#[derive(Debug)]
pub struct UnknownTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
}

pub fn new(tlv_type: u8, length: TlvLength, value: &TlvValue) -> UnknownTLV {
    UnknownTLV {
        tlv_type: TlvType::Unknown(tlv_type),
        tlv_length: length,
        tlv_value: value,
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