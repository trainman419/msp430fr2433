#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A0 Control Register 1"]
    pub uca0ctl1: UCA0CTL1,
    #[doc = "0x01 - USCI A0 Control Register 0"]
    pub uca0ctl0: UCA0CTL0,
    #[doc = "0x02 - USCI A0 Control Word Register 1"]
    pub uca0ctlw1: UCA0CTLW1,
    _reserved0: [u8; 2usize],
    #[doc = "0x06 - USCI A0 Baud Rate 0"]
    pub uca0br0: UCA0BR0,
    #[doc = "0x07 - USCI A0 Baud Rate 1"]
    pub uca0br1: UCA0BR1,
    #[doc = "0x08 - USCI A0 Modulation Control"]
    pub uca0mctlw: UCA0MCTLW,
    #[doc = "0x0a - USCI A0 Status Register"]
    pub uca0statw: UCA0STATW,
    _reserved1: [u8; 1usize],
    #[doc = "0x0c - USCI A0 Receive Buffer"]
    pub uca0rxbuf: UCA0RXBUF,
    #[doc = "0x0e - USCI A0 Transmit Buffer"]
    pub uca0txbuf: UCA0TXBUF,
    #[doc = "0x10 - USCI A0 LIN Control"]
    pub uca0abctl: UCA0ABCTL,
    _reserved2: [u8; 1usize],
    #[doc = "0x12 - USCI A0 IrDA Transmit Control"]
    pub uca0irtctl: UCA0IRTCTL,
    #[doc = "0x13 - USCI A0 IrDA Receive Control"]
    pub uca0irrctl: UCA0IRRCTL,
    _reserved3: [u8; 10usize],
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
#[doc = "USCI A0 LIN Control"]
pub struct UCA0ABCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 LIN Control"]
pub mod uca0abctl;
#[doc = "USCI A0 IrDA Transmit Control"]
pub struct UCA0IRTCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 IrDA Transmit Control"]
pub mod uca0irtctl;
#[doc = "USCI A0 IrDA Receive Control"]
pub struct UCA0IRRCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A0 IrDA Receive Control"]
pub mod uca0irrctl;
#[doc = "USCI A0 Control Word Register 1"]
pub struct UCA0CTLW1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A0 Control Word Register 1"]
pub mod uca0ctlw1;
#[doc = "USCI A0 Modulation Control"]
pub struct UCA0MCTLW {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A0 Modulation Control"]
pub mod uca0mctlw;
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
