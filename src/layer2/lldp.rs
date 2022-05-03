use std::fmt;
use std::fmt::Formatter;
use byteorder::{BigEndian, ByteOrder};
use crate::layer2::ethernet::MacAddress;
use nom::{bits, bits::complete::take, IResult};

use crate::misc::tlv::*;

#[derive(Debug)]
pub struct LLDPU<'a> {
    tlvs: Vec<Box<dyn TLV + 'a>>
}

impl<'a> LLDPU<'a> {
    pub fn new(bytes: &'a [u8]) -> LLDPU {
        let mut tlvs:  Vec<Box<dyn TLV>> = vec![];
        let type_and_length = BigEndian::read_u16(&bytes[..16]);

        let t = (type_and_length >> 9) as u8;
        let l = (((type_and_length << 7) as u16) >> 7) as u8;

        println!("type: {}, length: {}", t, l);

        match t {
            1 => tlvs.push(Box::new(ChassisIdTLV::new(l as TlvLength, bytes))),
            _ => {}
        }


        LLDPU {
            tlvs
        }
    }
}

impl fmt::Display for LLDPU<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = format!("{}", 5);
        write!(f, "{}", s)
    }
}
