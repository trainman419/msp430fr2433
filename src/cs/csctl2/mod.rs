#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL2 {
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
#[doc = "Possible values of the field `FLLN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN0R {
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
            FLLN0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN0R {
        match value {
            i => FLLN0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN1R {
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
            FLLN1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN1R {
        match value {
            i => FLLN1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN2R {
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
            FLLN2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN2R {
        match value {
            i => FLLN2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN3R {
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
            FLLN3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN3R {
        match value {
            i => FLLN3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN4R {
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
            FLLN4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN4R {
        match value {
            i => FLLN4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN5R {
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
            FLLN5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN5R {
        match value {
            i => FLLN5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN6R {
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
            FLLN6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN6R {
        match value {
            i => FLLN6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN7R {
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
            FLLN7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN7R {
        match value {
            i => FLLN7R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN8R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN8R {
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
            FLLN8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN8R {
        match value {
            i => FLLN8R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLN9R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLN9R {
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
            FLLN9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLN9R {
        match value {
            i => FLLN9R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLDR {
    #[doc = "Multiply Selected Loop Freq. By 1"]
    FLLD_0,
    #[doc = "Multiply Selected Loop Freq. By 2"]
    FLLD_1,
    #[doc = "Multiply Selected Loop Freq. By 4"]
    FLLD_2,
    #[doc = "Multiply Selected Loop Freq. By 8"]
    FLLD_3,
    #[doc = "Multiply Selected Loop Freq. By 16"]
    FLLD_4,
    #[doc = "Multiply Selected Loop Freq. By 32"]
    FLLD_5,
    #[doc = "Reserved"]
    FLLD_6,
    #[doc = "Reserved"]
    FLLD_7,
}
impl FLLDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLLDR::FLLD_0 => 0,
            FLLDR::FLLD_1 => 1,
            FLLDR::FLLD_2 => 2,
            FLLDR::FLLD_3 => 3,
            FLLDR::FLLD_4 => 4,
            FLLDR::FLLD_5 => 5,
            FLLDR::FLLD_6 => 6,
            FLLDR::FLLD_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLLDR {
        match value {
            0 => FLLDR::FLLD_0,
            1 => FLLDR::FLLD_1,
            2 => FLLDR::FLLD_2,
            3 => FLLDR::FLLD_3,
            4 => FLLDR::FLLD_4,
            5 => FLLDR::FLLD_5,
            6 => FLLDR::FLLD_6,
            7 => FLLDR::FLLD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLD_0`"]
    #[inline]
    pub fn is_flld_0(&self) -> bool {
        *self == FLLDR::FLLD_0
    }
    #[doc = "Checks if the value of the field is `FLLD_1`"]
    #[inline]
    pub fn is_flld_1(&self) -> bool {
        *self == FLLDR::FLLD_1
    }
    #[doc = "Checks if the value of the field is `FLLD_2`"]
    #[inline]
    pub fn is_flld_2(&self) -> bool {
        *self == FLLDR::FLLD_2
    }
    #[doc = "Checks if the value of the field is `FLLD_3`"]
    #[inline]
    pub fn is_flld_3(&self) -> bool {
        *self == FLLDR::FLLD_3
    }
    #[doc = "Checks if the value of the field is `FLLD_4`"]
    #[inline]
    pub fn is_flld_4(&self) -> bool {
        *self == FLLDR::FLLD_4
    }
    #[doc = "Checks if the value of the field is `FLLD_5`"]
    #[inline]
    pub fn is_flld_5(&self) -> bool {
        *self == FLLDR::FLLD_5
    }
    #[doc = "Checks if the value of the field is `FLLD_6`"]
    #[inline]
    pub fn is_flld_6(&self) -> bool {
        *self == FLLDR::FLLD_6
    }
    #[doc = "Checks if the value of the field is `FLLD_7`"]
    #[inline]
    pub fn is_flld_7(&self) -> bool {
        *self == FLLDR::FLLD_7
    }
}
#[doc = "Values that can be written to the field `FLLN0`"]
pub enum FLLN0W {}
impl FLLN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN0W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN1`"]
pub enum FLLN1W {}
impl FLLN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN1W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN2`"]
pub enum FLLN2W {}
impl FLLN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN2W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN3`"]
pub enum FLLN3W {}
impl FLLN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN3W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN4`"]
pub enum FLLN4W {}
impl FLLN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN4W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN5`"]
pub enum FLLN5W {}
impl FLLN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN5W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN6`"]
pub enum FLLN6W {}
impl FLLN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN6W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN7`"]
pub enum FLLN7W {}
impl FLLN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN7W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN7W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN8`"]
pub enum FLLN8W {}
impl FLLN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN8W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN8W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLN9`"]
pub enum FLLN9W {}
impl FLLN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLN9W<'a> {
    w: &'a mut W,
}
impl<'a> _FLLN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLN9W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLD`"]
pub enum FLLDW {
    #[doc = "Multiply Selected Loop Freq. By 1"]
    FLLD_0,
    #[doc = "Multiply Selected Loop Freq. By 2"]
    FLLD_1,
    #[doc = "Multiply Selected Loop Freq. By 4"]
    FLLD_2,
    #[doc = "Multiply Selected Loop Freq. By 8"]
    FLLD_3,
    #[doc = "Multiply Selected Loop Freq. By 16"]
    FLLD_4,
    #[doc = "Multiply Selected Loop Freq. By 32"]
    FLLD_5,
    #[doc = "Reserved"]
    FLLD_6,
    #[doc = "Reserved"]
    FLLD_7,
}
impl FLLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLLDW::FLLD_0 => 0,
            FLLDW::FLLD_1 => 1,
            FLLDW::FLLD_2 => 2,
            FLLDW::FLLD_3 => 3,
            FLLDW::FLLD_4 => 4,
            FLLDW::FLLD_5 => 5,
            FLLDW::FLLD_6 => 6,
            FLLDW::FLLD_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLLDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Multiply Selected Loop Freq. By 1"]
    #[inline]
    pub fn flld_0(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_0)
    }
    #[doc = "Multiply Selected Loop Freq. By 2"]
    #[inline]
    pub fn flld_1(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_1)
    }
    #[doc = "Multiply Selected Loop Freq. By 4"]
    #[inline]
    pub fn flld_2(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_2)
    }
    #[doc = "Multiply Selected Loop Freq. By 8"]
    #[inline]
    pub fn flld_3(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_3)
    }
    #[doc = "Multiply Selected Loop Freq. By 16"]
    #[inline]
    pub fn flld_4(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_4)
    }
    #[doc = "Multiply Selected Loop Freq. By 32"]
    #[inline]
    pub fn flld_5(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_5)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn flld_6(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_6)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn flld_7(self) -> &'a mut W {
        self.variant(FLLDW::FLLD_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline]
    pub fn flln0(&self) -> FLLN0R {
        FLLN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline]
    pub fn flln1(&self) -> FLLN1R {
        FLLN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline]
    pub fn flln2(&self) -> FLLN2R {
        FLLN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline]
    pub fn flln3(&self) -> FLLN3R {
        FLLN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline]
    pub fn flln4(&self) -> FLLN4R {
        FLLN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline]
    pub fn flln5(&self) -> FLLN5R {
        FLLN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline]
    pub fn flln6(&self) -> FLLN6R {
        FLLN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline]
    pub fn flln7(&self) -> FLLN7R {
        FLLN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline]
    pub fn flln8(&self) -> FLLN8R {
        FLLN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline]
    pub fn flln9(&self) -> FLLN9R {
        FLLN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline]
    pub fn flld(&self) -> FLLDR {
        FLLDR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline]
    pub fn flln0(&mut self) -> _FLLN0W {
        _FLLN0W { w: self }
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline]
    pub fn flln1(&mut self) -> _FLLN1W {
        _FLLN1W { w: self }
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline]
    pub fn flln2(&mut self) -> _FLLN2W {
        _FLLN2W { w: self }
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline]
    pub fn flln3(&mut self) -> _FLLN3W {
        _FLLN3W { w: self }
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline]
    pub fn flln4(&mut self) -> _FLLN4W {
        _FLLN4W { w: self }
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline]
    pub fn flln5(&mut self) -> _FLLN5W {
        _FLLN5W { w: self }
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline]
    pub fn flln6(&mut self) -> _FLLN6W {
        _FLLN6W { w: self }
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline]
    pub fn flln7(&mut self) -> _FLLN7W {
        _FLLN7W { w: self }
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline]
    pub fn flln8(&mut self) -> _FLLN8W {
        _FLLN8W { w: self }
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline]
    pub fn flln9(&mut self) -> _FLLN9W {
        _FLLN9W { w: self }
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline]
    pub fn flld(&mut self) -> _FLLDW {
        _FLLDW { w: self }
    }
}
