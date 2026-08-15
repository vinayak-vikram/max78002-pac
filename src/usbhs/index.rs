#[doc = "Register `INDEX` reader"]
pub type R = crate::R<IndexSpec>;
#[doc = "Register `INDEX` writer"]
pub type W = crate::W<IndexSpec>;
#[doc = "Field `INDEX` reader - Index Register Access Selector."]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - Index Register Access Selector."]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Index Register Access Selector."]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Index Register Access Selector."]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, IndexSpec> {
        IndexW::new(self, 0)
    }
}
#[doc = "Index for banked registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`index::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`index::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndexSpec;
impl crate::RegisterSpec for IndexSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`index::R`](R) reader structure"]
impl crate::Readable for IndexSpec {}
#[doc = "`write(|w| ..)` method takes [`index::W`](W) writer structure"]
impl crate::Writable for IndexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDEX to value 0"]
impl crate::Resettable for IndexSpec {}
