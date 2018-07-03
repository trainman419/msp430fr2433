#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Multiply Unsigned/Operand 1"]
    pub mpy: MPY,
    #[doc = "0x02 - Multiply Signed/Operand 1"]
    pub mpys: MPYS,
    #[doc = "0x04 - Multiply Unsigned and Accumulate/Operand 1"]
    pub mac: MAC,
    #[doc = "0x06 - Multiply Signed and Accumulate/Operand 1"]
    pub macs: MACS,
    #[doc = "0x08 - Operand 2"]
    pub op2: OP2,
    #[doc = "0x0a - Result Low Word"]
    pub reslo: RESLO,
    #[doc = "0x0c - Result High Word"]
    pub reshi: RESHI,
    #[doc = "0x0e - Sum Extend"]
    pub sumext: SUMEXT,
    _reserved0: [u8; 28usize],
    #[doc = "0x2c - MPY32 Control Register 0"]
    pub mpy32ctl0: MPY32CTL0,
}
#[doc = "Multiply Unsigned/Operand 1"]
pub struct MPY {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Multiply Unsigned/Operand 1"]
pub mod mpy;
#[doc = "Multiply Signed/Operand 1"]
pub struct MPYS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Multiply Signed/Operand 1"]
pub mod mpys;
#[doc = "Multiply Unsigned and Accumulate/Operand 1"]
pub struct MAC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Multiply Unsigned and Accumulate/Operand 1"]
pub mod mac;
#[doc = "Multiply Signed and Accumulate/Operand 1"]
pub struct MACS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Multiply Signed and Accumulate/Operand 1"]
pub mod macs;
#[doc = "Operand 2"]
pub struct OP2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Operand 2"]
pub mod op2;
#[doc = "Result Low Word"]
pub struct RESLO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Result Low Word"]
pub mod reslo;
#[doc = "Result High Word"]
pub struct RESHI {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Result High Word"]
pub mod reshi;
#[doc = "Sum Extend"]
pub struct SUMEXT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Sum Extend"]
pub mod sumext;
#[doc = "MPY32 Control Register 0"]
pub struct MPY32CTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "MPY32 Control Register 0"]
pub mod mpy32ctl0;
