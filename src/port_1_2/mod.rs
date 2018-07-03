#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    pub p1in: P1IN,
    #[doc = "0x01 - Port 2 Input"]
    pub p2in: P2IN,
    #[doc = "0x02 - Port 1 Output"]
    pub p1out: P1OUT,
    #[doc = "0x03 - Port 2 Output"]
    pub p2out: P2OUT,
    #[doc = "0x04 - Port 1 Direction"]
    pub p1dir: P1DIR,
    #[doc = "0x05 - Port 2 Direction"]
    pub p2dir: P2DIR,
    #[doc = "0x06 - Port 1 Resistor Enable"]
    pub p1ren: P1REN,
    #[doc = "0x07 - Port 2 Resistor Enable"]
    pub p2ren: P2REN,
    _reserved0: [u8; 2usize],
    #[doc = "0x0a - Port 1 Selection 0"]
    pub p1sel0: P1SEL0,
    #[doc = "0x0b - Port 2 Selection 0"]
    pub p2sel0: P2SEL0,
    #[doc = "0x0c - Port 1 Selection 1"]
    pub p1sel1: P1SEL1,
    #[doc = "0x0d - Port 2 Selection 1"]
    pub p2sel1: P2SEL1,
    #[doc = "0x0e - Port 1 Interrupt Vector Word"]
    pub p1iv: P1IV,
    _reserved1: [u8; 8usize],
    #[doc = "0x18 - Port 1 Interrupt Edge Select"]
    pub p1ies: P1IES,
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    pub p2ies: P2IES,
    #[doc = "0x1a - Port 1 Interrupt Enable"]
    pub p1ie: P1IE,
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    pub p2ie: P2IE,
    #[doc = "0x1c - Port 1 Interrupt Flag"]
    pub p1ifg: P1IFG,
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    pub p2ifg: P2IFG,
    #[doc = "0x1e - Port 2 Interrupt Vector Word"]
    pub p2iv: P2IV,
}
#[doc = "Port 1 Input"]
pub struct P1IN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "Port 2 Input"]
pub struct P2IN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "Port 1 Output"]
pub struct P1OUT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "Port 2 Output"]
pub struct P2OUT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "Port 1 Direction"]
pub struct P1DIR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "Port 2 Direction"]
pub struct P2DIR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "Port 1 Resistor Enable"]
pub struct P1REN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "Port 2 Resistor Enable"]
pub struct P2REN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "Port 1 Selection 0"]
pub struct P1SEL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Selection 0"]
pub mod p1sel0;
#[doc = "Port 2 Selection 0"]
pub struct P2SEL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Selection 0"]
pub mod p2sel0;
#[doc = "Port 1 Selection 1"]
pub struct P1SEL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Selection 1"]
pub mod p1sel1;
#[doc = "Port 2 Selection 1"]
pub struct P2SEL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Selection 1"]
pub mod p2sel1;
#[doc = "Port 1 Interrupt Edge Select"]
pub struct P1IES {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "Port 2 Interrupt Edge Select"]
pub struct P2IES {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "Port 1 Interrupt Enable"]
pub struct P1IE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "Port 2 Interrupt Enable"]
pub struct P2IE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "Port 1 Interrupt Flag"]
pub struct P1IFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "Port 2 Interrupt Flag"]
pub struct P2IFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "Port 1 Interrupt Vector Word"]
pub struct P1IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port 1 Interrupt Vector Word"]
pub mod p1iv;
#[doc = "Port 2 Interrupt Vector Word"]
pub struct P2IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Port 2 Interrupt Vector Word"]
pub mod p2iv;
