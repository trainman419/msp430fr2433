#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ADCCTL2 {
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
#[doc = "Possible values of the field `ADCSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCSRR {
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
            ADCSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCSRR {
        match value {
            i => ADCSRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCDFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADCDFR {
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
            ADCDFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCDFR {
        match value {
            i => ADCDFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ADCRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCRESR {
    #[doc = "8 bit"]
    ADCRES_0,
    #[doc = "10 bit"]
    ADCRES_1,
    #[doc = "Reserved"]
    ADCRES_2,
    #[doc = "Reserved"]
    ADCRES_3,
}
impl ADCRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCRESR::ADCRES_0 => 0,
            ADCRESR::ADCRES_1 => 1,
            ADCRESR::ADCRES_2 => 2,
            ADCRESR::ADCRES_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCRESR {
        match value {
            0 => ADCRESR::ADCRES_0,
            1 => ADCRESR::ADCRES_1,
            2 => ADCRESR::ADCRES_2,
            3 => ADCRESR::ADCRES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCRES_0`"]
    #[inline]
    pub fn is_adcres_0(&self) -> bool {
        *self == ADCRESR::ADCRES_0
    }
    #[doc = "Checks if the value of the field is `ADCRES_1`"]
    #[inline]
    pub fn is_adcres_1(&self) -> bool {
        *self == ADCRESR::ADCRES_1
    }
    #[doc = "Checks if the value of the field is `ADCRES_2`"]
    #[inline]
    pub fn is_adcres_2(&self) -> bool {
        *self == ADCRESR::ADCRES_2
    }
    #[doc = "Checks if the value of the field is `ADCRES_3`"]
    #[inline]
    pub fn is_adcres_3(&self) -> bool {
        *self == ADCRESR::ADCRES_3
    }
}
#[doc = "Possible values of the field `ADCPDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCPDIVR {
    #[doc = "ADC predivider /1"]
    ADCPDIV_0,
    #[doc = "ADC predivider /2"]
    ADCPDIV_1,
    #[doc = "ADC predivider /64"]
    ADCPDIV_2,
    #[doc = "ADC predivider reserved"]
    ADCPDIV_3,
}
impl ADCPDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCPDIVR::ADCPDIV_0 => 0,
            ADCPDIVR::ADCPDIV_1 => 1,
            ADCPDIVR::ADCPDIV_2 => 2,
            ADCPDIVR::ADCPDIV_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCPDIVR {
        match value {
            0 => ADCPDIVR::ADCPDIV_0,
            1 => ADCPDIVR::ADCPDIV_1,
            2 => ADCPDIVR::ADCPDIV_2,
            3 => ADCPDIVR::ADCPDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_0`"]
    #[inline]
    pub fn is_adcpdiv_0(&self) -> bool {
        *self == ADCPDIVR::ADCPDIV_0
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_1`"]
    #[inline]
    pub fn is_adcpdiv_1(&self) -> bool {
        *self == ADCPDIVR::ADCPDIV_1
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_2`"]
    #[inline]
    pub fn is_adcpdiv_2(&self) -> bool {
        *self == ADCPDIVR::ADCPDIV_2
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_3`"]
    #[inline]
    pub fn is_adcpdiv_3(&self) -> bool {
        *self == ADCPDIVR::ADCPDIV_3
    }
}
#[doc = "Values that can be written to the field `ADCSR`"]
pub enum ADCSRW {}
impl ADCSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCSRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSRW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCDF`"]
pub enum ADCDFW {}
impl ADCDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADCDFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCDFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ADCRES`"]
pub enum ADCRESW {
    #[doc = "8 bit"]
    ADCRES_0,
    #[doc = "10 bit"]
    ADCRES_1,
    #[doc = "Reserved"]
    ADCRES_2,
    #[doc = "Reserved"]
    ADCRES_3,
}
impl ADCRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCRESW::ADCRES_0 => 0,
            ADCRESW::ADCRES_1 => 1,
            ADCRESW::ADCRES_2 => 2,
            ADCRESW::ADCRES_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCRESW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCRESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 bit"]
    #[inline]
    pub fn adcres_0(self) -> &'a mut W {
        self.variant(ADCRESW::ADCRES_0)
    }
    #[doc = "10 bit"]
    #[inline]
    pub fn adcres_1(self) -> &'a mut W {
        self.variant(ADCRESW::ADCRES_1)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn adcres_2(self) -> &'a mut W {
        self.variant(ADCRESW::ADCRES_2)
    }
    #[doc = "Reserved"]
    #[inline]
    pub fn adcres_3(self) -> &'a mut W {
        self.variant(ADCRESW::ADCRES_3)
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
#[doc = "Values that can be written to the field `ADCPDIV`"]
pub enum ADCPDIVW {
    #[doc = "ADC predivider /1"]
    ADCPDIV_0,
    #[doc = "ADC predivider /2"]
    ADCPDIV_1,
    #[doc = "ADC predivider /64"]
    ADCPDIV_2,
    #[doc = "ADC predivider reserved"]
    ADCPDIV_3,
}
impl ADCPDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCPDIVW::ADCPDIV_0 => 0,
            ADCPDIVW::ADCPDIV_1 => 1,
            ADCPDIVW::ADCPDIV_2 => 2,
            ADCPDIVW::ADCPDIV_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCPDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCPDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCPDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC predivider /1"]
    #[inline]
    pub fn adcpdiv_0(self) -> &'a mut W {
        self.variant(ADCPDIVW::ADCPDIV_0)
    }
    #[doc = "ADC predivider /2"]
    #[inline]
    pub fn adcpdiv_1(self) -> &'a mut W {
        self.variant(ADCPDIVW::ADCPDIV_1)
    }
    #[doc = "ADC predivider /64"]
    #[inline]
    pub fn adcpdiv_2(self) -> &'a mut W {
        self.variant(ADCPDIVW::ADCPDIV_2)
    }
    #[doc = "ADC predivider reserved"]
    #[inline]
    pub fn adcpdiv_3(self) -> &'a mut W {
        self.variant(ADCPDIVW::ADCPDIV_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline]
    pub fn adcsr(&self) -> ADCSRR {
        ADCSRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline]
    pub fn adcdf(&self) -> ADCDFR {
        ADCDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline]
    pub fn adcres(&self) -> ADCRESR {
        ADCRESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline]
    pub fn adcpdiv(&self) -> ADCPDIVR {
        ADCPDIVR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline]
    pub fn adcsr(&mut self) -> _ADCSRW {
        _ADCSRW { w: self }
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline]
    pub fn adcdf(&mut self) -> _ADCDFW {
        _ADCDFW { w: self }
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline]
    pub fn adcres(&mut self) -> _ADCRESW {
        _ADCRESW { w: self }
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline]
    pub fn adcpdiv(&mut self) -> _ADCPDIVW {
        _ADCPDIVW { w: self }
    }
}
