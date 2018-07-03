#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer0_A3 Control"]
    pub ta0ctl: TA0CTL,
    #[doc = "0x02 - Timer0_A3 Capture/Compare Control 0"]
    pub ta0cctl0: TA0CCTL0,
    #[doc = "0x04 - Timer0_A3 Capture/Compare Control 1"]
    pub ta0cctl1: TA0CCTL1,
    #[doc = "0x06 - Timer0_A3 Capture/Compare Control 2"]
    pub ta0cctl2: TA0CCTL2,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Timer0_A3"]
    pub ta0r: TA0R,
    #[doc = "0x12 - Timer0_A3 Capture/Compare 0"]
    pub ta0ccr0: TA0CCR0,
    #[doc = "0x14 - Timer0_A3 Capture/Compare 1"]
    pub ta0ccr1: TA0CCR1,
    #[doc = "0x16 - Timer0_A3 Capture/Compare 2"]
    pub ta0ccr2: TA0CCR2,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Timer0_A3 Expansion Register 0"]
    pub ta0ex0: TA0EX0,
    _reserved2: [u8; 12usize],
    #[doc = "0x2e - Timer0_A3 Interrupt Vector Word"]
    pub ta0iv: TA0IV,
}
#[doc = "Timer0_A3 Control"]
pub struct TA0CTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Control"]
pub mod ta0ctl;
#[doc = "Timer0_A3 Capture/Compare Control 0"]
pub struct TA0CCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Capture/Compare Control 0"]
pub mod ta0cctl0;
#[doc = "Timer0_A3 Capture/Compare Control 1"]
pub struct TA0CCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Capture/Compare Control 1"]
pub mod ta0cctl1;
#[doc = "Timer0_A3 Capture/Compare Control 2"]
pub struct TA0CCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Capture/Compare Control 2"]
pub mod ta0cctl2;
#[doc = "Timer0_A3"]
pub struct TA0R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3"]
pub mod ta0r;
#[doc = "Timer0_A3 Capture/Compare 0"]
pub struct TA0CCR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Capture/Compare 0"]
pub mod ta0ccr0;
#[doc = "Timer0_A3 Capture/Compare 1"]
pub struct TA0CCR1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Capture/Compare 1"]
pub mod ta0ccr1;
#[doc = "Timer0_A3 Capture/Compare 2"]
pub struct TA0CCR2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Capture/Compare 2"]
pub mod ta0ccr2;
#[doc = "Timer0_A3 Expansion Register 0"]
pub struct TA0EX0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Expansion Register 0"]
pub mod ta0ex0;
#[doc = "Timer0_A3 Interrupt Vector Word"]
pub struct TA0IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer0_A3 Interrupt Vector Word"]
pub mod ta0iv;
