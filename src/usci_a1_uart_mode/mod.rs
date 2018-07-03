#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl1: UCA1CTL1,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl0: UCA1CTL0,
    #[doc = "0x02 - USCI A1 Control Word Register 1"]
    pub uca1ctlw1: UCA1CTLW1,
    _reserved0: [u8; 2usize],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0: UCA1BR0,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1: UCA1BR1,
    #[doc = "0x08 - USCI A1 Modulation Control"]
    pub uca1mctlw: UCA1MCTLW,
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1statw: UCA1STATW,
    _reserved1: [u8; 1usize],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf: UCA1RXBUF,
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf: UCA1TXBUF,
    #[doc = "0x10 - USCI A1 LIN Control"]
    pub uca1abctl: UCA1ABCTL,
    _reserved2: [u8; 1usize],
    #[doc = "0x12 - USCI A1 IrDA Transmit Control"]
    pub uca1irtctl: UCA1IRTCTL,
    #[doc = "0x13 - USCI A1 IrDA Receive Control"]
    pub uca1irrctl: UCA1IRRCTL,
    _reserved3: [u8; 10usize],
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
#[doc = "USCI A1 LIN Control"]
pub struct UCA1ABCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 LIN Control"]
pub mod uca1abctl;
#[doc = "USCI A1 IrDA Transmit Control"]
pub struct UCA1IRTCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 IrDA Transmit Control"]
pub mod uca1irtctl;
#[doc = "USCI A1 IrDA Receive Control"]
pub struct UCA1IRRCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI A1 IrDA Receive Control"]
pub mod uca1irrctl;
#[doc = "USCI A1 Control Word Register 1"]
pub struct UCA1CTLW1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A1 Control Word Register 1"]
pub mod uca1ctlw1;
#[doc = "USCI A1 Modulation Control"]
pub struct UCA1MCTLW {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI A1 Modulation Control"]
pub mod uca1mctlw;
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
