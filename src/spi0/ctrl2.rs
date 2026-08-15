#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Clock Phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkpha {
    #[doc = "0: Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    RisingEdge = 0,
    #[doc = "1: Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    FallingEdge = 1,
}
impl From<Clkpha> for bool {
    #[inline(always)]
    fn from(variant: Clkpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPHA` reader - Clock Phase."]
pub type ClkphaR = crate::BitReader<Clkpha>;
impl ClkphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkpha {
        match self.bits {
            false => Clkpha::RisingEdge,
            true => Clkpha::FallingEdge,
        }
    }
    #[doc = "Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Clkpha::RisingEdge
    }
    #[doc = "Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Clkpha::FallingEdge
    }
}
#[doc = "Field `CLKPHA` writer - Clock Phase."]
pub type ClkphaW<'a, REG> = crate::BitWriter<'a, REG, Clkpha>;
impl<'a, REG> ClkphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data Sampled on clock rising edge. Use when in SPI Mode 0 and Mode 2"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpha::RisingEdge)
    }
    #[doc = "Data Sampled on clock falling edge. Use when in SPI Mode 1 and Mode 3"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpha::FallingEdge)
    }
}
#[doc = "Clock Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clkpol {
    #[doc = "0: Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    Normal = 0,
    #[doc = "1: Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    Inverted = 1,
}
impl From<Clkpol> for bool {
    #[inline(always)]
    fn from(variant: Clkpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKPOL` reader - Clock Polarity."]
pub type ClkpolR = crate::BitReader<Clkpol>;
impl ClkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkpol {
        match self.bits {
            false => Clkpol::Normal,
            true => Clkpol::Inverted,
        }
    }
    #[doc = "Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Clkpol::Normal
    }
    #[doc = "Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Clkpol::Inverted
    }
}
#[doc = "Field `CLKPOL` writer - Clock Polarity."]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG, Clkpol>;
impl<'a, REG> ClkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Clock. Use when in SPI Mode 0 and Mode 1"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpol::Normal)
    }
    #[doc = "Inverted Clock. Use when in SPI Mode 2 and Mode 3"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Clkpol::Inverted)
    }
}
#[doc = "Field `SCLK_FB_INV` reader - SCLK_FB_INV."]
pub type SclkFbInvR = crate::BitReader;
#[doc = "Field `SCLK_FB_INV` writer - SCLK_FB_INV."]
pub type SclkFbInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Number of Bits per character.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numbits {
    #[doc = "0: 16 bits per character."]
    _0 = 0,
}
impl From<Numbits> for u8 {
    #[inline(always)]
    fn from(variant: Numbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Numbits {
    type Ux = u8;
}
impl crate::IsEnum for Numbits {}
#[doc = "Field `NUMBITS` reader - Number of Bits per character."]
pub type NumbitsR = crate::FieldReader<Numbits>;
impl NumbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Numbits> {
        match self.bits {
            0 => Some(Numbits::_0),
            _ => None,
        }
    }
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Numbits::_0
    }
}
#[doc = "Field `NUMBITS` writer - Number of Bits per character."]
pub type NumbitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Numbits>;
impl<'a, REG> NumbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 bits per character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Numbits::_0)
    }
}
#[doc = "SPI Data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataWidth {
    #[doc = "0: 1 data pin."]
    Mono = 0,
    #[doc = "1: 2 data pins."]
    Dual = 1,
    #[doc = "2: 4 data pins."]
    Quad = 2,
}
impl From<DataWidth> for u8 {
    #[inline(always)]
    fn from(variant: DataWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataWidth {
    type Ux = u8;
}
impl crate::IsEnum for DataWidth {}
#[doc = "Field `DATA_WIDTH` reader - SPI Data width."]
pub type DataWidthR = crate::FieldReader<DataWidth>;
impl DataWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DataWidth> {
        match self.bits {
            0 => Some(DataWidth::Mono),
            1 => Some(DataWidth::Dual),
            2 => Some(DataWidth::Quad),
            _ => None,
        }
    }
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == DataWidth::Mono
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == DataWidth::Dual
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == DataWidth::Quad
    }
}
#[doc = "Field `DATA_WIDTH` writer - SPI Data width."]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DataWidth>;
impl<'a, REG> DataWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 data pin."]
    #[inline(always)]
    pub fn mono(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::Mono)
    }
    #[doc = "2 data pins."]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::Dual)
    }
    #[doc = "4 data pins."]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::Quad)
    }
}
#[doc = "Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreeWire {
    #[doc = "0: Use four wire mode (Mono only)."]
    Dis = 0,
    #[doc = "1: Use three wire mode."]
    En = 1,
}
impl From<ThreeWire> for bool {
    #[inline(always)]
    fn from(variant: ThreeWire) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREE_WIRE` reader - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
pub type ThreeWireR = crate::BitReader<ThreeWire>;
impl ThreeWireR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ThreeWire {
        match self.bits {
            false => ThreeWire::Dis,
            true => ThreeWire::En,
        }
    }
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ThreeWire::Dis
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ThreeWire::En
    }
}
#[doc = "Field `THREE_WIRE` writer - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
pub type ThreeWireW<'a, REG> = crate::BitWriter<'a, REG, ThreeWire>;
impl<'a, REG> ThreeWireW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use four wire mode (Mono only)."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeWire::Dis)
    }
    #[doc = "Use three wire mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ThreeWire::En)
    }
}
#[doc = "Slave Select Polarity, each Slave Select can have unique polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SsPol {
    #[doc = "1: SS0 active high."]
    Ss0High = 1,
    #[doc = "2: SS1 active high."]
    Ss1High = 2,
    #[doc = "4: SS2 active high."]
    Ss2High = 4,
    #[doc = "8: SS3 active high."]
    Ss3High = 8,
}
impl From<SsPol> for u8 {
    #[inline(always)]
    fn from(variant: SsPol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SsPol {
    type Ux = u8;
}
impl crate::IsEnum for SsPol {}
#[doc = "Field `SS_POL` reader - Slave Select Polarity, each Slave Select can have unique polarity."]
pub type SsPolR = crate::FieldReader<SsPol>;
impl SsPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SsPol> {
        match self.bits {
            1 => Some(SsPol::Ss0High),
            2 => Some(SsPol::Ss1High),
            4 => Some(SsPol::Ss2High),
            8 => Some(SsPol::Ss3High),
            _ => None,
        }
    }
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn is_ss0_high(&self) -> bool {
        *self == SsPol::Ss0High
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn is_ss1_high(&self) -> bool {
        *self == SsPol::Ss1High
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn is_ss2_high(&self) -> bool {
        *self == SsPol::Ss2High
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn is_ss3_high(&self) -> bool {
        *self == SsPol::Ss3High
    }
}
#[doc = "Field `SS_POL` writer - Slave Select Polarity, each Slave Select can have unique polarity."]
pub type SsPolW<'a, REG> = crate::FieldWriter<'a, REG, 8, SsPol>;
impl<'a, REG> SsPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SS0 active high."]
    #[inline(always)]
    pub fn ss0_high(self) -> &'a mut crate::W<REG> {
        self.variant(SsPol::Ss0High)
    }
    #[doc = "SS1 active high."]
    #[inline(always)]
    pub fn ss1_high(self) -> &'a mut crate::W<REG> {
        self.variant(SsPol::Ss1High)
    }
    #[doc = "SS2 active high."]
    #[inline(always)]
    pub fn ss2_high(self) -> &'a mut crate::W<REG> {
        self.variant(SsPol::Ss2High)
    }
    #[doc = "SS3 active high."]
    #[inline(always)]
    pub fn ss3_high(self) -> &'a mut crate::W<REG> {
        self.variant(SsPol::Ss3High)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SCLK_FB_INV."]
    #[inline(always)]
    pub fn sclk_fb_inv(&self) -> SclkFbInvR {
        SclkFbInvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&self) -> NumbitsR {
        NumbitsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&self) -> ThreeWireR {
        ThreeWireR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&self) -> SsPolR {
        SsPolR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase."]
    #[inline(always)]
    pub fn clkpha(&mut self) -> ClkphaW<'_, Ctrl2Spec> {
        ClkphaW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Polarity."]
    #[inline(always)]
    pub fn clkpol(&mut self) -> ClkpolW<'_, Ctrl2Spec> {
        ClkpolW::new(self, 1)
    }
    #[doc = "Bit 4 - SCLK_FB_INV."]
    #[inline(always)]
    pub fn sclk_fb_inv(&mut self) -> SclkFbInvW<'_, Ctrl2Spec> {
        SclkFbInvW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Number of Bits per character."]
    #[inline(always)]
    pub fn numbits(&mut self) -> NumbitsW<'_, Ctrl2Spec> {
        NumbitsW::new(self, 8)
    }
    #[doc = "Bits 12:13 - SPI Data width."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DataWidthW<'_, Ctrl2Spec> {
        DataWidthW::new(self, 12)
    }
    #[doc = "Bit 15 - Three Wire mode. MOSI/MISO pin (s) shared. Only Mono mode suports Four-Wire."]
    #[inline(always)]
    pub fn three_wire(&mut self) -> ThreeWireW<'_, Ctrl2Spec> {
        ThreeWireW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Slave Select Polarity, each Slave Select can have unique polarity."]
    #[inline(always)]
    pub fn ss_pol(&mut self) -> SsPolW<'_, Ctrl2Spec> {
        SsPolW::new(self, 16)
    }
}
#[doc = "Register for controlling SPI peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {}
