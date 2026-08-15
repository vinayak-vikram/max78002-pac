#[doc = "Register `SDMA` reader"]
pub type R = crate::R<SdmaSpec>;
#[doc = "Register `SDMA` writer"]
pub type W = crate::W<SdmaSpec>;
#[doc = "Field `ADDR` reader - SDMA System Address / Argument 2 of Auto CMD23."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - SDMA System Address / Argument 2 of Auto CMD23."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SDMA System Address / Argument 2 of Auto CMD23."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SDMA System Address / Argument 2 of Auto CMD23."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SdmaSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "SDMA System Address / Argument 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmaSpec;
impl crate::RegisterSpec for SdmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdma::R`](R) reader structure"]
impl crate::Readable for SdmaSpec {}
#[doc = "`write(|w| ..)` method takes [`sdma::W`](W) writer structure"]
impl crate::Writable for SdmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDMA to value 0"]
impl crate::Resettable for SdmaSpec {}
