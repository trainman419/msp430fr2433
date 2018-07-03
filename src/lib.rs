#![feature(abi_msp430_interrupt)]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for MSP430FR2433 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(const_fn)]
#![feature(try_from)]
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
#[cfg(feature = "rt")]
pub use msp430_rt::default_handler;
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use interrupt::Interrupt;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port_1_2::RegisterBlock {
        512 as *const _
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    fn deref(&self) -> &port_1_2::RegisterBlock {
        unsafe { &*PORT_1_2::ptr() }
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "Port 3"]
pub struct PORT_3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_3 {}
impl PORT_3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port_3::RegisterBlock {
        544 as *const _
    }
}
impl Deref for PORT_3 {
    type Target = port_3::RegisterBlock;
    fn deref(&self) -> &port_3::RegisterBlock {
        unsafe { &*PORT_3::ptr() }
    }
}
#[doc = "Port 3"]
pub mod port_3;
#[doc = "USCI_A0 UART Mode"]
pub struct USCI_A0_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_UART_MODE {}
impl USCI_A0_UART_MODE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usci_a0_uart_mode::RegisterBlock {
        1280 as *const _
    }
}
impl Deref for USCI_A0_UART_MODE {
    type Target = usci_a0_uart_mode::RegisterBlock;
    fn deref(&self) -> &usci_a0_uart_mode::RegisterBlock {
        unsafe { &*USCI_A0_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A0 UART Mode"]
pub mod usci_a0_uart_mode;
#[doc = "USCI_A0 SPI Mode"]
pub struct USCI_A0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_SPI_MODE {}
impl USCI_A0_SPI_MODE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usci_a0_spi_mode::RegisterBlock {
        1280 as *const _
    }
}
impl Deref for USCI_A0_SPI_MODE {
    type Target = usci_a0_spi_mode::RegisterBlock;
    fn deref(&self) -> &usci_a0_spi_mode::RegisterBlock {
        unsafe { &*USCI_A0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A0 SPI Mode"]
pub mod usci_a0_spi_mode;
#[doc = "USCI_A1 UART Mode"]
pub struct USCI_A1_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_UART_MODE {}
impl USCI_A1_UART_MODE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usci_a1_uart_mode::RegisterBlock {
        1312 as *const _
    }
}
impl Deref for USCI_A1_UART_MODE {
    type Target = usci_a1_uart_mode::RegisterBlock;
    fn deref(&self) -> &usci_a1_uart_mode::RegisterBlock {
        unsafe { &*USCI_A1_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A1 UART Mode"]
pub mod usci_a1_uart_mode;
#[doc = "USCI_A1 SPI Mode"]
pub struct USCI_A1_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_SPI_MODE {}
impl USCI_A1_SPI_MODE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usci_a1_spi_mode::RegisterBlock {
        1312 as *const _
    }
}
impl Deref for USCI_A1_SPI_MODE {
    type Target = usci_a1_spi_mode::RegisterBlock;
    fn deref(&self) -> &usci_a1_spi_mode::RegisterBlock {
        unsafe { &*USCI_A1_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A1 SPI Mode"]
pub mod usci_a1_spi_mode;
#[doc = "USCI_B0 I2C Mode"]
pub struct USCI_B0_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_I2C_MODE {}
impl USCI_B0_I2C_MODE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usci_b0_i2c_mode::RegisterBlock {
        1344 as *const _
    }
}
impl Deref for USCI_B0_I2C_MODE {
    type Target = usci_b0_i2c_mode::RegisterBlock;
    fn deref(&self) -> &usci_b0_i2c_mode::RegisterBlock {
        unsafe { &*USCI_B0_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B0 I2C Mode"]
pub mod usci_b0_i2c_mode;
#[doc = "USCI_B0 SPI Mode"]
pub struct USCI_B0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_SPI_MODE {}
impl USCI_B0_SPI_MODE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usci_b0_spi_mode::RegisterBlock {
        1344 as *const _
    }
}
impl Deref for USCI_B0_SPI_MODE {
    type Target = usci_b0_spi_mode::RegisterBlock;
    fn deref(&self) -> &usci_b0_spi_mode::RegisterBlock {
        unsafe { &*USCI_B0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B0 SPI Mode"]
pub mod usci_b0_spi_mode;
#[doc = "SFR Special Function Registers"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sfr::RegisterBlock {
        256 as *const _
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    fn deref(&self) -> &sfr::RegisterBlock {
        unsafe { &*SFR::ptr() }
    }
}
#[doc = "SFR Special Function Registers"]
pub mod sfr;
#[doc = "PMM Power Management System"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmm::RegisterBlock {
        288 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    fn deref(&self) -> &pmm::RegisterBlock {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "PMM Power Management System"]
pub mod pmm;
#[doc = "SYS System Module"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys::RegisterBlock {
        320 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    fn deref(&self) -> &sys::RegisterBlock {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "SYS System Module"]
pub mod sys;
#[doc = "CS Clock System"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cs::RegisterBlock {
        384 as *const _
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    fn deref(&self) -> &cs::RegisterBlock {
        unsafe { &*CS::ptr() }
    }
}
#[doc = "CS Clock System"]
pub mod cs;
#[doc = "FRAM"]
pub struct FRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRAM {}
impl FRAM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fram::RegisterBlock {
        416 as *const _
    }
}
impl Deref for FRAM {
    type Target = fram::RegisterBlock;
    fn deref(&self) -> &fram::RegisterBlock {
        unsafe { &*FRAM::ptr() }
    }
}
#[doc = "FRAM"]
pub mod fram;
#[doc = "CRC16"]
pub struct CRC16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC16 {}
impl CRC16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc16::RegisterBlock {
        448 as *const _
    }
}
impl Deref for CRC16 {
    type Target = crc16::RegisterBlock;
    fn deref(&self) -> &crc16::RegisterBlock {
        unsafe { &*CRC16::ptr() }
    }
}
#[doc = "CRC16"]
pub mod crc16;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const watchdog_timer::RegisterBlock {
        460 as *const _
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    fn deref(&self) -> &watchdog_timer::RegisterBlock {
        unsafe { &*WATCHDOG_TIMER::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Real-Time Clock"]
pub struct REAL_TIME_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for REAL_TIME_CLOCK {}
impl REAL_TIME_CLOCK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const real_time_clock::RegisterBlock {
        768 as *const _
    }
}
impl Deref for REAL_TIME_CLOCK {
    type Target = real_time_clock::RegisterBlock;
    fn deref(&self) -> &real_time_clock::RegisterBlock {
        unsafe { &*REAL_TIME_CLOCK::ptr() }
    }
}
#[doc = "Real-Time Clock"]
pub mod real_time_clock;
#[doc = "Timer0_A3"]
pub struct TIMER_0_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_A3 {}
impl TIMER_0_A3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer_0_a3::RegisterBlock {
        896 as *const _
    }
}
impl Deref for TIMER_0_A3 {
    type Target = timer_0_a3::RegisterBlock;
    fn deref(&self) -> &timer_0_a3::RegisterBlock {
        unsafe { &*TIMER_0_A3::ptr() }
    }
}
#[doc = "Timer0_A3"]
pub mod timer_0_a3;
#[doc = "Timer1_A3"]
pub struct TIMER_1_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_1_A3 {}
impl TIMER_1_A3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer_1_a3::RegisterBlock {
        960 as *const _
    }
}
impl Deref for TIMER_1_A3 {
    type Target = timer_1_a3::RegisterBlock;
    fn deref(&self) -> &timer_1_a3::RegisterBlock {
        unsafe { &*TIMER_1_A3::ptr() }
    }
}
#[doc = "Timer1_A3"]
pub mod timer_1_a3;
#[doc = "Timer2_A2"]
pub struct TIMER_2_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_2_A2 {}
impl TIMER_2_A2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer_2_a2::RegisterBlock {
        1024 as *const _
    }
}
impl Deref for TIMER_2_A2 {
    type Target = timer_2_a2::RegisterBlock;
    fn deref(&self) -> &timer_2_a2::RegisterBlock {
        unsafe { &*TIMER_2_A2::ptr() }
    }
}
#[doc = "Timer2_A2"]
pub mod timer_2_a2;
#[doc = "Timer3_A2"]
pub struct TIMER_3_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_3_A2 {}
impl TIMER_3_A2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer_3_a2::RegisterBlock {
        1088 as *const _
    }
}
impl Deref for TIMER_3_A2 {
    type Target = timer_3_a2::RegisterBlock;
    fn deref(&self) -> &timer_3_a2::RegisterBlock {
        unsafe { &*TIMER_3_A2::ptr() }
    }
}
#[doc = "Timer3_A2"]
pub mod timer_3_a2;
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub struct MPY_16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_16 {}
impl MPY_16 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mpy_16::RegisterBlock {
        1216 as *const _
    }
}
impl Deref for MPY_16 {
    type Target = mpy_16::RegisterBlock;
    fn deref(&self) -> &mpy_16::RegisterBlock {
        unsafe { &*MPY_16::ptr() }
    }
}
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub mod mpy_16;
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub struct MPY_32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_32 {}
impl MPY_32 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const mpy_32::RegisterBlock {
        1232 as *const _
    }
}
impl Deref for MPY_32 {
    type Target = mpy_32::RegisterBlock;
    fn deref(&self) -> &mpy_32::RegisterBlock {
        unsafe { &*MPY_32::ptr() }
    }
}
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub mod mpy_32;
#[doc = "Backup Memory"]
pub struct BACKUP_MEMORY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BACKUP_MEMORY {}
impl BACKUP_MEMORY {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const backup_memory::RegisterBlock {
        1632 as *const _
    }
}
impl Deref for BACKUP_MEMORY {
    type Target = backup_memory::RegisterBlock;
    fn deref(&self) -> &backup_memory::RegisterBlock {
        unsafe { &*BACKUP_MEMORY::ptr() }
    }
}
#[doc = "Backup Memory"]
pub mod backup_memory;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1792 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "PORT_3"]
    pub PORT_3: PORT_3,
    #[doc = "USCI_A0_UART_MODE"]
    pub USCI_A0_UART_MODE: USCI_A0_UART_MODE,
    #[doc = "USCI_A0_SPI_MODE"]
    pub USCI_A0_SPI_MODE: USCI_A0_SPI_MODE,
    #[doc = "USCI_A1_UART_MODE"]
    pub USCI_A1_UART_MODE: USCI_A1_UART_MODE,
    #[doc = "USCI_A1_SPI_MODE"]
    pub USCI_A1_SPI_MODE: USCI_A1_SPI_MODE,
    #[doc = "USCI_B0_I2C_MODE"]
    pub USCI_B0_I2C_MODE: USCI_B0_I2C_MODE,
    #[doc = "USCI_B0_SPI_MODE"]
    pub USCI_B0_SPI_MODE: USCI_B0_SPI_MODE,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "FRAM"]
    pub FRAM: FRAM,
    #[doc = "CRC16"]
    pub CRC16: CRC16,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "REAL_TIME_CLOCK"]
    pub REAL_TIME_CLOCK: REAL_TIME_CLOCK,
    #[doc = "TIMER_0_A3"]
    pub TIMER_0_A3: TIMER_0_A3,
    #[doc = "TIMER_1_A3"]
    pub TIMER_1_A3: TIMER_1_A3,
    #[doc = "TIMER_2_A2"]
    pub TIMER_2_A2: TIMER_2_A2,
    #[doc = "TIMER_3_A2"]
    pub TIMER_3_A2: TIMER_3_A2,
    #[doc = "MPY_16"]
    pub MPY_16: MPY_16,
    #[doc = "MPY_32"]
    pub MPY_32: MPY_32,
    #[doc = "BACKUP_MEMORY"]
    pub BACKUP_MEMORY: BACKUP_MEMORY,
    #[doc = "ADC"]
    pub ADC: ADC,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            PORT_3: PORT_3 {
                _marker: PhantomData,
            },
            USCI_A0_UART_MODE: USCI_A0_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A0_SPI_MODE: USCI_A0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_A1_UART_MODE: USCI_A1_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A1_SPI_MODE: USCI_A1_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B0_I2C_MODE: USCI_B0_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B0_SPI_MODE: USCI_B0_SPI_MODE {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            FRAM: FRAM {
                _marker: PhantomData,
            },
            CRC16: CRC16 {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            REAL_TIME_CLOCK: REAL_TIME_CLOCK {
                _marker: PhantomData,
            },
            TIMER_0_A3: TIMER_0_A3 {
                _marker: PhantomData,
            },
            TIMER_1_A3: TIMER_1_A3 {
                _marker: PhantomData,
            },
            TIMER_2_A2: TIMER_2_A2 {
                _marker: PhantomData,
            },
            TIMER_3_A2: TIMER_3_A2 {
                _marker: PhantomData,
            },
            MPY_16: MPY_16 {
                _marker: PhantomData,
            },
            MPY_32: MPY_32 {
                _marker: PhantomData,
            },
            BACKUP_MEMORY: BACKUP_MEMORY {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
        }
    }
}
