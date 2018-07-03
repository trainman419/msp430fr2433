#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CSCTL6 {
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
#[doc = "Possible values of the field `XT1AUTOOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1AUTOOFFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl XT1AUTOOFFR {
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
            XT1AUTOOFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XT1AUTOOFFR {
        match value {
            i => XT1AUTOOFFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `XT1AGCOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1AGCOFFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl XT1AGCOFFR {
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
            XT1AGCOFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XT1AGCOFFR {
        match value {
            i => XT1AGCOFFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `XT1BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1BYPASSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl XT1BYPASSR {
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
            XT1BYPASSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XT1BYPASSR {
        match value {
            i => XT1BYPASSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `XTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XTSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl XTSR {
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
            XTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> XTSR {
        match value {
            i => XTSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `XT1DRIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XT1DRIVER {
    #[doc = "XT1 Drive Level mode: 0"]
    XT1DRIVE_0,
    #[doc = "XT1 Drive Level mode: 1"]
    XT1DRIVE_1,
    #[doc = "XT1 Drive Level mode: 2"]
    XT1DRIVE_2,
    #[doc = "XT1 Drive Level mode: 3"]
    XT1DRIVE_3,
}
impl XT1DRIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            XT1DRIVER::XT1DRIVE_0 => 0,
            XT1DRIVER::XT1DRIVE_1 => 1,
            XT1DRIVER::XT1DRIVE_2 => 2,
            XT1DRIVER::XT1DRIVE_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> XT1DRIVER {
        match value {
            0 => XT1DRIVER::XT1DRIVE_0,
            1 => XT1DRIVER::XT1DRIVE_1,
            2 => XT1DRIVER::XT1DRIVE_2,
            3 => XT1DRIVER::XT1DRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_0`"]
    #[inline]
    pub fn is_xt1drive_0(&self) -> bool {
        *self == XT1DRIVER::XT1DRIVE_0
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_1`"]
    #[inline]
    pub fn is_xt1drive_1(&self) -> bool {
        *self == XT1DRIVER::XT1DRIVE_1
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_2`"]
    #[inline]
    pub fn is_xt1drive_2(&self) -> bool {
        *self == XT1DRIVER::XT1DRIVE_2
    }
    #[doc = "Checks if the value of the field is `XT1DRIVE_3`"]
    #[inline]
    pub fn is_xt1drive_3(&self) -> bool {
        *self == XT1DRIVER::XT1DRIVE_3
    }
}
#[doc = "Values that can be written to the field `XT1AUTOOFF`"]
pub enum XT1AUTOOFFW {}
impl XT1AUTOOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _XT1AUTOOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _XT1AUTOOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XT1AUTOOFFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `XT1AGCOFF`"]
pub enum XT1AGCOFFW {}
impl XT1AGCOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _XT1AGCOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _XT1AGCOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XT1AGCOFFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `XT1BYPASS`"]
pub enum XT1BYPASSW {}
impl XT1BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _XT1BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _XT1BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XT1BYPASSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `XTS`"]
pub enum XTSW {}
impl XTSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _XTSW<'a> {
    w: &'a mut W,
}
impl<'a> _XTSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `XT1DRIVE`"]
pub enum XT1DRIVEW {
    #[doc = "XT1 Drive Level mode: 0"]
    XT1DRIVE_0,
    #[doc = "XT1 Drive Level mode: 1"]
    XT1DRIVE_1,
    #[doc = "XT1 Drive Level mode: 2"]
    XT1DRIVE_2,
    #[doc = "XT1 Drive Level mode: 3"]
    XT1DRIVE_3,
}
impl XT1DRIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            XT1DRIVEW::XT1DRIVE_0 => 0,
            XT1DRIVEW::XT1DRIVE_1 => 1,
            XT1DRIVEW::XT1DRIVE_2 => 2,
            XT1DRIVEW::XT1DRIVE_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XT1DRIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _XT1DRIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XT1DRIVEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "XT1 Drive Level mode: 0"]
    #[inline]
    pub fn xt1drive_0(self) -> &'a mut W {
        self.variant(XT1DRIVEW::XT1DRIVE_0)
    }
    #[doc = "XT1 Drive Level mode: 1"]
    #[inline]
    pub fn xt1drive_1(self) -> &'a mut W {
        self.variant(XT1DRIVEW::XT1DRIVE_1)
    }
    #[doc = "XT1 Drive Level mode: 2"]
    #[inline]
    pub fn xt1drive_2(self) -> &'a mut W {
        self.variant(XT1DRIVEW::XT1DRIVE_2)
    }
    #[doc = "XT1 Drive Level mode: 3"]
    #[inline]
    pub fn xt1drive_3(self) -> &'a mut W {
        self.variant(XT1DRIVEW::XT1DRIVE_3)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - XT1 automatic off enable"]
    #[inline]
    pub fn xt1autooff(&self) -> XT1AUTOOFFR {
        XT1AUTOOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - XT1 Automatic Gain Control (AGC) disable"]
    #[inline]
    pub fn xt1agcoff(&self) -> XT1AGCOFFR {
        XT1AGCOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline]
    pub fn xt1bypass(&self) -> XT1BYPASSR {
        XT1BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline]
    pub fn xts(&self) -> XTSR {
        XTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline]
    pub fn xt1drive(&self) -> XT1DRIVER {
        XT1DRIVER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - XT1 automatic off enable"]
    #[inline]
    pub fn xt1autooff(&mut self) -> _XT1AUTOOFFW {
        _XT1AUTOOFFW { w: self }
    }
    #[doc = "Bit 1 - XT1 Automatic Gain Control (AGC) disable"]
    #[inline]
    pub fn xt1agcoff(&mut self) -> _XT1AGCOFFW {
        _XT1AGCOFFW { w: self }
    }
    #[doc = "Bit 4 - XT1 bypass mode : 0: internal 1:sourced from external pin"]
    #[inline]
    pub fn xt1bypass(&mut self) -> _XT1BYPASSW {
        _XT1BYPASSW { w: self }
    }
    #[doc = "Bit 5 - 1: Selects high-freq. oscillator"]
    #[inline]
    pub fn xts(&mut self) -> _XTSW {
        _XTSW { w: self }
    }
    #[doc = "Bits 6:7 - XT1 Drive Level mode Bit 0"]
    #[inline]
    pub fn xt1drive(&mut self) -> _XT1DRIVEW {
        _XT1DRIVEW { w: self }
    }
}
