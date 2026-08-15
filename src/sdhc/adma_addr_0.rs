#[doc = "Register `ADMA_ADDR_0` reader"]
pub type R = crate::R<AdmaAddr0Spec>;
#[doc = "Register `ADMA_ADDR_0` writer"]
pub type W = crate::W<AdmaAddr0Spec>;
#[doc = "Field `ADDR` reader - ADMA System Address Part 1 (part 2 is ADMA_ADDR_1)."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - ADMA System Address Part 1 (part 2 is ADMA_ADDR_1)."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address Part 1 (part 2 is ADMA_ADDR_1)."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address Part 1 (part 2 is ADMA_ADDR_1)."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AdmaAddr0Spec> {
        AddrW::new(self, 0)
    }
}
#[doc = "ADMA System Address 0-31.\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_addr_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_addr_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaAddr0Spec;
impl crate::RegisterSpec for AdmaAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma_addr_0::R`](R) reader structure"]
impl crate::Readable for AdmaAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`adma_addr_0::W`](W) writer structure"]
impl crate::Writable for AdmaAddr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADMA_ADDR_0 to value 0"]
impl crate::Resettable for AdmaAddr0Spec {}
