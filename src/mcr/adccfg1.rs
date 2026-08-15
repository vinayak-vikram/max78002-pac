#[doc = "Register `ADCCFG1` reader"]
pub type R = crate::R<Adccfg1Spec>;
#[doc = "Register `ADCCFG1` writer"]
pub type W = crate::W<Adccfg1Spec>;
#[doc = "Field `CHX_PU_DYN` reader - ADC PU dynamic control"]
pub type ChxPuDynR = crate::FieldReader<u16>;
#[doc = "Field `CHX_PU_DYN` writer - ADC PU dynamic control"]
pub type ChxPuDynW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - ADC PU dynamic control"]
    #[inline(always)]
    pub fn chx_pu_dyn(&self) -> ChxPuDynR {
        ChxPuDynR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - ADC PU dynamic control"]
    #[inline(always)]
    pub fn chx_pu_dyn(&mut self) -> ChxPuDynW<'_, Adccfg1Spec> {
        ChxPuDynW::new(self, 0)
    }
}
#[doc = "ADC Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccfg1Spec;
impl crate::RegisterSpec for Adccfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccfg1::R`](R) reader structure"]
impl crate::Readable for Adccfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`adccfg1::W`](W) writer structure"]
impl crate::Writable for Adccfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCFG1 to value 0"]
impl crate::Resettable for Adccfg1Spec {}
