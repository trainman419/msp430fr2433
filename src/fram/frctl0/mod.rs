#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::FRCTL0 {
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
#[doc = "Possible values of the field `NWAITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NWAITSR {
    #[doc = "FRAM Wait state control: 0"]
    NWAITS_0,
    #[doc = "FRAM Wait state control: 1"]
    NWAITS_1,
    #[doc = "FRAM Wait state control: 2"]
    NWAITS_2,
    #[doc = "FRAM Wait state control: 3"]
    NWAITS_3,
    #[doc = "FRAM Wait state control: 4"]
    NWAITS_4,
    #[doc = "FRAM Wait state control: 5"]
    NWAITS_5,
    #[doc = "FRAM Wait state control: 6"]
    NWAITS_6,
    #[doc = "FRAM Wait state control: 7"]
    NWAITS_7,
}
impl NWAITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NWAITSR::NWAITS_0 => 0,
            NWAITSR::NWAITS_1 => 1,
            NWAITSR::NWAITS_2 => 2,
            NWAITSR::NWAITS_3 => 3,
            NWAITSR::NWAITS_4 => 4,
            NWAITSR::NWAITS_5 => 5,
            NWAITSR::NWAITS_6 => 6,
            NWAITSR::NWAITS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NWAITSR {
        match value {
            0 => NWAITSR::NWAITS_0,
            1 => NWAITSR::NWAITS_1,
            2 => NWAITSR::NWAITS_2,
            3 => NWAITSR::NWAITS_3,
            4 => NWAITSR::NWAITS_4,
            5 => NWAITSR::NWAITS_5,
            6 => NWAITSR::NWAITS_6,
            7 => NWAITSR::NWAITS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NWAITS_0`"]
    #[inline]
    pub fn is_nwaits_0(&self) -> bool {
        *self == NWAITSR::NWAITS_0
    }
    #[doc = "Checks if the value of the field is `NWAITS_1`"]
    #[inline]
    pub fn is_nwaits_1(&self) -> bool {
        *self == NWAITSR::NWAITS_1
    }
    #[doc = "Checks if the value of the field is `NWAITS_2`"]
    #[inline]
    pub fn is_nwaits_2(&self) -> bool {
        *self == NWAITSR::NWAITS_2
    }
    #[doc = "Checks if the value of the field is `NWAITS_3`"]
    #[inline]
    pub fn is_nwaits_3(&self) -> bool {
        *self == NWAITSR::NWAITS_3
    }
    #[doc = "Checks if the value of the field is `NWAITS_4`"]
    #[inline]
    pub fn is_nwaits_4(&self) -> bool {
        *self == NWAITSR::NWAITS_4
    }
    #[doc = "Checks if the value of the field is `NWAITS_5`"]
    #[inline]
    pub fn is_nwaits_5(&self) -> bool {
        *self == NWAITSR::NWAITS_5
    }
    #[doc = "Checks if the value of the field is `NWAITS_6`"]
    #[inline]
    pub fn is_nwaits_6(&self) -> bool {
        *self == NWAITSR::NWAITS_6
    }
    #[doc = "Checks if the value of the field is `NWAITS_7`"]
    #[inline]
    pub fn is_nwaits_7(&self) -> bool {
        *self == NWAITSR::NWAITS_7
    }
}
#[doc = "Values that can be written to the field `NWAITS`"]
pub enum NWAITSW {
    #[doc = "FRAM Wait state control: 0"]
    NWAITS_0,
    #[doc = "FRAM Wait state control: 1"]
    NWAITS_1,
    #[doc = "FRAM Wait state control: 2"]
    NWAITS_2,
    #[doc = "FRAM Wait state control: 3"]
    NWAITS_3,
    #[doc = "FRAM Wait state control: 4"]
    NWAITS_4,
    #[doc = "FRAM Wait state control: 5"]
    NWAITS_5,
    #[doc = "FRAM Wait state control: 6"]
    NWAITS_6,
    #[doc = "FRAM Wait state control: 7"]
    NWAITS_7,
}
impl NWAITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NWAITSW::NWAITS_0 => 0,
            NWAITSW::NWAITS_1 => 1,
            NWAITSW::NWAITS_2 => 2,
            NWAITSW::NWAITS_3 => 3,
            NWAITSW::NWAITS_4 => 4,
            NWAITSW::NWAITS_5 => 5,
            NWAITSW::NWAITS_6 => 6,
            NWAITSW::NWAITS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NWAITSW<'a> {
    w: &'a mut W,
}
impl<'a> _NWAITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NWAITSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FRAM Wait state control: 0"]
    #[inline]
    pub fn nwaits_0(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_0)
    }
    #[doc = "FRAM Wait state control: 1"]
    #[inline]
    pub fn nwaits_1(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_1)
    }
    #[doc = "FRAM Wait state control: 2"]
    #[inline]
    pub fn nwaits_2(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_2)
    }
    #[doc = "FRAM Wait state control: 3"]
    #[inline]
    pub fn nwaits_3(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_3)
    }
    #[doc = "FRAM Wait state control: 4"]
    #[inline]
    pub fn nwaits_4(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_4)
    }
    #[doc = "FRAM Wait state control: 5"]
    #[inline]
    pub fn nwaits_5(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_5)
    }
    #[doc = "FRAM Wait state control: 6"]
    #[inline]
    pub fn nwaits_6(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_6)
    }
    #[doc = "FRAM Wait state control: 7"]
    #[inline]
    pub fn nwaits_7(self) -> &'a mut W {
        self.variant(NWAITSW::NWAITS_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 4:6 - FRAM Wait state control Bit: 0"]
    #[inline]
    pub fn nwaits(&self) -> NWAITSR {
        NWAITSR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 4:6 - FRAM Wait state control Bit: 0"]
    #[inline]
    pub fn nwaits(&mut self) -> _NWAITSW {
        _NWAITSW { w: self }
    }
}
