#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control"]
    pub wdtctl: WDTCTL,
}
#[doc = "Watchdog Timer Control"]
pub struct WDTCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Watchdog Timer Control"]
pub mod wdtctl;
