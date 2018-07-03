#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::RTCCTL {
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
#[doc = "Possible values of the field `RTCIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCIFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTCIFR {
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
            RTCIFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCIFR {
        match value {
            i => RTCIFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `RTCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTCIER {
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
            RTCIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCIER {
        match value {
            i => RTCIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `RTCSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTCSRR {
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
            RTCSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCSRR {
        match value {
            i => RTCSRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `RTCPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCPSR {
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    RTCPS_0,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    RTCPS_1,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    RTCPS_2,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    RTCPS_3,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    RTCPS_4,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    RTCPS_5,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    RTCPS_6,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    RTCPS_7,
}
impl RTCPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCPSR::RTCPS_0 => 0,
            RTCPSR::RTCPS_1 => 1,
            RTCPSR::RTCPS_2 => 2,
            RTCPSR::RTCPS_3 => 3,
            RTCPSR::RTCPS_4 => 4,
            RTCPSR::RTCPS_5 => 5,
            RTCPSR::RTCPS_6 => 6,
            RTCPSR::RTCPS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCPSR {
        match value {
            0 => RTCPSR::RTCPS_0,
            1 => RTCPSR::RTCPS_1,
            2 => RTCPSR::RTCPS_2,
            3 => RTCPSR::RTCPS_3,
            4 => RTCPSR::RTCPS_4,
            5 => RTCPSR::RTCPS_5,
            6 => RTCPSR::RTCPS_6,
            7 => RTCPSR::RTCPS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCPS_0`"]
    #[inline]
    pub fn is_rtcps_0(&self) -> bool {
        *self == RTCPSR::RTCPS_0
    }
    #[doc = "Checks if the value of the field is `RTCPS_1`"]
    #[inline]
    pub fn is_rtcps_1(&self) -> bool {
        *self == RTCPSR::RTCPS_1
    }
    #[doc = "Checks if the value of the field is `RTCPS_2`"]
    #[inline]
    pub fn is_rtcps_2(&self) -> bool {
        *self == RTCPSR::RTCPS_2
    }
    #[doc = "Checks if the value of the field is `RTCPS_3`"]
    #[inline]
    pub fn is_rtcps_3(&self) -> bool {
        *self == RTCPSR::RTCPS_3
    }
    #[doc = "Checks if the value of the field is `RTCPS_4`"]
    #[inline]
    pub fn is_rtcps_4(&self) -> bool {
        *self == RTCPSR::RTCPS_4
    }
    #[doc = "Checks if the value of the field is `RTCPS_5`"]
    #[inline]
    pub fn is_rtcps_5(&self) -> bool {
        *self == RTCPSR::RTCPS_5
    }
    #[doc = "Checks if the value of the field is `RTCPS_6`"]
    #[inline]
    pub fn is_rtcps_6(&self) -> bool {
        *self == RTCPSR::RTCPS_6
    }
    #[doc = "Checks if the value of the field is `RTCPS_7`"]
    #[inline]
    pub fn is_rtcps_7(&self) -> bool {
        *self == RTCPSR::RTCPS_7
    }
}
#[doc = "Possible values of the field `RTCSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCSSR {
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    RTCSS_0,
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    RTCSS_1,
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    RTCSS_2,
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    RTCSS_3,
}
impl RTCSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCSSR::RTCSS_0 => 0,
            RTCSSR::RTCSS_1 => 1,
            RTCSSR::RTCSS_2 => 2,
            RTCSSR::RTCSS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCSSR {
        match value {
            0 => RTCSSR::RTCSS_0,
            1 => RTCSSR::RTCSS_1,
            2 => RTCSSR::RTCSS_2,
            3 => RTCSSR::RTCSS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCSS_0`"]
    #[inline]
    pub fn is_rtcss_0(&self) -> bool {
        *self == RTCSSR::RTCSS_0
    }
    #[doc = "Checks if the value of the field is `RTCSS_1`"]
    #[inline]
    pub fn is_rtcss_1(&self) -> bool {
        *self == RTCSSR::RTCSS_1
    }
    #[doc = "Checks if the value of the field is `RTCSS_2`"]
    #[inline]
    pub fn is_rtcss_2(&self) -> bool {
        *self == RTCSSR::RTCSS_2
    }
    #[doc = "Checks if the value of the field is `RTCSS_3`"]
    #[inline]
    pub fn is_rtcss_3(&self) -> bool {
        *self == RTCSSR::RTCSS_3
    }
}
#[doc = "Values that can be written to the field `RTCIF`"]
pub enum RTCIFW {}
impl RTCIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTCIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCIFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `RTCIE`"]
pub enum RTCIEW {}
impl RTCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCIEW) -> &'a mut W {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCSR`"]
pub enum RTCSRW {}
impl RTCSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTCSRW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCSRW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCPS`"]
pub enum RTCPSW {
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    RTCPS_0,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    RTCPS_1,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    RTCPS_2,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    RTCPS_3,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    RTCPS_4,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    RTCPS_5,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    RTCPS_6,
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    RTCPS_7,
}
impl RTCPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCPSW::RTCPS_0 => 0,
            RTCPSW::RTCPS_1 => 1,
            RTCPSW::RTCPS_2 => 2,
            RTCPSW::RTCPS_3 => 3,
            RTCPSW::RTCPS_4 => 4,
            RTCPSW::RTCPS_5 => 5,
            RTCPSW::RTCPS_6 => 6,
            RTCPSW::RTCPS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCPSW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline]
    pub fn rtcps_0(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_0)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline]
    pub fn rtcps_1(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_1)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline]
    pub fn rtcps_2(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_2)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline]
    pub fn rtcps_3(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_3)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline]
    pub fn rtcps_4(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_4)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline]
    pub fn rtcps_5(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_5)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline]
    pub fn rtcps_6(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_6)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline]
    pub fn rtcps_7(self) -> &'a mut W {
        self.variant(RTCPSW::RTCPS_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCSS`"]
pub enum RTCSSW {
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    RTCSS_0,
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    RTCSS_1,
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    RTCSS_2,
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    RTCSS_3,
}
impl RTCSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCSSW::RTCSS_0 => 0,
            RTCSSW::RTCSS_1 => 1,
            RTCSSW::RTCSS_2 => 2,
            RTCSSW::RTCSS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCSSW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCSSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline]
    pub fn rtcss_0(self) -> &'a mut W {
        self.variant(RTCSSW::RTCSS_0)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline]
    pub fn rtcss_1(self) -> &'a mut W {
        self.variant(RTCSSW::RTCSS_1)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline]
    pub fn rtcss_2(self) -> &'a mut W {
        self.variant(RTCSSW::RTCSS_2)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline]
    pub fn rtcss_3(self) -> &'a mut W {
        self.variant(RTCSSW::RTCSS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline]
    pub fn rtcif(&self) -> RTCIFR {
        RTCIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline]
    pub fn rtcie(&self) -> RTCIER {
        RTCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline]
    pub fn rtcsr(&self) -> RTCSRR {
        RTCSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline]
    pub fn rtcps(&self) -> RTCPSR {
        RTCPSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline]
    pub fn rtcss(&self) -> RTCSSR {
        RTCSSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline]
    pub fn rtcif(&mut self) -> _RTCIFW {
        _RTCIFW { w: self }
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline]
    pub fn rtcie(&mut self) -> _RTCIEW {
        _RTCIEW { w: self }
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline]
    pub fn rtcsr(&mut self) -> _RTCSRW {
        _RTCSRW { w: self }
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline]
    pub fn rtcps(&mut self) -> _RTCPSW {
        _RTCPSW { w: self }
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline]
    pub fn rtcss(&mut self) -> _RTCSSW {
        _RTCSSW { w: self }
    }
}
