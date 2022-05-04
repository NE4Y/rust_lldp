use std::fmt::Display;

pub mod tlv;

pub trait Packet : Display {
    fn get_size(&self) -> u32;
}