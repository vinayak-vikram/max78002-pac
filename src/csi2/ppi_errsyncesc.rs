#[doc = "Register `PPI_ERRSYNCESC` reader"]
pub type R = crate::R<PpiErrsyncescSpec>;
#[doc = "Register `PPI_ERRSYNCESC` writer"]
pub type W = crate::W<PpiErrsyncescSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPI_ERRSYNCESC.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errsyncesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errsyncesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiErrsyncescSpec;
impl crate::RegisterSpec for PpiErrsyncescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_errsyncesc::R`](R) reader structure"]
impl crate::Readable for PpiErrsyncescSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_errsyncesc::W`](W) writer structure"]
impl crate::Writable for PpiErrsyncescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_ERRSYNCESC to value 0"]
impl crate::Resettable for PpiErrsyncescSpec {}
