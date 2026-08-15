#[doc = "Register `ARG_1` reader"]
pub type R = crate::R<Arg1Spec>;
#[doc = "Register `ARG_1` writer"]
pub type W = crate::W<Arg1Spec>;
#[doc = "Field `CMD` reader - Command Argument 1."]
pub type CmdR = crate::FieldReader<u32>;
#[doc = "Field `CMD` writer - Command Argument 1."]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Argument 1."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument 1."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, Arg1Spec> {
        CmdW::new(self, 0)
    }
}
#[doc = "Argument 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`arg_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Arg1Spec;
impl crate::RegisterSpec for Arg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg_1::R`](R) reader structure"]
impl crate::Readable for Arg1Spec {}
#[doc = "`write(|w| ..)` method takes [`arg_1::W`](W) writer structure"]
impl crate::Writable for Arg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARG_1 to value 0"]
impl crate::Resettable for Arg1Spec {}
