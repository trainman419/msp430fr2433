#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::UCA0CTLW1 {
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
#[doc = "Possible values of the field `UCGLIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCGLITR {
    #[doc = "USCI Deglitch time: 0"]
    UCGLIT_0,
    #[doc = "USCI Deglitch time: 1"]
    UCGLIT_1,
    #[doc = "USCI Deglitch time: 2"]
    UCGLIT_2,
    #[doc = "USCI Deglitch time: 3"]
    UCGLIT_3,
}
impl UCGLITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UCGLITR::UCGLIT_0 => 0,
            UCGLITR::UCGLIT_1 => 1,
            UCGLITR::UCGLIT_2 => 2,
            UCGLITR::UCGLIT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UCGLITR {
        match value {
            0 => UCGLITR::UCGLIT_0,
            1 => UCGLITR::UCGLIT_1,
            2 => UCGLITR::UCGLIT_2,
            3 => UCGLITR::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLITR::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLITR::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLITR::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLITR::UCGLIT_3
    }
}
#[doc = "Values that can be written to the field `UCGLIT`"]
pub enum UCGLITW {
    #[doc = "USCI Deglitch time: 0"]
    UCGLIT_0,
    #[doc = "USCI Deglitch time: 1"]
    UCGLIT_1,
    #[doc = "USCI Deglitch time: 2"]
    UCGLIT_2,
    #[doc = "USCI Deglitch time: 3"]
    UCGLIT_3,
}
impl UCGLITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UCGLITW::UCGLIT_0 => 0,
            UCGLITW::UCGLIT_1 => 1,
            UCGLITW::UCGLIT_2 => 2,
            UCGLITW::UCGLIT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UCGLITW<'a> {
    w: &'a mut W,
}
impl<'a> _UCGLITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCGLITW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "USCI Deglitch time: 0"]
    #[inline]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_0)
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_1)
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_2)
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline]
    pub fn ucglit(&self) -> UCGLITR {
        UCGLITR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - USCI Deglitch Time Bit 1"]
    #[inline]
    pub fn ucglit(&mut self) -> _UCGLITW {
        _UCGLITW { w: self }
    }
}
