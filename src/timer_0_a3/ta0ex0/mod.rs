#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TA0EX0 {
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
#[doc = "Possible values of the field `TAIDEX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIDEXR {
    #[doc = "Timer A Input divider expansion : /1"]
    TAIDEX_0,
    #[doc = "Timer A Input divider expansion : /2"]
    TAIDEX_1,
    #[doc = "Timer A Input divider expansion : /3"]
    TAIDEX_2,
    #[doc = "Timer A Input divider expansion : /4"]
    TAIDEX_3,
    #[doc = "Timer A Input divider expansion : /5"]
    TAIDEX_4,
    #[doc = "Timer A Input divider expansion : /6"]
    TAIDEX_5,
    #[doc = "Timer A Input divider expansion : /7"]
    TAIDEX_6,
    #[doc = "Timer A Input divider expansion : /8"]
    TAIDEX_7,
}
impl TAIDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TAIDEXR::TAIDEX_0 => 0,
            TAIDEXR::TAIDEX_1 => 1,
            TAIDEXR::TAIDEX_2 => 2,
            TAIDEXR::TAIDEX_3 => 3,
            TAIDEXR::TAIDEX_4 => 4,
            TAIDEXR::TAIDEX_5 => 5,
            TAIDEXR::TAIDEX_6 => 6,
            TAIDEXR::TAIDEX_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TAIDEXR {
        match value {
            0 => TAIDEXR::TAIDEX_0,
            1 => TAIDEXR::TAIDEX_1,
            2 => TAIDEXR::TAIDEX_2,
            3 => TAIDEXR::TAIDEX_3,
            4 => TAIDEXR::TAIDEX_4,
            5 => TAIDEXR::TAIDEX_5,
            6 => TAIDEXR::TAIDEX_6,
            7 => TAIDEXR::TAIDEX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TAIDEX_0`"]
    #[inline]
    pub fn is_taidex_0(&self) -> bool {
        *self == TAIDEXR::TAIDEX_0
    }
    #[doc = "Checks if the value of the field is `TAIDEX_1`"]
    #[inline]
    pub fn is_taidex_1(&self) -> bool {
        *self == TAIDEXR::TAIDEX_1
    }
    #[doc = "Checks if the value of the field is `TAIDEX_2`"]
    #[inline]
    pub fn is_taidex_2(&self) -> bool {
        *self == TAIDEXR::TAIDEX_2
    }
    #[doc = "Checks if the value of the field is `TAIDEX_3`"]
    #[inline]
    pub fn is_taidex_3(&self) -> bool {
        *self == TAIDEXR::TAIDEX_3
    }
    #[doc = "Checks if the value of the field is `TAIDEX_4`"]
    #[inline]
    pub fn is_taidex_4(&self) -> bool {
        *self == TAIDEXR::TAIDEX_4
    }
    #[doc = "Checks if the value of the field is `TAIDEX_5`"]
    #[inline]
    pub fn is_taidex_5(&self) -> bool {
        *self == TAIDEXR::TAIDEX_5
    }
    #[doc = "Checks if the value of the field is `TAIDEX_6`"]
    #[inline]
    pub fn is_taidex_6(&self) -> bool {
        *self == TAIDEXR::TAIDEX_6
    }
    #[doc = "Checks if the value of the field is `TAIDEX_7`"]
    #[inline]
    pub fn is_taidex_7(&self) -> bool {
        *self == TAIDEXR::TAIDEX_7
    }
}
#[doc = "Values that can be written to the field `TAIDEX`"]
pub enum TAIDEXW {
    #[doc = "Timer A Input divider expansion : /1"]
    TAIDEX_0,
    #[doc = "Timer A Input divider expansion : /2"]
    TAIDEX_1,
    #[doc = "Timer A Input divider expansion : /3"]
    TAIDEX_2,
    #[doc = "Timer A Input divider expansion : /4"]
    TAIDEX_3,
    #[doc = "Timer A Input divider expansion : /5"]
    TAIDEX_4,
    #[doc = "Timer A Input divider expansion : /6"]
    TAIDEX_5,
    #[doc = "Timer A Input divider expansion : /7"]
    TAIDEX_6,
    #[doc = "Timer A Input divider expansion : /8"]
    TAIDEX_7,
}
impl TAIDEXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TAIDEXW::TAIDEX_0 => 0,
            TAIDEXW::TAIDEX_1 => 1,
            TAIDEXW::TAIDEX_2 => 2,
            TAIDEXW::TAIDEX_3 => 3,
            TAIDEXW::TAIDEX_4 => 4,
            TAIDEXW::TAIDEX_5 => 5,
            TAIDEXW::TAIDEX_6 => 6,
            TAIDEXW::TAIDEX_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAIDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIDEXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIDEXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer A Input divider expansion : /1"]
    #[inline]
    pub fn taidex_0(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_0)
    }
    #[doc = "Timer A Input divider expansion : /2"]
    #[inline]
    pub fn taidex_1(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_1)
    }
    #[doc = "Timer A Input divider expansion : /3"]
    #[inline]
    pub fn taidex_2(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_2)
    }
    #[doc = "Timer A Input divider expansion : /4"]
    #[inline]
    pub fn taidex_3(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_3)
    }
    #[doc = "Timer A Input divider expansion : /5"]
    #[inline]
    pub fn taidex_4(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_4)
    }
    #[doc = "Timer A Input divider expansion : /6"]
    #[inline]
    pub fn taidex_5(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_5)
    }
    #[doc = "Timer A Input divider expansion : /7"]
    #[inline]
    pub fn taidex_6(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_6)
    }
    #[doc = "Timer A Input divider expansion : /8"]
    #[inline]
    pub fn taidex_7(self) -> &'a mut W {
        self.variant(TAIDEXW::TAIDEX_7)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline]
    pub fn taidex(&self) -> TAIDEXR {
        TAIDEXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Timer A Input divider expansion Bit: 0"]
    #[inline]
    pub fn taidex(&mut self) -> _TAIDEXW {
        _TAIDEXW { w: self }
    }
}
