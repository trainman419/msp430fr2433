#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL5 {
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
#[doc = "Possible values of the field `DIVM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVMR {
    #[doc = "MCLK Source Divider 0"]
    DIVM_0,
    #[doc = "MCLK Source Divider 1"]
    DIVM_1,
    #[doc = "MCLK Source Divider 2"]
    DIVM_2,
    #[doc = "MCLK Source Divider 3"]
    DIVM_3,
    #[doc = "MCLK Source Divider 4"]
    DIVM_4,
    #[doc = "MCLK Source Divider 5"]
    DIVM_5,
    #[doc = "MCLK Source Divider 6"]
    DIVM_6,
    #[doc = "MCLK Source Divider 7"]
    DIVM_7,
}
impl DIVMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVMR::DIVM_0 => 0,
            DIVMR::DIVM_1 => 1,
            DIVMR::DIVM_2 => 2,
            DIVMR::DIVM_3 => 3,
            DIVMR::DIVM_4 => 4,
            DIVMR::DIVM_5 => 5,
            DIVMR::DIVM_6 => 6,
            DIVMR::DIVM_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVMR {
        match value {
            0 => DIVMR::DIVM_0,
            1 => DIVMR::DIVM_1,
            2 => DIVMR::DIVM_2,
            3 => DIVMR::DIVM_3,
            4 => DIVMR::DIVM_4,
            5 => DIVMR::DIVM_5,
            6 => DIVMR::DIVM_6,
            7 => DIVMR::DIVM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVM_0`"]
    #[inline]
    pub fn is_divm_0(&self) -> bool {
        *self == DIVMR::DIVM_0
    }
    #[doc = "Checks if the value of the field is `DIVM_1`"]
    #[inline]
    pub fn is_divm_1(&self) -> bool {
        *self == DIVMR::DIVM_1
    }
    #[doc = "Checks if the value of the field is `DIVM_2`"]
    #[inline]
    pub fn is_divm_2(&self) -> bool {
        *self == DIVMR::DIVM_2
    }
    #[doc = "Checks if the value of the field is `DIVM_3`"]
    #[inline]
    pub fn is_divm_3(&self) -> bool {
        *self == DIVMR::DIVM_3
    }
    #[doc = "Checks if the value of the field is `DIVM_4`"]
    #[inline]
    pub fn is_divm_4(&self) -> bool {
        *self == DIVMR::DIVM_4
    }
    #[doc = "Checks if the value of the field is `DIVM_5`"]
    #[inline]
    pub fn is_divm_5(&self) -> bool {
        *self == DIVMR::DIVM_5
    }
    #[doc = "Checks if the value of the field is `DIVM_6`"]
    #[inline]
    pub fn is_divm_6(&self) -> bool {
        *self == DIVMR::DIVM_6
    }
    #[doc = "Checks if the value of the field is `DIVM_7`"]
    #[inline]
    pub fn is_divm_7(&self) -> bool {
        *self == DIVMR::DIVM_7
    }
}
#[doc = "Possible values of the field `DIVS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSR {
    #[doc = "SMCLK Source Divider 0"]
    DIVS_0,
    #[doc = "SMCLK Source Divider 1"]
    DIVS_1,
    #[doc = "SMCLK Source Divider 2"]
    DIVS_2,
    #[doc = "SMCLK Source Divider 3"]
    DIVS_3,
}
impl DIVSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVSR::DIVS_0 => 0,
            DIVSR::DIVS_1 => 1,
            DIVSR::DIVS_2 => 2,
            DIVSR::DIVS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVSR {
        match value {
            0 => DIVSR::DIVS_0,
            1 => DIVSR::DIVS_1,
            2 => DIVSR::DIVS_2,
            3 => DIVSR::DIVS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVS_0`"]
    #[inline]
    pub fn is_divs_0(&self) -> bool {
        *self == DIVSR::DIVS_0
    }
    #[doc = "Checks if the value of the field is `DIVS_1`"]
    #[inline]
    pub fn is_divs_1(&self) -> bool {
        *self == DIVSR::DIVS_1
    }
    #[doc = "Checks if the value of the field is `DIVS_2`"]
    #[inline]
    pub fn is_divs_2(&self) -> bool {
        *self == DIVSR::DIVS_2
    }
    #[doc = "Checks if the value of the field is `DIVS_3`"]
    #[inline]
    pub fn is_divs_3(&self) -> bool {
        *self == DIVSR::DIVS_3
    }
}
#[doc = "Possible values of the field `SMCLKOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLKOFFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SMCLKOFFR {
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
            SMCLKOFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMCLKOFFR {
        match value {
            i => SMCLKOFFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `VLOAUTOOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLOAUTOOFFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VLOAUTOOFFR {
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
            VLOAUTOOFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLOAUTOOFFR {
        match value {
            i => VLOAUTOOFFR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `DIVM`"]
pub enum DIVMW {
    #[doc = "MCLK Source Divider 0"]
    DIVM_0,
    #[doc = "MCLK Source Divider 1"]
    DIVM_1,
    #[doc = "MCLK Source Divider 2"]
    DIVM_2,
    #[doc = "MCLK Source Divider 3"]
    DIVM_3,
    #[doc = "MCLK Source Divider 4"]
    DIVM_4,
    #[doc = "MCLK Source Divider 5"]
    DIVM_5,
    #[doc = "MCLK Source Divider 6"]
    DIVM_6,
    #[doc = "MCLK Source Divider 7"]
    DIVM_7,
}
impl DIVMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVMW::DIVM_0 => 0,
            DIVMW::DIVM_1 => 1,
            DIVMW::DIVM_2 => 2,
            DIVMW::DIVM_3 => 3,
            DIVMW::DIVM_4 => 4,
            DIVMW::DIVM_5 => 5,
            DIVMW::DIVM_6 => 6,
            DIVMW::DIVM_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVMW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MCLK Source Divider 0"]
    #[inline]
    pub fn divm_0(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_0)
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline]
    pub fn divm_1(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_1)
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline]
    pub fn divm_2(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_2)
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline]
    pub fn divm_3(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_3)
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline]
    pub fn divm_4(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_4)
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline]
    pub fn divm_5(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_5)
    }
    #[doc = "MCLK Source Divider 6"]
    #[inline]
    pub fn divm_6(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_6)
    }
    #[doc = "MCLK Source Divider 7"]
    #[inline]
    pub fn divm_7(self) -> &'a mut W {
        self.variant(DIVMW::DIVM_7)
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
#[doc = "Values that can be written to the field `DIVS`"]
pub enum DIVSW {
    #[doc = "SMCLK Source Divider 0"]
    DIVS_0,
    #[doc = "SMCLK Source Divider 1"]
    DIVS_1,
    #[doc = "SMCLK Source Divider 2"]
    DIVS_2,
    #[doc = "SMCLK Source Divider 3"]
    DIVS_3,
}
impl DIVSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVSW::DIVS_0 => 0,
            DIVSW::DIVS_1 => 1,
            DIVSW::DIVS_2 => 2,
            DIVSW::DIVS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVSW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SMCLK Source Divider 0"]
    #[inline]
    pub fn divs_0(self) -> &'a mut W {
        self.variant(DIVSW::DIVS_0)
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline]
    pub fn divs_1(self) -> &'a mut W {
        self.variant(DIVSW::DIVS_1)
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline]
    pub fn divs_2(self) -> &'a mut W {
        self.variant(DIVSW::DIVS_2)
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline]
    pub fn divs_3(self) -> &'a mut W {
        self.variant(DIVSW::DIVS_3)
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
#[doc = "Values that can be written to the field `SMCLKOFF`"]
pub enum SMCLKOFFW {}
impl SMCLKOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SMCLKOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _SMCLKOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMCLKOFFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `VLOAUTOOFF`"]
pub enum VLOAUTOOFFW {}
impl VLOAUTOOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _VLOAUTOOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _VLOAUTOOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLOAUTOOFFW) -> &'a mut W {
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline]
    pub fn divm(&self) -> DIVMR {
        DIVMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline]
    pub fn divs(&self) -> DIVSR {
        DIVSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline]
    pub fn smclkoff(&self) -> SMCLKOFFR {
        SMCLKOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline]
    pub fn vloautooff(&self) -> VLOAUTOOFFR {
        VLOAUTOOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline]
    pub fn divm(&mut self) -> _DIVMW {
        _DIVMW { w: self }
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline]
    pub fn divs(&mut self) -> _DIVSW {
        _DIVSW { w: self }
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline]
    pub fn smclkoff(&mut self) -> _SMCLKOFFW {
        _SMCLKOFFW { w: self }
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline]
    pub fn vloautooff(&mut self) -> _VLOAUTOOFFW {
        _VLOAUTOOFFW { w: self }
    }
}
