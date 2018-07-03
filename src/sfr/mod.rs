#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub sfrie1: SFRIE1,
    #[doc = "0x02 - Interrupt Flag 1"]
    pub sfrifg1: SFRIFG1,
    #[doc = "0x04 - RESET Pin Control Register"]
    pub sfrrpcr: SFRRPCR,
}
#[doc = "Interrupt Enable 1"]
pub struct SFRIE1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Enable 1"]
pub mod sfrie1;
#[doc = "Interrupt Flag 1"]
pub struct SFRIFG1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Interrupt Flag 1"]
pub mod sfrifg1;
#[doc = "RESET Pin Control Register"]
pub struct SFRRPCR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RESET Pin Control Register"]
pub mod sfrrpcr;
