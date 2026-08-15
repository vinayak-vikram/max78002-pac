#[doc = "Register `PPI_ERRESC` reader"]
pub type R = crate::R<PpiErrescSpec>;
#[doc = "Register `PPI_ERRESC` writer"]
pub type W = crate::W<PpiErrescSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPI_ERRESC.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_erresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_erresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiErrescSpec;
impl crate::RegisterSpec for PpiErrescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_erresc::R`](R) reader structure"]
impl crate::Readable for PpiErrescSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_erresc::W`](W) writer structure"]
impl crate::Writable for PpiErrescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_ERRESC to value 0"]
impl crate::Resettable for PpiErrescSpec {}
