#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 32-bit operand 1 - multiply - low word"]
    pub mpy32l: MPY32L,
    #[doc = "0x02 - 32-bit operand 1 - multiply - high word"]
    pub mpy32h: MPY32H,
    #[doc = "0x04 - 32-bit operand 1 - signed multiply - low word"]
    pub mpys32l: MPYS32L,
    #[doc = "0x06 - 32-bit operand 1 - signed multiply - high word"]
    pub mpys32h: MPYS32H,
    #[doc = "0x08 - 32-bit operand 1 - multiply accumulate - low word"]
    pub mac32l: MAC32L,
    #[doc = "0x0a - 32-bit operand 1 - multiply accumulate - high word"]
    pub mac32h: MAC32H,
    #[doc = "0x0c - 32-bit operand 1 - signed multiply accumulate - low word"]
    pub macs32l: MACS32L,
    #[doc = "0x0e - 32-bit operand 1 - signed multiply accumulate - high word"]
    pub macs32h: MACS32H,
    #[doc = "0x10 - 32-bit operand 2 - low word"]
    pub op2l: OP2L,
    #[doc = "0x12 - 32-bit operand 2 - high word"]
    pub op2h: OP2H,
    #[doc = "0x14 - 32x32-bit result 0 - least significant word"]
    pub res0: RES0,
    #[doc = "0x16 - 32x32-bit result 1"]
    pub res1: RES1,
    #[doc = "0x18 - 32x32-bit result 2"]
    pub res2: RES2,
    #[doc = "0x1a - 32x32-bit result 3 - most significant word"]
    pub res3: RES3,
}
#[doc = "32-bit operand 1 - multiply - low word"]
pub struct MPY32L {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - multiply - low word"]
pub mod mpy32l;
#[doc = "32-bit operand 1 - multiply - high word"]
pub struct MPY32H {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - multiply - high word"]
pub mod mpy32h;
#[doc = "32-bit operand 1 - signed multiply - low word"]
pub struct MPYS32L {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - signed multiply - low word"]
pub mod mpys32l;
#[doc = "32-bit operand 1 - signed multiply - high word"]
pub struct MPYS32H {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - signed multiply - high word"]
pub mod mpys32h;
#[doc = "32-bit operand 1 - multiply accumulate - low word"]
pub struct MAC32L {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - multiply accumulate - low word"]
pub mod mac32l;
#[doc = "32-bit operand 1 - multiply accumulate - high word"]
pub struct MAC32H {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - multiply accumulate - high word"]
pub mod mac32h;
#[doc = "32-bit operand 1 - signed multiply accumulate - low word"]
pub struct MACS32L {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - signed multiply accumulate - low word"]
pub mod macs32l;
#[doc = "32-bit operand 1 - signed multiply accumulate - high word"]
pub struct MACS32H {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 1 - signed multiply accumulate - high word"]
pub mod macs32h;
#[doc = "32-bit operand 2 - low word"]
pub struct OP2L {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 2 - low word"]
pub mod op2l;
#[doc = "32-bit operand 2 - high word"]
pub struct OP2H {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32-bit operand 2 - high word"]
pub mod op2h;
#[doc = "32x32-bit result 0 - least significant word"]
pub struct RES0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32x32-bit result 0 - least significant word"]
pub mod res0;
#[doc = "32x32-bit result 1"]
pub struct RES1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32x32-bit result 1"]
pub mod res1;
#[doc = "32x32-bit result 2"]
pub struct RES2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32x32-bit result 2"]
pub mod res2;
#[doc = "32x32-bit result 3 - most significant word"]
pub struct RES3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "32x32-bit result 3 - most significant word"]
pub mod res3;
