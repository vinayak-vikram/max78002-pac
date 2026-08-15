#[doc = "Register `XCFGI_DW0B` reader"]
pub type R = crate::R<XcfgiDw0bSpec>;
#[doc = "Register `XCFGI_DW0B` writer"]
pub type W = crate::W<XcfgiDw0bSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "XCFGI_DW0B.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XcfgiDw0bSpec;
impl crate::RegisterSpec for XcfgiDw0bSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcfgi_dw0b::R`](R) reader structure"]
impl crate::Readable for XcfgiDw0bSpec {}
#[doc = "`write(|w| ..)` method takes [`xcfgi_dw0b::W`](W) writer structure"]
impl crate::Writable for XcfgiDw0bSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCFGI_DW0B to value 0"]
impl crate::Resettable for XcfgiDw0bSpec {}
