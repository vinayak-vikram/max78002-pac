#[doc = "Register `MPSOV2` reader"]
pub type R = crate::R<Mpsov2Spec>;
#[doc = "Register `MPSOV2` writer"]
pub type W = crate::W<Mpsov2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MPSOV2.\n\nYou can [`read`](crate::Reg::read) this register and get [`mpsov2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpsov2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mpsov2Spec;
impl crate::RegisterSpec for Mpsov2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpsov2::R`](R) reader structure"]
impl crate::Readable for Mpsov2Spec {}
#[doc = "`write(|w| ..)` method takes [`mpsov2::W`](W) writer structure"]
impl crate::Writable for Mpsov2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MPSOV2 to value 0"]
impl crate::Resettable for Mpsov2Spec {}
