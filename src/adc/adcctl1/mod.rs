#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ADCCTL1 {
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
#[doc = "Possible values of the field `ADCBUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCBUSYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCBUSYR {
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
            ADCBUSYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCBUSYR {
        match value {
            i => ADCBUSYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCCONSEQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCCONSEQR {
    #[doc = "ADC Conversion Sequence Select: 0"]
    ADCCONSEQ_0,
    #[doc = "ADC Conversion Sequence Select: 1"]
    ADCCONSEQ_1,
    #[doc = "ADC Conversion Sequence Select: 2"]
    ADCCONSEQ_2,
    #[doc = "ADC Conversion Sequence Select: 3"]
    ADCCONSEQ_3,
}
impl ADCCONSEQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCCONSEQR::ADCCONSEQ_0 => 0,
            ADCCONSEQR::ADCCONSEQ_1 => 1,
            ADCCONSEQR::ADCCONSEQ_2 => 2,
            ADCCONSEQR::ADCCONSEQ_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCCONSEQR {
        match value {
            0 => ADCCONSEQR::ADCCONSEQ_0,
            1 => ADCCONSEQR::ADCCONSEQ_1,
            2 => ADCCONSEQR::ADCCONSEQ_2,
            3 => ADCCONSEQR::ADCCONSEQ_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_0`"]
    #[inline]
    pub fn is_adcconseq_0(&self) -> bool {
        *self == ADCCONSEQR::ADCCONSEQ_0
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_1`"]
    #[inline]
    pub fn is_adcconseq_1(&self) -> bool {
        *self == ADCCONSEQR::ADCCONSEQ_1
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_2`"]
    #[inline]
    pub fn is_adcconseq_2(&self) -> bool {
        *self == ADCCONSEQR::ADCCONSEQ_2
    }
    #[doc = "Checks if the value of the field is `ADCCONSEQ_3`"]
    #[inline]
    pub fn is_adcconseq_3(&self) -> bool {
        *self == ADCCONSEQR::ADCCONSEQ_3
    }
}
#[doc = "Possible values of the field `ADCSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSSELR {
    #[doc = "ADC Clock Source Select: 0"]
    ADCSSEL_0,
    #[doc = "ADC Clock Source Select: 1"]
    ADCSSEL_1,
    #[doc = "ADC Clock Source Select: 2"]
    ADCSSEL_2,
    #[doc = "ADC Clock Source Select: 3"]
    ADCSSEL_3,
}
impl ADCSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCSSELR::ADCSSEL_0 => 0,
            ADCSSELR::ADCSSEL_1 => 1,
            ADCSSELR::ADCSSEL_2 => 2,
            ADCSSELR::ADCSSEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCSSELR {
        match value {
            0 => ADCSSELR::ADCSSEL_0,
            1 => ADCSSELR::ADCSSEL_1,
            2 => ADCSSELR::ADCSSEL_2,
            3 => ADCSSELR::ADCSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_0`"]
    #[inline]
    pub fn is_adcssel_0(&self) -> bool {
        *self == ADCSSELR::ADCSSEL_0
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_1`"]
    #[inline]
    pub fn is_adcssel_1(&self) -> bool {
        *self == ADCSSELR::ADCSSEL_1
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_2`"]
    #[inline]
    pub fn is_adcssel_2(&self) -> bool {
        *self == ADCSSELR::ADCSSEL_2
    }
    #[doc = "Checks if the value of the field is `ADCSSEL_3`"]
    #[inline]
    pub fn is_adcssel_3(&self) -> bool {
        *self == ADCSSELR::ADCSSEL_3
    }
}
#[doc = "Possible values of the field `ADCDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCDIVR {
    #[doc = "ADC Clock Divider Select: 0"]
    ADCDIV_0,
    #[doc = "ADC Clock Divider Select: 1"]
    ADCDIV_1,
    #[doc = "ADC Clock Divider Select: 2"]
    ADCDIV_2,
    #[doc = "ADC Clock Divider Select: 3"]
    ADCDIV_3,
    #[doc = "ADC Clock Divider Select: 4"]
    ADCDIV_4,
    #[doc = "ADC Clock Divider Select: 5"]
    ADCDIV_5,
    #[doc = "ADC Clock Divider Select: 6"]
    ADCDIV_6,
    #[doc = "ADC Clock Divider Select: 7"]
    ADCDIV_7,
}
impl ADCDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCDIVR::ADCDIV_0 => 0,
            ADCDIVR::ADCDIV_1 => 1,
            ADCDIVR::ADCDIV_2 => 2,
            ADCDIVR::ADCDIV_3 => 3,
            ADCDIVR::ADCDIV_4 => 4,
            ADCDIVR::ADCDIV_5 => 5,
            ADCDIVR::ADCDIV_6 => 6,
            ADCDIVR::ADCDIV_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCDIVR {
        match value {
            0 => ADCDIVR::ADCDIV_0,
            1 => ADCDIVR::ADCDIV_1,
            2 => ADCDIVR::ADCDIV_2,
            3 => ADCDIVR::ADCDIV_3,
            4 => ADCDIVR::ADCDIV_4,
            5 => ADCDIVR::ADCDIV_5,
            6 => ADCDIVR::ADCDIV_6,
            7 => ADCDIVR::ADCDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCDIV_0`"]
    #[inline]
    pub fn is_adcdiv_0(&self) -> bool {
        *self == ADCDIVR::ADCDIV_0
    }
    #[doc = "Checks if the value of the field is `ADCDIV_1`"]
    #[inline]
    pub fn is_adcdiv_1(&self) -> bool {
        *self == ADCDIVR::ADCDIV_1
    }
    #[doc = "Checks if the value of the field is `ADCDIV_2`"]
    #[inline]
    pub fn is_adcdiv_2(&self) -> bool {
        *self == ADCDIVR::ADCDIV_2
    }
    #[doc = "Checks if the value of the field is `ADCDIV_3`"]
    #[inline]
    pub fn is_adcdiv_3(&self) -> bool {
        *self == ADCDIVR::ADCDIV_3
    }
    #[doc = "Checks if the value of the field is `ADCDIV_4`"]
    #[inline]
    pub fn is_adcdiv_4(&self) -> bool {
        *self == ADCDIVR::ADCDIV_4
    }
    #[doc = "Checks if the value of the field is `ADCDIV_5`"]
    #[inline]
    pub fn is_adcdiv_5(&self) -> bool {
        *self == ADCDIVR::ADCDIV_5
    }
    #[doc = "Checks if the value of the field is `ADCDIV_6`"]
    #[inline]
    pub fn is_adcdiv_6(&self) -> bool {
        *self == ADCDIVR::ADCDIV_6
    }
    #[doc = "Checks if the value of the field is `ADCDIV_7`"]
    #[inline]
    pub fn is_adcdiv_7(&self) -> bool {
        *self == ADCDIVR::ADCDIV_7
    }
}
#[doc = "Possible values of the field `ADCISSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCISSHR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCISSHR {
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
            ADCISSHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCISSHR {
        match value {
            i => ADCISSHR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCSHP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSHPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCSHPR {
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
            ADCSHPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCSHPR {
        match value {
            i => ADCSHPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCSHS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSHSR {
    #[doc = "ADC Sample/Hold Source: 0"]
    ADCSHS_0,
    #[doc = "ADC Sample/Hold Source: 1"]
    ADCSHS_1,
    #[doc = "ADC Sample/Hold Source: 2"]
    ADCSHS_2,
    #[doc = "ADC Sample/Hold Source: 3"]
    ADCSHS_3,
}
impl ADCSHSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCSHSR::ADCSHS_0 => 0,
            ADCSHSR::ADCSHS_1 => 1,
            ADCSHSR::ADCSHS_2 => 2,
            ADCSHSR::ADCSHS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCSHSR {
        match value {
            0 => ADCSHSR::ADCSHS_0,
            1 => ADCSHSR::ADCSHS_1,
            2 => ADCSHSR::ADCSHS_2,
            3 => ADCSHSR::ADCSHS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHS_0`"]
    #[inline]
    pub fn is_adcshs_0(&self) -> bool {
        *self == ADCSHSR::ADCSHS_0
    }
    #[doc = "Checks if the value of the field is `ADCSHS_1`"]
    #[inline]
    pub fn is_adcshs_1(&self) -> bool {
        *self == ADCSHSR::ADCSHS_1
    }
    #[doc = "Checks if the value of the field is `ADCSHS_2`"]
    #[inline]
    pub fn is_adcshs_2(&self) -> bool {
        *self == ADCSHSR::ADCSHS_2
    }
    #[doc = "Checks if the value of the field is `ADCSHS_3`"]
    #[inline]
    pub fn is_adcshs_3(&self) -> bool {
        *self == ADCSHSR::ADCSHS_3
    }
}
#[doc = "Values that can be written to the field `ADCBUSY`"]
pub enum ADCBUSYW {}
impl ADCBUSYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCBUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCBUSYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCBUSYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCCONSEQ`"]
pub enum ADCCONSEQW {
    #[doc = "ADC Conversion Sequence Select: 0"]
    ADCCONSEQ_0,
    #[doc = "ADC Conversion Sequence Select: 1"]
    ADCCONSEQ_1,
    #[doc = "ADC Conversion Sequence Select: 2"]
    ADCCONSEQ_2,
    #[doc = "ADC Conversion Sequence Select: 3"]
    ADCCONSEQ_3,
}
impl ADCCONSEQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCCONSEQW::ADCCONSEQ_0 => 0,
            ADCCONSEQW::ADCCONSEQ_1 => 1,
            ADCCONSEQW::ADCCONSEQ_2 => 2,
            ADCCONSEQW::ADCCONSEQ_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCCONSEQW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCCONSEQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCCONSEQW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Conversion Sequence Select: 0"]
    #[inline]
    pub fn adcconseq_0(self) -> &'a mut W {
        self.variant(ADCCONSEQW::ADCCONSEQ_0)
    }
    #[doc = "ADC Conversion Sequence Select: 1"]
    #[inline]
    pub fn adcconseq_1(self) -> &'a mut W {
        self.variant(ADCCONSEQW::ADCCONSEQ_1)
    }
    #[doc = "ADC Conversion Sequence Select: 2"]
    #[inline]
    pub fn adcconseq_2(self) -> &'a mut W {
        self.variant(ADCCONSEQW::ADCCONSEQ_2)
    }
    #[doc = "ADC Conversion Sequence Select: 3"]
    #[inline]
    pub fn adcconseq_3(self) -> &'a mut W {
        self.variant(ADCCONSEQW::ADCCONSEQ_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCSSEL`"]
pub enum ADCSSELW {
    #[doc = "ADC Clock Source Select: 0"]
    ADCSSEL_0,
    #[doc = "ADC Clock Source Select: 1"]
    ADCSSEL_1,
    #[doc = "ADC Clock Source Select: 2"]
    ADCSSEL_2,
    #[doc = "ADC Clock Source Select: 3"]
    ADCSSEL_3,
}
impl ADCSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCSSELW::ADCSSEL_0 => 0,
            ADCSSELW::ADCSSEL_1 => 1,
            ADCSSELW::ADCSSEL_2 => 2,
            ADCSSELW::ADCSSEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Clock Source Select: 0"]
    #[inline]
    pub fn adcssel_0(self) -> &'a mut W {
        self.variant(ADCSSELW::ADCSSEL_0)
    }
    #[doc = "ADC Clock Source Select: 1"]
    #[inline]
    pub fn adcssel_1(self) -> &'a mut W {
        self.variant(ADCSSELW::ADCSSEL_1)
    }
    #[doc = "ADC Clock Source Select: 2"]
    #[inline]
    pub fn adcssel_2(self) -> &'a mut W {
        self.variant(ADCSSELW::ADCSSEL_2)
    }
    #[doc = "ADC Clock Source Select: 3"]
    #[inline]
    pub fn adcssel_3(self) -> &'a mut W {
        self.variant(ADCSSELW::ADCSSEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCDIV`"]
pub enum ADCDIVW {
    #[doc = "ADC Clock Divider Select: 0"]
    ADCDIV_0,
    #[doc = "ADC Clock Divider Select: 1"]
    ADCDIV_1,
    #[doc = "ADC Clock Divider Select: 2"]
    ADCDIV_2,
    #[doc = "ADC Clock Divider Select: 3"]
    ADCDIV_3,
    #[doc = "ADC Clock Divider Select: 4"]
    ADCDIV_4,
    #[doc = "ADC Clock Divider Select: 5"]
    ADCDIV_5,
    #[doc = "ADC Clock Divider Select: 6"]
    ADCDIV_6,
    #[doc = "ADC Clock Divider Select: 7"]
    ADCDIV_7,
}
impl ADCDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCDIVW::ADCDIV_0 => 0,
            ADCDIVW::ADCDIV_1 => 1,
            ADCDIVW::ADCDIV_2 => 2,
            ADCDIVW::ADCDIV_3 => 3,
            ADCDIVW::ADCDIV_4 => 4,
            ADCDIVW::ADCDIV_5 => 5,
            ADCDIVW::ADCDIV_6 => 6,
            ADCDIVW::ADCDIV_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Clock Divider Select: 0"]
    #[inline]
    pub fn adcdiv_0(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_0)
    }
    #[doc = "ADC Clock Divider Select: 1"]
    #[inline]
    pub fn adcdiv_1(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_1)
    }
    #[doc = "ADC Clock Divider Select: 2"]
    #[inline]
    pub fn adcdiv_2(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_2)
    }
    #[doc = "ADC Clock Divider Select: 3"]
    #[inline]
    pub fn adcdiv_3(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_3)
    }
    #[doc = "ADC Clock Divider Select: 4"]
    #[inline]
    pub fn adcdiv_4(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_4)
    }
    #[doc = "ADC Clock Divider Select: 5"]
    #[inline]
    pub fn adcdiv_5(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_5)
    }
    #[doc = "ADC Clock Divider Select: 6"]
    #[inline]
    pub fn adcdiv_6(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_6)
    }
    #[doc = "ADC Clock Divider Select: 7"]
    #[inline]
    pub fn adcdiv_7(self) -> &'a mut W {
        self.variant(ADCDIVW::ADCDIV_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCISSH`"]
pub enum ADCISSHW {}
impl ADCISSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCISSHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCISSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCISSHW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCSHP`"]
pub enum ADCSHPW {}
impl ADCSHPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCSHPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSHPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSHPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCSHS`"]
pub enum ADCSHSW {
    #[doc = "ADC Sample/Hold Source: 0"]
    ADCSHS_0,
    #[doc = "ADC Sample/Hold Source: 1"]
    ADCSHS_1,
    #[doc = "ADC Sample/Hold Source: 2"]
    ADCSHS_2,
    #[doc = "ADC Sample/Hold Source: 3"]
    ADCSHS_3,
}
impl ADCSHSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCSHSW::ADCSHS_0 => 0,
            ADCSHSW::ADCSHS_1 => 1,
            ADCSHSW::ADCSHS_2 => 2,
            ADCSHSW::ADCSHS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCSHSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSHSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSHSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Sample/Hold Source: 0"]
    #[inline]
    pub fn adcshs_0(self) -> &'a mut W {
        self.variant(ADCSHSW::ADCSHS_0)
    }
    #[doc = "ADC Sample/Hold Source: 1"]
    #[inline]
    pub fn adcshs_1(self) -> &'a mut W {
        self.variant(ADCSHSW::ADCSHS_1)
    }
    #[doc = "ADC Sample/Hold Source: 2"]
    #[inline]
    pub fn adcshs_2(self) -> &'a mut W {
        self.variant(ADCSHSW::ADCSHS_2)
    }
    #[doc = "ADC Sample/Hold Source: 3"]
    #[inline]
    pub fn adcshs_3(self) -> &'a mut W {
        self.variant(ADCSHSW::ADCSHS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - ADC Busy"]
    #[inline]
    pub fn adcbusy(&self) -> ADCBUSYR {
        ADCBUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 1:2 - ADC Conversion Sequence Select 0"]
    #[inline]
    pub fn adcconseq(&self) -> ADCCONSEQR {
        ADCCONSEQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 3:4 - ADC Clock Source Select 0"]
    #[inline]
    pub fn adcssel(&self) -> ADCSSELR {
        ADCSSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 5:7 - ADC Clock Divider Select 0"]
    #[inline]
    pub fn adcdiv(&self) -> ADCDIVR {
        ADCDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - ADC Invert Sample Hold Signal"]
    #[inline]
    pub fn adcissh(&self) -> ADCISSHR {
        ADCISSHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 9 - ADC Sample/Hold Pulse Mode"]
    #[inline]
    pub fn adcshp(&self) -> ADCSHPR {
        ADCSHPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 10:11 - ADC Sample/Hold Source 0"]
    #[inline]
    pub fn adcshs(&self) -> ADCSHSR {
        ADCSHSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - ADC Busy"]
    #[inline]
    pub fn adcbusy(&mut self) -> _ADCBUSYW {
        _ADCBUSYW { w: self }
    }
    #[doc = "Bits 1:2 - ADC Conversion Sequence Select 0"]
    #[inline]
    pub fn adcconseq(&mut self) -> _ADCCONSEQW {
        _ADCCONSEQW { w: self }
    }
    #[doc = "Bits 3:4 - ADC Clock Source Select 0"]
    #[inline]
    pub fn adcssel(&mut self) -> _ADCSSELW {
        _ADCSSELW { w: self }
    }
    #[doc = "Bits 5:7 - ADC Clock Divider Select 0"]
    #[inline]
    pub fn adcdiv(&mut self) -> _ADCDIVW {
        _ADCDIVW { w: self }
    }
    #[doc = "Bit 8 - ADC Invert Sample Hold Signal"]
    #[inline]
    pub fn adcissh(&mut self) -> _ADCISSHW {
        _ADCISSHW { w: self }
    }
    #[doc = "Bit 9 - ADC Sample/Hold Pulse Mode"]
    #[inline]
    pub fn adcshp(&mut self) -> _ADCSHPW {
        _ADCSHPW { w: self }
    }
    #[doc = "Bits 10:11 - ADC Sample/Hold Source 0"]
    #[inline]
    pub fn adcshs(&mut self) -> _ADCSHSW {
        _ADCSHSW { w: self }
    }
}
