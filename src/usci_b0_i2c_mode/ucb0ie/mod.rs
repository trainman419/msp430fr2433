#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::UCB0IE {
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
#[doc = "Possible values of the field `UCRXIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIE0R {
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
            UCRXIE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIE0R {
        match value {
            i => UCRXIE0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIE0R {
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
            UCTXIE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIE0R {
        match value {
            i => UCTXIE0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCSTTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTTIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCSTTIER {
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
            UCSTTIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCSTTIER {
        match value {
            i => UCSTTIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCSTPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTPIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCSTPIER {
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
            UCSTPIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCSTPIER {
        match value {
            i => UCSTPIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCALIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCALIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCALIER {
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
            UCALIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCALIER {
        match value {
            i => UCALIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCNACKIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCNACKIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCNACKIER {
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
            UCNACKIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCNACKIER {
        match value {
            i => UCNACKIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNTIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNTIER {
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
            UCBCNTIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNTIER {
        match value {
            i => UCBCNTIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCCLTOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCLTOIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCCLTOIER {
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
            UCCLTOIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCCLTOIER {
        match value {
            i => UCCLTOIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCRXIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIE1R {
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
            UCRXIE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIE1R {
        match value {
            i => UCRXIE1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIE1R {
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
            UCTXIE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIE1R {
        match value {
            i => UCTXIE1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCRXIE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIE2R {
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
            UCRXIE2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIE2R {
        match value {
            i => UCRXIE2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIE2R {
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
            UCTXIE2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIE2R {
        match value {
            i => UCTXIE2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCRXIE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCRXIE3R {
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
            UCRXIE3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCRXIE3R {
        match value {
            i => UCRXIE3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCTXIE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCTXIE3R {
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
            UCTXIE3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCTXIE3R {
        match value {
            i => UCTXIE3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBIT9IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBIT9IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBIT9IER {
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
            UCBIT9IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBIT9IER {
        match value {
            i => UCBIT9IER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `UCRXIE0`"]
pub enum UCRXIE0W {}
impl UCRXIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIE0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIE0`"]
pub enum UCTXIE0W {}
impl UCTXIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIE0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCSTTIE`"]
pub enum UCSTTIEW {}
impl UCSTTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCSTTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCSTTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCSTTIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCSTPIE`"]
pub enum UCSTPIEW {}
impl UCSTPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCSTPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCSTPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCSTPIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCALIE`"]
pub enum UCALIEW {}
impl UCALIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCALIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCALIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCALIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCNACKIE`"]
pub enum UCNACKIEW {}
impl UCNACKIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCNACKIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCNACKIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCNACKIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBCNTIE`"]
pub enum UCBCNTIEW {}
impl UCBCNTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNTIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCCLTOIE`"]
pub enum UCCLTOIEW {}
impl UCCLTOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCCLTOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCCLTOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCCLTOIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCRXIE1`"]
pub enum UCRXIE1W {}
impl UCRXIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIE1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIE1`"]
pub enum UCTXIE1W {}
impl UCTXIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIE1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCRXIE2`"]
pub enum UCRXIE2W {}
impl UCRXIE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIE2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIE2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIE2`"]
pub enum UCTXIE2W {}
impl UCTXIE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIE2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIE2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCRXIE3`"]
pub enum UCRXIE3W {}
impl UCRXIE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCRXIE3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCRXIE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCRXIE3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCTXIE3`"]
pub enum UCTXIE3W {}
impl UCTXIE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCTXIE3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCTXIE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCTXIE3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBIT9IE`"]
pub enum UCBIT9IEW {}
impl UCBIT9IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBIT9IEW<'a> {
    w: &'a mut W,
}
impl<'a> _UCBIT9IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBIT9IEW) -> &'a mut W {
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
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline]
    pub fn ucrxie0(&self) -> UCRXIE0R {
        UCRXIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline]
    pub fn uctxie0(&self) -> UCTXIE0R {
        UCTXIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline]
    pub fn ucsttie(&self) -> UCSTTIER {
        UCSTTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline]
    pub fn ucstpie(&self) -> UCSTPIER {
        UCSTPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline]
    pub fn ucalie(&self) -> UCALIER {
        UCALIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline]
    pub fn ucnackie(&self) -> UCNACKIER {
        UCNACKIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline]
    pub fn ucbcntie(&self) -> UCBCNTIER {
        UCBCNTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline]
    pub fn uccltoie(&self) -> UCCLTOIER {
        UCCLTOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline]
    pub fn ucrxie1(&self) -> UCRXIE1R {
        UCRXIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline]
    pub fn uctxie1(&self) -> UCTXIE1R {
        UCTXIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline]
    pub fn ucrxie2(&self) -> UCRXIE2R {
        UCRXIE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline]
    pub fn uctxie2(&self) -> UCTXIE2R {
        UCTXIE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline]
    pub fn ucrxie3(&self) -> UCRXIE3R {
        UCRXIE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline]
    pub fn uctxie3(&self) -> UCTXIE3R {
        UCTXIE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline]
    pub fn ucbit9ie(&self) -> UCBIT9IER {
        UCBIT9IER::_from({
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
    #[doc = "Bit 0 - I2C Receive Interrupt Enable 0"]
    #[inline]
    pub fn ucrxie0(&mut self) -> _UCRXIE0W {
        _UCRXIE0W { w: self }
    }
    #[doc = "Bit 1 - I2C Transmit Interrupt Enable 0"]
    #[inline]
    pub fn uctxie0(&mut self) -> _UCTXIE0W {
        _UCTXIE0W { w: self }
    }
    #[doc = "Bit 2 - I2C START Condition interrupt enable"]
    #[inline]
    pub fn ucsttie(&mut self) -> _UCSTTIEW {
        _UCSTTIEW { w: self }
    }
    #[doc = "Bit 3 - I2C STOP Condition interrupt enable"]
    #[inline]
    pub fn ucstpie(&mut self) -> _UCSTPIEW {
        _UCSTPIEW { w: self }
    }
    #[doc = "Bit 4 - I2C Arbitration Lost interrupt enable"]
    #[inline]
    pub fn ucalie(&mut self) -> _UCALIEW {
        _UCALIEW { w: self }
    }
    #[doc = "Bit 5 - I2C NACK Condition interrupt enable"]
    #[inline]
    pub fn ucnackie(&mut self) -> _UCNACKIEW {
        _UCNACKIEW { w: self }
    }
    #[doc = "Bit 6 - I2C Automatic stop assertion interrupt enable"]
    #[inline]
    pub fn ucbcntie(&mut self) -> _UCBCNTIEW {
        _UCBCNTIEW { w: self }
    }
    #[doc = "Bit 7 - I2C Clock Low Timeout interrupt enable"]
    #[inline]
    pub fn uccltoie(&mut self) -> _UCCLTOIEW {
        _UCCLTOIEW { w: self }
    }
    #[doc = "Bit 8 - I2C Receive Interrupt Enable 1"]
    #[inline]
    pub fn ucrxie1(&mut self) -> _UCRXIE1W {
        _UCRXIE1W { w: self }
    }
    #[doc = "Bit 9 - I2C Transmit Interrupt Enable 1"]
    #[inline]
    pub fn uctxie1(&mut self) -> _UCTXIE1W {
        _UCTXIE1W { w: self }
    }
    #[doc = "Bit 10 - I2C Receive Interrupt Enable 2"]
    #[inline]
    pub fn ucrxie2(&mut self) -> _UCRXIE2W {
        _UCRXIE2W { w: self }
    }
    #[doc = "Bit 11 - I2C Transmit Interrupt Enable 2"]
    #[inline]
    pub fn uctxie2(&mut self) -> _UCTXIE2W {
        _UCTXIE2W { w: self }
    }
    #[doc = "Bit 12 - I2C Receive Interrupt Enable 3"]
    #[inline]
    pub fn ucrxie3(&mut self) -> _UCRXIE3W {
        _UCRXIE3W { w: self }
    }
    #[doc = "Bit 13 - I2C Transmit Interrupt Enable 3"]
    #[inline]
    pub fn uctxie3(&mut self) -> _UCTXIE3W {
        _UCTXIE3W { w: self }
    }
    #[doc = "Bit 14 - I2C Bit 9 Position Interrupt Enable 3"]
    #[inline]
    pub fn ucbit9ie(&mut self) -> _UCBIT9IEW {
        _UCBIT9IEW { w: self }
    }
}
