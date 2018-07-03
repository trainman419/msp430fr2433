#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TA2CCTL0 {
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
#[doc = "Possible values of the field `CCIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CCIFGR {
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
            CCIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIFGR {
        match value {
            i => CCIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `COV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl COVR {
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
            COVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COVR {
        match value {
            i => COVR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `OUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTR {
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
            OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTR {
        match value {
            i => OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `CCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CCIR {
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
            CCIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIR {
        match value {
            i => CCIR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `CCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CCIER {
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
            CCIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCIER {
        match value {
            i => CCIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `OUTMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTMODR {
    #[doc = "PWM output mode: 0 - output only"]
    OUTMOD_0,
    #[doc = "PWM output mode: 1 - set"]
    OUTMOD_1,
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    OUTMOD_2,
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    OUTMOD_3,
    #[doc = "PWM output mode: 4 - toggle"]
    OUTMOD_4,
    #[doc = "PWM output mode: 5 - Reset"]
    OUTMOD_5,
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    OUTMOD_6,
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    OUTMOD_7,
}
impl OUTMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTMODR::OUTMOD_0 => 0,
            OUTMODR::OUTMOD_1 => 1,
            OUTMODR::OUTMOD_2 => 2,
            OUTMODR::OUTMOD_3 => 3,
            OUTMODR::OUTMOD_4 => 4,
            OUTMODR::OUTMOD_5 => 5,
            OUTMODR::OUTMOD_6 => 6,
            OUTMODR::OUTMOD_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTMODR {
        match value {
            0 => OUTMODR::OUTMOD_0,
            1 => OUTMODR::OUTMOD_1,
            2 => OUTMODR::OUTMOD_2,
            3 => OUTMODR::OUTMOD_3,
            4 => OUTMODR::OUTMOD_4,
            5 => OUTMODR::OUTMOD_5,
            6 => OUTMODR::OUTMOD_6,
            7 => OUTMODR::OUTMOD_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTMOD_0`"]
    #[inline]
    pub fn is_outmod_0(&self) -> bool {
        *self == OUTMODR::OUTMOD_0
    }
    #[doc = "Checks if the value of the field is `OUTMOD_1`"]
    #[inline]
    pub fn is_outmod_1(&self) -> bool {
        *self == OUTMODR::OUTMOD_1
    }
    #[doc = "Checks if the value of the field is `OUTMOD_2`"]
    #[inline]
    pub fn is_outmod_2(&self) -> bool {
        *self == OUTMODR::OUTMOD_2
    }
    #[doc = "Checks if the value of the field is `OUTMOD_3`"]
    #[inline]
    pub fn is_outmod_3(&self) -> bool {
        *self == OUTMODR::OUTMOD_3
    }
    #[doc = "Checks if the value of the field is `OUTMOD_4`"]
    #[inline]
    pub fn is_outmod_4(&self) -> bool {
        *self == OUTMODR::OUTMOD_4
    }
    #[doc = "Checks if the value of the field is `OUTMOD_5`"]
    #[inline]
    pub fn is_outmod_5(&self) -> bool {
        *self == OUTMODR::OUTMOD_5
    }
    #[doc = "Checks if the value of the field is `OUTMOD_6`"]
    #[inline]
    pub fn is_outmod_6(&self) -> bool {
        *self == OUTMODR::OUTMOD_6
    }
    #[doc = "Checks if the value of the field is `OUTMOD_7`"]
    #[inline]
    pub fn is_outmod_7(&self) -> bool {
        *self == OUTMODR::OUTMOD_7
    }
}
#[doc = "Possible values of the field `CAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CAPR {
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
            CAPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAPR {
        match value {
            i => CAPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `SCCI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCCIR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SCCIR {
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
            SCCIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCCIR {
        match value {
            i => SCCIR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `SCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SCSR {
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
            SCSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCSR {
        match value {
            i => SCSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `CCIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCISR {
    #[doc = "Capture input select: 0 - CCIxA"]
    CCIS_0,
    #[doc = "Capture input select: 1 - CCIxB"]
    CCIS_1,
    #[doc = "Capture input select: 2 - GND"]
    CCIS_2,
    #[doc = "Capture input select: 3 - Vcc"]
    CCIS_3,
}
impl CCISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CCISR::CCIS_0 => 0,
            CCISR::CCIS_1 => 1,
            CCISR::CCIS_2 => 2,
            CCISR::CCIS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CCISR {
        match value {
            0 => CCISR::CCIS_0,
            1 => CCISR::CCIS_1,
            2 => CCISR::CCIS_2,
            3 => CCISR::CCIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCIS_0`"]
    #[inline]
    pub fn is_ccis_0(&self) -> bool {
        *self == CCISR::CCIS_0
    }
    #[doc = "Checks if the value of the field is `CCIS_1`"]
    #[inline]
    pub fn is_ccis_1(&self) -> bool {
        *self == CCISR::CCIS_1
    }
    #[doc = "Checks if the value of the field is `CCIS_2`"]
    #[inline]
    pub fn is_ccis_2(&self) -> bool {
        *self == CCISR::CCIS_2
    }
    #[doc = "Checks if the value of the field is `CCIS_3`"]
    #[inline]
    pub fn is_ccis_3(&self) -> bool {
        *self == CCISR::CCIS_3
    }
}
#[doc = "Possible values of the field `CM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMR {
    #[doc = "Capture mode: 0 - disabled"]
    CM_0,
    #[doc = "Capture mode: 1 - pos. edge"]
    CM_1,
    #[doc = "Capture mode: 1 - neg. edge"]
    CM_2,
    #[doc = "Capture mode: 1 - both edges"]
    CM_3,
}
impl CMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMR::CM_0 => 0,
            CMR::CM_1 => 1,
            CMR::CM_2 => 2,
            CMR::CM_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMR {
        match value {
            0 => CMR::CM_0,
            1 => CMR::CM_1,
            2 => CMR::CM_2,
            3 => CMR::CM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline]
    pub fn is_cm_0(&self) -> bool {
        *self == CMR::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_1`"]
    #[inline]
    pub fn is_cm_1(&self) -> bool {
        *self == CMR::CM_1
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline]
    pub fn is_cm_2(&self) -> bool {
        *self == CMR::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline]
    pub fn is_cm_3(&self) -> bool {
        *self == CMR::CM_3
    }
}
#[doc = "Values that can be written to the field `CCIFG`"]
pub enum CCIFGW {}
impl CCIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CCIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `COV`"]
pub enum COVW {}
impl COVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _COVW<'a> {
    w: &'a mut W,
}
impl<'a> _COVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COVW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `OUT`"]
pub enum OUTW {}
impl OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `CCI`"]
pub enum CCIW {}
impl CCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CCIW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `CCIE`"]
pub enum CCIEW {}
impl CCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `OUTMOD`"]
pub enum OUTMODW {
    #[doc = "PWM output mode: 0 - output only"]
    OUTMOD_0,
    #[doc = "PWM output mode: 1 - set"]
    OUTMOD_1,
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    OUTMOD_2,
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    OUTMOD_3,
    #[doc = "PWM output mode: 4 - toggle"]
    OUTMOD_4,
    #[doc = "PWM output mode: 5 - Reset"]
    OUTMOD_5,
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    OUTMOD_6,
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    OUTMOD_7,
}
impl OUTMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTMODW::OUTMOD_0 => 0,
            OUTMODW::OUTMOD_1 => 1,
            OUTMODW::OUTMOD_2 => 2,
            OUTMODW::OUTMOD_3 => 3,
            OUTMODW::OUTMOD_4 => 4,
            OUTMODW::OUTMOD_5 => 5,
            OUTMODW::OUTMOD_6 => 6,
            OUTMODW::OUTMOD_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTMODW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "PWM output mode: 0 - output only"]
    #[inline]
    pub fn outmod_0(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_0)
    }
    #[doc = "PWM output mode: 1 - set"]
    #[inline]
    pub fn outmod_1(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_1)
    }
    #[doc = "PWM output mode: 2 - PWM toggle/reset"]
    #[inline]
    pub fn outmod_2(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_2)
    }
    #[doc = "PWM output mode: 3 - PWM set/reset"]
    #[inline]
    pub fn outmod_3(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_3)
    }
    #[doc = "PWM output mode: 4 - toggle"]
    #[inline]
    pub fn outmod_4(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_4)
    }
    #[doc = "PWM output mode: 5 - Reset"]
    #[inline]
    pub fn outmod_5(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_5)
    }
    #[doc = "PWM output mode: 6 - PWM toggle/set"]
    #[inline]
    pub fn outmod_6(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_6)
    }
    #[doc = "PWM output mode: 7 - PWM reset/set"]
    #[inline]
    pub fn outmod_7(self) -> &'a mut W {
        self.variant(OUTMODW::OUTMOD_7)
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
#[doc = "Values that can be written to the field `CAP`"]
pub enum CAPW {}
impl CAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CAPW<'a> {
    w: &'a mut W,
}
impl<'a> _CAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `SCCI`"]
pub enum SCCIW {}
impl SCCIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SCCIW<'a> {
    w: &'a mut W,
}
impl<'a> _SCCIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCCIW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `SCS`"]
pub enum SCSW {}
impl SCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SCSW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `CCIS`"]
pub enum CCISW {
    #[doc = "Capture input select: 0 - CCIxA"]
    CCIS_0,
    #[doc = "Capture input select: 1 - CCIxB"]
    CCIS_1,
    #[doc = "Capture input select: 2 - GND"]
    CCIS_2,
    #[doc = "Capture input select: 3 - Vcc"]
    CCIS_3,
}
impl CCISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CCISW::CCIS_0 => 0,
            CCISW::CCIS_1 => 1,
            CCISW::CCIS_2 => 2,
            CCISW::CCIS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCISW<'a> {
    w: &'a mut W,
}
impl<'a> _CCISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Capture input select: 0 - CCIxA"]
    #[inline]
    pub fn ccis_0(self) -> &'a mut W {
        self.variant(CCISW::CCIS_0)
    }
    #[doc = "Capture input select: 1 - CCIxB"]
    #[inline]
    pub fn ccis_1(self) -> &'a mut W {
        self.variant(CCISW::CCIS_1)
    }
    #[doc = "Capture input select: 2 - GND"]
    #[inline]
    pub fn ccis_2(self) -> &'a mut W {
        self.variant(CCISW::CCIS_2)
    }
    #[doc = "Capture input select: 3 - Vcc"]
    #[inline]
    pub fn ccis_3(self) -> &'a mut W {
        self.variant(CCISW::CCIS_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CM`"]
pub enum CMW {
    #[doc = "Capture mode: 0 - disabled"]
    CM_0,
    #[doc = "Capture mode: 1 - pos. edge"]
    CM_1,
    #[doc = "Capture mode: 1 - neg. edge"]
    CM_2,
    #[doc = "Capture mode: 1 - both edges"]
    CM_3,
}
impl CMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMW::CM_0 => 0,
            CMW::CM_1 => 1,
            CMW::CM_2 => 2,
            CMW::CM_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMW<'a> {
    w: &'a mut W,
}
impl<'a> _CMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Capture mode: 0 - disabled"]
    #[inline]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CMW::CM_0)
    }
    #[doc = "Capture mode: 1 - pos. edge"]
    #[inline]
    pub fn cm_1(self) -> &'a mut W {
        self.variant(CMW::CM_1)
    }
    #[doc = "Capture mode: 1 - neg. edge"]
    #[inline]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CMW::CM_2)
    }
    #[doc = "Capture mode: 1 - both edges"]
    #[inline]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CMW::CM_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline]
    pub fn ccifg(&self) -> CCIFGR {
        CCIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline]
    pub fn cov(&self) -> COVR {
        COVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline]
    pub fn out(&self) -> OUTR {
        OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline]
    pub fn cci(&self) -> CCIR {
        CCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline]
    pub fn ccie(&self) -> CCIER {
        CCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline]
    pub fn outmod(&self) -> OUTMODR {
        OUTMODR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline]
    pub fn cap(&self) -> CAPR {
        CAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 10 - Latched capture signal (read)"]
    #[inline]
    pub fn scci(&self) -> SCCIR {
        SCCIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline]
    pub fn scs(&self) -> SCSR {
        SCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline]
    pub fn ccis(&self) -> CCISR {
        CCISR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline]
    pub fn cm(&self) -> CMR {
        CMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - Capture/compare interrupt flag"]
    #[inline]
    pub fn ccifg(&mut self) -> _CCIFGW {
        _CCIFGW { w: self }
    }
    #[doc = "Bit 1 - Capture/compare overflow flag"]
    #[inline]
    pub fn cov(&mut self) -> _COVW {
        _COVW { w: self }
    }
    #[doc = "Bit 2 - PWM Output signal if output mode 0"]
    #[inline]
    pub fn out(&mut self) -> _OUTW {
        _OUTW { w: self }
    }
    #[doc = "Bit 3 - Capture input signal (read)"]
    #[inline]
    pub fn cci(&mut self) -> _CCIW {
        _CCIW { w: self }
    }
    #[doc = "Bit 4 - Capture/compare interrupt enable"]
    #[inline]
    pub fn ccie(&mut self) -> _CCIEW {
        _CCIEW { w: self }
    }
    #[doc = "Bits 5:7 - Output mode 2"]
    #[inline]
    pub fn outmod(&mut self) -> _OUTMODW {
        _OUTMODW { w: self }
    }
    #[doc = "Bit 8 - Capture mode: 1 /Compare mode : 0"]
    #[inline]
    pub fn cap(&mut self) -> _CAPW {
        _CAPW { w: self }
    }
    #[doc = "Bit 10 - Latched capture signal (read)"]
    #[inline]
    pub fn scci(&mut self) -> _SCCIW {
        _SCCIW { w: self }
    }
    #[doc = "Bit 11 - Capture sychronize"]
    #[inline]
    pub fn scs(&mut self) -> _SCSW {
        _SCSW { w: self }
    }
    #[doc = "Bits 12:13 - Capture input select 1"]
    #[inline]
    pub fn ccis(&mut self) -> _CCISW {
        _CCISW { w: self }
    }
    #[doc = "Bits 14:15 - Capture mode 1"]
    #[inline]
    pub fn cm(&mut self) -> _CMW {
        _CMW { w: self }
    }
}
