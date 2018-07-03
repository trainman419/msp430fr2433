#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::ADCMCTL0 {
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
#[doc = "Possible values of the field `ADCINCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCINCHR {
    #[doc = "ADC Input Channel 0"]
    ADCINCH_0,
    #[doc = "ADC Input Channel 1"]
    ADCINCH_1,
    #[doc = "ADC Input Channel 2"]
    ADCINCH_2,
    #[doc = "ADC Input Channel 3"]
    ADCINCH_3,
    #[doc = "ADC Input Channel 4"]
    ADCINCH_4,
    #[doc = "ADC Input Channel 5"]
    ADCINCH_5,
    #[doc = "ADC Input Channel 6"]
    ADCINCH_6,
    #[doc = "ADC Input Channel 7"]
    ADCINCH_7,
    #[doc = "ADC Input Channel 8"]
    ADCINCH_8,
    #[doc = "ADC Input Channel 9"]
    ADCINCH_9,
    #[doc = "ADC Input Channel 10"]
    ADCINCH_10,
    #[doc = "ADC Input Channel 11"]
    ADCINCH_11,
    #[doc = "ADC Input Channel 12"]
    ADCINCH_12,
    #[doc = "ADC Input Channel 13"]
    ADCINCH_13,
    #[doc = "ADC Input Channel 14"]
    ADCINCH_14,
    #[doc = "ADC Input Channel 15"]
    ADCINCH_15,
}
impl ADCINCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCINCHR::ADCINCH_0 => 0,
            ADCINCHR::ADCINCH_1 => 1,
            ADCINCHR::ADCINCH_2 => 2,
            ADCINCHR::ADCINCH_3 => 3,
            ADCINCHR::ADCINCH_4 => 4,
            ADCINCHR::ADCINCH_5 => 5,
            ADCINCHR::ADCINCH_6 => 6,
            ADCINCHR::ADCINCH_7 => 7,
            ADCINCHR::ADCINCH_8 => 8,
            ADCINCHR::ADCINCH_9 => 9,
            ADCINCHR::ADCINCH_10 => 10,
            ADCINCHR::ADCINCH_11 => 11,
            ADCINCHR::ADCINCH_12 => 12,
            ADCINCHR::ADCINCH_13 => 13,
            ADCINCHR::ADCINCH_14 => 14,
            ADCINCHR::ADCINCH_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCINCHR {
        match value {
            0 => ADCINCHR::ADCINCH_0,
            1 => ADCINCHR::ADCINCH_1,
            2 => ADCINCHR::ADCINCH_2,
            3 => ADCINCHR::ADCINCH_3,
            4 => ADCINCHR::ADCINCH_4,
            5 => ADCINCHR::ADCINCH_5,
            6 => ADCINCHR::ADCINCH_6,
            7 => ADCINCHR::ADCINCH_7,
            8 => ADCINCHR::ADCINCH_8,
            9 => ADCINCHR::ADCINCH_9,
            10 => ADCINCHR::ADCINCH_10,
            11 => ADCINCHR::ADCINCH_11,
            12 => ADCINCHR::ADCINCH_12,
            13 => ADCINCHR::ADCINCH_13,
            14 => ADCINCHR::ADCINCH_14,
            15 => ADCINCHR::ADCINCH_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCINCH_0`"]
    #[inline]
    pub fn is_adcinch_0(&self) -> bool {
        *self == ADCINCHR::ADCINCH_0
    }
    #[doc = "Checks if the value of the field is `ADCINCH_1`"]
    #[inline]
    pub fn is_adcinch_1(&self) -> bool {
        *self == ADCINCHR::ADCINCH_1
    }
    #[doc = "Checks if the value of the field is `ADCINCH_2`"]
    #[inline]
    pub fn is_adcinch_2(&self) -> bool {
        *self == ADCINCHR::ADCINCH_2
    }
    #[doc = "Checks if the value of the field is `ADCINCH_3`"]
    #[inline]
    pub fn is_adcinch_3(&self) -> bool {
        *self == ADCINCHR::ADCINCH_3
    }
    #[doc = "Checks if the value of the field is `ADCINCH_4`"]
    #[inline]
    pub fn is_adcinch_4(&self) -> bool {
        *self == ADCINCHR::ADCINCH_4
    }
    #[doc = "Checks if the value of the field is `ADCINCH_5`"]
    #[inline]
    pub fn is_adcinch_5(&self) -> bool {
        *self == ADCINCHR::ADCINCH_5
    }
    #[doc = "Checks if the value of the field is `ADCINCH_6`"]
    #[inline]
    pub fn is_adcinch_6(&self) -> bool {
        *self == ADCINCHR::ADCINCH_6
    }
    #[doc = "Checks if the value of the field is `ADCINCH_7`"]
    #[inline]
    pub fn is_adcinch_7(&self) -> bool {
        *self == ADCINCHR::ADCINCH_7
    }
    #[doc = "Checks if the value of the field is `ADCINCH_8`"]
    #[inline]
    pub fn is_adcinch_8(&self) -> bool {
        *self == ADCINCHR::ADCINCH_8
    }
    #[doc = "Checks if the value of the field is `ADCINCH_9`"]
    #[inline]
    pub fn is_adcinch_9(&self) -> bool {
        *self == ADCINCHR::ADCINCH_9
    }
    #[doc = "Checks if the value of the field is `ADCINCH_10`"]
    #[inline]
    pub fn is_adcinch_10(&self) -> bool {
        *self == ADCINCHR::ADCINCH_10
    }
    #[doc = "Checks if the value of the field is `ADCINCH_11`"]
    #[inline]
    pub fn is_adcinch_11(&self) -> bool {
        *self == ADCINCHR::ADCINCH_11
    }
    #[doc = "Checks if the value of the field is `ADCINCH_12`"]
    #[inline]
    pub fn is_adcinch_12(&self) -> bool {
        *self == ADCINCHR::ADCINCH_12
    }
    #[doc = "Checks if the value of the field is `ADCINCH_13`"]
    #[inline]
    pub fn is_adcinch_13(&self) -> bool {
        *self == ADCINCHR::ADCINCH_13
    }
    #[doc = "Checks if the value of the field is `ADCINCH_14`"]
    #[inline]
    pub fn is_adcinch_14(&self) -> bool {
        *self == ADCINCHR::ADCINCH_14
    }
    #[doc = "Checks if the value of the field is `ADCINCH_15`"]
    #[inline]
    pub fn is_adcinch_15(&self) -> bool {
        *self == ADCINCHR::ADCINCH_15
    }
}
#[doc = "Possible values of the field `ADCSREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCSREFR {
    #[doc = "ADC Select Reference 0"]
    ADCSREF_0,
    #[doc = "ADC Select Reference 1"]
    ADCSREF_1,
    #[doc = "ADC Select Reference 2"]
    ADCSREF_2,
    #[doc = "ADC Select Reference 3"]
    ADCSREF_3,
    #[doc = "ADC Select Reference 4"]
    ADCSREF_4,
    #[doc = "ADC Select Reference 5"]
    ADCSREF_5,
    #[doc = "ADC Select Reference 6"]
    ADCSREF_6,
    #[doc = "ADC Select Reference 7"]
    ADCSREF_7,
}
impl ADCSREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADCSREFR::ADCSREF_0 => 0,
            ADCSREFR::ADCSREF_1 => 1,
            ADCSREFR::ADCSREF_2 => 2,
            ADCSREFR::ADCSREF_3 => 3,
            ADCSREFR::ADCSREF_4 => 4,
            ADCSREFR::ADCSREF_5 => 5,
            ADCSREFR::ADCSREF_6 => 6,
            ADCSREFR::ADCSREF_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADCSREFR {
        match value {
            0 => ADCSREFR::ADCSREF_0,
            1 => ADCSREFR::ADCSREF_1,
            2 => ADCSREFR::ADCSREF_2,
            3 => ADCSREFR::ADCSREF_3,
            4 => ADCSREFR::ADCSREF_4,
            5 => ADCSREFR::ADCSREF_5,
            6 => ADCSREFR::ADCSREF_6,
            7 => ADCSREFR::ADCSREF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSREF_0`"]
    #[inline]
    pub fn is_adcsref_0(&self) -> bool {
        *self == ADCSREFR::ADCSREF_0
    }
    #[doc = "Checks if the value of the field is `ADCSREF_1`"]
    #[inline]
    pub fn is_adcsref_1(&self) -> bool {
        *self == ADCSREFR::ADCSREF_1
    }
    #[doc = "Checks if the value of the field is `ADCSREF_2`"]
    #[inline]
    pub fn is_adcsref_2(&self) -> bool {
        *self == ADCSREFR::ADCSREF_2
    }
    #[doc = "Checks if the value of the field is `ADCSREF_3`"]
    #[inline]
    pub fn is_adcsref_3(&self) -> bool {
        *self == ADCSREFR::ADCSREF_3
    }
    #[doc = "Checks if the value of the field is `ADCSREF_4`"]
    #[inline]
    pub fn is_adcsref_4(&self) -> bool {
        *self == ADCSREFR::ADCSREF_4
    }
    #[doc = "Checks if the value of the field is `ADCSREF_5`"]
    #[inline]
    pub fn is_adcsref_5(&self) -> bool {
        *self == ADCSREFR::ADCSREF_5
    }
    #[doc = "Checks if the value of the field is `ADCSREF_6`"]
    #[inline]
    pub fn is_adcsref_6(&self) -> bool {
        *self == ADCSREFR::ADCSREF_6
    }
    #[doc = "Checks if the value of the field is `ADCSREF_7`"]
    #[inline]
    pub fn is_adcsref_7(&self) -> bool {
        *self == ADCSREFR::ADCSREF_7
    }
}
#[doc = "Values that can be written to the field `ADCINCH`"]
pub enum ADCINCHW {
    #[doc = "ADC Input Channel 0"]
    ADCINCH_0,
    #[doc = "ADC Input Channel 1"]
    ADCINCH_1,
    #[doc = "ADC Input Channel 2"]
    ADCINCH_2,
    #[doc = "ADC Input Channel 3"]
    ADCINCH_3,
    #[doc = "ADC Input Channel 4"]
    ADCINCH_4,
    #[doc = "ADC Input Channel 5"]
    ADCINCH_5,
    #[doc = "ADC Input Channel 6"]
    ADCINCH_6,
    #[doc = "ADC Input Channel 7"]
    ADCINCH_7,
    #[doc = "ADC Input Channel 8"]
    ADCINCH_8,
    #[doc = "ADC Input Channel 9"]
    ADCINCH_9,
    #[doc = "ADC Input Channel 10"]
    ADCINCH_10,
    #[doc = "ADC Input Channel 11"]
    ADCINCH_11,
    #[doc = "ADC Input Channel 12"]
    ADCINCH_12,
    #[doc = "ADC Input Channel 13"]
    ADCINCH_13,
    #[doc = "ADC Input Channel 14"]
    ADCINCH_14,
    #[doc = "ADC Input Channel 15"]
    ADCINCH_15,
}
impl ADCINCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCINCHW::ADCINCH_0 => 0,
            ADCINCHW::ADCINCH_1 => 1,
            ADCINCHW::ADCINCH_2 => 2,
            ADCINCHW::ADCINCH_3 => 3,
            ADCINCHW::ADCINCH_4 => 4,
            ADCINCHW::ADCINCH_5 => 5,
            ADCINCHW::ADCINCH_6 => 6,
            ADCINCHW::ADCINCH_7 => 7,
            ADCINCHW::ADCINCH_8 => 8,
            ADCINCHW::ADCINCH_9 => 9,
            ADCINCHW::ADCINCH_10 => 10,
            ADCINCHW::ADCINCH_11 => 11,
            ADCINCHW::ADCINCH_12 => 12,
            ADCINCHW::ADCINCH_13 => 13,
            ADCINCHW::ADCINCH_14 => 14,
            ADCINCHW::ADCINCH_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCINCHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCINCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCINCHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Input Channel 0"]
    #[inline]
    pub fn adcinch_0(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_0)
    }
    #[doc = "ADC Input Channel 1"]
    #[inline]
    pub fn adcinch_1(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_1)
    }
    #[doc = "ADC Input Channel 2"]
    #[inline]
    pub fn adcinch_2(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_2)
    }
    #[doc = "ADC Input Channel 3"]
    #[inline]
    pub fn adcinch_3(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_3)
    }
    #[doc = "ADC Input Channel 4"]
    #[inline]
    pub fn adcinch_4(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_4)
    }
    #[doc = "ADC Input Channel 5"]
    #[inline]
    pub fn adcinch_5(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_5)
    }
    #[doc = "ADC Input Channel 6"]
    #[inline]
    pub fn adcinch_6(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_6)
    }
    #[doc = "ADC Input Channel 7"]
    #[inline]
    pub fn adcinch_7(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_7)
    }
    #[doc = "ADC Input Channel 8"]
    #[inline]
    pub fn adcinch_8(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_8)
    }
    #[doc = "ADC Input Channel 9"]
    #[inline]
    pub fn adcinch_9(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_9)
    }
    #[doc = "ADC Input Channel 10"]
    #[inline]
    pub fn adcinch_10(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_10)
    }
    #[doc = "ADC Input Channel 11"]
    #[inline]
    pub fn adcinch_11(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_11)
    }
    #[doc = "ADC Input Channel 12"]
    #[inline]
    pub fn adcinch_12(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_12)
    }
    #[doc = "ADC Input Channel 13"]
    #[inline]
    pub fn adcinch_13(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_13)
    }
    #[doc = "ADC Input Channel 14"]
    #[inline]
    pub fn adcinch_14(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_14)
    }
    #[doc = "ADC Input Channel 15"]
    #[inline]
    pub fn adcinch_15(self) -> &'a mut W {
        self.variant(ADCINCHW::ADCINCH_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADCSREF`"]
pub enum ADCSREFW {
    #[doc = "ADC Select Reference 0"]
    ADCSREF_0,
    #[doc = "ADC Select Reference 1"]
    ADCSREF_1,
    #[doc = "ADC Select Reference 2"]
    ADCSREF_2,
    #[doc = "ADC Select Reference 3"]
    ADCSREF_3,
    #[doc = "ADC Select Reference 4"]
    ADCSREF_4,
    #[doc = "ADC Select Reference 5"]
    ADCSREF_5,
    #[doc = "ADC Select Reference 6"]
    ADCSREF_6,
    #[doc = "ADC Select Reference 7"]
    ADCSREF_7,
}
impl ADCSREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADCSREFW::ADCSREF_0 => 0,
            ADCSREFW::ADCSREF_1 => 1,
            ADCSREFW::ADCSREF_2 => 2,
            ADCSREFW::ADCSREF_3 => 3,
            ADCSREFW::ADCSREF_4 => 4,
            ADCSREFW::ADCSREF_5 => 5,
            ADCSREFW::ADCSREF_6 => 6,
            ADCSREFW::ADCSREF_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCSREFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCSREFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC Select Reference 0"]
    #[inline]
    pub fn adcsref_0(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_0)
    }
    #[doc = "ADC Select Reference 1"]
    #[inline]
    pub fn adcsref_1(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_1)
    }
    #[doc = "ADC Select Reference 2"]
    #[inline]
    pub fn adcsref_2(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_2)
    }
    #[doc = "ADC Select Reference 3"]
    #[inline]
    pub fn adcsref_3(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_3)
    }
    #[doc = "ADC Select Reference 4"]
    #[inline]
    pub fn adcsref_4(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_4)
    }
    #[doc = "ADC Select Reference 5"]
    #[inline]
    pub fn adcsref_5(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_5)
    }
    #[doc = "ADC Select Reference 6"]
    #[inline]
    pub fn adcsref_6(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_6)
    }
    #[doc = "ADC Select Reference 7"]
    #[inline]
    pub fn adcsref_7(self) -> &'a mut W {
        self.variant(ADCSREFW::ADCSREF_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline]
    pub fn adcinch(&self) -> ADCINCHR {
        ADCINCHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline]
    pub fn adcsref(&self) -> ADCSREFR {
        ADCSREFR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:3 - ADC Input Channel Select Bit 0"]
    #[inline]
    pub fn adcinch(&mut self) -> _ADCINCHW {
        _ADCINCHW { w: self }
    }
    #[doc = "Bits 4:6 - ADC Select Reference Bit 0"]
    #[inline]
    pub fn adcsref(&mut self) -> _ADCSREFW {
        _ADCSREFW { w: self }
    }
}
