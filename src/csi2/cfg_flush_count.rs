#[doc = "Register `CFG_FLUSH_COUNT` reader"]
pub type R = crate::R<CfgFlushCountSpec>;
#[doc = "Register `CFG_FLUSH_COUNT` writer"]
pub type W = crate::W<CfgFlushCountSpec>;
#[doc = "Field `COUNT` reader - Flush count setting for controller."]
pub type CountR = crate::FieldReader;
#[doc = "Field `COUNT` writer - Flush count setting for controller."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Flush count setting for controller."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flush count setting for controller."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, CfgFlushCountSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "CFG_FLUSH_COUNT.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_flush_count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_flush_count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgFlushCountSpec;
impl crate::RegisterSpec for CfgFlushCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_flush_count::R`](R) reader structure"]
impl crate::Readable for CfgFlushCountSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_flush_count::W`](W) writer structure"]
impl crate::Writable for CfgFlushCountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_FLUSH_COUNT to value 0"]
impl crate::Resettable for CfgFlushCountSpec {}
