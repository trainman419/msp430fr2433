#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL3 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `FLLREFDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLREFDIVR {
    #[doc = "Reference Divider: f(LFCLK)/1"]
    FLLREFDIV_0,
    #[doc = "Reference Divider: f(LFCLK)/2"]
    FLLREFDIV_1,
    #[doc = "Reference Divider: f(LFCLK)/4"]
    FLLREFDIV_2,
    #[doc = "Reference Divider: f(LFCLK)/8"]
    FLLREFDIV_3,
    #[doc = "Reference Divider: f(LFCLK)/12"]
    FLLREFDIV_4,
    #[doc = "Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_5,
    #[doc = "Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_6,
    #[doc = "Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_7,
}
impl FLLREFDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLLREFDIVR::FLLREFDIV_0 => 0,
            FLLREFDIVR::FLLREFDIV_1 => 1,
            FLLREFDIVR::FLLREFDIV_2 => 2,
            FLLREFDIVR::FLLREFDIV_3 => 3,
            FLLREFDIVR::FLLREFDIV_4 => 4,
            FLLREFDIVR::FLLREFDIV_5 => 5,
            FLLREFDIVR::FLLREFDIV_6 => 6,
            FLLREFDIVR::FLLREFDIV_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLLREFDIVR {
        match value {
            0 => FLLREFDIVR::FLLREFDIV_0,
            1 => FLLREFDIVR::FLLREFDIV_1,
            2 => FLLREFDIVR::FLLREFDIV_2,
            3 => FLLREFDIVR::FLLREFDIV_3,
            4 => FLLREFDIVR::FLLREFDIV_4,
            5 => FLLREFDIVR::FLLREFDIV_5,
            6 => FLLREFDIVR::FLLREFDIV_6,
            7 => FLLREFDIVR::FLLREFDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_0`"]
    #[inline]
    pub fn is_fllrefdiv_0(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_0
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_1`"]
    #[inline]
    pub fn is_fllrefdiv_1(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_1
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_2`"]
    #[inline]
    pub fn is_fllrefdiv_2(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_2
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_3`"]
    #[inline]
    pub fn is_fllrefdiv_3(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_3
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_4`"]
    #[inline]
    pub fn is_fllrefdiv_4(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_4
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_5`"]
    #[inline]
    pub fn is_fllrefdiv_5(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_5
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_6`"]
    #[inline]
    pub fn is_fllrefdiv_6(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_6
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_7`"]
    #[inline]
    pub fn is_fllrefdiv_7(&self) -> bool {
        *self == FLLREFDIVR::FLLREFDIV_7
    }
}
#[doc = "Possible values of the field `SELREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELREFR {
    #[doc = "FLL Reference Clock Select 0"]
    SELREF_0,
    #[doc = "FLL Reference Clock Select 1"]
    SELREF_1,
    #[doc = "FLL Reference Clock Select 2"]
    SELREF_2,
    #[doc = "FLL Reference Clock Select 3"]
    SELREF_3,
}
impl SELREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELREFR::SELREF_0 => 0,
            SELREFR::SELREF_1 => 1,
            SELREFR::SELREF_2 => 2,
            SELREFR::SELREF_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELREFR {
        match value {
            0 => SELREFR::SELREF_0,
            1 => SELREFR::SELREF_1,
            2 => SELREFR::SELREF_2,
            3 => SELREFR::SELREF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELREF_0`"]
    #[inline]
    pub fn is_selref_0(&self) -> bool {
        *self == SELREFR::SELREF_0
    }
    #[doc = "Checks if the value of the field is `SELREF_1`"]
    #[inline]
    pub fn is_selref_1(&self) -> bool {
        *self == SELREFR::SELREF_1
    }
    #[doc = "Checks if the value of the field is `SELREF_2`"]
    #[inline]
    pub fn is_selref_2(&self) -> bool {
        *self == SELREFR::SELREF_2
    }
    #[doc = "Checks if the value of the field is `SELREF_3`"]
    #[inline]
    pub fn is_selref_3(&self) -> bool {
        *self == SELREFR::SELREF_3
    }
}
#[doc = "Values that can be written to the field `FLLREFDIV`"]
pub enum FLLREFDIVW {
    #[doc = "Reference Divider: f(LFCLK)/1"]
    FLLREFDIV_0,
    #[doc = "Reference Divider: f(LFCLK)/2"]
    FLLREFDIV_1,
    #[doc = "Reference Divider: f(LFCLK)/4"]
    FLLREFDIV_2,
    #[doc = "Reference Divider: f(LFCLK)/8"]
    FLLREFDIV_3,
    #[doc = "Reference Divider: f(LFCLK)/12"]
    FLLREFDIV_4,
    #[doc = "Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_5,
    #[doc = "Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_6,
    #[doc = "Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_7,
}
impl FLLREFDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLLREFDIVW::FLLREFDIV_0 => 0,
            FLLREFDIVW::FLLREFDIV_1 => 1,
            FLLREFDIVW::FLLREFDIV_2 => 2,
            FLLREFDIVW::FLLREFDIV_3 => 3,
            FLLREFDIVW::FLLREFDIV_4 => 4,
            FLLREFDIVW::FLLREFDIV_5 => 5,
            FLLREFDIVW::FLLREFDIV_6 => 6,
            FLLREFDIVW::FLLREFDIV_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLLREFDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLREFDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLREFDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline]
    pub fn fllrefdiv_0(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_0)
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline]
    pub fn fllrefdiv_1(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_1)
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline]
    pub fn fllrefdiv_2(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_2)
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline]
    pub fn fllrefdiv_3(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_3)
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline]
    pub fn fllrefdiv_4(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_4)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline]
    pub fn fllrefdiv_5(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_5)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline]
    pub fn fllrefdiv_6(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_6)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline]
    pub fn fllrefdiv_7(self) -> &'a mut W {
        self.variant(FLLREFDIVW::FLLREFDIV_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SELREF`"]
pub enum SELREFW {
    #[doc = "FLL Reference Clock Select 0"]
    SELREF_0,
    #[doc = "FLL Reference Clock Select 1"]
    SELREF_1,
    #[doc = "FLL Reference Clock Select 2"]
    SELREF_2,
    #[doc = "FLL Reference Clock Select 3"]
    SELREF_3,
}
impl SELREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELREFW::SELREF_0 => 0,
            SELREFW::SELREF_1 => 1,
            SELREFW::SELREF_2 => 2,
            SELREFW::SELREF_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELREFW<'a> {
    w: &'a mut W,
}
impl<'a> _SELREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELREFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FLL Reference Clock Select 0"]
    #[inline]
    pub fn selref_0(self) -> &'a mut W {
        self.variant(SELREFW::SELREF_0)
    }
    #[doc = "FLL Reference Clock Select 1"]
    #[inline]
    pub fn selref_1(self) -> &'a mut W {
        self.variant(SELREFW::SELREF_1)
    }
    #[doc = "FLL Reference Clock Select 2"]
    #[inline]
    pub fn selref_2(self) -> &'a mut W {
        self.variant(SELREFW::SELREF_2)
    }
    #[doc = "FLL Reference Clock Select 3"]
    #[inline]
    pub fn selref_3(self) -> &'a mut W {
        self.variant(SELREFW::SELREF_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline]
    pub fn fllrefdiv(&self) -> FLLREFDIVR {
        FLLREFDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - FLL Reference Clock Select Bit : 0"]
    #[inline]
    pub fn selref(&self) -> SELREFR {
        SELREFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline]
    pub fn fllrefdiv(&mut self) -> _FLLREFDIVW {
        _FLLREFDIVW { w: self }
    }
    #[doc = "Bits 4:5 - FLL Reference Clock Select Bit : 0"]
    #[inline]
    pub fn selref(&mut self) -> _SELREFW {
        _SELREFW { w: self }
    }
}
