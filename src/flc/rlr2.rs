#[doc = "Register `RLR2` reader"]
pub type R = crate::R<Rlr2Spec>;
#[doc = "Register `RLR2` writer"]
pub type W = crate::W<Rlr2Spec>;
#[doc = "Field `RLR2` reader - Access control."]
pub type Rlr2R = crate::FieldReader<u32>;
#[doc = "Field `RLR2` writer - Access control."]
pub type Rlr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr2(&self) -> Rlr2R {
        Rlr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr2(&mut self) -> Rlr2W<'_, Rlr2Spec> {
        Rlr2W::new(self, 0)
    }
}
#[doc = "RLR2\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rlr2Spec;
impl crate::RegisterSpec for Rlr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr2::R`](R) reader structure"]
impl crate::Readable for Rlr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rlr2::W`](W) writer structure"]
impl crate::Writable for Rlr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RLR2 to value 0"]
impl crate::Resettable for Rlr2Spec {}
