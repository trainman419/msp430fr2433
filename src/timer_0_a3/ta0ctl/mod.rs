#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TA0CTL {
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
#[doc = "Possible values of the field `TAIFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIFGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TAIFGR {
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
            TAIFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAIFGR {
        match value {
            i => TAIFGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `TAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TAIER {
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
            TAIER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAIER {
        match value {
            i => TAIER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `TACLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TACLRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TACLRR {
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
            TACLRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TACLRR {
        match value {
            i => TACLRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `MC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCR {
    #[doc = "Timer A mode control: 0 - Stop"]
    MC_0,
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    MC_1,
    #[doc = "Timer A mode control: 2 - Continuous up"]
    MC_2,
    #[doc = "Timer A mode control: 3 - Up/Down"]
    MC_3,
}
impl MCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MCR::MC_0 => 0,
            MCR::MC_1 => 1,
            MCR::MC_2 => 2,
            MCR::MC_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MCR {
        match value {
            0 => MCR::MC_0,
            1 => MCR::MC_1,
            2 => MCR::MC_2,
            3 => MCR::MC_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MC_0`"]
    #[inline]
    pub fn is_mc_0(&self) -> bool {
        *self == MCR::MC_0
    }
    #[doc = "Checks if the value of the field is `MC_1`"]
    #[inline]
    pub fn is_mc_1(&self) -> bool {
        *self == MCR::MC_1
    }
    #[doc = "Checks if the value of the field is `MC_2`"]
    #[inline]
    pub fn is_mc_2(&self) -> bool {
        *self == MCR::MC_2
    }
    #[doc = "Checks if the value of the field is `MC_3`"]
    #[inline]
    pub fn is_mc_3(&self) -> bool {
        *self == MCR::MC_3
    }
}
#[doc = "Possible values of the field `ID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR {
    #[doc = "Timer A input divider: 0 - /1"]
    ID_0,
    #[doc = "Timer A input divider: 1 - /2"]
    ID_1,
    #[doc = "Timer A input divider: 2 - /4"]
    ID_2,
    #[doc = "Timer A input divider: 3 - /8"]
    ID_3,
}
impl IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDR::ID_0 => 0,
            IDR::ID_1 => 1,
            IDR::ID_2 => 2,
            IDR::ID_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDR {
        match value {
            0 => IDR::ID_0,
            1 => IDR::ID_1,
            2 => IDR::ID_2,
            3 => IDR::ID_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ID_0`"]
    #[inline]
    pub fn is_id_0(&self) -> bool {
        *self == IDR::ID_0
    }
    #[doc = "Checks if the value of the field is `ID_1`"]
    #[inline]
    pub fn is_id_1(&self) -> bool {
        *self == IDR::ID_1
    }
    #[doc = "Checks if the value of the field is `ID_2`"]
    #[inline]
    pub fn is_id_2(&self) -> bool {
        *self == IDR::ID_2
    }
    #[doc = "Checks if the value of the field is `ID_3`"]
    #[inline]
    pub fn is_id_3(&self) -> bool {
        *self == IDR::ID_3
    }
}
#[doc = "Possible values of the field `TASSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASSELR {
    #[doc = "Timer A clock source select: 0 - TACLK"]
    TASSEL_0,
    #[doc = "Timer A clock source select: 1 - ACLK"]
    TASSEL_1,
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    TASSEL_2,
    #[doc = "Timer A clock source select: 3 - INCLK"]
    TASSEL_3,
}
impl TASSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TASSELR::TASSEL_0 => 0,
            TASSELR::TASSEL_1 => 1,
            TASSELR::TASSEL_2 => 2,
            TASSELR::TASSEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TASSELR {
        match value {
            0 => TASSELR::TASSEL_0,
            1 => TASSELR::TASSEL_1,
            2 => TASSELR::TASSEL_2,
            3 => TASSELR::TASSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TASSEL_0`"]
    #[inline]
    pub fn is_tassel_0(&self) -> bool {
        *self == TASSELR::TASSEL_0
    }
    #[doc = "Checks if the value of the field is `TASSEL_1`"]
    #[inline]
    pub fn is_tassel_1(&self) -> bool {
        *self == TASSELR::TASSEL_1
    }
    #[doc = "Checks if the value of the field is `TASSEL_2`"]
    #[inline]
    pub fn is_tassel_2(&self) -> bool {
        *self == TASSELR::TASSEL_2
    }
    #[doc = "Checks if the value of the field is `TASSEL_3`"]
    #[inline]
    pub fn is_tassel_3(&self) -> bool {
        *self == TASSELR::TASSEL_3
    }
}
#[doc = "Values that can be written to the field `TAIFG`"]
pub enum TAIFGW {}
impl TAIFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TAIFGW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIFGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TAIE`"]
pub enum TAIEW {}
impl TAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `TACLR`"]
pub enum TACLRW {}
impl TACLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TACLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TACLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TACLRW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `MC`"]
pub enum MCW {
    #[doc = "Timer A mode control: 0 - Stop"]
    MC_0,
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    MC_1,
    #[doc = "Timer A mode control: 2 - Continuous up"]
    MC_2,
    #[doc = "Timer A mode control: 3 - Up/Down"]
    MC_3,
}
impl MCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MCW::MC_0 => 0,
            MCW::MC_1 => 1,
            MCW::MC_2 => 2,
            MCW::MC_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCW<'a> {
    w: &'a mut W,
}
impl<'a> _MCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer A mode control: 0 - Stop"]
    #[inline]
    pub fn mc_0(self) -> &'a mut W {
        self.variant(MCW::MC_0)
    }
    #[doc = "Timer A mode control: 1 - Up to CCR0"]
    #[inline]
    pub fn mc_1(self) -> &'a mut W {
        self.variant(MCW::MC_1)
    }
    #[doc = "Timer A mode control: 2 - Continuous up"]
    #[inline]
    pub fn mc_2(self) -> &'a mut W {
        self.variant(MCW::MC_2)
    }
    #[doc = "Timer A mode control: 3 - Up/Down"]
    #[inline]
    pub fn mc_3(self) -> &'a mut W {
        self.variant(MCW::MC_3)
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
#[doc = "Values that can be written to the field `ID`"]
pub enum IDW {
    #[doc = "Timer A input divider: 0 - /1"]
    ID_0,
    #[doc = "Timer A input divider: 1 - /2"]
    ID_1,
    #[doc = "Timer A input divider: 2 - /4"]
    ID_2,
    #[doc = "Timer A input divider: 3 - /8"]
    ID_3,
}
impl IDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDW::ID_0 => 0,
            IDW::ID_1 => 1,
            IDW::ID_2 => 2,
            IDW::ID_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDW<'a> {
    w: &'a mut W,
}
impl<'a> _IDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer A input divider: 0 - /1"]
    #[inline]
    pub fn id_0(self) -> &'a mut W {
        self.variant(IDW::ID_0)
    }
    #[doc = "Timer A input divider: 1 - /2"]
    #[inline]
    pub fn id_1(self) -> &'a mut W {
        self.variant(IDW::ID_1)
    }
    #[doc = "Timer A input divider: 2 - /4"]
    #[inline]
    pub fn id_2(self) -> &'a mut W {
        self.variant(IDW::ID_2)
    }
    #[doc = "Timer A input divider: 3 - /8"]
    #[inline]
    pub fn id_3(self) -> &'a mut W {
        self.variant(IDW::ID_3)
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
#[doc = "Values that can be written to the field `TASSEL`"]
pub enum TASSELW {
    #[doc = "Timer A clock source select: 0 - TACLK"]
    TASSEL_0,
    #[doc = "Timer A clock source select: 1 - ACLK"]
    TASSEL_1,
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    TASSEL_2,
    #[doc = "Timer A clock source select: 3 - INCLK"]
    TASSEL_3,
}
impl TASSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TASSELW::TASSEL_0 => 0,
            TASSELW::TASSEL_1 => 1,
            TASSELW::TASSEL_2 => 2,
            TASSELW::TASSEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TASSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TASSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TASSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer A clock source select: 0 - TACLK"]
    #[inline]
    pub fn tassel_0(self) -> &'a mut W {
        self.variant(TASSELW::TASSEL_0)
    }
    #[doc = "Timer A clock source select: 1 - ACLK"]
    #[inline]
    pub fn tassel_1(self) -> &'a mut W {
        self.variant(TASSELW::TASSEL_1)
    }
    #[doc = "Timer A clock source select: 2 - SMCLK"]
    #[inline]
    pub fn tassel_2(self) -> &'a mut W {
        self.variant(TASSELW::TASSEL_2)
    }
    #[doc = "Timer A clock source select: 3 - INCLK"]
    #[inline]
    pub fn tassel_3(self) -> &'a mut W {
        self.variant(TASSELW::TASSEL_3)
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
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline]
    pub fn taifg(&self) -> TAIFGR {
        TAIFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline]
    pub fn taie(&self) -> TAIER {
        TAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline]
    pub fn taclr(&self) -> TACLRR {
        TACLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline]
    pub fn mc(&self) -> MCR {
        MCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline]
    pub fn id(&self) -> IDR {
        IDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline]
    pub fn tassel(&self) -> TASSELR {
        TASSELR::_from({
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
    #[doc = "Bit 0 - Timer A counter interrupt flag"]
    #[inline]
    pub fn taifg(&mut self) -> _TAIFGW {
        _TAIFGW { w: self }
    }
    #[doc = "Bit 1 - Timer A counter interrupt enable"]
    #[inline]
    pub fn taie(&mut self) -> _TAIEW {
        _TAIEW { w: self }
    }
    #[doc = "Bit 2 - Timer A counter clear"]
    #[inline]
    pub fn taclr(&mut self) -> _TACLRW {
        _TACLRW { w: self }
    }
    #[doc = "Bits 4:5 - Timer A mode control 1"]
    #[inline]
    pub fn mc(&mut self) -> _MCW {
        _MCW { w: self }
    }
    #[doc = "Bits 6:7 - Timer A clock input divider 1"]
    #[inline]
    pub fn id(&mut self) -> _IDW {
        _IDW { w: self }
    }
    #[doc = "Bits 8:9 - Timer A clock source select 1"]
    #[inline]
    pub fn tassel(&mut self) -> _TASSELW {
        _TASSELW { w: self }
    }
}
