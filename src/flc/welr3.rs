#[doc = "Register `WELR3` reader"]
pub type R = crate::R<Welr3Spec>;
#[doc = "Register `WELR3` writer"]
pub type W = crate::W<Welr3Spec>;
#[doc = "Field `WELR3` reader - Access control."]
pub type Welr3R = crate::FieldReader<u32>;
#[doc = "Field `WELR3` writer - Access control."]
pub type Welr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr3(&self) -> Welr3R {
        Welr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr3(&mut self) -> Welr3W<'_, Welr3Spec> {
        Welr3W::new(self, 0)
    }
}
#[doc = "WELR3\n\nYou can [`read`](crate::Reg::read) this register and get [`welr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Welr3Spec;
impl crate::RegisterSpec for Welr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr3::R`](R) reader structure"]
impl crate::Readable for Welr3Spec {}
#[doc = "`write(|w| ..)` method takes [`welr3::W`](W) writer structure"]
impl crate::Writable for Welr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WELR3 to value 0"]
impl crate::Resettable for Welr3Spec {}
