#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer2_A2 Control"]
    pub ta2ctl: TA2CTL,
    #[doc = "0x02 - Timer2_A2 Capture/Compare Control 0"]
    pub ta2cctl0: TA2CCTL0,
    #[doc = "0x04 - Timer2_A2 Capture/Compare Control 1"]
    pub ta2cctl1: TA2CCTL1,
    _reserved0: [u8; 10usize],
    #[doc = "0x10 - Timer2_A2"]
    pub ta2r: TA2R,
    #[doc = "0x12 - Timer2_A2 Capture/Compare 0"]
    pub ta2ccr0: TA2CCR0,
    #[doc = "0x14 - Timer2_A2 Capture/Compare 1"]
    pub ta2ccr1: TA2CCR1,
    _reserved1: [u8; 10usize],
    #[doc = "0x20 - Timer2_A2 Expansion Register 0"]
    pub ta2ex0: TA2EX0,
    _reserved2: [u8; 12usize],
    #[doc = "0x2e - Timer2_A2 Interrupt Vector Word"]
    pub ta2iv: TA2IV,
}
#[doc = "Timer2_A2 Control"]
pub struct TA2CTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Control"]
pub mod ta2ctl;
#[doc = "Timer2_A2 Capture/Compare Control 0"]
pub struct TA2CCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Capture/Compare Control 0"]
pub mod ta2cctl0;
#[doc = "Timer2_A2 Capture/Compare Control 1"]
pub struct TA2CCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Capture/Compare Control 1"]
pub mod ta2cctl1;
#[doc = "Timer2_A2"]
pub struct TA2R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2"]
pub mod ta2r;
#[doc = "Timer2_A2 Capture/Compare 0"]
pub struct TA2CCR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Capture/Compare 0"]
pub mod ta2ccr0;
#[doc = "Timer2_A2 Capture/Compare 1"]
pub struct TA2CCR1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Capture/Compare 1"]
pub mod ta2ccr1;
#[doc = "Timer2_A2 Expansion Register 0"]
pub struct TA2EX0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Expansion Register 0"]
pub mod ta2ex0;
#[doc = "Timer2_A2 Interrupt Vector Word"]
pub struct TA2IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer2_A2 Interrupt Vector Word"]
pub mod ta2iv;
