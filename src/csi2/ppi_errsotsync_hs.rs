#[doc = "Register `PPI_ERRSOTSYNC_HS` reader"]
pub type R = crate::R<PpiErrsotsyncHsSpec>;
#[doc = "Register `PPI_ERRSOTSYNC_HS` writer"]
pub type W = crate::W<PpiErrsotsyncHsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPI_ERRSOTSYNC_HS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errsotsync_hs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errsotsync_hs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiErrsotsyncHsSpec;
impl crate::RegisterSpec for PpiErrsotsyncHsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_errsotsync_hs::R`](R) reader structure"]
impl crate::Readable for PpiErrsotsyncHsSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_errsotsync_hs::W`](W) writer structure"]
impl crate::Writable for PpiErrsotsyncHsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_ERRSOTSYNC_HS to value 0"]
impl crate::Resettable for PpiErrsotsyncHsSpec {}
