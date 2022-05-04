use std::fmt::Display;

pub mod ethernet;
pub mod lldp;

pub trait Packet : Display {
    fn get_size(&self) -> usize;
}