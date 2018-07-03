#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL1 {
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
#[doc = "Possible values of the field `DISMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISMODR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DISMODR {
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
            DISMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISMODR {
        match value {
            i => DISMODR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `DCORSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCORSELR {
    #[doc = "DCO frequency range select: 0"]
    DCORSEL_0,
    #[doc = "DCO frequency range select: 1"]
    DCORSEL_1,
    #[doc = "DCO frequency range select: 2"]
    DCORSEL_2,
    #[doc = "DCO frequency range select: 3"]
    DCORSEL_3,
    #[doc = "DCO frequency range select: 4"]
    DCORSEL_4,
    #[doc = "DCO frequency range select: 5"]
    DCORSEL_5,
    #[doc = "DCO frequency range select: 6"]
    DCORSEL_6,
    #[doc = "DCO frequency range select: 7"]
    DCORSEL_7,
}
impl DCORSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCORSELR::DCORSEL_0 => 0,
            DCORSELR::DCORSEL_1 => 1,
            DCORSELR::DCORSEL_2 => 2,
            DCORSELR::DCORSEL_3 => 3,
            DCORSELR::DCORSEL_4 => 4,
            DCORSELR::DCORSEL_5 => 5,
            DCORSELR::DCORSEL_6 => 6,
            DCORSELR::DCORSEL_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCORSELR {
        match value {
            0 => DCORSELR::DCORSEL_0,
            1 => DCORSELR::DCORSEL_1,
            2 => DCORSELR::DCORSEL_2,
            3 => DCORSELR::DCORSEL_3,
            4 => DCORSELR::DCORSEL_4,
            5 => DCORSELR::DCORSEL_5,
            6 => DCORSELR::DCORSEL_6,
            7 => DCORSELR::DCORSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSELR::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSELR::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSELR::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSELR::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSELR::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSELR::DCORSEL_5
    }
    #[doc = "Checks if the value of the field is `DCORSEL_6`"]
    #[inline]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == DCORSELR::DCORSEL_6
    }
    #[doc = "Checks if the value of the field is `DCORSEL_7`"]
    #[inline]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == DCORSELR::DCORSEL_7
    }
}
#[doc = "Possible values of the field `DCOFTRIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFTRIMR {
    #[doc = "DCO frequency trim: 0"]
    DCOFTRIM_0,
    #[doc = "DCO frequency trim: 1"]
    DCOFTRIM_1,
    #[doc = "DCO frequency trim: 2"]
    DCOFTRIM_2,
    #[doc = "DCO frequency trim: 3"]
    DCOFTRIM_3,
    #[doc = "DCO frequency trim: 4"]
    DCOFTRIM_4,
    #[doc = "DCO frequency trim: 5"]
    DCOFTRIM_5,
    #[doc = "DCO frequency trim: 6"]
    DCOFTRIM_6,
    #[doc = "DCO frequency trim: 7"]
    DCOFTRIM_7,
}
impl DCOFTRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCOFTRIMR::DCOFTRIM_0 => 0,
            DCOFTRIMR::DCOFTRIM_1 => 1,
            DCOFTRIMR::DCOFTRIM_2 => 2,
            DCOFTRIMR::DCOFTRIM_3 => 3,
            DCOFTRIMR::DCOFTRIM_4 => 4,
            DCOFTRIMR::DCOFTRIM_5 => 5,
            DCOFTRIMR::DCOFTRIM_6 => 6,
            DCOFTRIMR::DCOFTRIM_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCOFTRIMR {
        match value {
            0 => DCOFTRIMR::DCOFTRIM_0,
            1 => DCOFTRIMR::DCOFTRIM_1,
            2 => DCOFTRIMR::DCOFTRIM_2,
            3 => DCOFTRIMR::DCOFTRIM_3,
            4 => DCOFTRIMR::DCOFTRIM_4,
            5 => DCOFTRIMR::DCOFTRIM_5,
            6 => DCOFTRIMR::DCOFTRIM_6,
            7 => DCOFTRIMR::DCOFTRIM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_0`"]
    #[inline]
    pub fn is_dcoftrim_0(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_0
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_1`"]
    #[inline]
    pub fn is_dcoftrim_1(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_1
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_2`"]
    #[inline]
    pub fn is_dcoftrim_2(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_2
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_3`"]
    #[inline]
    pub fn is_dcoftrim_3(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_3
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_4`"]
    #[inline]
    pub fn is_dcoftrim_4(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_4
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_5`"]
    #[inline]
    pub fn is_dcoftrim_5(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_5
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_6`"]
    #[inline]
    pub fn is_dcoftrim_6(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_6
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_7`"]
    #[inline]
    pub fn is_dcoftrim_7(&self) -> bool {
        *self == DCOFTRIMR::DCOFTRIM_7
    }
}
#[doc = "Possible values of the field `DCOFTRIMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFTRIMENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DCOFTRIMENR {
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
            DCOFTRIMENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOFTRIMENR {
        match value {
            i => DCOFTRIMENR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `DISMOD`"]
pub enum DISMODW {}
impl DISMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DISMODW<'a> {
    w: &'a mut W,
}
impl<'a> _DISMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISMODW) -> &'a mut W {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCORSEL`"]
pub enum DCORSELW {
    #[doc = "DCO frequency range select: 0"]
    DCORSEL_0,
    #[doc = "DCO frequency range select: 1"]
    DCORSEL_1,
    #[doc = "DCO frequency range select: 2"]
    DCORSEL_2,
    #[doc = "DCO frequency range select: 3"]
    DCORSEL_3,
    #[doc = "DCO frequency range select: 4"]
    DCORSEL_4,
    #[doc = "DCO frequency range select: 5"]
    DCORSEL_5,
    #[doc = "DCO frequency range select: 6"]
    DCORSEL_6,
    #[doc = "DCO frequency range select: 7"]
    DCORSEL_7,
}
impl DCORSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCORSELW::DCORSEL_0 => 0,
            DCORSELW::DCORSEL_1 => 1,
            DCORSELW::DCORSEL_2 => 2,
            DCORSELW::DCORSEL_3 => 3,
            DCORSELW::DCORSEL_4 => 4,
            DCORSELW::DCORSEL_5 => 5,
            DCORSELW::DCORSEL_6 => 6,
            DCORSELW::DCORSEL_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCORSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DCORSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCORSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DCO frequency range select: 0"]
    #[inline]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_0)
    }
    #[doc = "DCO frequency range select: 1"]
    #[inline]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_1)
    }
    #[doc = "DCO frequency range select: 2"]
    #[inline]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_2)
    }
    #[doc = "DCO frequency range select: 3"]
    #[inline]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_3)
    }
    #[doc = "DCO frequency range select: 4"]
    #[inline]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_4)
    }
    #[doc = "DCO frequency range select: 5"]
    #[inline]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_5)
    }
    #[doc = "DCO frequency range select: 6"]
    #[inline]
    pub fn dcorsel_6(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_6)
    }
    #[doc = "DCO frequency range select: 7"]
    #[inline]
    pub fn dcorsel_7(self) -> &'a mut W {
        self.variant(DCORSELW::DCORSEL_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCOFTRIM`"]
pub enum DCOFTRIMW {
    #[doc = "DCO frequency trim: 0"]
    DCOFTRIM_0,
    #[doc = "DCO frequency trim: 1"]
    DCOFTRIM_1,
    #[doc = "DCO frequency trim: 2"]
    DCOFTRIM_2,
    #[doc = "DCO frequency trim: 3"]
    DCOFTRIM_3,
    #[doc = "DCO frequency trim: 4"]
    DCOFTRIM_4,
    #[doc = "DCO frequency trim: 5"]
    DCOFTRIM_5,
    #[doc = "DCO frequency trim: 6"]
    DCOFTRIM_6,
    #[doc = "DCO frequency trim: 7"]
    DCOFTRIM_7,
}
impl DCOFTRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCOFTRIMW::DCOFTRIM_0 => 0,
            DCOFTRIMW::DCOFTRIM_1 => 1,
            DCOFTRIMW::DCOFTRIM_2 => 2,
            DCOFTRIMW::DCOFTRIM_3 => 3,
            DCOFTRIMW::DCOFTRIM_4 => 4,
            DCOFTRIMW::DCOFTRIM_5 => 5,
            DCOFTRIMW::DCOFTRIM_6 => 6,
            DCOFTRIMW::DCOFTRIM_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCOFTRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOFTRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOFTRIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DCO frequency trim: 0"]
    #[inline]
    pub fn dcoftrim_0(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_0)
    }
    #[doc = "DCO frequency trim: 1"]
    #[inline]
    pub fn dcoftrim_1(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_1)
    }
    #[doc = "DCO frequency trim: 2"]
    #[inline]
    pub fn dcoftrim_2(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_2)
    }
    #[doc = "DCO frequency trim: 3"]
    #[inline]
    pub fn dcoftrim_3(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_3)
    }
    #[doc = "DCO frequency trim: 4"]
    #[inline]
    pub fn dcoftrim_4(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_4)
    }
    #[doc = "DCO frequency trim: 5"]
    #[inline]
    pub fn dcoftrim_5(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_5)
    }
    #[doc = "DCO frequency trim: 6"]
    #[inline]
    pub fn dcoftrim_6(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_6)
    }
    #[doc = "DCO frequency trim: 7"]
    #[inline]
    pub fn dcoftrim_7(self) -> &'a mut W {
        self.variant(DCOFTRIMW::DCOFTRIM_7)
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
#[doc = "Values that can be written to the field `DCOFTRIMEN`"]
pub enum DCOFTRIMENW {}
impl DCOFTRIMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DCOFTRIMENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOFTRIMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOFTRIMENW) -> &'a mut W {
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline]
    pub fn dismod(&self) -> DISMODR {
        DISMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline]
    pub fn dcorsel(&self) -> DCORSELR {
        DCORSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline]
    pub fn dcoftrim(&self) -> DCOFTRIMR {
        DCOFTRIMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline]
    pub fn dcoftrimen(&self) -> DCOFTRIMENR {
        DCOFTRIMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline]
    pub fn dismod(&mut self) -> _DISMODW {
        _DISMODW { w: self }
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline]
    pub fn dcorsel(&mut self) -> _DCORSELW {
        _DCORSELW { w: self }
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline]
    pub fn dcoftrim(&mut self) -> _DCOFTRIMW {
        _DCOFTRIMW { w: self }
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline]
    pub fn dcoftrimen(&mut self) -> _DCOFTRIMENW {
        _DCOFTRIMENW { w: self }
    }
}
