#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::UCB0BCNT {
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
#[doc = "Possible values of the field `UCBCNT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT0R {
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
            UCBCNT0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT0R {
        match value {
            i => UCBCNT0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT1R {
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
            UCBCNT1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT1R {
        match value {
            i => UCBCNT1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT2R {
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
            UCBCNT2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT2R {
        match value {
            i => UCBCNT2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT3R {
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
            UCBCNT3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT3R {
        match value {
            i => UCBCNT3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT4R {
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
            UCBCNT4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT4R {
        match value {
            i => UCBCNT4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT5R {
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
            UCBCNT5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT5R {
        match value {
            i => UCBCNT5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT6R {
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
            UCBCNT6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT6R {
        match value {
            i => UCBCNT6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBCNT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNT7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBCNT7R {
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
            UCBCNT7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBCNT7R {
        match value {
            i => UCBCNT7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `UCBCNT0`"]
pub enum UCBCNT0W {}
impl UCBCNT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT0W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT1`"]
pub enum UCBCNT1W {}
impl UCBCNT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT1W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT2`"]
pub enum UCBCNT2W {}
impl UCBCNT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT2W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT3`"]
pub enum UCBCNT3W {}
impl UCBCNT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT3W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT4`"]
pub enum UCBCNT4W {}
impl UCBCNT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT4W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT4W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT5`"]
pub enum UCBCNT5W {}
impl UCBCNT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT5W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT5W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT6`"]
pub enum UCBCNT6W {}
impl UCBCNT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT6W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT6W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBCNT7`"]
pub enum UCBCNT7W {}
impl UCBCNT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBCNT7W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBCNT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBCNT7W) -> &'a mut W {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline]
    pub fn ucbcnt0(&self) -> UCBCNT0R {
        UCBCNT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline]
    pub fn ucbcnt1(&self) -> UCBCNT1R {
        UCBCNT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline]
    pub fn ucbcnt2(&self) -> UCBCNT2R {
        UCBCNT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline]
    pub fn ucbcnt3(&self) -> UCBCNT3R {
        UCBCNT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline]
    pub fn ucbcnt4(&self) -> UCBCNT4R {
        UCBCNT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline]
    pub fn ucbcnt5(&self) -> UCBCNT5R {
        UCBCNT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline]
    pub fn ucbcnt6(&self) -> UCBCNT6R {
        UCBCNT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline]
    pub fn ucbcnt7(&self) -> UCBCNT7R {
        UCBCNT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - USCI Byte Counter Bit 0"]
    #[inline]
    pub fn ucbcnt0(&mut self) -> _UCBCNT0W {
        _UCBCNT0W { w: self }
    }
    #[doc = "Bit 1 - USCI Byte Counter Bit 1"]
    #[inline]
    pub fn ucbcnt1(&mut self) -> _UCBCNT1W {
        _UCBCNT1W { w: self }
    }
    #[doc = "Bit 2 - USCI Byte Counter Bit 2"]
    #[inline]
    pub fn ucbcnt2(&mut self) -> _UCBCNT2W {
        _UCBCNT2W { w: self }
    }
    #[doc = "Bit 3 - USCI Byte Counter Bit 3"]
    #[inline]
    pub fn ucbcnt3(&mut self) -> _UCBCNT3W {
        _UCBCNT3W { w: self }
    }
    #[doc = "Bit 4 - USCI Byte Counter Bit 4"]
    #[inline]
    pub fn ucbcnt4(&mut self) -> _UCBCNT4W {
        _UCBCNT4W { w: self }
    }
    #[doc = "Bit 5 - USCI Byte Counter Bit 5"]
    #[inline]
    pub fn ucbcnt5(&mut self) -> _UCBCNT5W {
        _UCBCNT5W { w: self }
    }
    #[doc = "Bit 6 - USCI Byte Counter Bit 6"]
    #[inline]
    pub fn ucbcnt6(&mut self) -> _UCBCNT6W {
        _UCBCNT6W { w: self }
    }
    #[doc = "Bit 7 - USCI Byte Counter Bit 7"]
    #[inline]
    pub fn ucbcnt7(&mut self) -> _UCBCNT7W {
        _UCBCNT7W { w: self }
    }
}
