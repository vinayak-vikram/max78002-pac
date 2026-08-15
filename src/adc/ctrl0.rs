#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "ADC Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcEn {
    #[doc = "0: Disable ADC."]
    Dis = 0,
    #[doc = "1: enable ADC."]
    En = 1,
}
impl From<AdcEn> for bool {
    #[inline(always)]
    fn from(variant: AdcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_EN` reader - ADC Enable."]
pub type AdcEnR = crate::BitReader<AdcEn>;
impl AdcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcEn {
        match self.bits {
            false => AdcEn::Dis,
            true => AdcEn::En,
        }
    }
    #[doc = "Disable ADC."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == AdcEn::Dis
    }
    #[doc = "enable ADC."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == AdcEn::En
    }
}
#[doc = "Field `ADC_EN` writer - ADC Enable."]
pub type AdcEnW<'a, REG> = crate::BitWriter<'a, REG, AdcEn>;
impl<'a, REG> AdcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable ADC."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(AdcEn::Dis)
    }
    #[doc = "enable ADC."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(AdcEn::En)
    }
}
#[doc = "Bias Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BiasEn {
    #[doc = "0: Disable Bias."]
    Dis = 0,
    #[doc = "1: Enable Bias."]
    En = 1,
}
impl From<BiasEn> for bool {
    #[inline(always)]
    fn from(variant: BiasEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIAS_EN` reader - Bias Enable."]
pub type BiasEnR = crate::BitReader<BiasEn>;
impl BiasEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BiasEn {
        match self.bits {
            false => BiasEn::Dis,
            true => BiasEn::En,
        }
    }
    #[doc = "Disable Bias."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == BiasEn::Dis
    }
    #[doc = "Enable Bias."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == BiasEn::En
    }
}
#[doc = "Field `BIAS_EN` writer - Bias Enable."]
pub type BiasEnW<'a, REG> = crate::BitWriter<'a, REG, BiasEn>;
impl<'a, REG> BiasEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Bias."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(BiasEn::Dis)
    }
    #[doc = "Enable Bias."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(BiasEn::En)
    }
}
#[doc = "Skip Calibration Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipCal {
    #[doc = "0: Do not skip calibration."]
    NoSkip = 0,
    #[doc = "1: Skip calibration."]
    Skip = 1,
}
impl From<SkipCal> for bool {
    #[inline(always)]
    fn from(variant: SkipCal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKIP_CAL` reader - Skip Calibration Enable."]
pub type SkipCalR = crate::BitReader<SkipCal>;
impl SkipCalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SkipCal {
        match self.bits {
            false => SkipCal::NoSkip,
            true => SkipCal::Skip,
        }
    }
    #[doc = "Do not skip calibration."]
    #[inline(always)]
    pub fn is_no_skip(&self) -> bool {
        *self == SkipCal::NoSkip
    }
    #[doc = "Skip calibration."]
    #[inline(always)]
    pub fn is_skip(&self) -> bool {
        *self == SkipCal::Skip
    }
}
#[doc = "Field `SKIP_CAL` writer - Skip Calibration Enable."]
pub type SkipCalW<'a, REG> = crate::BitWriter<'a, REG, SkipCal>;
impl<'a, REG> SkipCalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not skip calibration."]
    #[inline(always)]
    pub fn no_skip(self) -> &'a mut crate::W<REG> {
        self.variant(SkipCal::NoSkip)
    }
    #[doc = "Skip calibration."]
    #[inline(always)]
    pub fn skip(self) -> &'a mut crate::W<REG> {
        self.variant(SkipCal::Skip)
    }
}
#[doc = "Chop Force Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChopForce {
    #[doc = "0: Do not force chop mode."]
    Dis = 0,
    #[doc = "1: Force chop Mode."]
    En = 1,
}
impl From<ChopForce> for bool {
    #[inline(always)]
    fn from(variant: ChopForce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHOP_FORCE` reader - Chop Force Control."]
pub type ChopForceR = crate::BitReader<ChopForce>;
impl ChopForceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChopForce {
        match self.bits {
            false => ChopForce::Dis,
            true => ChopForce::En,
        }
    }
    #[doc = "Do not force chop mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ChopForce::Dis
    }
    #[doc = "Force chop Mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ChopForce::En
    }
}
#[doc = "Field `CHOP_FORCE` writer - Chop Force Control."]
pub type ChopForceW<'a, REG> = crate::BitWriter<'a, REG, ChopForce>;
impl<'a, REG> ChopForceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not force chop mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ChopForce::Dis)
    }
    #[doc = "Force chop Mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ChopForce::En)
    }
}
#[doc = "Reset ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resetb {
    #[doc = "0: reset ADC."]
    Reset = 0,
    #[doc = "1: activate ADC."]
    Activate = 1,
}
impl From<Resetb> for bool {
    #[inline(always)]
    fn from(variant: Resetb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETB` reader - Reset ADC."]
pub type ResetbR = crate::BitReader<Resetb>;
impl ResetbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resetb {
        match self.bits {
            false => Resetb::Reset,
            true => Resetb::Activate,
        }
    }
    #[doc = "reset ADC."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Resetb::Reset
    }
    #[doc = "activate ADC."]
    #[inline(always)]
    pub fn is_activate(&self) -> bool {
        *self == Resetb::Activate
    }
}
#[doc = "Field `RESETB` writer - Reset ADC."]
pub type ResetbW<'a, REG> = crate::BitWriter<'a, REG, Resetb>;
impl<'a, REG> ResetbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "reset ADC."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Resetb::Reset)
    }
    #[doc = "activate ADC."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(Resetb::Activate)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Enable."]
    #[inline(always)]
    pub fn adc_en(&self) -> AdcEnR {
        AdcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bias Enable."]
    #[inline(always)]
    pub fn bias_en(&self) -> BiasEnR {
        BiasEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Skip Calibration Enable."]
    #[inline(always)]
    pub fn skip_cal(&self) -> SkipCalR {
        SkipCalR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chop Force Control."]
    #[inline(always)]
    pub fn chop_force(&self) -> ChopForceR {
        ChopForceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset ADC."]
    #[inline(always)]
    pub fn resetb(&self) -> ResetbR {
        ResetbR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Enable."]
    #[inline(always)]
    pub fn adc_en(&mut self) -> AdcEnW<'_, Ctrl0Spec> {
        AdcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Bias Enable."]
    #[inline(always)]
    pub fn bias_en(&mut self) -> BiasEnW<'_, Ctrl0Spec> {
        BiasEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Skip Calibration Enable."]
    #[inline(always)]
    pub fn skip_cal(&mut self) -> SkipCalW<'_, Ctrl0Spec> {
        SkipCalW::new(self, 2)
    }
    #[doc = "Bit 3 - Chop Force Control."]
    #[inline(always)]
    pub fn chop_force(&mut self) -> ChopForceW<'_, Ctrl0Spec> {
        ChopForceW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset ADC."]
    #[inline(always)]
    pub fn resetb(&mut self) -> ResetbW<'_, Ctrl0Spec> {
        ResetbW::new(self, 4)
    }
}
#[doc = "Control Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {}
