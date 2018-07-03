#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::WDTCTL {
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
#[doc = "Possible values of the field `WDTIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTISR {
    #[doc = "WDT - Timer Interval Select: /2G"]
    WDTIS_0,
    #[doc = "WDT - Timer Interval Select: /128M"]
    WDTIS_1,
    #[doc = "WDT - Timer Interval Select: /8192k"]
    WDTIS_2,
    #[doc = "WDT - Timer Interval Select: /512k"]
    WDTIS_3,
    #[doc = "WDT - Timer Interval Select: /32k"]
    WDTIS_4,
    #[doc = "WDT - Timer Interval Select: /8192"]
    WDTIS_5,
    #[doc = "WDT - Timer Interval Select: /512"]
    WDTIS_6,
    #[doc = "WDT - Timer Interval Select: /64"]
    WDTIS_7,
}
impl WDTISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDTISR::WDTIS_0 => 0,
            WDTISR::WDTIS_1 => 1,
            WDTISR::WDTIS_2 => 2,
            WDTISR::WDTIS_3 => 3,
            WDTISR::WDTIS_4 => 4,
            WDTISR::WDTIS_5 => 5,
            WDTISR::WDTIS_6 => 6,
            WDTISR::WDTIS_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDTISR {
        match value {
            0 => WDTISR::WDTIS_0,
            1 => WDTISR::WDTIS_1,
            2 => WDTISR::WDTIS_2,
            3 => WDTISR::WDTIS_3,
            4 => WDTISR::WDTIS_4,
            5 => WDTISR::WDTIS_5,
            6 => WDTISR::WDTIS_6,
            7 => WDTISR::WDTIS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTIS_0`"]
    #[inline]
    pub fn is_wdtis_0(&self) -> bool {
        *self == WDTISR::WDTIS_0
    }
    #[doc = "Checks if the value of the field is `WDTIS_1`"]
    #[inline]
    pub fn is_wdtis_1(&self) -> bool {
        *self == WDTISR::WDTIS_1
    }
    #[doc = "Checks if the value of the field is `WDTIS_2`"]
    #[inline]
    pub fn is_wdtis_2(&self) -> bool {
        *self == WDTISR::WDTIS_2
    }
    #[doc = "Checks if the value of the field is `WDTIS_3`"]
    #[inline]
    pub fn is_wdtis_3(&self) -> bool {
        *self == WDTISR::WDTIS_3
    }
    #[doc = "Checks if the value of the field is `WDTIS_4`"]
    #[inline]
    pub fn is_wdtis_4(&self) -> bool {
        *self == WDTISR::WDTIS_4
    }
    #[doc = "Checks if the value of the field is `WDTIS_5`"]
    #[inline]
    pub fn is_wdtis_5(&self) -> bool {
        *self == WDTISR::WDTIS_5
    }
    #[doc = "Checks if the value of the field is `WDTIS_6`"]
    #[inline]
    pub fn is_wdtis_6(&self) -> bool {
        *self == WDTISR::WDTIS_6
    }
    #[doc = "Checks if the value of the field is `WDTIS_7`"]
    #[inline]
    pub fn is_wdtis_7(&self) -> bool {
        *self == WDTISR::WDTIS_7
    }
}
#[doc = "Possible values of the field `WDTCNTCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCNTCLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WDTCNTCLR {
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
            WDTCNTCLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTCNTCLR {
        match value {
            i => WDTCNTCLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `WDTTMSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTTMSELR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WDTTMSELR {
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
            WDTTMSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTTMSELR {
        match value {
            i => WDTTMSELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `WDTSSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTSSELR {
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    WDTSSEL_0,
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    WDTSSEL_1,
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    WDTSSEL_2,
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    WDTSSEL_3,
}
impl WDTSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDTSSELR::WDTSSEL_0 => 0,
            WDTSSELR::WDTSSEL_1 => 1,
            WDTSSELR::WDTSSEL_2 => 2,
            WDTSSELR::WDTSSEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDTSSELR {
        match value {
            0 => WDTSSELR::WDTSSEL_0,
            1 => WDTSSELR::WDTSSEL_1,
            2 => WDTSSELR::WDTSSEL_2,
            3 => WDTSSELR::WDTSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_0`"]
    #[inline]
    pub fn is_wdtssel_0(&self) -> bool {
        *self == WDTSSELR::WDTSSEL_0
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_1`"]
    #[inline]
    pub fn is_wdtssel_1(&self) -> bool {
        *self == WDTSSELR::WDTSSEL_1
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_2`"]
    #[inline]
    pub fn is_wdtssel_2(&self) -> bool {
        *self == WDTSSELR::WDTSSEL_2
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_3`"]
    #[inline]
    pub fn is_wdtssel_3(&self) -> bool {
        *self == WDTSSELR::WDTSSEL_3
    }
}
#[doc = "Possible values of the field `WDTHOLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTHOLDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WDTHOLDR {
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
            WDTHOLDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTHOLDR {
        match value {
            i => WDTHOLDR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `WDTIS`"]
pub enum WDTISW {
    #[doc = "WDT - Timer Interval Select: /2G"]
    WDTIS_0,
    #[doc = "WDT - Timer Interval Select: /128M"]
    WDTIS_1,
    #[doc = "WDT - Timer Interval Select: /8192k"]
    WDTIS_2,
    #[doc = "WDT - Timer Interval Select: /512k"]
    WDTIS_3,
    #[doc = "WDT - Timer Interval Select: /32k"]
    WDTIS_4,
    #[doc = "WDT - Timer Interval Select: /8192"]
    WDTIS_5,
    #[doc = "WDT - Timer Interval Select: /512"]
    WDTIS_6,
    #[doc = "WDT - Timer Interval Select: /64"]
    WDTIS_7,
}
impl WDTISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDTISW::WDTIS_0 => 0,
            WDTISW::WDTIS_1 => 1,
            WDTISW::WDTIS_2 => 2,
            WDTISW::WDTIS_3 => 3,
            WDTISW::WDTIS_4 => 4,
            WDTISW::WDTIS_5 => 5,
            WDTISW::WDTIS_6 => 6,
            WDTISW::WDTIS_7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTISW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTISW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "WDT - Timer Interval Select: /2G"]
    #[inline]
    pub fn wdtis_0(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_0)
    }
    #[doc = "WDT - Timer Interval Select: /128M"]
    #[inline]
    pub fn wdtis_1(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_1)
    }
    #[doc = "WDT - Timer Interval Select: /8192k"]
    #[inline]
    pub fn wdtis_2(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_2)
    }
    #[doc = "WDT - Timer Interval Select: /512k"]
    #[inline]
    pub fn wdtis_3(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_3)
    }
    #[doc = "WDT - Timer Interval Select: /32k"]
    #[inline]
    pub fn wdtis_4(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_4)
    }
    #[doc = "WDT - Timer Interval Select: /8192"]
    #[inline]
    pub fn wdtis_5(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_5)
    }
    #[doc = "WDT - Timer Interval Select: /512"]
    #[inline]
    pub fn wdtis_6(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_6)
    }
    #[doc = "WDT - Timer Interval Select: /64"]
    #[inline]
    pub fn wdtis_7(self) -> &'a mut W {
        self.variant(WDTISW::WDTIS_7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTCNTCL`"]
pub enum WDTCNTCLW {}
impl WDTCNTCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDTCNTCLW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTCNTCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTCNTCLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `WDTTMSEL`"]
pub enum WDTTMSELW {}
impl WDTTMSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDTTMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTTMSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTTMSELW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `WDTSSEL`"]
pub enum WDTSSELW {
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    WDTSSEL_0,
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    WDTSSEL_1,
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    WDTSSEL_2,
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    WDTSSEL_3,
}
impl WDTSSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDTSSELW::WDTSSEL_0 => 0,
            WDTSSELW::WDTSSEL_1 => 1,
            WDTSSELW::WDTSSEL_2 => 2,
            WDTSSELW::WDTSSEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTSSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTSSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline]
    pub fn wdtssel_0(self) -> &'a mut W {
        self.variant(WDTSSELW::WDTSSEL_0)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline]
    pub fn wdtssel_1(self) -> &'a mut W {
        self.variant(WDTSSELW::WDTSSEL_1)
    }
    #[doc = "WDT - Timer Clock Source Select: VLO_CLK"]
    #[inline]
    pub fn wdtssel_2(self) -> &'a mut W {
        self.variant(WDTSSELW::WDTSSEL_2)
    }
    #[doc = "WDT - Timer Clock Source Select: reserved"]
    #[inline]
    pub fn wdtssel_3(self) -> &'a mut W {
        self.variant(WDTSSELW::WDTSSEL_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDTHOLD`"]
pub enum WDTHOLDW {}
impl WDTHOLDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDTHOLDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTHOLDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTHOLDW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline]
    pub fn wdtis(&self) -> WDTISR {
        WDTISR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline]
    pub fn wdtcntcl(&self) -> WDTCNTCLR {
        WDTCNTCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline]
    pub fn wdttmsel(&self) -> WDTTMSELR {
        WDTTMSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline]
    pub fn wdtssel(&self) -> WDTSSELR {
        WDTSSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        })
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline]
    pub fn wdthold(&self) -> WDTHOLDR {
        WDTHOLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline]
    pub fn wdtis(&mut self) -> _WDTISW {
        _WDTISW { w: self }
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline]
    pub fn wdtcntcl(&mut self) -> _WDTCNTCLW {
        _WDTCNTCLW { w: self }
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline]
    pub fn wdttmsel(&mut self) -> _WDTTMSELW {
        _WDTTMSELW { w: self }
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline]
    pub fn wdtssel(&mut self) -> _WDTSSELW {
        _WDTSSELW { w: self }
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline]
    pub fn wdthold(&mut self) -> _WDTHOLDW {
        _WDTHOLDW { w: self }
    }
}
