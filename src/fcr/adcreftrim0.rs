#[doc = "Register `ADCREFTRIM0` reader"]
pub type R = crate::R<Adcreftrim0Spec>;
#[doc = "Register `ADCREFTRIM0` writer"]
pub type W = crate::W<Adcreftrim0Spec>;
#[doc = "Field `VREFP` reader - Trimming code for VREFP output of reference buffer"]
pub type VrefpR = crate::FieldReader;
#[doc = "Field `VREFP` writer - Trimming code for VREFP output of reference buffer"]
pub type VrefpW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VREFM` reader - Trimming code for VREFM output of reference buffer"]
pub type VrefmR = crate::FieldReader;
#[doc = "Field `VREFM` writer - Trimming code for VREFM output of reference buffer"]
pub type VrefmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VCM` reader - Trimming code for VCM output of reference buffer"]
pub type VcmR = crate::FieldReader;
#[doc = "Field `VCM` writer - Trimming code for VCM output of reference buffer"]
pub type VcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VX2_TUNE` reader - Controls tuning capacitor in fine DAC (offset binary)"]
pub type Vx2TuneR = crate::FieldReader;
#[doc = "Field `VX2_TUNE` writer - Controls tuning capacitor in fine DAC (offset binary)"]
pub type Vx2TuneW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:6 - Trimming code for VREFP output of reference buffer"]
    #[inline(always)]
    pub fn vrefp(&self) -> VrefpR {
        VrefpR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Trimming code for VREFM output of reference buffer"]
    #[inline(always)]
    pub fn vrefm(&self) -> VrefmR {
        VrefmR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17 - Trimming code for VCM output of reference buffer"]
    #[inline(always)]
    pub fn vcm(&self) -> VcmR {
        VcmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Controls tuning capacitor in fine DAC (offset binary)"]
    #[inline(always)]
    pub fn vx2_tune(&self) -> Vx2TuneR {
        Vx2TuneR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trimming code for VREFP output of reference buffer"]
    #[inline(always)]
    pub fn vrefp(&mut self) -> VrefpW<'_, Adcreftrim0Spec> {
        VrefpW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Trimming code for VREFM output of reference buffer"]
    #[inline(always)]
    pub fn vrefm(&mut self) -> VrefmW<'_, Adcreftrim0Spec> {
        VrefmW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Trimming code for VCM output of reference buffer"]
    #[inline(always)]
    pub fn vcm(&mut self) -> VcmW<'_, Adcreftrim0Spec> {
        VcmW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Controls tuning capacitor in fine DAC (offset binary)"]
    #[inline(always)]
    pub fn vx2_tune(&mut self) -> Vx2TuneW<'_, Adcreftrim0Spec> {
        Vx2TuneW::new(self, 24)
    }
}
#[doc = "Temp Sensor trim1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcreftrim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcreftrim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcreftrim0Spec;
impl crate::RegisterSpec for Adcreftrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcreftrim0::R`](R) reader structure"]
impl crate::Readable for Adcreftrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcreftrim0::W`](W) writer structure"]
impl crate::Writable for Adcreftrim0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCREFTRIM0 to value 0"]
impl crate::Resettable for Adcreftrim0Spec {}
