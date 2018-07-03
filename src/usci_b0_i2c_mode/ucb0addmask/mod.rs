#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::UCB0ADDMASK {
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
#[doc = "Possible values of the field `UCADDMASK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK0R {
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
            UCADDMASK0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK0R {
        match value {
            i => UCADDMASK0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK1R {
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
            UCADDMASK1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK1R {
        match value {
            i => UCADDMASK1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK2R {
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
            UCADDMASK2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK2R {
        match value {
            i => UCADDMASK2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK3R {
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
            UCADDMASK3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK3R {
        match value {
            i => UCADDMASK3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK4R {
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
            UCADDMASK4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK4R {
        match value {
            i => UCADDMASK4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK5R {
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
            UCADDMASK5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK5R {
        match value {
            i => UCADDMASK5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK6R {
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
            UCADDMASK6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK6R {
        match value {
            i => UCADDMASK6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK7R {
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
            UCADDMASK7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK7R {
        match value {
            i => UCADDMASK7R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK8R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK8R {
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
            UCADDMASK8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK8R {
        match value {
            i => UCADDMASK8R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCADDMASK9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCADDMASK9R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCADDMASK9R {
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
            UCADDMASK9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCADDMASK9R {
        match value {
            i => UCADDMASK9R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `UCADDMASK0`"]
pub enum UCADDMASK0W {}
impl UCADDMASK0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK1`"]
pub enum UCADDMASK1W {}
impl UCADDMASK1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK2`"]
pub enum UCADDMASK2W {}
impl UCADDMASK2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK3`"]
pub enum UCADDMASK3W {}
impl UCADDMASK3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK4`"]
pub enum UCADDMASK4W {}
impl UCADDMASK4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK4W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK5`"]
pub enum UCADDMASK5W {}
impl UCADDMASK5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK5W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK6`"]
pub enum UCADDMASK6W {}
impl UCADDMASK6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK6W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK7`"]
pub enum UCADDMASK7W {}
impl UCADDMASK7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK7W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK7W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK8`"]
pub enum UCADDMASK8W {}
impl UCADDMASK8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK8W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK8W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCADDMASK9`"]
pub enum UCADDMASK9W {}
impl UCADDMASK9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCADDMASK9W<'a> {
    w: &'a mut W,
}
impl<'a> _UCADDMASK9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCADDMASK9W) -> &'a mut W {
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
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline]
    pub fn ucaddmask0(&self) -> UCADDMASK0R {
        UCADDMASK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline]
    pub fn ucaddmask1(&self) -> UCADDMASK1R {
        UCADDMASK1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline]
    pub fn ucaddmask2(&self) -> UCADDMASK2R {
        UCADDMASK2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline]
    pub fn ucaddmask3(&self) -> UCADDMASK3R {
        UCADDMASK3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline]
    pub fn ucaddmask4(&self) -> UCADDMASK4R {
        UCADDMASK4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline]
    pub fn ucaddmask5(&self) -> UCADDMASK5R {
        UCADDMASK5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline]
    pub fn ucaddmask6(&self) -> UCADDMASK6R {
        UCADDMASK6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline]
    pub fn ucaddmask7(&self) -> UCADDMASK7R {
        UCADDMASK7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline]
    pub fn ucaddmask8(&self) -> UCADDMASK8R {
        UCADDMASK8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline]
    pub fn ucaddmask9(&self) -> UCADDMASK9R {
        UCADDMASK9R::_from({
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
    #[doc = "Bit 0 - I2C Address Mask Bit 0"]
    #[inline]
    pub fn ucaddmask0(&mut self) -> _UCADDMASK0W {
        _UCADDMASK0W { w: self }
    }
    #[doc = "Bit 1 - I2C Address Mask Bit 1"]
    #[inline]
    pub fn ucaddmask1(&mut self) -> _UCADDMASK1W {
        _UCADDMASK1W { w: self }
    }
    #[doc = "Bit 2 - I2C Address Mask Bit 2"]
    #[inline]
    pub fn ucaddmask2(&mut self) -> _UCADDMASK2W {
        _UCADDMASK2W { w: self }
    }
    #[doc = "Bit 3 - I2C Address Mask Bit 3"]
    #[inline]
    pub fn ucaddmask3(&mut self) -> _UCADDMASK3W {
        _UCADDMASK3W { w: self }
    }
    #[doc = "Bit 4 - I2C Address Mask Bit 4"]
    #[inline]
    pub fn ucaddmask4(&mut self) -> _UCADDMASK4W {
        _UCADDMASK4W { w: self }
    }
    #[doc = "Bit 5 - I2C Address Mask Bit 5"]
    #[inline]
    pub fn ucaddmask5(&mut self) -> _UCADDMASK5W {
        _UCADDMASK5W { w: self }
    }
    #[doc = "Bit 6 - I2C Address Mask Bit 6"]
    #[inline]
    pub fn ucaddmask6(&mut self) -> _UCADDMASK6W {
        _UCADDMASK6W { w: self }
    }
    #[doc = "Bit 7 - I2C Address Mask Bit 7"]
    #[inline]
    pub fn ucaddmask7(&mut self) -> _UCADDMASK7W {
        _UCADDMASK7W { w: self }
    }
    #[doc = "Bit 8 - I2C Address Mask Bit 8"]
    #[inline]
    pub fn ucaddmask8(&mut self) -> _UCADDMASK8W {
        _UCADDMASK8W { w: self }
    }
    #[doc = "Bit 9 - I2C Address Mask Bit 9"]
    #[inline]
    pub fn ucaddmask9(&mut self) -> _UCADDMASK9W {
        _UCADDMASK9W { w: self }
    }
}
