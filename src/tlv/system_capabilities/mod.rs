use std::fmt;
use std::fmt::{Display, Formatter};
use crate::tlv::{TLV, TlvLength, TlvType, TlvValue};

#[derive(Debug)]
enum Capabilities {
    Other(bool),
    Repeater(bool),
    Bridge(bool),
    WLANAccessPoint(bool),
    Router(bool),
    Telephone(bool),
    DOCSIS(bool),
    StationOnly(bool),
}

impl Display for Capabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct SystemCapabilities<'a> {
    tlv_type: TlvType,
    tlv_length: TlvLength,
    capabilities: Vec<Capabilities>,
    enabled: Vec<Capabilities>,
    tlv_value: &'a TlvValue,
}

pub fn new(length: TlvLength, value: &TlvValue) -> SystemCapabilities {
    let capabilities = vec![Capabilities::Other((value[1] & 1) == 1),
                            Capabilities::Repeater(((value[1] >> 1) & 1) == 1),
                            Capabilities::Bridge(((value[1] >> 2) & 1) == 1),
                            Capabilities::WLANAccessPoint(((value[1] >> 3) & 1) == 1),
                            Capabilities::Router(((value[1] >> 4) & 1) == 1),
                            Capabilities::Telephone(((value[1] >> 5) & 1) == 1),
                            Capabilities::DOCSIS(((value[1] >> 6) & 1) == 1),
                            Capabilities::StationOnly(((value[1] >> 7) & 1) == 1)];

    let enabled = vec![Capabilities::Other((value[3] & 1) == 1),
                       Capabilities::Repeater(((value[3] >> 1) & 1) == 1),
                       Capabilities::Bridge(((value[3] >> 2) & 1) == 1),
                       Capabilities::WLANAccessPoint(((value[3] >> 3) & 1) == 1),
                       Capabilities::Router(((value[3] >> 4) & 1) == 1),
                       Capabilities::Telephone(((value[3] >> 5) & 1) == 1),
                       Capabilities::DOCSIS(((value[3] >> 6) & 1) == 1),
                       Capabilities::StationOnly(((value[3] >> 7) & 1) == 1)];

    SystemCapabilities {
        tlv_type: TlvType::SystemCapabilities,
        tlv_length: length,
        tlv_value: value,
        capabilities,
        enabled,
    }
}

impl<'a> Display for SystemCapabilities<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (length {}) - Capabilities: {:?} - Enabled: {:?}",
               self.tlv_type,
               self.tlv_length,
               self.capabilities,
               self.enabled)
    }
}

impl<'a> TLV for SystemCapabilities<'a> {
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