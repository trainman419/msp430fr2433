#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::MPY32CTL0 {
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
#[doc = "Possible values of the field `MPYC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MPYCR {
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
            MPYCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPYCR {
        match value {
            i => MPYCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `MPYFRAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYFRACR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MPYFRACR {
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
            MPYFRACR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPYFRACR {
        match value {
            i => MPYFRACR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `MPYSAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYSATR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MPYSATR {
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
            MPYSATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPYSATR {
        match value {
            i => MPYSATR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `MPYM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYMR {
    #[doc = "Multiplier mode: MPY"]
    MPYM_0,
    #[doc = "Multiplier mode: MPYS"]
    MPYM_1,
    #[doc = "Multiplier mode: MAC"]
    MPYM_2,
    #[doc = "Multiplier mode: MACS"]
    MPYM_3,
}
impl MPYMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MPYMR::MPYM_0 => 0,
            MPYMR::MPYM_1 => 1,
            MPYMR::MPYM_2 => 2,
            MPYMR::MPYM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MPYMR {
        match value {
            0 => MPYMR::MPYM_0,
            1 => MPYMR::MPYM_1,
            2 => MPYMR::MPYM_2,
            3 => MPYMR::MPYM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MPYM_0`"]
    #[inline]
    pub fn is_mpym_0(&self) -> bool {
        *self == MPYMR::MPYM_0
    }
    #[doc = "Checks if the value of the field is `MPYM_1`"]
    #[inline]
    pub fn is_mpym_1(&self) -> bool {
        *self == MPYMR::MPYM_1
    }
    #[doc = "Checks if the value of the field is `MPYM_2`"]
    #[inline]
    pub fn is_mpym_2(&self) -> bool {
        *self == MPYMR::MPYM_2
    }
    #[doc = "Checks if the value of the field is `MPYM_3`"]
    #[inline]
    pub fn is_mpym_3(&self) -> bool {
        *self == MPYMR::MPYM_3
    }
}
#[doc = "Possible values of the field `OP1_32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP1_32R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OP1_32R {
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
            OP1_32R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OP1_32R {
        match value {
            i => OP1_32R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `OP2_32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP2_32R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OP2_32R {
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
            OP2_32R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OP2_32R {
        match value {
            i => OP2_32R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `MPYDLYWRTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYDLYWRTENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MPYDLYWRTENR {
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
            MPYDLYWRTENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPYDLYWRTENR {
        match value {
            i => MPYDLYWRTENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `MPYDLY32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPYDLY32R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MPYDLY32R {
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
            MPYDLY32R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPYDLY32R {
        match value {
            i => MPYDLY32R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `MPYC`"]
pub enum MPYCW {}
impl MPYCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MPYCW<'a> {
    w: &'a mut W,
}
impl<'a> _MPYCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPYCW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `MPYFRAC`"]
pub enum MPYFRACW {}
impl MPYFRACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MPYFRACW<'a> {
    w: &'a mut W,
}
impl<'a> _MPYFRACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPYFRACW) -> &'a mut W {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPYSAT`"]
pub enum MPYSATW {}
impl MPYSATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MPYSATW<'a> {
    w: &'a mut W,
}
impl<'a> _MPYSATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPYSATW) -> &'a mut W {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPYM`"]
pub enum MPYMW {
    #[doc = "Multiplier mode: MPY"]
    MPYM_0,
    #[doc = "Multiplier mode: MPYS"]
    MPYM_1,
    #[doc = "Multiplier mode: MAC"]
    MPYM_2,
    #[doc = "Multiplier mode: MACS"]
    MPYM_3,
}
impl MPYMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MPYMW::MPYM_0 => 0,
            MPYMW::MPYM_1 => 1,
            MPYMW::MPYM_2 => 2,
            MPYMW::MPYM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPYMW<'a> {
    w: &'a mut W,
}
impl<'a> _MPYMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPYMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Multiplier mode: MPY"]
    #[inline]
    pub fn mpym_0(self) -> &'a mut W {
        self.variant(MPYMW::MPYM_0)
    }
    #[doc = "Multiplier mode: MPYS"]
    #[inline]
    pub fn mpym_1(self) -> &'a mut W {
        self.variant(MPYMW::MPYM_1)
    }
    #[doc = "Multiplier mode: MAC"]
    #[inline]
    pub fn mpym_2(self) -> &'a mut W {
        self.variant(MPYMW::MPYM_2)
    }
    #[doc = "Multiplier mode: MACS"]
    #[inline]
    pub fn mpym_3(self) -> &'a mut W {
        self.variant(MPYMW::MPYM_3)
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
#[doc = "Values that can be written to the field `OP1_32`"]
pub enum OP1_32W {}
impl OP1_32W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OP1_32W<'a> {
    w: &'a mut W,
}
impl<'a> _OP1_32W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OP1_32W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `OP2_32`"]
pub enum OP2_32W {}
impl OP2_32W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OP2_32W<'a> {
    w: &'a mut W,
}
impl<'a> _OP2_32W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OP2_32W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `MPYDLYWRTEN`"]
pub enum MPYDLYWRTENW {}
impl MPYDLYWRTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MPYDLYWRTENW<'a> {
    w: &'a mut W,
}
impl<'a> _MPYDLYWRTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPYDLYWRTENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `MPYDLY32`"]
pub enum MPYDLY32W {}
impl MPYDLY32W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MPYDLY32W<'a> {
    w: &'a mut W,
}
impl<'a> _MPYDLY32W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPYDLY32W) -> &'a mut W {
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline]
    pub fn mpyc(&self) -> MPYCR {
        MPYCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline]
    pub fn mpyfrac(&self) -> MPYFRACR {
        MPYFRACR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline]
    pub fn mpysat(&self) -> MPYSATR {
        MPYSATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline]
    pub fn mpym(&self) -> MPYMR {
        MPYMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline]
    pub fn op1_32(&self) -> OP1_32R {
        OP1_32R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline]
    pub fn op2_32(&self) -> OP2_32R {
        OP2_32R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline]
    pub fn mpydlywrten(&self) -> MPYDLYWRTENR {
        MPYDLYWRTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline]
    pub fn mpydly32(&self) -> MPYDLY32R {
        MPYDLY32R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Carry of the multiplier"]
    #[inline]
    pub fn mpyc(&mut self) -> _MPYCW {
        _MPYCW { w: self }
    }
    #[doc = "Bit 2 - Fractional mode"]
    #[inline]
    pub fn mpyfrac(&mut self) -> _MPYFRACW {
        _MPYFRACW { w: self }
    }
    #[doc = "Bit 3 - Saturation mode"]
    #[inline]
    pub fn mpysat(&mut self) -> _MPYSATW {
        _MPYSATW { w: self }
    }
    #[doc = "Bits 4:5 - Multiplier mode Bit:0"]
    #[inline]
    pub fn mpym(&mut self) -> _MPYMW {
        _MPYMW { w: self }
    }
    #[doc = "Bit 6 - Bit-width of operand 1 0:16Bit / 1:32Bit"]
    #[inline]
    pub fn op1_32(&mut self) -> _OP1_32W {
        _OP1_32W { w: self }
    }
    #[doc = "Bit 7 - Bit-width of operand 2 0:16Bit / 1:32Bit"]
    #[inline]
    pub fn op2_32(&mut self) -> _OP2_32W {
        _OP2_32W { w: self }
    }
    #[doc = "Bit 8 - Delayed write enable"]
    #[inline]
    pub fn mpydlywrten(&mut self) -> _MPYDLYWRTENW {
        _MPYDLYWRTENW { w: self }
    }
    #[doc = "Bit 9 - Delayed write mode"]
    #[inline]
    pub fn mpydly32(&mut self) -> _MPYDLY32W {
        _MPYDLY32W { w: self }
    }
}
