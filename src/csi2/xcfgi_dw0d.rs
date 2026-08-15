#[doc = "Register `XCFGI_DW0D` reader"]
pub type R = crate::R<XcfgiDw0dSpec>;
#[doc = "Register `XCFGI_DW0D` writer"]
pub type W = crate::W<XcfgiDw0dSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XCFGI_DW0D.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XcfgiDw0dSpec;
impl crate::RegisterSpec for XcfgiDw0dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcfgi_dw0d::R`](R) reader structure"]
impl crate::Readable for XcfgiDw0dSpec {}
#[doc = "`write(|w| ..)` method takes [`xcfgi_dw0d::W`](W) writer structure"]
impl crate::Writable for XcfgiDw0dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCFGI_DW0D to value 0"]
impl crate::Resettable for XcfgiDw0dSpec {}
