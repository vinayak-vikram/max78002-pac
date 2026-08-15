#[doc = "Register `XCFGI_DW08` reader"]
pub type R = crate::R<XcfgiDw08Spec>;
#[doc = "Register `XCFGI_DW08` writer"]
pub type W = crate::W<XcfgiDw08Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XCFGI_DW08.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XcfgiDw08Spec;
impl crate::RegisterSpec for XcfgiDw08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcfgi_dw08::R`](R) reader structure"]
impl crate::Readable for XcfgiDw08Spec {}
#[doc = "`write(|w| ..)` method takes [`xcfgi_dw08::W`](W) writer structure"]
impl crate::Writable for XcfgiDw08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCFGI_DW08 to value 0"]
impl crate::Resettable for XcfgiDw08Spec {}
