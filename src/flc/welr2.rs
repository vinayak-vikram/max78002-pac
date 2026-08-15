#[doc = "Register `WELR2` reader"]
pub type R = crate::R<Welr2Spec>;
#[doc = "Register `WELR2` writer"]
pub type W = crate::W<Welr2Spec>;
#[doc = "Field `WELR2` reader - Access control."]
pub type Welr2R = crate::FieldReader<u32>;
#[doc = "Field `WELR2` writer - Access control."]
pub type Welr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr2(&self) -> Welr2R {
        Welr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr2(&mut self) -> Welr2W<'_, Welr2Spec> {
        Welr2W::new(self, 0)
    }
}
#[doc = "WELR2\n\nYou can [`read`](crate::Reg::read) this register and get [`welr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Welr2Spec;
impl crate::RegisterSpec for Welr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr2::R`](R) reader structure"]
impl crate::Readable for Welr2Spec {}
#[doc = "`write(|w| ..)` method takes [`welr2::W`](W) writer structure"]
impl crate::Writable for Welr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WELR2 to value 0"]
impl crate::Resettable for Welr2Spec {}
