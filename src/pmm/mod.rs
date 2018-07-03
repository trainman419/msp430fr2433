#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    pub pmmctl0: PMMCTL0,
    #[doc = "0x02 - PMM Control 1"]
    pub pmmctl1: PMMCTL1,
    #[doc = "0x04 - PMM Control 2"]
    pub pmmctl2: PMMCTL2,
    _reserved0: [u8; 4usize],
    #[doc = "0x0a - PMM Interrupt Flag"]
    pub pmmifg: PMMIFG,
    _reserved1: [u8; 2usize],
    #[doc = "0x0e - PMM Interrupt Enable"]
    pub pmmie: PMMIE,
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    pub pm5ctl0: PM5CTL0,
}
#[doc = "PMM Control 0"]
pub struct PMMCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMM Control 1"]
pub struct PMMCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PMM Control 1"]
pub mod pmmctl1;
#[doc = "PMM Control 2"]
pub struct PMMCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PMM Control 2"]
pub mod pmmctl2;
#[doc = "PMM Interrupt Flag"]
pub struct PMMIFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PMM Interrupt Enable"]
pub struct PMMIE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PMM Interrupt Enable"]
pub mod pmmie;
#[doc = "PMM Power Mode 5 Control Register 0"]
pub struct PM5CTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
