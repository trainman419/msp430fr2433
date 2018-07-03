#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL4 {
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
#[doc = "Possible values of the field `SELMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMSR {
    #[doc = "MCLK and SMCLK Source Select 0"]
    SELMS_0,
    #[doc = "MCLK and SMCLK Source Select 1"]
    SELMS_1,
    #[doc = "MCLK and SMCLK Source Select 2"]
    SELMS_2,
    #[doc = "MCLK and SMCLK Source Select 3"]
    SELMS_3,
    #[doc = "MCLK and SMCLK Source Select 4"]
    SELMS_4,
    #[doc = "MCLK and SMCLK Source Select 5"]
    SELMS_5,
    #[doc = "MCLK and SMCLK Source Select 6"]
    SELMS_6,
    #[doc = "MCLK and SMCLK Source Select 7"]
    SELMS_7,
}
impl SELMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELMSR::SELMS_0 => 0,
            SELMSR::SELMS_1 => 1,
            SELMSR::SELMS_2 => 2,
            SELMSR::SELMS_3 => 3,
            SELMSR::SELMS_4 => 4,
            SELMSR::SELMS_5 => 5,
            SELMSR::SELMS_6 => 6,
            SELMSR::SELMS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELMSR {
        match value {
            0 => SELMSR::SELMS_0,
            1 => SELMSR::SELMS_1,
            2 => SELMSR::SELMS_2,
            3 => SELMSR::SELMS_3,
            4 => SELMSR::SELMS_4,
            5 => SELMSR::SELMS_5,
            6 => SELMSR::SELMS_6,
            7 => SELMSR::SELMS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SELMS_0`"]
    #[inline]
    pub fn is_selms_0(&self) -> bool {
        *self == SELMSR::SELMS_0
    }
    #[doc = "Checks if the value of the field is `SELMS_1`"]
    #[inline]
    pub fn is_selms_1(&self) -> bool {
        *self == SELMSR::SELMS_1
    }
    #[doc = "Checks if the value of the field is `SELMS_2`"]
    #[inline]
    pub fn is_selms_2(&self) -> bool {
        *self == SELMSR::SELMS_2
    }
    #[doc = "Checks if the value of the field is `SELMS_3`"]
    #[inline]
    pub fn is_selms_3(&self) -> bool {
        *self == SELMSR::SELMS_3
    }
    #[doc = "Checks if the value of the field is `SELMS_4`"]
    #[inline]
    pub fn is_selms_4(&self) -> bool {
        *self == SELMSR::SELMS_4
    }
    #[doc = "Checks if the value of the field is `SELMS_5`"]
    #[inline]
    pub fn is_selms_5(&self) -> bool {
        *self == SELMSR::SELMS_5
    }
    #[doc = "Checks if the value of the field is `SELMS_6`"]
    #[inline]
    pub fn is_selms_6(&self) -> bool {
        *self == SELMSR::SELMS_6
    }
    #[doc = "Checks if the value of the field is `SELMS_7`"]
    #[inline]
    pub fn is_selms_7(&self) -> bool {
        *self == SELMSR::SELMS_7
    }
}
#[doc = "Possible values of the field `SELA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELAR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SELAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SELAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SELAR {
        match value {
            i => SELAR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `SELMS`"]
pub enum SELMSW {
    #[doc = "MCLK and SMCLK Source Select 0"]
    SELMS_0,
    #[doc = "MCLK and SMCLK Source Select 1"]
    SELMS_1,
    #[doc = "MCLK and SMCLK Source Select 2"]
    SELMS_2,
    #[doc = "MCLK and SMCLK Source Select 3"]
    SELMS_3,
    #[doc = "MCLK and SMCLK Source Select 4"]
    SELMS_4,
    #[doc = "MCLK and SMCLK Source Select 5"]
    SELMS_5,
    #[doc = "MCLK and SMCLK Source Select 6"]
    SELMS_6,
    #[doc = "MCLK and SMCLK Source Select 7"]
    SELMS_7,
}
impl SELMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELMSW::SELMS_0 => 0,
            SELMSW::SELMS_1 => 1,
            SELMSW::SELMS_2 => 2,
            SELMSW::SELMS_3 => 3,
            SELMSW::SELMS_4 => 4,
            SELMSW::SELMS_5 => 5,
            SELMSW::SELMS_6 => 6,
            SELMSW::SELMS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SELMSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MCLK and SMCLK Source Select 0"]
    #[inline]
    pub fn selms_0(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_0)
    }
    #[doc = "MCLK and SMCLK Source Select 1"]
    #[inline]
    pub fn selms_1(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_1)
    }
    #[doc = "MCLK and SMCLK Source Select 2"]
    #[inline]
    pub fn selms_2(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_2)
    }
    #[doc = "MCLK and SMCLK Source Select 3"]
    #[inline]
    pub fn selms_3(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_3)
    }
    #[doc = "MCLK and SMCLK Source Select 4"]
    #[inline]
    pub fn selms_4(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_4)
    }
    #[doc = "MCLK and SMCLK Source Select 5"]
    #[inline]
    pub fn selms_5(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_5)
    }
    #[doc = "MCLK and SMCLK Source Select 6"]
    #[inline]
    pub fn selms_6(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_6)
    }
    #[doc = "MCLK and SMCLK Source Select 7"]
    #[inline]
    pub fn selms_7(self) -> &'a mut W {
        self.variant(SELMSW::SELMS_7)
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
#[doc = "Values that can be written to the field `SELA`"]
pub enum SELAW {}
impl SELAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SELAW<'a> {
    w: &'a mut W,
}
impl<'a> _SELAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SELAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline]
    pub fn selms(&self) -> SELMSR {
        SELMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline]
    pub fn sela(&self) -> SELAR {
        SELAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
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
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline]
    pub fn selms(&mut self) -> _SELMSW {
        _SELMSW { w: self }
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline]
    pub fn sela(&mut self) -> _SELAW {
        _SELAW { w: self }
    }
}
