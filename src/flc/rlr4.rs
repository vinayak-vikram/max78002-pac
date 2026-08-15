#[doc = "Register `RLR4` reader"]
pub type R = crate::R<Rlr4Spec>;
#[doc = "Register `RLR4` writer"]
pub type W = crate::W<Rlr4Spec>;
#[doc = "Field `RLR4` reader - Access control."]
pub type Rlr4R = crate::FieldReader<u32>;
#[doc = "Field `RLR4` writer - Access control."]
pub type Rlr4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr4(&self) -> Rlr4R {
        Rlr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr4(&mut self) -> Rlr4W<'_, Rlr4Spec> {
        Rlr4W::new(self, 0)
    }
}
#[doc = "RLR4\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rlr4Spec;
impl crate::RegisterSpec for Rlr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr4::R`](R) reader structure"]
impl crate::Readable for Rlr4Spec {}
#[doc = "`write(|w| ..)` method takes [`rlr4::W`](W) writer structure"]
impl crate::Writable for Rlr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RLR4 to value 0"]
impl crate::Resettable for Rlr4Spec {}
