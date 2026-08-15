#[doc = "Register `XCFGI_DW0A` reader"]
pub type R = crate::R<XcfgiDw0aSpec>;
#[doc = "Register `XCFGI_DW0A` writer"]
pub type W = crate::W<XcfgiDw0aSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XCFGI_DW0A.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XcfgiDw0aSpec;
impl crate::RegisterSpec for XcfgiDw0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcfgi_dw0a::R`](R) reader structure"]
impl crate::Readable for XcfgiDw0aSpec {}
#[doc = "`write(|w| ..)` method takes [`xcfgi_dw0a::W`](W) writer structure"]
impl crate::Writable for XcfgiDw0aSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCFGI_DW0A to value 0"]
impl crate::Resettable for XcfgiDw0aSpec {}
