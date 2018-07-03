#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI B0 Control Register 1"]
    pub ucb0ctl1: UCB0CTL1,
    #[doc = "0x01 - USCI B0 Control Register 0"]
    pub ucb0ctl0: UCB0CTL0,
    #[doc = "0x02 - USCI B0 Control Word Register 1"]
    pub ucb0ctlw1: UCB0CTLW1,
    _reserved0: [u8; 2usize],
    #[doc = "0x06 - USCI B0 Baud Rate 0"]
    pub ucb0br0: UCB0BR0,
    #[doc = "0x07 - USCI B0 Baud Rate 1"]
    pub ucb0br1: UCB0BR1,
    #[doc = "0x08 - USCI B0 Status Register"]
    pub ucb0stat: UCB0STAT,
    #[doc = "0x09 - USCI B0 Byte Counter Register"]
    pub ucb0bcnt: UCB0BCNT,
    #[doc = "0x0a - USCI B0 Byte Counter Threshold Register"]
    pub ucb0tbcnt: UCB0TBCNT,
    #[doc = "0x0c - USCI B0 Receive Buffer"]
    pub ucb0rxbuf: UCB0RXBUF,
    #[doc = "0x0e - USCI B0 Transmit Buffer"]
    pub ucb0txbuf: UCB0TXBUF,
    _reserved1: [u8; 4usize],
    #[doc = "0x14 - USCI B0 I2C Own Address 0"]
    pub ucb0i2coa0: UCB0I2COA0,
    #[doc = "0x16 - USCI B0 I2C Own Address 1"]
    pub ucb0i2coa1: UCB0I2COA1,
    #[doc = "0x18 - USCI B0 I2C Own Address 2"]
    pub ucb0i2coa2: UCB0I2COA2,
    #[doc = "0x1a - USCI B0 I2C Own Address 3"]
    pub ucb0i2coa3: UCB0I2COA3,
    #[doc = "0x1c - USCI B0 Received Address Register"]
    pub ucb0addrx: UCB0ADDRX,
    #[doc = "0x1e - USCI B0 Address Mask Register"]
    pub ucb0addmask: UCB0ADDMASK,
    #[doc = "0x20 - USCI B0 I2C Slave Address"]
    pub ucb0i2csa: UCB0I2CSA,
    _reserved2: [u8; 8usize],
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
#[doc = "USCI B0 Status Register"]
pub struct UCB0STAT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI B0 Status Register"]
pub mod ucb0stat;
#[doc = "USCI B0 Byte Counter Register"]
pub struct UCB0BCNT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USCI B0 Byte Counter Register"]
pub mod ucb0bcnt;
#[doc = "USCI B0 Control Word Register 1"]
pub struct UCB0CTLW1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Control Word Register 1"]
pub mod ucb0ctlw1;
#[doc = "USCI B0 Byte Counter Threshold Register"]
pub struct UCB0TBCNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Byte Counter Threshold Register"]
pub mod ucb0tbcnt;
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
#[doc = "USCI B0 I2C Own Address 0"]
pub struct UCB0I2COA0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 I2C Own Address 0"]
pub mod ucb0i2coa0;
#[doc = "USCI B0 I2C Own Address 1"]
pub struct UCB0I2COA1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 I2C Own Address 1"]
pub mod ucb0i2coa1;
#[doc = "USCI B0 I2C Own Address 2"]
pub struct UCB0I2COA2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 I2C Own Address 2"]
pub mod ucb0i2coa2;
#[doc = "USCI B0 I2C Own Address 3"]
pub struct UCB0I2COA3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 I2C Own Address 3"]
pub mod ucb0i2coa3;
#[doc = "USCI B0 Received Address Register"]
pub struct UCB0ADDRX {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Received Address Register"]
pub mod ucb0addrx;
#[doc = "USCI B0 Address Mask Register"]
pub struct UCB0ADDMASK {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 Address Mask Register"]
pub mod ucb0addmask;
#[doc = "USCI B0 I2C Slave Address"]
pub struct UCB0I2CSA {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USCI B0 I2C Slave Address"]
pub mod ucb0i2csa;
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
