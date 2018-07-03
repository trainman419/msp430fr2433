#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System control"]
    pub sysctl: SYSCTL,
    #[doc = "0x02 - Boot strap configuration area"]
    pub sysbslc: SYSBSLC,
    _reserved0: [u8; 2usize],
    #[doc = "0x06 - JTAG mailbox control"]
    pub sysjmbc: SYSJMBC,
    #[doc = "0x08 - JTAG mailbox input 0"]
    pub sysjmbi0: SYSJMBI0,
    #[doc = "0x0a - JTAG mailbox input 1"]
    pub sysjmbi1: SYSJMBI1,
    #[doc = "0x0c - JTAG mailbox output 0"]
    pub sysjmbo0: SYSJMBO0,
    #[doc = "0x0e - JTAG mailbox output 1"]
    pub sysjmbo1: SYSJMBO1,
    _reserved1: [u8; 8usize],
    #[doc = "0x18 - Bus Error vector generator"]
    pub sysberriv: SYSBERRIV,
    #[doc = "0x1a - User NMI vector generator"]
    pub sysuniv: SYSUNIV,
    #[doc = "0x1c - System NMI vector generator"]
    pub syssniv: SYSSNIV,
    #[doc = "0x1e - Reset vector generator"]
    pub sysrstiv: SYSRSTIV,
    #[doc = "0x20 - System Configuration 0"]
    pub syscfg0: SYSCFG0,
    #[doc = "0x22 - System Configuration 1"]
    pub syscfg1: SYSCFG1,
    #[doc = "0x24 - System Configuration 2"]
    pub syscfg2: SYSCFG2,
}
#[doc = "System control"]
pub struct SYSCTL {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "System control"]
pub mod sysctl;
#[doc = "Boot strap configuration area"]
pub struct SYSBSLC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Boot strap configuration area"]
pub mod sysbslc;
#[doc = "JTAG mailbox control"]
pub struct SYSJMBC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "JTAG mailbox control"]
pub mod sysjmbc;
#[doc = "JTAG mailbox input 0"]
pub struct SYSJMBI0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "JTAG mailbox input 0"]
pub mod sysjmbi0;
#[doc = "JTAG mailbox input 1"]
pub struct SYSJMBI1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "JTAG mailbox input 1"]
pub mod sysjmbi1;
#[doc = "JTAG mailbox output 0"]
pub struct SYSJMBO0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "JTAG mailbox output 0"]
pub mod sysjmbo0;
#[doc = "JTAG mailbox output 1"]
pub struct SYSJMBO1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "JTAG mailbox output 1"]
pub mod sysjmbo1;
#[doc = "Bus Error vector generator"]
pub struct SYSBERRIV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Bus Error vector generator"]
pub mod sysberriv;
#[doc = "User NMI vector generator"]
pub struct SYSUNIV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "User NMI vector generator"]
pub mod sysuniv;
#[doc = "System NMI vector generator"]
pub struct SYSSNIV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "System NMI vector generator"]
pub mod syssniv;
#[doc = "Reset vector generator"]
pub struct SYSRSTIV {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Reset vector generator"]
pub mod sysrstiv;
#[doc = "System Configuration 0"]
pub struct SYSCFG0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "System Configuration 1"]
pub struct SYSCFG1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "System Configuration 2"]
pub struct SYSCFG2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "System Configuration 2"]
pub mod syscfg2;
