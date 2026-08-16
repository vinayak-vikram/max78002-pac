#[doc = "Register `IS` reader"]
pub type R = crate::R<IsSpec>;
#[doc = "Register `IS` writer"]
pub type W = crate::W<IsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`is::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`is::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsSpec;
impl crate::RegisterSpec for IsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`is::R`](R) reader structure"]
impl crate::Readable for IsSpec {}
#[doc = "`write(|w| ..)` method takes [`is::W`](W) writer structure"]
impl crate::Writable for IsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IsSpec {}
