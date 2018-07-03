#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: P3IN,
    _reserved0: [u8; 1usize],
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: P3OUT,
    _reserved1: [u8; 1usize],
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: P3DIR,
    _reserved2: [u8; 1usize],
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: P3REN,
    _reserved3: [u8; 3usize],
    #[doc = "0x0a - Port 3 Selection0"]
    pub p3sel0: P3SEL0,
    _reserved4: [u8; 1usize],
    #[doc = "0x0c - Port 3 Selection1"]
    pub p3sel1: P3SEL1,
}
#[doc = "Port 3 Input"]
pub struct P3IN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "Port 3 Output"]
pub struct P3OUT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "Port 3 Direction"]
pub struct P3DIR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "Port 3 Resistor Enable"]
pub struct P3REN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "Port 3 Selection0"]
pub struct P3SEL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 3 Selection0"]
pub mod p3sel0;
#[doc = "Port 3 Selection1"]
pub struct P3SEL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 3 Selection1"]
pub mod p3sel1;
