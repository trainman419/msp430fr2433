#[cfg(feature = "rt")]
global_asm!(
    "\n                DH_TRAMPOLINE:\n                    br #DEFAULT_HANDLER\n                "
);
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak PORT2\nPORT2 = DH_TRAMPOLINE\n.weak PORT1\nPORT1 = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak USCI_B0\nUSCI_B0 = DH_TRAMPOLINE\n.weak USCI_A1\nUSCI_A1 = DH_TRAMPOLINE\n.weak USCI_A0\nUSCI_A0 = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak TIMER3_A1\nTIMER3_A1 = DH_TRAMPOLINE\n.weak TIMER3_A0\nTIMER3_A0 = DH_TRAMPOLINE\n.weak TIMER2_A1\nTIMER2_A1 = DH_TRAMPOLINE\n.weak TIMER2_A0\nTIMER2_A0 = DH_TRAMPOLINE\n.weak TIMER1_A1\nTIMER1_A1 = DH_TRAMPOLINE\n.weak TIMER1_A0\nTIMER1_A0 = DH_TRAMPOLINE\n.weak TIMER0_A1\nTIMER0_A1 = DH_TRAMPOLINE\n.weak TIMER0_A0\nTIMER0_A0 = DH_TRAMPOLINE\n.weak UNMI\nUNMI = DH_TRAMPOLINE\n.weak SYSNMI\nSYSNMI = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT2();
    fn PORT1();
    fn ADC();
    fn USCI_B0();
    fn USCI_A1();
    fn USCI_A0();
    fn WDT();
    fn RTC();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u32,
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Vector; 59] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT2 },
    Vector { _handler: PORT1 },
    Vector { _handler: ADC },
    Vector { _handler: USCI_B0 },
    Vector { _handler: USCI_A1 },
    Vector { _handler: USCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector {
        _handler: TIMER3_A1,
    },
    Vector {
        _handler: TIMER3_A0,
    },
    Vector {
        _handler: TIMER2_A1,
    },
    Vector {
        _handler: TIMER2_A0,
    },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
use core::convert::TryFrom;
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "41 - 0xFFDA Port 2"]
    PORT2,
    #[doc = "42 - 0xFFDC Port 1"]
    PORT1,
    #[doc = "43 - 0xFFDE ADC"]
    ADC,
    #[doc = "44 - 0xFFE0 USCI B0 Receive/Transmit"]
    USCI_B0,
    #[doc = "45 - 0xFFE2 USCI A1 Receive/Transmit"]
    USCI_A1,
    #[doc = "46 - 0xFFE4 USCI A0 Receive/Transmit"]
    USCI_A0,
    #[doc = "47 - 0xFFE6 Watchdog Timer"]
    WDT,
    #[doc = "48 - 0xFFE8 RTC"]
    RTC,
    #[doc = "49 - 0xFFEA Timer3_A2 CC1, TA"]
    TIMER3_A1,
    #[doc = "50 - 0xFFEC Timer3_A2 CC0"]
    TIMER3_A0,
    #[doc = "51 - 0xFFEE Timer2_A2 CC1, TA"]
    TIMER2_A1,
    #[doc = "52 - 0xFFF0 Timer2_A2 CC0"]
    TIMER2_A0,
    #[doc = "53 - 0xFFF2 Timer1_A3 CC1-2, TA"]
    TIMER1_A1,
    #[doc = "54 - 0xFFF4 Timer1_A3 CC0"]
    TIMER1_A0,
    #[doc = "55 - 0xFFF6 Timer0_A3 CC1-2, TA"]
    TIMER0_A1,
    #[doc = "56 - 0xFFE8 Timer0_A3 CC0"]
    TIMER0_A0,
    #[doc = "57 - 0xFFFA User Non-maskable"]
    UNMI,
    #[doc = "58 - 0xFFFC System Non-maskable"]
    SYSNMI,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PORT2 => 41,
            Interrupt::PORT1 => 42,
            Interrupt::ADC => 43,
            Interrupt::USCI_B0 => 44,
            Interrupt::USCI_A1 => 45,
            Interrupt::USCI_A0 => 46,
            Interrupt::WDT => 47,
            Interrupt::RTC => 48,
            Interrupt::TIMER3_A1 => 49,
            Interrupt::TIMER3_A0 => 50,
            Interrupt::TIMER2_A1 => 51,
            Interrupt::TIMER2_A0 => 52,
            Interrupt::TIMER1_A1 => 53,
            Interrupt::TIMER1_A0 => 54,
            Interrupt::TIMER0_A1 => 55,
            Interrupt::TIMER0_A0 => 56,
            Interrupt::UNMI => 57,
            Interrupt::SYSNMI => 58,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            41 => Ok(Interrupt::PORT2),
            42 => Ok(Interrupt::PORT1),
            43 => Ok(Interrupt::ADC),
            44 => Ok(Interrupt::USCI_B0),
            45 => Ok(Interrupt::USCI_A1),
            46 => Ok(Interrupt::USCI_A0),
            47 => Ok(Interrupt::WDT),
            48 => Ok(Interrupt::RTC),
            49 => Ok(Interrupt::TIMER3_A1),
            50 => Ok(Interrupt::TIMER3_A0),
            51 => Ok(Interrupt::TIMER2_A1),
            52 => Ok(Interrupt::TIMER2_A0),
            53 => Ok(Interrupt::TIMER1_A1),
            54 => Ok(Interrupt::TIMER1_A0),
            55 => Ok(Interrupt::TIMER0_A1),
            56 => Ok(Interrupt::TIMER0_A0),
            57 => Ok(Interrupt::UNMI),
            58 => Ok(Interrupt::SYSNMI),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
