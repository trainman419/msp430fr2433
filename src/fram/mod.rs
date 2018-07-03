#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FRAM Controller Control 0"]
    pub frctl0: FRCTL0,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - General Control 0"]
    pub gcctl0: GCCTL0,
    #[doc = "0x06 - General Control 1"]
    pub gcctl1: GCCTL1,
}
#[doc = "FRAM Controller Control 0"]
pub struct FRCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "FRAM Controller Control 0"]
pub mod frctl0;
#[doc = "General Control 0"]
pub struct GCCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "General Control 0"]
pub mod gcctl0;
#[doc = "General Control 1"]
pub struct GCCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "General Control 1"]
pub mod gcctl1;
