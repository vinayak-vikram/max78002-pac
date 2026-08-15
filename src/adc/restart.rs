#[doc = "Register `RESTART` reader"]
pub type R = crate::R<RestartSpec>;
#[doc = "Register `RESTART` writer"]
pub type W = crate::W<RestartSpec>;
#[doc = "Field `CNT` reader - Number of sample periods to skip before restarting a continuous mode sequence"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of sample periods to skip before restarting a continuous mode sequence"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of sample periods to skip before restarting a continuous mode sequence"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of sample periods to skip before restarting a continuous mode sequence"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, RestartSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Restart Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`restart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RestartSpec;
impl crate::RegisterSpec for RestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restart::R`](R) reader structure"]
impl crate::Readable for RestartSpec {}
#[doc = "`write(|w| ..)` method takes [`restart::W`](W) writer structure"]
impl crate::Writable for RestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RestartSpec {}
