#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Data In Register"]
    pub crcdi: CRCDI,
    #[doc = "0x02 - CRC data in reverse byte Register"]
    pub crcdirb: CRCDIRB,
    #[doc = "0x04 - CRC Initialisation Register and Result Register"]
    pub crcinires: CRCINIRES,
    #[doc = "0x06 - CRC reverse result Register"]
    pub crcresr: CRCRESR,
}
#[doc = "CRC Data In Register"]
pub struct CRCDI {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC Data In Register"]
pub mod crcdi;
#[doc = "CRC data in reverse byte Register"]
pub struct CRCDIRB {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC data in reverse byte Register"]
pub mod crcdirb;
#[doc = "CRC Initialisation Register and Result Register"]
pub struct CRCINIRES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC Initialisation Register and Result Register"]
pub mod crcinires;
#[doc = "CRC reverse result Register"]
pub struct CRCRESR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "CRC reverse result Register"]
pub mod crcresr;
