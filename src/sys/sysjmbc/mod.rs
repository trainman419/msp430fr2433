#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::SYSJMBC {
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
#[doc = "Possible values of the field `JMBIN0FG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBIN0FGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBIN0FGR {
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
            JMBIN0FGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBIN0FGR {
        match value {
            i => JMBIN0FGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `JMBIN1FG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBIN1FGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBIN1FGR {
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
            JMBIN1FGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBIN1FGR {
        match value {
            i => JMBIN1FGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `JMBOUT0FG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBOUT0FGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBOUT0FGR {
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
            JMBOUT0FGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBOUT0FGR {
        match value {
            i => JMBOUT0FGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `JMBOUT1FG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBOUT1FGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBOUT1FGR {
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
            JMBOUT1FGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBOUT1FGR {
        match value {
            i => JMBOUT1FGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `JMBMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBMODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBMODER {
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
            JMBMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBMODER {
        match value {
            i => JMBMODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `JMBCLR0OFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBCLR0OFFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBCLR0OFFR {
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
            JMBCLR0OFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBCLR0OFFR {
        match value {
            i => JMBCLR0OFFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `JMBCLR1OFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JMBCLR1OFFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl JMBCLR1OFFR {
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
            JMBCLR1OFFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JMBCLR1OFFR {
        match value {
            i => JMBCLR1OFFR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `JMBIN0FG`"]
pub enum JMBIN0FGW {}
impl JMBIN0FGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBIN0FGW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBIN0FGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBIN0FGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `JMBIN1FG`"]
pub enum JMBIN1FGW {}
impl JMBIN1FGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBIN1FGW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBIN1FGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBIN1FGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `JMBOUT0FG`"]
pub enum JMBOUT0FGW {}
impl JMBOUT0FGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBOUT0FGW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBOUT0FGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBOUT0FGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `JMBOUT1FG`"]
pub enum JMBOUT1FGW {}
impl JMBOUT1FGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBOUT1FGW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBOUT1FGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBOUT1FGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `JMBMODE`"]
pub enum JMBMODEW {}
impl JMBMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBMODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `JMBCLR0OFF`"]
pub enum JMBCLR0OFFW {}
impl JMBCLR0OFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBCLR0OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBCLR0OFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBCLR0OFFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `JMBCLR1OFF`"]
pub enum JMBCLR1OFFW {}
impl JMBCLR1OFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _JMBCLR1OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _JMBCLR1OFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JMBCLR1OFFW) -> &'a mut W {
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
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline]
    pub fn jmbin0fg(&self) -> JMBIN0FGR {
        JMBIN0FGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline]
    pub fn jmbin1fg(&self) -> JMBIN1FGR {
        JMBIN1FGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline]
    pub fn jmbout0fg(&self) -> JMBOUT0FGR {
        JMBOUT0FGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline]
    pub fn jmbout1fg(&self) -> JMBOUT1FGR {
        JMBOUT1FGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline]
    pub fn jmbmode(&self) -> JMBMODER {
        JMBMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline]
    pub fn jmbclr0off(&self) -> JMBCLR0OFFR {
        JMBCLR0OFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline]
    pub fn jmbclr1off(&self) -> JMBCLR1OFFR {
        JMBCLR1OFFR::_from({
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
    #[doc = "Bit 0 - SYS - Incoming JTAG Mailbox 0 Flag"]
    #[inline]
    pub fn jmbin0fg(&mut self) -> _JMBIN0FGW {
        _JMBIN0FGW { w: self }
    }
    #[doc = "Bit 1 - SYS - Incoming JTAG Mailbox 1 Flag"]
    #[inline]
    pub fn jmbin1fg(&mut self) -> _JMBIN1FGW {
        _JMBIN1FGW { w: self }
    }
    #[doc = "Bit 2 - SYS - Outgoing JTAG Mailbox 0 Flag"]
    #[inline]
    pub fn jmbout0fg(&mut self) -> _JMBOUT0FGW {
        _JMBOUT0FGW { w: self }
    }
    #[doc = "Bit 3 - SYS - Outgoing JTAG Mailbox 1 Flag"]
    #[inline]
    pub fn jmbout1fg(&mut self) -> _JMBOUT1FGW {
        _JMBOUT1FGW { w: self }
    }
    #[doc = "Bit 4 - SYS - JMB 16/32 Bit Mode"]
    #[inline]
    pub fn jmbmode(&mut self) -> _JMBMODEW {
        _JMBMODEW { w: self }
    }
    #[doc = "Bit 6 - SYS - Incoming JTAG Mailbox 0 Flag auto-clear disalbe"]
    #[inline]
    pub fn jmbclr0off(&mut self) -> _JMBCLR0OFFW {
        _JMBCLR0OFFW { w: self }
    }
    #[doc = "Bit 7 - SYS - Incoming JTAG Mailbox 1 Flag auto-clear disalbe"]
    #[inline]
    pub fn jmbclr1off(&mut self) -> _JMBCLR1OFFW {
        _JMBCLR1OFFW { w: self }
    }
}
