#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control 0"]
    pub adcctl0: ADCCTL0,
    #[doc = "0x02 - ADC Control 1"]
    pub adcctl1: ADCCTL1,
    #[doc = "0x04 - ADC Control 2"]
    pub adcctl2: ADCCTL2,
    #[doc = "0x06 - ADC Window Comparator High Threshold"]
    pub adclo: ADCLO,
    #[doc = "0x08 - ADC Window Comparator High Threshold"]
    pub adchi: ADCHI,
    #[doc = "0x0a - ADC Memory Control 0"]
    pub adcmctl0: ADCMCTL0,
    _reserved0: [u8; 6usize],
    #[doc = "0x12 - ADC Conversion Memory 0"]
    pub adcmem0: ADCMEM0,
    _reserved1: [u8; 6usize],
    #[doc = "0x1a - ADC Interrupt Enable"]
    pub adcie: ADCIE,
    #[doc = "0x1c - ADC Interrupt Flag"]
    pub adcifg: ADCIFG,
    #[doc = "0x1e - ADC Interrupt Vector Word"]
    pub adciv: ADCIV,
}
#[doc = "ADC Control 0"]
pub struct ADCCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Control 0"]
pub mod adcctl0;
#[doc = "ADC Control 1"]
pub struct ADCCTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Control 1"]
pub mod adcctl1;
#[doc = "ADC Control 2"]
pub struct ADCCTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Control 2"]
pub mod adcctl2;
#[doc = "ADC Window Comparator High Threshold"]
pub struct ADCLO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Window Comparator High Threshold"]
pub mod adclo;
#[doc = "ADC Window Comparator High Threshold"]
pub struct ADCHI {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Window Comparator High Threshold"]
pub mod adchi;
#[doc = "ADC Memory Control 0"]
pub struct ADCMCTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Memory Control 0"]
pub mod adcmctl0;
#[doc = "ADC Conversion Memory 0"]
pub struct ADCMEM0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Conversion Memory 0"]
pub mod adcmem0;
#[doc = "ADC Interrupt Enable"]
pub struct ADCIE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Interrupt Enable"]
pub mod adcie;
#[doc = "ADC Interrupt Flag"]
pub struct ADCIFG {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Interrupt Flag"]
pub mod adcifg;
#[doc = "ADC Interrupt Vector Word"]
pub struct ADCIV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "ADC Interrupt Vector Word"]
pub mod adciv;
