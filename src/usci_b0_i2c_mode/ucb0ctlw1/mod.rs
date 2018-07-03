#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::UCB0CTLW1 {
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
#[doc = "Possible values of the field `UCGLIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCGLITR {
    #[doc = "USCI Deglitch time: 0"]
    UCGLIT_0,
    #[doc = "USCI Deglitch time: 1"]
    UCGLIT_1,
    #[doc = "USCI Deglitch time: 2"]
    UCGLIT_2,
    #[doc = "USCI Deglitch time: 3"]
    UCGLIT_3,
}
impl UCGLITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UCGLITR::UCGLIT_0 => 0,
            UCGLITR::UCGLIT_1 => 1,
            UCGLITR::UCGLIT_2 => 2,
            UCGLITR::UCGLIT_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UCGLITR {
        match value {
            0 => UCGLITR::UCGLIT_0,
            1 => UCGLITR::UCGLIT_1,
            2 => UCGLITR::UCGLIT_2,
            3 => UCGLITR::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline]
    pub fn is_ucglit_0(&self) -> bool {
        *self == UCGLITR::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline]
    pub fn is_ucglit_1(&self) -> bool {
        *self == UCGLITR::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline]
    pub fn is_ucglit_2(&self) -> bool {
        *self == UCGLITR::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline]
    pub fn is_ucglit_3(&self) -> bool {
        *self == UCGLITR::UCGLIT_3
    }
}
#[doc = "Possible values of the field `UCASTP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCASTPR {
    #[doc = "USCI Automatic Stop condition generation: 0"]
    UCASTP_0,
    #[doc = "USCI Automatic Stop condition generation: 1"]
    UCASTP_1,
    #[doc = "USCI Automatic Stop condition generation: 2"]
    UCASTP_2,
    #[doc = "USCI Automatic Stop condition generation: 3"]
    UCASTP_3,
}
impl UCASTPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UCASTPR::UCASTP_0 => 0,
            UCASTPR::UCASTP_1 => 1,
            UCASTPR::UCASTP_2 => 2,
            UCASTPR::UCASTP_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UCASTPR {
        match value {
            0 => UCASTPR::UCASTP_0,
            1 => UCASTPR::UCASTP_1,
            2 => UCASTPR::UCASTP_2,
            3 => UCASTPR::UCASTP_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCASTP_0`"]
    #[inline]
    pub fn is_ucastp_0(&self) -> bool {
        *self == UCASTPR::UCASTP_0
    }
    #[doc = "Checks if the value of the field is `UCASTP_1`"]
    #[inline]
    pub fn is_ucastp_1(&self) -> bool {
        *self == UCASTPR::UCASTP_1
    }
    #[doc = "Checks if the value of the field is `UCASTP_2`"]
    #[inline]
    pub fn is_ucastp_2(&self) -> bool {
        *self == UCASTPR::UCASTP_2
    }
    #[doc = "Checks if the value of the field is `UCASTP_3`"]
    #[inline]
    pub fn is_ucastp_3(&self) -> bool {
        *self == UCASTPR::UCASTP_3
    }
}
#[doc = "Possible values of the field `UCSWACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSWACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCSWACKR {
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
            UCSWACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCSWACKR {
        match value {
            i => UCSWACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCSTPNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTPNACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCSTPNACKR {
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
            UCSTPNACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCSTPNACKR {
        match value {
            i => UCSTPNACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `UCCLTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCLTOR {
    #[doc = "USCI Clock low timeout: 0"]
    UCCLTO_0,
    #[doc = "USCI Clock low timeout: 1"]
    UCCLTO_1,
    #[doc = "USCI Clock low timeout: 2"]
    UCCLTO_2,
    #[doc = "USCI Clock low timeout: 3"]
    UCCLTO_3,
}
impl UCCLTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UCCLTOR::UCCLTO_0 => 0,
            UCCLTOR::UCCLTO_1 => 1,
            UCCLTOR::UCCLTO_2 => 2,
            UCCLTOR::UCCLTO_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UCCLTOR {
        match value {
            0 => UCCLTOR::UCCLTO_0,
            1 => UCCLTOR::UCCLTO_1,
            2 => UCCLTOR::UCCLTO_2,
            3 => UCCLTOR::UCCLTO_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCCLTO_0`"]
    #[inline]
    pub fn is_ucclto_0(&self) -> bool {
        *self == UCCLTOR::UCCLTO_0
    }
    #[doc = "Checks if the value of the field is `UCCLTO_1`"]
    #[inline]
    pub fn is_ucclto_1(&self) -> bool {
        *self == UCCLTOR::UCCLTO_1
    }
    #[doc = "Checks if the value of the field is `UCCLTO_2`"]
    #[inline]
    pub fn is_ucclto_2(&self) -> bool {
        *self == UCCLTOR::UCCLTO_2
    }
    #[doc = "Checks if the value of the field is `UCCLTO_3`"]
    #[inline]
    pub fn is_ucclto_3(&self) -> bool {
        *self == UCCLTOR::UCCLTO_3
    }
}
#[doc = "Possible values of the field `UCETXINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCETXINTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UCETXINTR {
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
            UCETXINTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UCETXINTR {
        match value {
            i => UCETXINTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `UCGLIT`"]
pub enum UCGLITW {
    #[doc = "USCI Deglitch time: 0"]
    UCGLIT_0,
    #[doc = "USCI Deglitch time: 1"]
    UCGLIT_1,
    #[doc = "USCI Deglitch time: 2"]
    UCGLIT_2,
    #[doc = "USCI Deglitch time: 3"]
    UCGLIT_3,
}
impl UCGLITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UCGLITW::UCGLIT_0 => 0,
            UCGLITW::UCGLIT_1 => 1,
            UCGLITW::UCGLIT_2 => 2,
            UCGLITW::UCGLIT_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UCGLITW<'a> {
    w: &'a mut W,
}
impl<'a> _UCGLITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCGLITW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "USCI Deglitch time: 0"]
    #[inline]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_0)
    }
    #[doc = "USCI Deglitch time: 1"]
    #[inline]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_1)
    }
    #[doc = "USCI Deglitch time: 2"]
    #[inline]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_2)
    }
    #[doc = "USCI Deglitch time: 3"]
    #[inline]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLITW::UCGLIT_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCASTP`"]
pub enum UCASTPW {
    #[doc = "USCI Automatic Stop condition generation: 0"]
    UCASTP_0,
    #[doc = "USCI Automatic Stop condition generation: 1"]
    UCASTP_1,
    #[doc = "USCI Automatic Stop condition generation: 2"]
    UCASTP_2,
    #[doc = "USCI Automatic Stop condition generation: 3"]
    UCASTP_3,
}
impl UCASTPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UCASTPW::UCASTP_0 => 0,
            UCASTPW::UCASTP_1 => 1,
            UCASTPW::UCASTP_2 => 2,
            UCASTPW::UCASTP_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UCASTPW<'a> {
    w: &'a mut W,
}
impl<'a> _UCASTPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCASTPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "USCI Automatic Stop condition generation: 0"]
    #[inline]
    pub fn ucastp_0(self) -> &'a mut W {
        self.variant(UCASTPW::UCASTP_0)
    }
    #[doc = "USCI Automatic Stop condition generation: 1"]
    #[inline]
    pub fn ucastp_1(self) -> &'a mut W {
        self.variant(UCASTPW::UCASTP_1)
    }
    #[doc = "USCI Automatic Stop condition generation: 2"]
    #[inline]
    pub fn ucastp_2(self) -> &'a mut W {
        self.variant(UCASTPW::UCASTP_2)
    }
    #[doc = "USCI Automatic Stop condition generation: 3"]
    #[inline]
    pub fn ucastp_3(self) -> &'a mut W {
        self.variant(UCASTPW::UCASTP_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCSWACK`"]
pub enum UCSWACKW {}
impl UCSWACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCSWACKW<'a> {
    w: &'a mut W,
}
impl<'a> _UCSWACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCSWACKW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCSTPNACK`"]
pub enum UCSTPNACKW {}
impl UCSTPNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCSTPNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _UCSTPNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCSTPNACKW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `UCCLTO`"]
pub enum UCCLTOW {
    #[doc = "USCI Clock low timeout: 0"]
    UCCLTO_0,
    #[doc = "USCI Clock low timeout: 1"]
    UCCLTO_1,
    #[doc = "USCI Clock low timeout: 2"]
    UCCLTO_2,
    #[doc = "USCI Clock low timeout: 3"]
    UCCLTO_3,
}
impl UCCLTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UCCLTOW::UCCLTO_0 => 0,
            UCCLTOW::UCCLTO_1 => 1,
            UCCLTOW::UCCLTO_2 => 2,
            UCCLTOW::UCCLTO_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UCCLTOW<'a> {
    w: &'a mut W,
}
impl<'a> _UCCLTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCCLTOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "USCI Clock low timeout: 0"]
    #[inline]
    pub fn ucclto_0(self) -> &'a mut W {
        self.variant(UCCLTOW::UCCLTO_0)
    }
    #[doc = "USCI Clock low timeout: 1"]
    #[inline]
    pub fn ucclto_1(self) -> &'a mut W {
        self.variant(UCCLTOW::UCCLTO_1)
    }
    #[doc = "USCI Clock low timeout: 2"]
    #[inline]
    pub fn ucclto_2(self) -> &'a mut W {
        self.variant(UCCLTOW::UCCLTO_2)
    }
    #[doc = "USCI Clock low timeout: 3"]
    #[inline]
    pub fn ucclto_3(self) -> &'a mut W {
        self.variant(UCCLTOW::UCCLTO_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UCETXINT`"]
pub enum UCETXINTW {}
impl UCETXINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UCETXINTW<'a> {
    w: &'a mut W,
}
impl<'a> _UCETXINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UCETXINTW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:1 - USCI Deglitch time Bit: 1"]
    #[inline]
    pub fn ucglit(&self) -> UCGLITR {
        UCGLITR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 2:3 - USCI Automatic Stop condition generation Bit: 1"]
    #[inline]
    pub fn ucastp(&self) -> UCASTPR {
        UCASTPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 4 - USCI Software controlled ACK"]
    #[inline]
    pub fn ucswack(&self) -> UCSWACKR {
        UCSWACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - USCI Acknowledge Stop last byte"]
    #[inline]
    pub fn ucstpnack(&self) -> UCSTPNACKR {
        UCSTPNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 6:7 - USCI Clock low timeout Bit: 1"]
    #[inline]
    pub fn ucclto(&self) -> UCCLTOR {
        UCCLTOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 8 - USCI Early UCTXIFG0"]
    #[inline]
    pub fn ucetxint(&self) -> UCETXINTR {
        UCETXINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - USCI Deglitch time Bit: 1"]
    #[inline]
    pub fn ucglit(&mut self) -> _UCGLITW {
        _UCGLITW { w: self }
    }
    #[doc = "Bits 2:3 - USCI Automatic Stop condition generation Bit: 1"]
    #[inline]
    pub fn ucastp(&mut self) -> _UCASTPW {
        _UCASTPW { w: self }
    }
    #[doc = "Bit 4 - USCI Software controlled ACK"]
    #[inline]
    pub fn ucswack(&mut self) -> _UCSWACKW {
        _UCSWACKW { w: self }
    }
    #[doc = "Bit 5 - USCI Acknowledge Stop last byte"]
    #[inline]
    pub fn ucstpnack(&mut self) -> _UCSTPNACKW {
        _UCSTPNACKW { w: self }
    }
    #[doc = "Bits 6:7 - USCI Clock low timeout Bit: 1"]
    #[inline]
    pub fn ucclto(&mut self) -> _UCCLTOW {
        _UCCLTOW { w: self }
    }
    #[doc = "Bit 8 - USCI Early UCTXIFG0"]
    #[inline]
    pub fn ucetxint(&mut self) -> _UCETXINTW {
        _UCETXINTW { w: self }
    }
}
