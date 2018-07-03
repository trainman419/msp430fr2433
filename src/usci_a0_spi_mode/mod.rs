#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    pub uca0ctl1: UCA0CTL1,
    #[doc = "0x01 - USCI A0 Control Register 0"]
    pub uca0ctl0: UCA0CTL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    pub uca0br0: UCA0BR0,
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    pub uca0br1: UCA0BR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0statw: UCA0STATW,
    _reserved2: [u8; 1usize],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf: UCA0RXBUF,
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf: UCA0TXBUF,
    _reserved3: [u8; 10usize],
    #[doc = "0x1a - USCI A0 Interrupt Enable Register"]
    pub uca0ie: UCA0IE,
    _reserved4: [u8; 1usize],
    #[doc = "0x1c - USCI A0 Interrupt Flags Register"]
    pub uca0ifg: UCA0IFG,
    _reserved5: [u8; 1usize],
    #[doc = "0x1e - USCI A0 Interrupt Vector Register"]
    pub uca0iv: UCA0IV,
}
#[doc = "USCI A0 Control Register 1"]
pub struct UCA0CTL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Control Register 1"]
pub mod uca0ctl1;
#[doc = "USCI A0 Control Register 0"]
pub struct UCA0CTL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Control Register 0"]
pub mod uca0ctl0;
#[doc = "USCI A0 Baud Rate 0"]
pub struct UCA0BR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Baud Rate 0"]
pub mod uca0br0;
#[doc = "USCI A0 Baud Rate 1"]
pub struct UCA0BR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Baud Rate 1"]
pub mod uca0br1;
#[doc = "USCI A0 Status Register"]
pub struct UCA0STATW {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Status Register"]
pub mod uca0statw;
#[doc = "USCI A0 Interrupt Enable Register"]
pub struct UCA0IE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Interrupt Enable Register"]
pub mod uca0ie;
#[doc = "USCI A0 Interrupt Flags Register"]
pub struct UCA0IFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 Interrupt Flags Register"]
pub mod uca0ifg;
#[doc = "USCI A0 Receive Buffer"]
pub struct UCA0RXBUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A0 Receive Buffer"]
pub mod uca0rxbuf;
#[doc = "USCI A0 Transmit Buffer"]
pub struct UCA0TXBUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A0 Transmit Buffer"]
pub mod uca0txbuf;
#[doc = "USCI A0 Interrupt Vector Register"]
pub struct UCA0IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A0 Interrupt Vector Register"]
pub mod uca0iv;
