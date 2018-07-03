#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer1_A3 Control"]
    pub ta1ctl: TA1CTL,
    #[doc = "0x02 - Timer1_A3 Capture/Compare Control 0"]
    pub ta1cctl0: TA1CCTL0,
    #[doc = "0x04 - Timer1_A3 Capture/Compare Control 1"]
    pub ta1cctl1: TA1CCTL1,
    #[doc = "0x06 - Timer1_A3 Capture/Compare Control 2"]
    pub ta1cctl2: TA1CCTL2,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Timer1_A3"]
    pub ta1r: TA1R,
    #[doc = "0x12 - Timer1_A3 Capture/Compare 0"]
    pub ta1ccr0: TA1CCR0,
    #[doc = "0x14 - Timer1_A3 Capture/Compare 1"]
    pub ta1ccr1: TA1CCR1,
    #[doc = "0x16 - Timer1_A3 Capture/Compare 2"]
    pub ta1ccr2: TA1CCR2,
    _reserved1: [u8; 8usize],
    #[doc = "0x20 - Timer1_A3 Expansion Register 0"]
    pub ta1ex0: TA1EX0,
    _reserved2: [u8; 12usize],
    #[doc = "0x2e - Timer1_A3 Interrupt Vector Word"]
    pub ta1iv: TA1IV,
}
#[doc = "Timer1_A3 Control"]
pub struct TA1CTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Control"]
pub mod ta1ctl;
#[doc = "Timer1_A3 Capture/Compare Control 0"]
pub struct TA1CCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Capture/Compare Control 0"]
pub mod ta1cctl0;
#[doc = "Timer1_A3 Capture/Compare Control 1"]
pub struct TA1CCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Capture/Compare Control 1"]
pub mod ta1cctl1;
#[doc = "Timer1_A3 Capture/Compare Control 2"]
pub struct TA1CCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Capture/Compare Control 2"]
pub mod ta1cctl2;
#[doc = "Timer1_A3"]
pub struct TA1R {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3"]
pub mod ta1r;
#[doc = "Timer1_A3 Capture/Compare 0"]
pub struct TA1CCR0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Capture/Compare 0"]
pub mod ta1ccr0;
#[doc = "Timer1_A3 Capture/Compare 1"]
pub struct TA1CCR1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Capture/Compare 1"]
pub mod ta1ccr1;
#[doc = "Timer1_A3 Capture/Compare 2"]
pub struct TA1CCR2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Capture/Compare 2"]
pub mod ta1ccr2;
#[doc = "Timer1_A3 Expansion Register 0"]
pub struct TA1EX0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Expansion Register 0"]
pub mod ta1ex0;
#[doc = "Timer1_A3 Interrupt Vector Word"]
pub struct TA1IV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Timer1_A3 Interrupt Vector Word"]
pub mod ta1iv;
