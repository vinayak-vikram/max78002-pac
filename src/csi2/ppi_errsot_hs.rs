#[doc = "Register `PPI_ERRSOT_HS` reader"]
pub type R = crate::R<PpiErrsotHsSpec>;
#[doc = "Register `PPI_ERRSOT_HS` writer"]
pub type W = crate::W<PpiErrsotHsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPI_ERRSOT_HS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errsot_hs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errsot_hs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiErrsotHsSpec;
impl crate::RegisterSpec for PpiErrsotHsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_errsot_hs::R`](R) reader structure"]
impl crate::Readable for PpiErrsotHsSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_errsot_hs::W`](W) writer structure"]
impl crate::Writable for PpiErrsotHsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_ERRSOT_HS to value 0"]
impl crate::Resettable for PpiErrsotHsSpec {}
