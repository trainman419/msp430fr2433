#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer3_A2 Control"]
    pub ta3ctl: TA3CTL,
    #[doc = "0x02 - Timer3_A2 Capture/Compare Control 0"]
    pub ta3cctl0: TA3CCTL0,
    #[doc = "0x04 - Timer3_A2 Capture/Compare Control 1"]
    pub ta3cctl1: TA3CCTL1,
    _reserved0: [u8; 10usize],
    #[doc = "0x10 - Timer3_A2"]
    pub ta3r: TA3R,
    #[doc = "0x12 - Timer3_A2 Capture/Compare 0"]
    pub ta3ccr0: TA3CCR0,
    #[doc = "0x14 - Timer3_A2 Capture/Compare 1"]
    pub ta3ccr1: TA3CCR1,
    _reserved1: [u8; 10usize],
    #[doc = "0x20 - Timer3_A2 Expansion Register 0"]
    pub ta3ex0: TA3EX0,
    _reserved2: [u8; 12usize],
    #[doc = "0x2e - Timer3_A2 Interrupt Vector Word"]
    pub ta3iv: TA3IV,
}
#[doc = "Timer3_A2 Control"]
pub struct TA3CTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Control"]
pub mod ta3ctl;
#[doc = "Timer3_A2 Capture/Compare Control 0"]
pub struct TA3CCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Capture/Compare Control 0"]
pub mod ta3cctl0;
#[doc = "Timer3_A2 Capture/Compare Control 1"]
pub struct TA3CCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Capture/Compare Control 1"]
pub mod ta3cctl1;
#[doc = "Timer3_A2"]
pub struct TA3R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2"]
pub mod ta3r;
#[doc = "Timer3_A2 Capture/Compare 0"]
pub struct TA3CCR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Capture/Compare 0"]
pub mod ta3ccr0;
#[doc = "Timer3_A2 Capture/Compare 1"]
pub struct TA3CCR1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Capture/Compare 1"]
pub mod ta3ccr1;
#[doc = "Timer3_A2 Expansion Register 0"]
pub struct TA3EX0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Expansion Register 0"]
pub mod ta3ex0;
#[doc = "Timer3_A2 Interrupt Vector Word"]
pub struct TA3IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer3_A2 Interrupt Vector Word"]
pub mod ta3iv;
