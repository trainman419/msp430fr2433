#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ADCCTL0 {
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
#[doc = "Possible values of the field `ADCSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCSCR {
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
            ADCSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCSCR {
        match value {
            i => ADCSCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCENC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCENCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCENCR {
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
            ADCENCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCENCR {
        match value {
            i => ADCENCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCONR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCONR {
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
            ADCONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCONR {
        match value {
            i => ADCONR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCMSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMSCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCMSCR {
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
            ADCMSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCMSCR {
        match value {
            i => ADCMSCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCSHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSHTR {
    #[doc = "ADC Sample Hold Select 0"]
    ADCSHT_0,
    #[doc = "ADC Sample Hold Select 1"]
    ADCSHT_1,
    #[doc = "ADC Sample Hold Select 2"]
    ADCSHT_2,
    #[doc = "ADC Sample Hold Select 3"]
    ADCSHT_3,
    #[doc = "ADC Sample Hold Select 4"]
    ADCSHT_4,
    #[doc = "ADC Sample Hold Select 5"]
    ADCSHT_5,
    #[doc = "ADC Sample Hold Select 6"]
    ADCSHT_6,
    #[doc = "ADC Sample Hold Select 7"]
    ADCSHT_7,
    #[doc = "ADC Sample Hold Select 8"]
    ADCSHT_8,
    #[doc = "ADC Sample Hold Select 9"]
    ADCSHT_9,
    #[doc = "ADC Sample Hold Select 10"]
    ADCSHT_10,
    #[doc = "ADC Sample Hold Select 11"]
    ADCSHT_11,
    #[doc = "ADC Sample Hold Select 12"]
    ADCSHT_12,
    #[doc = "ADC Sample Hold Select 13"]
    ADCSHT_13,
    #[doc = "ADC Sample Hold Select 14"]
    ADCSHT_14,
    #[doc = "ADC Sample Hold Select 15"]
    ADCSHT_15,
}
impl ADCSHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCSHTR::ADCSHT_0 => 0,
            ADCSHTR::ADCSHT_1 => 1,
            ADCSHTR::ADCSHT_2 => 2,
            ADCSHTR::ADCSHT_3 => 3,
            ADCSHTR::ADCSHT_4 => 4,
            ADCSHTR::ADCSHT_5 => 5,
            ADCSHTR::ADCSHT_6 => 6,
            ADCSHTR::ADCSHT_7 => 7,
            ADCSHTR::ADCSHT_8 => 8,
            ADCSHTR::ADCSHT_9 => 9,
            ADCSHTR::ADCSHT_10 => 10,
            ADCSHTR::ADCSHT_11 => 11,
            ADCSHTR::ADCSHT_12 => 12,
            ADCSHTR::ADCSHT_13 => 13,
            ADCSHTR::ADCSHT_14 => 14,
            ADCSHTR::ADCSHT_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCSHTR {
        match value {
            0 => ADCSHTR::ADCSHT_0,
            1 => ADCSHTR::ADCSHT_1,
            2 => ADCSHTR::ADCSHT_2,
            3 => ADCSHTR::ADCSHT_3,
            4 => ADCSHTR::ADCSHT_4,
            5 => ADCSHTR::ADCSHT_5,
            6 => ADCSHTR::ADCSHT_6,
            7 => ADCSHTR::ADCSHT_7,
            8 => ADCSHTR::ADCSHT_8,
            9 => ADCSHTR::ADCSHT_9,
            10 => ADCSHTR::ADCSHT_10,
            11 => ADCSHTR::ADCSHT_11,
            12 => ADCSHTR::ADCSHT_12,
            13 => ADCSHTR::ADCSHT_13,
            14 => ADCSHTR::ADCSHT_14,
            15 => ADCSHTR::ADCSHT_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHT_0`"]
    #[inline]
    pub fn is_adcsht_0(&self) -> bool {
        *self == ADCSHTR::ADCSHT_0
    }
    #[doc = "Checks if the value of the field is `ADCSHT_1`"]
    #[inline]
    pub fn is_adcsht_1(&self) -> bool {
        *self == ADCSHTR::ADCSHT_1
    }
    #[doc = "Checks if the value of the field is `ADCSHT_2`"]
    #[inline]
    pub fn is_adcsht_2(&self) -> bool {
        *self == ADCSHTR::ADCSHT_2
    }
    #[doc = "Checks if the value of the field is `ADCSHT_3`"]
    #[inline]
    pub fn is_adcsht_3(&self) -> bool {
        *self == ADCSHTR::ADCSHT_3
    }
    #[doc = "Checks if the value of the field is `ADCSHT_4`"]
    #[inline]
    pub fn is_adcsht_4(&self) -> bool {
        *self == ADCSHTR::ADCSHT_4
    }
    #[doc = "Checks if the value of the field is `ADCSHT_5`"]
    #[inline]
    pub fn is_adcsht_5(&self) -> bool {
        *self == ADCSHTR::ADCSHT_5
    }
    #[doc = "Checks if the value of the field is `ADCSHT_6`"]
    #[inline]
    pub fn is_adcsht_6(&self) -> bool {
        *self == ADCSHTR::ADCSHT_6
    }
    #[doc = "Checks if the value of the field is `ADCSHT_7`"]
    #[inline]
    pub fn is_adcsht_7(&self) -> bool {
        *self == ADCSHTR::ADCSHT_7
    }
    #[doc = "Checks if the value of the field is `ADCSHT_8`"]
    #[inline]
    pub fn is_adcsht_8(&self) -> bool {
        *self == ADCSHTR::ADCSHT_8
    }
    #[doc = "Checks if the value of the field is `ADCSHT_9`"]
    #[inline]
    pub fn is_adcsht_9(&self) -> bool {
        *self == ADCSHTR::ADCSHT_9
    }
    #[doc = "Checks if the value of the field is `ADCSHT_10`"]
    #[inline]
    pub fn is_adcsht_10(&self) -> bool {
        *self == ADCSHTR::ADCSHT_10
    }
    #[doc = "Checks if the value of the field is `ADCSHT_11`"]
    #[inline]
    pub fn is_adcsht_11(&self) -> bool {
        *self == ADCSHTR::ADCSHT_11
    }
    #[doc = "Checks if the value of the field is `ADCSHT_12`"]
    #[inline]
    pub fn is_adcsht_12(&self) -> bool {
        *self == ADCSHTR::ADCSHT_12
    }
    #[doc = "Checks if the value of the field is `ADCSHT_13`"]
    #[inline]
    pub fn is_adcsht_13(&self) -> bool {
        *self == ADCSHTR::ADCSHT_13
    }
    #[doc = "Checks if the value of the field is `ADCSHT_14`"]
    #[inline]
    pub fn is_adcsht_14(&self) -> bool {
        *self == ADCSHTR::ADCSHT_14
    }
    #[doc = "Checks if the value of the field is `ADCSHT_15`"]
    #[inline]
    pub fn is_adcsht_15(&self) -> bool {
        *self == ADCSHTR::ADCSHT_15
    }
}
#[doc = "Values that can be written to the field `ADCSC`"]
pub enum ADCSCW {}
impl ADCSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCSCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSCW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCENC`"]
pub enum ADCENCW {}
impl ADCENCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCENCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCENCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCENCW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCON`"]
pub enum ADCONW {}
impl ADCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCONW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCONW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCMSC`"]
pub enum ADCMSCW {}
impl ADCMSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCMSCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCMSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCMSCW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCSHT`"]
pub enum ADCSHTW {
    #[doc = "ADC Sample Hold Select 0"]
    ADCSHT_0,
    #[doc = "ADC Sample Hold Select 1"]
    ADCSHT_1,
    #[doc = "ADC Sample Hold Select 2"]
    ADCSHT_2,
    #[doc = "ADC Sample Hold Select 3"]
    ADCSHT_3,
    #[doc = "ADC Sample Hold Select 4"]
    ADCSHT_4,
    #[doc = "ADC Sample Hold Select 5"]
    ADCSHT_5,
    #[doc = "ADC Sample Hold Select 6"]
    ADCSHT_6,
    #[doc = "ADC Sample Hold Select 7"]
    ADCSHT_7,
    #[doc = "ADC Sample Hold Select 8"]
    ADCSHT_8,
    #[doc = "ADC Sample Hold Select 9"]
    ADCSHT_9,
    #[doc = "ADC Sample Hold Select 10"]
    ADCSHT_10,
    #[doc = "ADC Sample Hold Select 11"]
    ADCSHT_11,
    #[doc = "ADC Sample Hold Select 12"]
    ADCSHT_12,
    #[doc = "ADC Sample Hold Select 13"]
    ADCSHT_13,
    #[doc = "ADC Sample Hold Select 14"]
    ADCSHT_14,
    #[doc = "ADC Sample Hold Select 15"]
    ADCSHT_15,
}
impl ADCSHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCSHTW::ADCSHT_0 => 0,
            ADCSHTW::ADCSHT_1 => 1,
            ADCSHTW::ADCSHT_2 => 2,
            ADCSHTW::ADCSHT_3 => 3,
            ADCSHTW::ADCSHT_4 => 4,
            ADCSHTW::ADCSHT_5 => 5,
            ADCSHTW::ADCSHT_6 => 6,
            ADCSHTW::ADCSHT_7 => 7,
            ADCSHTW::ADCSHT_8 => 8,
            ADCSHTW::ADCSHT_9 => 9,
            ADCSHTW::ADCSHT_10 => 10,
            ADCSHTW::ADCSHT_11 => 11,
            ADCSHTW::ADCSHT_12 => 12,
            ADCSHTW::ADCSHT_13 => 13,
            ADCSHTW::ADCSHT_14 => 14,
            ADCSHTW::ADCSHT_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCSHTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSHTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Sample Hold Select 0"]
    #[inline]
    pub fn adcsht_0(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_0)
    }
    #[doc = "ADC Sample Hold Select 1"]
    #[inline]
    pub fn adcsht_1(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_1)
    }
    #[doc = "ADC Sample Hold Select 2"]
    #[inline]
    pub fn adcsht_2(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_2)
    }
    #[doc = "ADC Sample Hold Select 3"]
    #[inline]
    pub fn adcsht_3(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_3)
    }
    #[doc = "ADC Sample Hold Select 4"]
    #[inline]
    pub fn adcsht_4(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_4)
    }
    #[doc = "ADC Sample Hold Select 5"]
    #[inline]
    pub fn adcsht_5(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_5)
    }
    #[doc = "ADC Sample Hold Select 6"]
    #[inline]
    pub fn adcsht_6(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_6)
    }
    #[doc = "ADC Sample Hold Select 7"]
    #[inline]
    pub fn adcsht_7(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_7)
    }
    #[doc = "ADC Sample Hold Select 8"]
    #[inline]
    pub fn adcsht_8(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_8)
    }
    #[doc = "ADC Sample Hold Select 9"]
    #[inline]
    pub fn adcsht_9(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_9)
    }
    #[doc = "ADC Sample Hold Select 10"]
    #[inline]
    pub fn adcsht_10(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_10)
    }
    #[doc = "ADC Sample Hold Select 11"]
    #[inline]
    pub fn adcsht_11(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_11)
    }
    #[doc = "ADC Sample Hold Select 12"]
    #[inline]
    pub fn adcsht_12(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_12)
    }
    #[doc = "ADC Sample Hold Select 13"]
    #[inline]
    pub fn adcsht_13(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_13)
    }
    #[doc = "ADC Sample Hold Select 14"]
    #[inline]
    pub fn adcsht_14(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_14)
    }
    #[doc = "ADC Sample Hold Select 15"]
    #[inline]
    pub fn adcsht_15(self) -> &'a mut W {
        self.variant(ADCSHTW::ADCSHT_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline]
    pub fn adcsc(&self) -> ADCSCR {
        ADCSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline]
    pub fn adcenc(&self) -> ADCENCR {
        ADCENCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline]
    pub fn adcon(&self) -> ADCONR {
        ADCONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline]
    pub fn adcmsc(&self) -> ADCMSCR {
        ADCMSCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline]
    pub fn adcsht(&self) -> ADCSHTR {
        ADCSHTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline]
    pub fn adcsc(&mut self) -> _ADCSCW {
        _ADCSCW { w: self }
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline]
    pub fn adcenc(&mut self) -> _ADCENCW {
        _ADCENCW { w: self }
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline]
    pub fn adcon(&mut self) -> _ADCONW {
        _ADCONW { w: self }
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline]
    pub fn adcmsc(&mut self) -> _ADCMSCW {
        _ADCMSCW { w: self }
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline]
    pub fn adcsht(&mut self) -> _ADCSHTW {
        _ADCSHTW { w: self }
    }
}
