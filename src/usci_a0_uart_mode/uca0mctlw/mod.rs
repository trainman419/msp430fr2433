#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::UCA0MCTLW {
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
#[doc = "Possible values of the field `UCOS16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOS16R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCOS16R {
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
            UCOS16R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCOS16R {
        match value {
            i => UCOS16R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRFR {
    #[doc = "USCI First Stage Modulation: 0"]
    UCBRF_0,
    #[doc = "USCI First Stage Modulation: 1"]
    UCBRF_1,
    #[doc = "USCI First Stage Modulation: 2"]
    UCBRF_2,
    #[doc = "USCI First Stage Modulation: 3"]
    UCBRF_3,
    #[doc = "USCI First Stage Modulation: 4"]
    UCBRF_4,
    #[doc = "USCI First Stage Modulation: 5"]
    UCBRF_5,
    #[doc = "USCI First Stage Modulation: 6"]
    UCBRF_6,
    #[doc = "USCI First Stage Modulation: 7"]
    UCBRF_7,
    #[doc = "USCI First Stage Modulation: 8"]
    UCBRF_8,
    #[doc = "USCI First Stage Modulation: 9"]
    UCBRF_9,
    #[doc = "USCI First Stage Modulation: A"]
    UCBRF_10,
    #[doc = "USCI First Stage Modulation: B"]
    UCBRF_11,
    #[doc = "USCI First Stage Modulation: C"]
    UCBRF_12,
    #[doc = "USCI First Stage Modulation: D"]
    UCBRF_13,
    #[doc = "USCI First Stage Modulation: E"]
    UCBRF_14,
    #[doc = "USCI First Stage Modulation: F"]
    UCBRF_15,
}
impl UCBRFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UCBRFR::UCBRF_0 => 0,
            UCBRFR::UCBRF_1 => 1,
            UCBRFR::UCBRF_2 => 2,
            UCBRFR::UCBRF_3 => 3,
            UCBRFR::UCBRF_4 => 4,
            UCBRFR::UCBRF_5 => 5,
            UCBRFR::UCBRF_6 => 6,
            UCBRFR::UCBRF_7 => 7,
            UCBRFR::UCBRF_8 => 8,
            UCBRFR::UCBRF_9 => 9,
            UCBRFR::UCBRF_10 => 10,
            UCBRFR::UCBRF_11 => 11,
            UCBRFR::UCBRF_12 => 12,
            UCBRFR::UCBRF_13 => 13,
            UCBRFR::UCBRF_14 => 14,
            UCBRFR::UCBRF_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UCBRFR {
        match value {
            0 => UCBRFR::UCBRF_0,
            1 => UCBRFR::UCBRF_1,
            2 => UCBRFR::UCBRF_2,
            3 => UCBRFR::UCBRF_3,
            4 => UCBRFR::UCBRF_4,
            5 => UCBRFR::UCBRF_5,
            6 => UCBRFR::UCBRF_6,
            7 => UCBRFR::UCBRF_7,
            8 => UCBRFR::UCBRF_8,
            9 => UCBRFR::UCBRF_9,
            10 => UCBRFR::UCBRF_10,
            11 => UCBRFR::UCBRF_11,
            12 => UCBRFR::UCBRF_12,
            13 => UCBRFR::UCBRF_13,
            14 => UCBRFR::UCBRF_14,
            15 => UCBRFR::UCBRF_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCBRF_0`"]
    #[inline]
    pub fn is_ucbrf_0(&self) -> bool {
        *self == UCBRFR::UCBRF_0
    }
    #[doc = "Checks if the value of the field is `UCBRF_1`"]
    #[inline]
    pub fn is_ucbrf_1(&self) -> bool {
        *self == UCBRFR::UCBRF_1
    }
    #[doc = "Checks if the value of the field is `UCBRF_2`"]
    #[inline]
    pub fn is_ucbrf_2(&self) -> bool {
        *self == UCBRFR::UCBRF_2
    }
    #[doc = "Checks if the value of the field is `UCBRF_3`"]
    #[inline]
    pub fn is_ucbrf_3(&self) -> bool {
        *self == UCBRFR::UCBRF_3
    }
    #[doc = "Checks if the value of the field is `UCBRF_4`"]
    #[inline]
    pub fn is_ucbrf_4(&self) -> bool {
        *self == UCBRFR::UCBRF_4
    }
    #[doc = "Checks if the value of the field is `UCBRF_5`"]
    #[inline]
    pub fn is_ucbrf_5(&self) -> bool {
        *self == UCBRFR::UCBRF_5
    }
    #[doc = "Checks if the value of the field is `UCBRF_6`"]
    #[inline]
    pub fn is_ucbrf_6(&self) -> bool {
        *self == UCBRFR::UCBRF_6
    }
    #[doc = "Checks if the value of the field is `UCBRF_7`"]
    #[inline]
    pub fn is_ucbrf_7(&self) -> bool {
        *self == UCBRFR::UCBRF_7
    }
    #[doc = "Checks if the value of the field is `UCBRF_8`"]
    #[inline]
    pub fn is_ucbrf_8(&self) -> bool {
        *self == UCBRFR::UCBRF_8
    }
    #[doc = "Checks if the value of the field is `UCBRF_9`"]
    #[inline]
    pub fn is_ucbrf_9(&self) -> bool {
        *self == UCBRFR::UCBRF_9
    }
    #[doc = "Checks if the value of the field is `UCBRF_10`"]
    #[inline]
    pub fn is_ucbrf_10(&self) -> bool {
        *self == UCBRFR::UCBRF_10
    }
    #[doc = "Checks if the value of the field is `UCBRF_11`"]
    #[inline]
    pub fn is_ucbrf_11(&self) -> bool {
        *self == UCBRFR::UCBRF_11
    }
    #[doc = "Checks if the value of the field is `UCBRF_12`"]
    #[inline]
    pub fn is_ucbrf_12(&self) -> bool {
        *self == UCBRFR::UCBRF_12
    }
    #[doc = "Checks if the value of the field is `UCBRF_13`"]
    #[inline]
    pub fn is_ucbrf_13(&self) -> bool {
        *self == UCBRFR::UCBRF_13
    }
    #[doc = "Checks if the value of the field is `UCBRF_14`"]
    #[inline]
    pub fn is_ucbrf_14(&self) -> bool {
        *self == UCBRFR::UCBRF_14
    }
    #[doc = "Checks if the value of the field is `UCBRF_15`"]
    #[inline]
    pub fn is_ucbrf_15(&self) -> bool {
        *self == UCBRFR::UCBRF_15
    }
}
#[doc = "Possible values of the field `UCBRS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS0R {
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
            UCBRS0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS0R {
        match value {
            i => UCBRS0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS1R {
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
            UCBRS1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS1R {
        match value {
            i => UCBRS1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS2R {
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
            UCBRS2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS2R {
        match value {
            i => UCBRS2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS3R {
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
            UCBRS3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS3R {
        match value {
            i => UCBRS3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS4R {
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
            UCBRS4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS4R {
        match value {
            i => UCBRS4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS5R {
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
            UCBRS5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS5R {
        match value {
            i => UCBRS5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS6R {
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
            UCBRS6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS6R {
        match value {
            i => UCBRS6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCBRS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBRS7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCBRS7R {
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
            UCBRS7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCBRS7R {
        match value {
            i => UCBRS7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `UCOS16`"]
pub enum UCOS16W {}
impl UCOS16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCOS16W<'a> {
    w: &'a mut W,
}
impl<'a> _UCOS16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCOS16W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRF`"]
pub enum UCBRFW {
    #[doc = "USCI First Stage Modulation: 0"]
    UCBRF_0,
    #[doc = "USCI First Stage Modulation: 1"]
    UCBRF_1,
    #[doc = "USCI First Stage Modulation: 2"]
    UCBRF_2,
    #[doc = "USCI First Stage Modulation: 3"]
    UCBRF_3,
    #[doc = "USCI First Stage Modulation: 4"]
    UCBRF_4,
    #[doc = "USCI First Stage Modulation: 5"]
    UCBRF_5,
    #[doc = "USCI First Stage Modulation: 6"]
    UCBRF_6,
    #[doc = "USCI First Stage Modulation: 7"]
    UCBRF_7,
    #[doc = "USCI First Stage Modulation: 8"]
    UCBRF_8,
    #[doc = "USCI First Stage Modulation: 9"]
    UCBRF_9,
    #[doc = "USCI First Stage Modulation: A"]
    UCBRF_10,
    #[doc = "USCI First Stage Modulation: B"]
    UCBRF_11,
    #[doc = "USCI First Stage Modulation: C"]
    UCBRF_12,
    #[doc = "USCI First Stage Modulation: D"]
    UCBRF_13,
    #[doc = "USCI First Stage Modulation: E"]
    UCBRF_14,
    #[doc = "USCI First Stage Modulation: F"]
    UCBRF_15,
}
impl UCBRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UCBRFW::UCBRF_0 => 0,
            UCBRFW::UCBRF_1 => 1,
            UCBRFW::UCBRF_2 => 2,
            UCBRFW::UCBRF_3 => 3,
            UCBRFW::UCBRF_4 => 4,
            UCBRFW::UCBRF_5 => 5,
            UCBRFW::UCBRF_6 => 6,
            UCBRFW::UCBRF_7 => 7,
            UCBRFW::UCBRF_8 => 8,
            UCBRFW::UCBRF_9 => 9,
            UCBRFW::UCBRF_10 => 10,
            UCBRFW::UCBRF_11 => 11,
            UCBRFW::UCBRF_12 => 12,
            UCBRFW::UCBRF_13 => 13,
            UCBRFW::UCBRF_14 => 14,
            UCBRFW::UCBRF_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UCBRFW<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "USCI First Stage Modulation: 0"]
    #[inline]
    pub fn ucbrf_0(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_0)
    }
    #[doc = "USCI First Stage Modulation: 1"]
    #[inline]
    pub fn ucbrf_1(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_1)
    }
    #[doc = "USCI First Stage Modulation: 2"]
    #[inline]
    pub fn ucbrf_2(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_2)
    }
    #[doc = "USCI First Stage Modulation: 3"]
    #[inline]
    pub fn ucbrf_3(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_3)
    }
    #[doc = "USCI First Stage Modulation: 4"]
    #[inline]
    pub fn ucbrf_4(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_4)
    }
    #[doc = "USCI First Stage Modulation: 5"]
    #[inline]
    pub fn ucbrf_5(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_5)
    }
    #[doc = "USCI First Stage Modulation: 6"]
    #[inline]
    pub fn ucbrf_6(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_6)
    }
    #[doc = "USCI First Stage Modulation: 7"]
    #[inline]
    pub fn ucbrf_7(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_7)
    }
    #[doc = "USCI First Stage Modulation: 8"]
    #[inline]
    pub fn ucbrf_8(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_8)
    }
    #[doc = "USCI First Stage Modulation: 9"]
    #[inline]
    pub fn ucbrf_9(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_9)
    }
    #[doc = "USCI First Stage Modulation: A"]
    #[inline]
    pub fn ucbrf_10(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_10)
    }
    #[doc = "USCI First Stage Modulation: B"]
    #[inline]
    pub fn ucbrf_11(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_11)
    }
    #[doc = "USCI First Stage Modulation: C"]
    #[inline]
    pub fn ucbrf_12(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_12)
    }
    #[doc = "USCI First Stage Modulation: D"]
    #[inline]
    pub fn ucbrf_13(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_13)
    }
    #[doc = "USCI First Stage Modulation: E"]
    #[inline]
    pub fn ucbrf_14(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_14)
    }
    #[doc = "USCI First Stage Modulation: F"]
    #[inline]
    pub fn ucbrf_15(self) -> &'a mut W {
        self.variant(UCBRFW::UCBRF_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCBRS0`"]
pub enum UCBRS0W {}
impl UCBRS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS0W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS1`"]
pub enum UCBRS1W {}
impl UCBRS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS1W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS2`"]
pub enum UCBRS2W {}
impl UCBRS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS2W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS3`"]
pub enum UCBRS3W {}
impl UCBRS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS3W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS4`"]
pub enum UCBRS4W {}
impl UCBRS4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS4W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS5`"]
pub enum UCBRS5W {}
impl UCBRS5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS5W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS6`"]
pub enum UCBRS6W {}
impl UCBRS6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS6W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCBRS7`"]
pub enum UCBRS7W {}
impl UCBRS7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCBRS7W<'a> {
    w: &'a mut W,
}
impl<'a> _UCBRS7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCBRS7W) -> &'a mut W {
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline]
    pub fn ucos16(&self) -> UCOS16R {
        UCOS16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline]
    pub fn ucbrf(&self) -> UCBRFR {
        UCBRFR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline]
    pub fn ucbrs0(&self) -> UCBRS0R {
        UCBRS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline]
    pub fn ucbrs1(&self) -> UCBRS1R {
        UCBRS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline]
    pub fn ucbrs2(&self) -> UCBRS2R {
        UCBRS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline]
    pub fn ucbrs3(&self) -> UCBRS3R {
        UCBRS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline]
    pub fn ucbrs4(&self) -> UCBRS4R {
        UCBRS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline]
    pub fn ucbrs5(&self) -> UCBRS5R {
        UCBRS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline]
    pub fn ucbrs6(&self) -> UCBRS6R {
        UCBRS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline]
    pub fn ucbrs7(&self) -> UCBRS7R {
        UCBRS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - USCI 16-times Oversampling enable"]
    #[inline]
    pub fn ucos16(&mut self) -> _UCOS16W {
        _UCOS16W { w: self }
    }
    #[doc = "Bits 4:7 - USCI First Stage Modulation Select 3"]
    #[inline]
    pub fn ucbrf(&mut self) -> _UCBRFW {
        _UCBRFW { w: self }
    }
    #[doc = "Bit 8 - USCI Second Stage Modulation Select 0"]
    #[inline]
    pub fn ucbrs0(&mut self) -> _UCBRS0W {
        _UCBRS0W { w: self }
    }
    #[doc = "Bit 9 - USCI Second Stage Modulation Select 1"]
    #[inline]
    pub fn ucbrs1(&mut self) -> _UCBRS1W {
        _UCBRS1W { w: self }
    }
    #[doc = "Bit 10 - USCI Second Stage Modulation Select 2"]
    #[inline]
    pub fn ucbrs2(&mut self) -> _UCBRS2W {
        _UCBRS2W { w: self }
    }
    #[doc = "Bit 11 - USCI Second Stage Modulation Select 3"]
    #[inline]
    pub fn ucbrs3(&mut self) -> _UCBRS3W {
        _UCBRS3W { w: self }
    }
    #[doc = "Bit 12 - USCI Second Stage Modulation Select 4"]
    #[inline]
    pub fn ucbrs4(&mut self) -> _UCBRS4W {
        _UCBRS4W { w: self }
    }
    #[doc = "Bit 13 - USCI Second Stage Modulation Select 5"]
    #[inline]
    pub fn ucbrs5(&mut self) -> _UCBRS5W {
        _UCBRS5W { w: self }
    }
    #[doc = "Bit 14 - USCI Second Stage Modulation Select 6"]
    #[inline]
    pub fn ucbrs6(&mut self) -> _UCBRS6W {
        _UCBRS6W { w: self }
    }
    #[doc = "Bit 15 - USCI Second Stage Modulation Select 7"]
    #[inline]
    pub fn ucbrs7(&mut self) -> _UCBRS7W {
        _UCBRS7W { w: self }
    }
}
