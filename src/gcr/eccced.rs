#[doc = "Register `ECCCED` reader"]
pub type R = crate::R<EcccedSpec>;
#[doc = "Register `ECCCED` writer"]
pub type W = crate::W<EcccedSpec>;
#[doc = "Field `RAM` reader - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RamR = crate::BitReader;
#[doc = "Field `RAM` writer - ECC System RAM0 Error Flag. Write 1 to clear."]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Error Flag. Write 1 to clear."]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<'_, EcccedSpec> {
        RamW::new(self, 0)
    }
}
#[doc = "ECC Not Double Error Detect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccced::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccced::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcccedSpec;
impl crate::RegisterSpec for EcccedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccced::R`](R) reader structure"]
impl crate::Readable for EcccedSpec {}
#[doc = "`write(|w| ..)` method takes [`eccced::W`](W) writer structure"]
impl crate::Writable for EcccedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECCCED to value 0"]
impl crate::Resettable for EcccedSpec {}
