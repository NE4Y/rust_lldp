use std::fmt;
use std::fmt::{Display, Formatter};
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

#[derive(Debug)]
pub struct SystemDescription<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue
}

pub fn new(length: TlvLength, value: &TlvValue) -> SystemDescription {
    SystemDescription {
        tlv_type: TlvType::SystemDescription,
        tlv_length: length,
        tlv_value: value
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