#[doc = "Register `SFRSTATUS` reader"]
pub type R = crate::R<SfrstatusSpec>;
#[doc = "Register `SFRSTATUS` writer"]
pub type W = crate::W<SfrstatusSpec>;
#[doc = "Field `NACK` reader - NACK status for SAR Digital SFR communication"]
pub type NackR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NACK status for SAR Digital SFR communication"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "SFR Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrstatusSpec;
impl crate::RegisterSpec for SfrstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfrstatus::R`](R) reader structure"]
impl crate::Readable for SfrstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrstatus::W`](W) writer structure"]
impl crate::Writable for SfrstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRSTATUS to value 0"]
impl crate::Resettable for SfrstatusSpec {}
