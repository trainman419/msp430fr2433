#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL7 {
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
#[doc = "Possible values of the field `DCOFFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCOFFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DCOFFGR {
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
            DCOFFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCOFFGR {
        match value {
            i => DCOFFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `XT1OFFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1OFFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl XT1OFFGR {
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
            XT1OFFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XT1OFFGR {
        match value {
            i => XT1OFFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLULIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLULIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLULIFGR {
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
            FLLULIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLULIFGR {
        match value {
            i => FLLULIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ENSTFCNT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENSTFCNT1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ENSTFCNT1R {
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
            ENSTFCNT1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENSTFCNT1R {
        match value {
            i => ENSTFCNT1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLUNLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLUNLOCKR {
    #[doc = "FLL unlock condition: 0"]
    FLLUNLOCK_0,
    #[doc = "FLL unlock condition: 1"]
    FLLUNLOCK_1,
    #[doc = "FLL unlock condition: 2"]
    FLLUNLOCK_2,
    #[doc = "FLL unlock condition: 3"]
    FLLUNLOCK_3,
}
impl FLLUNLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLLUNLOCKR::FLLUNLOCK_0 => 0,
            FLLUNLOCKR::FLLUNLOCK_1 => 1,
            FLLUNLOCKR::FLLUNLOCK_2 => 2,
            FLLUNLOCKR::FLLUNLOCK_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLLUNLOCKR {
        match value {
            0 => FLLUNLOCKR::FLLUNLOCK_0,
            1 => FLLUNLOCKR::FLLUNLOCK_1,
            2 => FLLUNLOCKR::FLLUNLOCK_2,
            3 => FLLUNLOCKR::FLLUNLOCK_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_0`"]
    #[inline]
    pub fn is_fllunlock_0(&self) -> bool {
        *self == FLLUNLOCKR::FLLUNLOCK_0
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_1`"]
    #[inline]
    pub fn is_fllunlock_1(&self) -> bool {
        *self == FLLUNLOCKR::FLLUNLOCK_1
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_2`"]
    #[inline]
    pub fn is_fllunlock_2(&self) -> bool {
        *self == FLLUNLOCKR::FLLUNLOCK_2
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCK_3`"]
    #[inline]
    pub fn is_fllunlock_3(&self) -> bool {
        *self == FLLUNLOCKR::FLLUNLOCK_3
    }
}
#[doc = "Possible values of the field `FLLUNLOCKHIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLUNLOCKHISR {
    #[doc = "Unlock history: 0"]
    FLLUNLOCKHIS_0,
    #[doc = "Unlock history: 1"]
    FLLUNLOCKHIS_1,
    #[doc = "Unlock history: 2"]
    FLLUNLOCKHIS_2,
    #[doc = "Unlock history: 3"]
    FLLUNLOCKHIS_3,
}
impl FLLUNLOCKHISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLLUNLOCKHISR::FLLUNLOCKHIS_0 => 0,
            FLLUNLOCKHISR::FLLUNLOCKHIS_1 => 1,
            FLLUNLOCKHISR::FLLUNLOCKHIS_2 => 2,
            FLLUNLOCKHISR::FLLUNLOCKHIS_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLLUNLOCKHISR {
        match value {
            0 => FLLUNLOCKHISR::FLLUNLOCKHIS_0,
            1 => FLLUNLOCKHISR::FLLUNLOCKHIS_1,
            2 => FLLUNLOCKHISR::FLLUNLOCKHIS_2,
            3 => FLLUNLOCKHISR::FLLUNLOCKHIS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_0`"]
    #[inline]
    pub fn is_fllunlockhis_0(&self) -> bool {
        *self == FLLUNLOCKHISR::FLLUNLOCKHIS_0
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_1`"]
    #[inline]
    pub fn is_fllunlockhis_1(&self) -> bool {
        *self == FLLUNLOCKHISR::FLLUNLOCKHIS_1
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_2`"]
    #[inline]
    pub fn is_fllunlockhis_2(&self) -> bool {
        *self == FLLUNLOCKHISR::FLLUNLOCKHIS_2
    }
    #[doc = "Checks if the value of the field is `FLLUNLOCKHIS_3`"]
    #[inline]
    pub fn is_fllunlockhis_3(&self) -> bool {
        *self == FLLUNLOCKHISR::FLLUNLOCKHIS_3
    }
}
#[doc = "Possible values of the field `FLLULPUC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLULPUCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLULPUCR {
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
            FLLULPUCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLULPUCR {
        match value {
            i => FLLULPUCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `FLLWARNEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLLWARNENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLLWARNENR {
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
            FLLWARNENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLLWARNENR {
        match value {
            i => FLLWARNENR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `DCOFFG`"]
pub enum DCOFFGW {}
impl DCOFFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DCOFFGW<'a> {
    w: &'a mut W,
}
impl<'a> _DCOFFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCOFFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `XT1OFFG`"]
pub enum XT1OFFGW {}
impl XT1OFFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _XT1OFFGW<'a> {
    w: &'a mut W,
}
impl<'a> _XT1OFFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XT1OFFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLULIFG`"]
pub enum FLLULIFGW {}
impl FLLULIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLULIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLULIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLULIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ENSTFCNT1`"]
pub enum ENSTFCNT1W {}
impl ENSTFCNT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ENSTFCNT1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENSTFCNT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENSTFCNT1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLUNLOCK`"]
pub enum FLLUNLOCKW {
    #[doc = "FLL unlock condition: 0"]
    FLLUNLOCK_0,
    #[doc = "FLL unlock condition: 1"]
    FLLUNLOCK_1,
    #[doc = "FLL unlock condition: 2"]
    FLLUNLOCK_2,
    #[doc = "FLL unlock condition: 3"]
    FLLUNLOCK_3,
}
impl FLLUNLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLLUNLOCKW::FLLUNLOCK_0 => 0,
            FLLUNLOCKW::FLLUNLOCK_1 => 1,
            FLLUNLOCKW::FLLUNLOCK_2 => 2,
            FLLUNLOCKW::FLLUNLOCK_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLLUNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLUNLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLUNLOCKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "FLL unlock condition: 0"]
    #[inline]
    pub fn fllunlock_0(self) -> &'a mut W {
        self.variant(FLLUNLOCKW::FLLUNLOCK_0)
    }
    #[doc = "FLL unlock condition: 1"]
    #[inline]
    pub fn fllunlock_1(self) -> &'a mut W {
        self.variant(FLLUNLOCKW::FLLUNLOCK_1)
    }
    #[doc = "FLL unlock condition: 2"]
    #[inline]
    pub fn fllunlock_2(self) -> &'a mut W {
        self.variant(FLLUNLOCKW::FLLUNLOCK_2)
    }
    #[doc = "FLL unlock condition: 3"]
    #[inline]
    pub fn fllunlock_3(self) -> &'a mut W {
        self.variant(FLLUNLOCKW::FLLUNLOCK_3)
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
#[doc = "Values that can be written to the field `FLLUNLOCKHIS`"]
pub enum FLLUNLOCKHISW {
    #[doc = "Unlock history: 0"]
    FLLUNLOCKHIS_0,
    #[doc = "Unlock history: 1"]
    FLLUNLOCKHIS_1,
    #[doc = "Unlock history: 2"]
    FLLUNLOCKHIS_2,
    #[doc = "Unlock history: 3"]
    FLLUNLOCKHIS_3,
}
impl FLLUNLOCKHISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLLUNLOCKHISW::FLLUNLOCKHIS_0 => 0,
            FLLUNLOCKHISW::FLLUNLOCKHIS_1 => 1,
            FLLUNLOCKHISW::FLLUNLOCKHIS_2 => 2,
            FLLUNLOCKHISW::FLLUNLOCKHIS_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLLUNLOCKHISW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLUNLOCKHISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLUNLOCKHISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Unlock history: 0"]
    #[inline]
    pub fn fllunlockhis_0(self) -> &'a mut W {
        self.variant(FLLUNLOCKHISW::FLLUNLOCKHIS_0)
    }
    #[doc = "Unlock history: 1"]
    #[inline]
    pub fn fllunlockhis_1(self) -> &'a mut W {
        self.variant(FLLUNLOCKHISW::FLLUNLOCKHIS_1)
    }
    #[doc = "Unlock history: 2"]
    #[inline]
    pub fn fllunlockhis_2(self) -> &'a mut W {
        self.variant(FLLUNLOCKHISW::FLLUNLOCKHIS_2)
    }
    #[doc = "Unlock history: 3"]
    #[inline]
    pub fn fllunlockhis_3(self) -> &'a mut W {
        self.variant(FLLUNLOCKHISW::FLLUNLOCKHIS_3)
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
#[doc = "Values that can be written to the field `FLLULPUC`"]
pub enum FLLULPUCW {}
impl FLLULPUCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLULPUCW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLULPUCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLULPUCW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `FLLWARNEN`"]
pub enum FLLWARNENW {}
impl FLLWARNENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLLWARNENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLLWARNENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLLWARNENW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline]
    pub fn dcoffg(&self) -> DCOFFGR {
        DCOFFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline]
    pub fn xt1offg(&self) -> XT1OFFGR {
        XT1OFFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline]
    pub fn fllulifg(&self) -> FLLULIFGR {
        FLLULIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline]
    pub fn enstfcnt1(&self) -> ENSTFCNT1R {
        ENSTFCNT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline]
    pub fn fllunlock(&self) -> FLLUNLOCKR {
        FLLUNLOCKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline]
    pub fn fllunlockhis(&self) -> FLLUNLOCKHISR {
        FLLUNLOCKHISR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline]
    pub fn fllulpuc(&self) -> FLLULPUCR {
        FLLULPUCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline]
    pub fn fllwarnen(&self) -> FLLWARNENR {
        FLLWARNENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - DCO fault flag"]
    #[inline]
    pub fn dcoffg(&mut self) -> _DCOFFGW {
        _DCOFFGW { w: self }
    }
    #[doc = "Bit 1 - XT1 Low Frequency Oscillator Fault Flag"]
    #[inline]
    pub fn xt1offg(&mut self) -> _XT1OFFGW {
        _XT1OFFGW { w: self }
    }
    #[doc = "Bit 4 - FLL unlock interrupt flag"]
    #[inline]
    pub fn fllulifg(&mut self) -> _FLLULIFGW {
        _FLLULIFGW { w: self }
    }
    #[doc = "Bit 6 - Enable start counter for XT1"]
    #[inline]
    pub fn enstfcnt1(&mut self) -> _ENSTFCNT1W {
        _ENSTFCNT1W { w: self }
    }
    #[doc = "Bits 8:9 - FLL unlock condition Bit: 0"]
    #[inline]
    pub fn fllunlock(&mut self) -> _FLLUNLOCKW {
        _FLLUNLOCKW { w: self }
    }
    #[doc = "Bits 10:11 - Unlock history Bit: 0"]
    #[inline]
    pub fn fllunlockhis(&mut self) -> _FLLUNLOCKHISW {
        _FLLUNLOCKHISW { w: self }
    }
    #[doc = "Bit 12 - FLL unlock PUC enable"]
    #[inline]
    pub fn fllulpuc(&mut self) -> _FLLULPUCW {
        _FLLULPUCW { w: self }
    }
    #[doc = "Bit 13 - Warning enable"]
    #[inline]
    pub fn fllwarnen(&mut self) -> _FLLWARNENW {
        _FLLWARNENW { w: self }
    }
}
