#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::P1SEL1 {
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
#[doc = "Possible values of the field `P1SEL1_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_0R {
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
            P1SEL1_0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_0R {
        match value {
            i => P1SEL1_0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_1R {
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
            P1SEL1_1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_1R {
        match value {
            i => P1SEL1_1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_2R {
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
            P1SEL1_2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_2R {
        match value {
            i => P1SEL1_2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_3R {
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
            P1SEL1_3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_3R {
        match value {
            i => P1SEL1_3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_4R {
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
            P1SEL1_4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_4R {
        match value {
            i => P1SEL1_4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_5R {
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
            P1SEL1_5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_5R {
        match value {
            i => P1SEL1_5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_6R {
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
            P1SEL1_6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_6R {
        match value {
            i => P1SEL1_6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `P1SEL1_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1SEL1_7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl P1SEL1_7R {
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
            P1SEL1_7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> P1SEL1_7R {
        match value {
            i => P1SEL1_7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `P1SEL1_0`"]
pub enum P1SEL1_0W {}
impl P1SEL1_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_0W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_1`"]
pub enum P1SEL1_1W {}
impl P1SEL1_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_1W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_2`"]
pub enum P1SEL1_2W {}
impl P1SEL1_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_2W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_3`"]
pub enum P1SEL1_3W {}
impl P1SEL1_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_3W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_4`"]
pub enum P1SEL1_4W {}
impl P1SEL1_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_4W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_5`"]
pub enum P1SEL1_5W {}
impl P1SEL1_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_5W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_6`"]
pub enum P1SEL1_6W {}
impl P1SEL1_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_6W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `P1SEL1_7`"]
pub enum P1SEL1_7W {}
impl P1SEL1_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _P1SEL1_7W<'a> {
    w: &'a mut W,
}
impl<'a> _P1SEL1_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: P1SEL1_7W) -> &'a mut W {
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
    #[doc = "Bit 0 - P1SEL1_0"]
    #[inline]
    pub fn p1sel1_0(&self) -> P1SEL1_0R {
        P1SEL1_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - P1SEL1_1"]
    #[inline]
    pub fn p1sel1_1(&self) -> P1SEL1_1R {
        P1SEL1_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - P1SEL1_2"]
    #[inline]
    pub fn p1sel1_2(&self) -> P1SEL1_2R {
        P1SEL1_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - P1SEL1_3"]
    #[inline]
    pub fn p1sel1_3(&self) -> P1SEL1_3R {
        P1SEL1_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - P1SEL1_4"]
    #[inline]
    pub fn p1sel1_4(&self) -> P1SEL1_4R {
        P1SEL1_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - P1SEL1_5"]
    #[inline]
    pub fn p1sel1_5(&self) -> P1SEL1_5R {
        P1SEL1_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - P1SEL1_6"]
    #[inline]
    pub fn p1sel1_6(&self) -> P1SEL1_6R {
        P1SEL1_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - P1SEL1_7"]
    #[inline]
    pub fn p1sel1_7(&self) -> P1SEL1_7R {
        P1SEL1_7R::_from({
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
    #[doc = "Bit 0 - P1SEL1_0"]
    #[inline]
    pub fn p1sel1_0(&mut self) -> _P1SEL1_0W {
        _P1SEL1_0W { w: self }
    }
    #[doc = "Bit 1 - P1SEL1_1"]
    #[inline]
    pub fn p1sel1_1(&mut self) -> _P1SEL1_1W {
        _P1SEL1_1W { w: self }
    }
    #[doc = "Bit 2 - P1SEL1_2"]
    #[inline]
    pub fn p1sel1_2(&mut self) -> _P1SEL1_2W {
        _P1SEL1_2W { w: self }
    }
    #[doc = "Bit 3 - P1SEL1_3"]
    #[inline]
    pub fn p1sel1_3(&mut self) -> _P1SEL1_3W {
        _P1SEL1_3W { w: self }
    }
    #[doc = "Bit 4 - P1SEL1_4"]
    #[inline]
    pub fn p1sel1_4(&mut self) -> _P1SEL1_4W {
        _P1SEL1_4W { w: self }
    }
    #[doc = "Bit 5 - P1SEL1_5"]
    #[inline]
    pub fn p1sel1_5(&mut self) -> _P1SEL1_5W {
        _P1SEL1_5W { w: self }
    }
    #[doc = "Bit 6 - P1SEL1_6"]
    #[inline]
    pub fn p1sel1_6(&mut self) -> _P1SEL1_6W {
        _P1SEL1_6W { w: self }
    }
    #[doc = "Bit 7 - P1SEL1_7"]
    #[inline]
    pub fn p1sel1_7(&mut self) -> _P1SEL1_7W {
        _P1SEL1_7W { w: self }
    }
}
