#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::UCB0IFG {
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
#[doc = "Possible values of the field `UCRXIFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIFG0R {
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
            UCRXIFG0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIFG0R {
        match value {
            i => UCRXIFG0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIFG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIFG0R {
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
            UCTXIFG0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIFG0R {
        match value {
            i => UCTXIFG0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCSTTIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTTIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCSTTIFGR {
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
            UCSTTIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCSTTIFGR {
        match value {
            i => UCSTTIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCSTPIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTPIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCSTPIFGR {
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
            UCSTPIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCSTPIFGR {
        match value {
            i => UCSTPIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCALIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCALIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCALIFGR {
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
            UCALIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCALIFGR {
        match value {
            i => UCALIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCNACKIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCNACKIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCNACKIFGR {
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
            UCNACKIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCNACKIFGR {
        match value {
            i => UCNACKIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNTIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNTIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNTIFGR {
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
            UCBCNTIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNTIFGR {
        match value {
            i => UCBCNTIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCCLTOIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCLTOIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCCLTOIFGR {
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
            UCCLTOIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCCLTOIFGR {
        match value {
            i => UCCLTOIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCRXIFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIFG1R {
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
            UCRXIFG1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIFG1R {
        match value {
            i => UCRXIFG1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIFG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIFG1R {
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
            UCTXIFG1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIFG1R {
        match value {
            i => UCTXIFG1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCRXIFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIFG2R {
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
            UCRXIFG2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIFG2R {
        match value {
            i => UCRXIFG2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIFG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIFG2R {
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
            UCTXIFG2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIFG2R {
        match value {
            i => UCTXIFG2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCRXIFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIFG3R {
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
            UCRXIFG3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIFG3R {
        match value {
            i => UCRXIFG3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIFG3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIFG3R {
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
            UCTXIFG3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIFG3R {
        match value {
            i => UCTXIFG3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBIT9IFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBIT9IFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBIT9IFGR {
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
            UCBIT9IFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBIT9IFGR {
        match value {
            i => UCBIT9IFGR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `UCRXIFG0`"]
pub enum UCRXIFG0W {}
impl UCRXIFG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIFG0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIFG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIFG0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIFG0`"]
pub enum UCTXIFG0W {}
impl UCTXIFG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIFG0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIFG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIFG0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCSTTIFG`"]
pub enum UCSTTIFGW {}
impl UCSTTIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCSTTIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCSTTIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCSTTIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCSTPIFG`"]
pub enum UCSTPIFGW {}
impl UCSTPIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCSTPIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCSTPIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCSTPIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCALIFG`"]
pub enum UCALIFGW {}
impl UCALIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCALIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCALIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCALIFGW) -> &'a mut W {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCNACKIFG`"]
pub enum UCNACKIFGW {}
impl UCNACKIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCNACKIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCNACKIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCNACKIFGW) -> &'a mut W {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNTIFG`"]
pub enum UCBCNTIFGW {}
impl UCBCNTIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNTIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNTIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNTIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCCLTOIFG`"]
pub enum UCCLTOIFGW {}
impl UCCLTOIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCCLTOIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCCLTOIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCCLTOIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCRXIFG1`"]
pub enum UCRXIFG1W {}
impl UCRXIFG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIFG1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIFG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIFG1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIFG1`"]
pub enum UCTXIFG1W {}
impl UCTXIFG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIFG1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIFG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIFG1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCRXIFG2`"]
pub enum UCRXIFG2W {}
impl UCRXIFG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIFG2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIFG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIFG2W) -> &'a mut W {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCTXIFG2`"]
pub enum UCTXIFG2W {}
impl UCTXIFG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIFG2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIFG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIFG2W) -> &'a mut W {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCRXIFG3`"]
pub enum UCRXIFG3W {}
impl UCRXIFG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIFG3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIFG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIFG3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIFG3`"]
pub enum UCTXIFG3W {}
impl UCTXIFG3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIFG3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIFG3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIFG3W) -> &'a mut W {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBIT9IFG`"]
pub enum UCBIT9IFGW {}
impl UCBIT9IFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBIT9IFGW<'a> {
    w: &'a mut W,
}
impl<'a> _UCBIT9IFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBIT9IFGW) -> &'a mut W {
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline]
    pub fn ucrxifg0(&self) -> UCRXIFG0R {
        UCRXIFG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline]
    pub fn uctxifg0(&self) -> UCTXIFG0R {
        UCTXIFG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline]
    pub fn ucsttifg(&self) -> UCSTTIFGR {
        UCSTTIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline]
    pub fn ucstpifg(&self) -> UCSTPIFGR {
        UCSTPIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline]
    pub fn ucalifg(&self) -> UCALIFGR {
        UCALIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline]
    pub fn ucnackifg(&self) -> UCNACKIFGR {
        UCNACKIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline]
    pub fn ucbcntifg(&self) -> UCBCNTIFGR {
        UCBCNTIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline]
    pub fn uccltoifg(&self) -> UCCLTOIFGR {
        UCCLTOIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline]
    pub fn ucrxifg1(&self) -> UCRXIFG1R {
        UCRXIFG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline]
    pub fn uctxifg1(&self) -> UCTXIFG1R {
        UCTXIFG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline]
    pub fn ucrxifg2(&self) -> UCRXIFG2R {
        UCRXIFG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline]
    pub fn uctxifg2(&self) -> UCTXIFG2R {
        UCTXIFG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline]
    pub fn ucrxifg3(&self) -> UCRXIFG3R {
        UCRXIFG3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline]
    pub fn uctxifg3(&self) -> UCTXIFG3R {
        UCTXIFG3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline]
    pub fn ucbit9ifg(&self) -> UCBIT9IFGR {
        UCBIT9IFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - I2C Receive Interrupt Flag 0"]
    #[inline]
    pub fn ucrxifg0(&mut self) -> _UCRXIFG0W {
        _UCRXIFG0W { w: self }
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Flag 0"]
    #[inline]
    pub fn uctxifg0(&mut self) -> _UCTXIFG0W {
        _UCTXIFG0W { w: self }
    }
    #[doc = "Bit 2 - I2C START Condition interrupt Flag"]
    #[inline]
    pub fn ucsttifg(&mut self) -> _UCSTTIFGW {
        _UCSTTIFGW { w: self }
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt Flag"]
    #[inline]
    pub fn ucstpifg(&mut self) -> _UCSTPIFGW {
        _UCSTPIFGW { w: self }
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt Flag"]
    #[inline]
    pub fn ucalifg(&mut self) -> _UCALIFGW {
        _UCALIFGW { w: self }
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt Flag"]
    #[inline]
    pub fn ucnackifg(&mut self) -> _UCNACKIFGW {
        _UCNACKIFGW { w: self }
    }
    #[doc = "Bit 6 - I2C Byte counter interrupt flag"]
    #[inline]
    pub fn ucbcntifg(&mut self) -> _UCBCNTIFGW {
        _UCBCNTIFGW { w: self }
    }
    #[doc = "Bit 7 - I2C Clock low Timeout interrupt Flag"]
    #[inline]
    pub fn uccltoifg(&mut self) -> _UCCLTOIFGW {
        _UCCLTOIFGW { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Flag 1"]
    #[inline]
    pub fn ucrxifg1(&mut self) -> _UCRXIFG1W {
        _UCRXIFG1W { w: self }
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Flag 1"]
    #[inline]
    pub fn uctxifg1(&mut self) -> _UCTXIFG1W {
        _UCTXIFG1W { w: self }
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Flag 2"]
    #[inline]
    pub fn ucrxifg2(&mut self) -> _UCRXIFG2W {
        _UCRXIFG2W { w: self }
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Flag 2"]
    #[inline]
    pub fn uctxifg2(&mut self) -> _UCTXIFG2W {
        _UCTXIFG2W { w: self }
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Flag 3"]
    #[inline]
    pub fn ucrxifg3(&mut self) -> _UCRXIFG3W {
        _UCRXIFG3W { w: self }
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Flag 3"]
    #[inline]
    pub fn uctxifg3(&mut self) -> _UCTXIFG3W {
        _UCTXIFG3W { w: self }
    }
    #[doc = "Bit 14 - I2C Bit 9 Possition Interrupt Flag 3"]
    #[inline]
    pub fn ucbit9ifg(&mut self) -> _UCBIT9IFGW {
        _UCBIT9IFGW { w: self }
    }
}
