#[doc = "Register `XCFGI_DW05` reader"]
pub type R = crate::R<XcfgiDw05Spec>;
#[doc = "Register `XCFGI_DW05` writer"]
pub type W = crate::W<XcfgiDw05Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XCFGI_DW05.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw05::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw05::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XcfgiDw05Spec;
impl crate::RegisterSpec for XcfgiDw05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcfgi_dw05::R`](R) reader structure"]
impl crate::Readable for XcfgiDw05Spec {}
#[doc = "`write(|w| ..)` method takes [`xcfgi_dw05::W`](W) writer structure"]
impl crate::Writable for XcfgiDw05Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCFGI_DW05 to value 0"]
impl crate::Resettable for XcfgiDw05Spec {}
