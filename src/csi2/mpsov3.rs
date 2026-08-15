#[doc = "Register `MPSOV3` reader"]
pub type R = crate::R<Mpsov3Spec>;
#[doc = "Register `MPSOV3` writer"]
pub type W = crate::W<Mpsov3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MPSOV3.\n\nYou can [`read`](crate::Reg::read) this register and get [`mpsov3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpsov3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpsov3Spec;
impl crate::RegisterSpec for Mpsov3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpsov3::R`](R) reader structure"]
impl crate::Readable for Mpsov3Spec {}
#[doc = "`write(|w| ..)` method takes [`mpsov3::W`](W) writer structure"]
impl crate::Writable for Mpsov3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPSOV3 to value 0"]
impl crate::Resettable for Mpsov3Spec {}
