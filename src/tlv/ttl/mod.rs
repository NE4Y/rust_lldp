use std::fmt;
use std::fmt::{Display, Formatter};
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

#[derive(Debug)]
pub struct TimeToLiveTLV<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    tlv_value: &'a TlvValue,
    ttl: u16,
}

pub fn new(length: TlvLength, value: &TlvValue) -> TimeToLiveTLV {
    let mut s = 0;

    // parse seconds
    for (i, v) in value.iter().rev().enumerate() {
        s += (*v as u16) << i;
    }

    TimeToLiveTLV {
        tlv_type: TlvType::TimeToLive,
        tlv_length: length,
        tlv_value: value,
        ttl: s,
    }
}

impl<'a> Display for TimeToLiveTLV<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Seconds: {}", self.tlv_type, self.tlv_length, self.ttl)
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