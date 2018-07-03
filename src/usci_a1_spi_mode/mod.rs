#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl1: UCA1CTL1,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl0: UCA1CTL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0: UCA1BR0,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1: UCA1BR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1statw: UCA1STATW,
    _reserved2: [u8; 1usize],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf: UCA1RXBUF,
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf: UCA1TXBUF,
    _reserved3: [u8; 10usize],
    #[doc = "0x1a - USCI A1 Interrupt Enable Register"]
    pub uca1ie: UCA1IE,
    _reserved4: [u8; 1usize],
    #[doc = "0x1c - USCI A1 Interrupt Flags Register"]
    pub uca1ifg: UCA1IFG,
    _reserved5: [u8; 1usize],
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    pub uca1iv: UCA1IV,
}
#[doc = "USCI A1 Control Register 1"]
pub struct UCA1CTL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1;
#[doc = "USCI A1 Control Register 0"]
pub struct UCA1CTL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0;
#[doc = "USCI A1 Baud Rate 0"]
pub struct UCA1BR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0;
#[doc = "USCI A1 Baud Rate 1"]
pub struct UCA1BR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1;
#[doc = "USCI A1 Status Register"]
pub struct UCA1STATW {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Status Register"]
pub mod uca1statw;
#[doc = "USCI A1 Interrupt Enable Register"]
pub struct UCA1IE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie;
#[doc = "USCI A1 Interrupt Flags Register"]
pub struct UCA1IFG {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg;
#[doc = "USCI A1 Receive Buffer"]
pub struct UCA1RXBUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf;
#[doc = "USCI A1 Transmit Buffer"]
pub struct UCA1TXBUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf;
#[doc = "USCI A1 Interrupt Vector Register"]
pub struct UCA1IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv;
