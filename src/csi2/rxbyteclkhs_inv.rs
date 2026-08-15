#[doc = "Register `RXBYTECLKHS_INV` reader"]
pub type R = crate::R<RxbyteclkhsInvSpec>;
#[doc = "Register `RXBYTECLKHS_INV` writer"]
pub type W = crate::W<RxbyteclkhsInvSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RXBYTECLKHS_INV.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbyteclkhs_inv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbyteclkhs_inv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbyteclkhsInvSpec;
impl crate::RegisterSpec for RxbyteclkhsInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbyteclkhs_inv::R`](R) reader structure"]
impl crate::Readable for RxbyteclkhsInvSpec {}
#[doc = "`write(|w| ..)` method takes [`rxbyteclkhs_inv::W`](W) writer structure"]
impl crate::Writable for RxbyteclkhsInvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXBYTECLKHS_INV to value 0"]
impl crate::Resettable for RxbyteclkhsInvSpec {}
