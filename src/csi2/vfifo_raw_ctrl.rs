#[doc = "Register `VFIFO_RAW_CTRL` reader"]
pub type R = crate::R<VfifoRawCtrlSpec>;
#[doc = "Register `VFIFO_RAW_CTRL` writer"]
pub type W = crate::W<VfifoRawCtrlSpec>;
#[doc = "Field `RAW_CEN` reader - RAW conversion enable."]
pub type RawCenR = crate::BitReader;
#[doc = "Field `RAW_CEN` writer - RAW conversion enable."]
pub type RawCenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_FF_AFO` reader - RAW conversion FIFO automatic flush-out."]
pub type RawFfAfoR = crate::BitReader;
#[doc = "Field `RAW_FF_AFO` writer - RAW conversion FIFO automatic flush-out."]
pub type RawFfAfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_FF_FO` reader - RAW conversion FIFO flush-out trigger."]
pub type RawFfFoR = crate::BitReader;
#[doc = "Field `RAW_FF_FO` writer - RAW conversion FIFO flush-out trigger."]
pub type RawFfFoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RAW format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RawFmt {
    #[doc = "0: RGRG GBGB"]
    RgrgGbgb = 0,
    #[doc = "1: GRGR BGBG"]
    GrgrBgbg = 1,
    #[doc = "2: GBGB RGRG"]
    GbgbRgrg = 2,
    #[doc = "3: BGBG GRGR"]
    BgbgGrgr = 3,
}
impl From<RawFmt> for u8 {
    #[inline(always)]
    fn from(variant: RawFmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RawFmt {
    type Ux = u8;
}
impl crate::IsEnum for RawFmt {}
#[doc = "Field `RAW_FMT` reader - RAW format."]
pub type RawFmtR = crate::FieldReader<RawFmt>;
impl RawFmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RawFmt {
        match self.bits {
            0 => RawFmt::RgrgGbgb,
            1 => RawFmt::GrgrBgbg,
            2 => RawFmt::GbgbRgrg,
            3 => RawFmt::BgbgGrgr,
            _ => unreachable!(),
        }
    }
    #[doc = "RGRG GBGB"]
    #[inline(always)]
    pub fn is_rgrg_gbgb(&self) -> bool {
        *self == RawFmt::RgrgGbgb
    }
    #[doc = "GRGR BGBG"]
    #[inline(always)]
    pub fn is_grgr_bgbg(&self) -> bool {
        *self == RawFmt::GrgrBgbg
    }
    #[doc = "GBGB RGRG"]
    #[inline(always)]
    pub fn is_gbgb_rgrg(&self) -> bool {
        *self == RawFmt::GbgbRgrg
    }
    #[doc = "BGBG GRGR"]
    #[inline(always)]
    pub fn is_bgbg_grgr(&self) -> bool {
        *self == RawFmt::BgbgGrgr
    }
}
#[doc = "Field `RAW_FMT` writer - RAW format."]
pub type RawFmtW<'a, REG> = crate::FieldWriter<'a, REG, 2, RawFmt, crate::Safe>;
impl<'a, REG> RawFmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGRG GBGB"]
    #[inline(always)]
    pub fn rgrg_gbgb(self) -> &'a mut crate::W<REG> {
        self.variant(RawFmt::RgrgGbgb)
    }
    #[doc = "GRGR BGBG"]
    #[inline(always)]
    pub fn grgr_bgbg(self) -> &'a mut crate::W<REG> {
        self.variant(RawFmt::GrgrBgbg)
    }
    #[doc = "GBGB RGRG"]
    #[inline(always)]
    pub fn gbgb_rgrg(self) -> &'a mut crate::W<REG> {
        self.variant(RawFmt::GbgbRgrg)
    }
    #[doc = "BGBG GRGR"]
    #[inline(always)]
    pub fn bgbg_grgr(self) -> &'a mut crate::W<REG> {
        self.variant(RawFmt::BgbgGrgr)
    }
}
#[doc = "RGB type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RgbTyp {
    #[doc = "0: RGB444."]
    Rgb444 = 0,
    #[doc = "1: RGB555."]
    Rgb555 = 1,
    #[doc = "2: RGB565."]
    Rgb565 = 2,
    #[doc = "3: RGB666."]
    Rgb666 = 3,
    #[doc = "4: RGG888."]
    Rgg888 = 4,
}
impl From<RgbTyp> for u8 {
    #[inline(always)]
    fn from(variant: RgbTyp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RgbTyp {
    type Ux = u8;
}
impl crate::IsEnum for RgbTyp {}
#[doc = "Field `RGB_TYP` reader - RGB type."]
pub type RgbTypR = crate::FieldReader<RgbTyp>;
impl RgbTypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RgbTyp> {
        match self.bits {
            0 => Some(RgbTyp::Rgb444),
            1 => Some(RgbTyp::Rgb555),
            2 => Some(RgbTyp::Rgb565),
            3 => Some(RgbTyp::Rgb666),
            4 => Some(RgbTyp::Rgg888),
            _ => None,
        }
    }
    #[doc = "RGB444."]
    #[inline(always)]
    pub fn is_rgb444(&self) -> bool {
        *self == RgbTyp::Rgb444
    }
    #[doc = "RGB555."]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        *self == RgbTyp::Rgb555
    }
    #[doc = "RGB565."]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == RgbTyp::Rgb565
    }
    #[doc = "RGB666."]
    #[inline(always)]
    pub fn is_rgb666(&self) -> bool {
        *self == RgbTyp::Rgb666
    }
    #[doc = "RGG888."]
    #[inline(always)]
    pub fn is_rgg888(&self) -> bool {
        *self == RgbTyp::Rgg888
    }
}
#[doc = "Field `RGB_TYP` writer - RGB type."]
pub type RgbTypW<'a, REG> = crate::FieldWriter<'a, REG, 3, RgbTyp>;
impl<'a, REG> RgbTypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB444."]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut crate::W<REG> {
        self.variant(RgbTyp::Rgb444)
    }
    #[doc = "RGB555."]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut crate::W<REG> {
        self.variant(RgbTyp::Rgb555)
    }
    #[doc = "RGB565."]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(RgbTyp::Rgb565)
    }
    #[doc = "RGB666."]
    #[inline(always)]
    pub fn rgb666(self) -> &'a mut crate::W<REG> {
        self.variant(RgbTyp::Rgb666)
    }
    #[doc = "RGG888."]
    #[inline(always)]
    pub fn rgg888(self) -> &'a mut crate::W<REG> {
        self.variant(RgbTyp::Rgg888)
    }
}
impl R {
    #[doc = "Bit 0 - RAW conversion enable."]
    #[inline(always)]
    pub fn raw_cen(&self) -> RawCenR {
        RawCenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAW conversion FIFO automatic flush-out."]
    #[inline(always)]
    pub fn raw_ff_afo(&self) -> RawFfAfoR {
        RawFfAfoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RAW conversion FIFO flush-out trigger."]
    #[inline(always)]
    pub fn raw_ff_fo(&self) -> RawFfFoR {
        RawFfFoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RAW format."]
    #[inline(always)]
    pub fn raw_fmt(&self) -> RawFmtR {
        RawFmtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - RGB type."]
    #[inline(always)]
    pub fn rgb_typ(&self) -> RgbTypR {
        RgbTypR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RAW conversion enable."]
    #[inline(always)]
    pub fn raw_cen(&mut self) -> RawCenW<'_, VfifoRawCtrlSpec> {
        RawCenW::new(self, 0)
    }
    #[doc = "Bit 1 - RAW conversion FIFO automatic flush-out."]
    #[inline(always)]
    pub fn raw_ff_afo(&mut self) -> RawFfAfoW<'_, VfifoRawCtrlSpec> {
        RawFfAfoW::new(self, 1)
    }
    #[doc = "Bit 4 - RAW conversion FIFO flush-out trigger."]
    #[inline(always)]
    pub fn raw_ff_fo(&mut self) -> RawFfFoW<'_, VfifoRawCtrlSpec> {
        RawFfFoW::new(self, 4)
    }
    #[doc = "Bits 8:9 - RAW format."]
    #[inline(always)]
    pub fn raw_fmt(&mut self) -> RawFmtW<'_, VfifoRawCtrlSpec> {
        RawFmtW::new(self, 8)
    }
    #[doc = "Bits 12:14 - RGB type."]
    #[inline(always)]
    pub fn rgb_typ(&mut self) -> RgbTypW<'_, VfifoRawCtrlSpec> {
        RgbTypW::new(self, 12)
    }
}
#[doc = "Video FIFO RAW-to-RGB Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_raw_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_raw_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoRawCtrlSpec;
impl crate::RegisterSpec for VfifoRawCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_raw_ctrl::R`](R) reader structure"]
impl crate::Readable for VfifoRawCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_raw_ctrl::W`](W) writer structure"]
impl crate::Writable for VfifoRawCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_RAW_CTRL to value 0"]
impl crate::Resettable for VfifoRawCtrlSpec {}
