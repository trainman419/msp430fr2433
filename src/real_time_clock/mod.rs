#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control Register"]
    pub rtcctl: RTCCTL,
    _reserved0: [u8; 2usize],
    #[doc = "0x04 - RTC interrupt vector"]
    pub rtciv: RTCIV,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - RTC moduloRegister"]
    pub rtcmod: RTCMOD,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - RTC counter Register"]
    pub rtccnt: RTCCNT,
}
#[doc = "RTC control Register"]
pub struct RTCCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RTC control Register"]
pub mod rtcctl;
#[doc = "RTC interrupt vector"]
pub struct RTCIV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RTC interrupt vector"]
pub mod rtciv;
#[doc = "RTC moduloRegister"]
pub struct RTCMOD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RTC moduloRegister"]
pub mod rtcmod;
#[doc = "RTC counter Register"]
pub struct RTCCNT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "RTC counter Register"]
pub mod rtccnt;
