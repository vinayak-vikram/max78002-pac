#[doc = "Register `RLR3` reader"]
pub type R = crate::R<Rlr3Spec>;
#[doc = "Register `RLR3` writer"]
pub type W = crate::W<Rlr3Spec>;
#[doc = "Field `RLR3` reader - Access control."]
pub type Rlr3R = crate::FieldReader<u32>;
#[doc = "Field `RLR3` writer - Access control."]
pub type Rlr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr3(&self) -> Rlr3R {
        Rlr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn rlr3(&mut self) -> Rlr3W<'_, Rlr3Spec> {
        Rlr3W::new(self, 0)
    }
}
#[doc = "RLR3\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rlr3Spec;
impl crate::RegisterSpec for Rlr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr3::R`](R) reader structure"]
impl crate::Readable for Rlr3Spec {}
#[doc = "`write(|w| ..)` method takes [`rlr3::W`](W) writer structure"]
impl crate::Writable for Rlr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RLR3 to value 0"]
impl crate::Resettable for Rlr3Spec {}
