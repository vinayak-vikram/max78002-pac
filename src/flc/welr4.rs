#[doc = "Register `WELR4` reader"]
pub type R = crate::R<Welr4Spec>;
#[doc = "Register `WELR4` writer"]
pub type W = crate::W<Welr4Spec>;
#[doc = "Field `WELR4` reader - Access control."]
pub type Welr4R = crate::FieldReader<u32>;
#[doc = "Field `WELR4` writer - Access control."]
pub type Welr4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr4(&self) -> Welr4R {
        Welr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn welr4(&mut self) -> Welr4W<'_, Welr4Spec> {
        Welr4W::new(self, 0)
    }
}
#[doc = "WELR4\n\nYou can [`read`](crate::Reg::read) this register and get [`welr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Welr4Spec;
impl crate::RegisterSpec for Welr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`welr4::R`](R) reader structure"]
impl crate::Readable for Welr4Spec {}
#[doc = "`write(|w| ..)` method takes [`welr4::W`](W) writer structure"]
impl crate::Writable for Welr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WELR4 to value 0"]
impl crate::Resettable for Welr4Spec {}
