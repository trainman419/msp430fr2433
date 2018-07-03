#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CS Control Register 0"]
    pub csctl0: CSCTL0,
    #[doc = "0x02 - CS Control Register 1"]
    pub csctl1: CSCTL1,
    #[doc = "0x04 - CS Control Register 2"]
    pub csctl2: CSCTL2,
    #[doc = "0x06 - CS Control Register 3"]
    pub csctl3: CSCTL3,
    #[doc = "0x08 - CS Control Register 4"]
    pub csctl4: CSCTL4,
    #[doc = "0x0a - CS Control Register 5"]
    pub csctl5: CSCTL5,
    #[doc = "0x0c - CS Control Register 6"]
    pub csctl6: CSCTL6,
    #[doc = "0x0e - CS Control Register 7"]
    pub csctl7: CSCTL7,
    #[doc = "0x10 - CS Control Register 8"]
    pub csctl8: CSCTL8,
}
#[doc = "CS Control Register 0"]
pub struct CSCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 0"]
pub mod csctl0;
#[doc = "CS Control Register 1"]
pub struct CSCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 1"]
pub mod csctl1;
#[doc = "CS Control Register 2"]
pub struct CSCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 2"]
pub mod csctl2;
#[doc = "CS Control Register 3"]
pub struct CSCTL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 3"]
pub mod csctl3;
#[doc = "CS Control Register 4"]
pub struct CSCTL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 4"]
pub mod csctl4;
#[doc = "CS Control Register 5"]
pub struct CSCTL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 5"]
pub mod csctl5;
#[doc = "CS Control Register 6"]
pub struct CSCTL6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 6"]
pub mod csctl6;
#[doc = "CS Control Register 7"]
pub struct CSCTL7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 7"]
pub mod csctl7;
#[doc = "CS Control Register 8"]
pub struct CSCTL8 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CS Control Register 8"]
pub mod csctl8;
