#![no_std]
#![feature(inherent_associated_types)]

pub mod spi;

#[repr(u8)]
pub enum Register {
    DevId = 0x00,
    BwRate = 0x2C,
    FifoCtl = 0x2D,
    DataX0 = 0x32,
    DataX1 = 0x33,
    DataY0 = 0x34,
    DataY1 = 0x35,
    DataZ0 = 0x36,
    DataZ1 = 0x37,
}

#[repr(u8)]
pub enum DataRate {
    Hz0_10 = 0x00,
    Hz0_20 = 0x01,
    Hz0_39 = 0x02,
    Hz0_78 = 0x03,
    Hz1_56 = 0x04,
    Hz3_13 = 0x05,
    Hz6_25 = 0x06,
    Hz12_5 = 0x07,
    Hz25 = 0x08,
    Hz50 = 0x09,
    Hz100 = 0x0A,
    Hz200 = 0x0B,
    Hz400 = 0x0C,   
    Hz800 = 0x0D,
    Hz1600 = 0x0E,
    Hz3200 = 0x0F,   
}

#[repr(u8)]
pub enum FifoMode {
    Bypass = 0x00,
    FIFO = 0x01,
    Stream = 0x02,
    Trigger = 0x03,
}

pub const FIFO_SIZE_BYTES : usize = 32 * 6; // 32 samples of 6 bytes each