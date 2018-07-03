#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1: UCB0CTL1,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0: UCB0CTL0,
    _reserved0: [u8; 4usize],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0: UCB0BR0,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1: UCB0BR1,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: UCB0RXBUF,
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: UCB0TXBUF,
    _reserved2: [u8; 26usize],
    #[doc = "0x2a - USCI B0 Interrupt Enable Register"]
    pub ucb0ie: UCB0IE,
    #[doc = "0x2c - USCI B0 Interrupt Flags Register"]
    pub ucb0ifg: UCB0IFG,
    #[doc = "0x2e - USCI B0 Interrupt Vector Register"]
    pub ucb0iv: UCB0IV,
}
#[doc = "USCI B0 Control Register 1"]
pub struct UCB0CTL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI B0 Control Register 1"]
pub mod ucb0ctl1;
#[doc = "USCI B0 Control Register 0"]
pub struct UCB0CTL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI B0 Control Register 0"]
pub mod ucb0ctl0;
#[doc = "USCI B0 Baud Rate 0"]
pub struct UCB0BR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI B0 Baud Rate 0"]
pub mod ucb0br0;
#[doc = "USCI B0 Baud Rate 1"]
pub struct UCB0BR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI B0 Baud Rate 1"]
pub mod ucb0br1;
#[doc = "USCI B0 Receive Buffer"]
pub struct UCB0RXBUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Receive Buffer"]
pub mod ucb0rxbuf;
#[doc = "USCI B0 Transmit Buffer"]
pub struct UCB0TXBUF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Transmit Buffer"]
pub mod ucb0txbuf;
#[doc = "USCI B0 Interrupt Enable Register"]
pub struct UCB0IE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Interrupt Enable Register"]
pub mod ucb0ie;
#[doc = "USCI B0 Interrupt Flags Register"]
pub struct UCB0IFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Interrupt Flags Register"]
pub mod ucb0ifg;
#[doc = "USCI B0 Interrupt Vector Register"]
pub struct UCB0IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Interrupt Vector Register"]
pub mod ucb0iv;
