use std::fmt;
use std::fmt::Formatter;
use byteorder::{BigEndian, ByteOrder};
use crate::layer2::ethernet::MacAddress;
use nom::{bits, bits::complete::take, IResult};
use crate::misc::Packet;

use crate::misc::tlv::*;

#[derive(Debug)]
pub struct LLDPU<'a> {
    tlvs: Vec<Box<dyn TLV + 'a>>
}

impl<'a> LLDPU<'a> {
    pub fn new(bytes: &'a [u8]) -> LLDPU {
        let mut tlvs:  Vec<Box<dyn TLV>> = vec![];

        let mut pos = 0;

        // parse all tlvs
        while pos < bytes.len() {
            // type + length = 2 bytes
            let type_and_length = BigEndian::read_u16(&bytes[pos..pos+2]);
            let t = (type_and_length >> 9) as u8;
            let l = (((type_and_length << 7) as u16) >> 7) as u8;

            // end of LLDPU
            if t == 0 {
                break
            }

            // payload according to length
            let value = &bytes[pos+2..(pos + 2 + l as usize) as usize];
            pos += 2 + l as usize;

            let tlv: Box<dyn TLV> = match t {
                1 => Box::new(ChassisIdTLV::new(l as TlvLength, value)),
                2 => Box::new(PortTLV::new(l as TlvLength, value)),
                3 => Box::new(TimeToLiveTLV::new(l as TlvLength, value)),
                _ => Box::new(UnknownTLV::new(t, l as TlvLength, value))
            };

            tlvs.push(tlv);
        }



        LLDPU {
            tlvs
        }
    }
}

impl fmt::Display for LLDPU<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("LLDP Frame (size {})", self.get_size()));

        for tlv in &self.tlvs {
            write!(f, "\r\n{}", tlv);
        }

        Ok(())
    }
}

impl<'a> Packet for LLDPU<'a> {
    fn get_size(&self) -> u32 {
        let mut size: u32 = 0;

        for tlv in &self.tlvs {
            size += tlv.get_length() as u32 + 2;
        }

        size
    }
}
