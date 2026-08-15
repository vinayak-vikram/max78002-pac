#[doc = "Register `PPI_ERRCONTROL` reader"]
pub type R = crate::R<PpiErrcontrolSpec>;
#[doc = "Register `PPI_ERRCONTROL` writer"]
pub type W = crate::W<PpiErrcontrolSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPI_ERRCONTROL.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiErrcontrolSpec;
impl crate::RegisterSpec for PpiErrcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_errcontrol::R`](R) reader structure"]
impl crate::Readable for PpiErrcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_errcontrol::W`](W) writer structure"]
impl crate::Writable for PpiErrcontrolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_ERRCONTROL to value 0"]
impl crate::Resettable for PpiErrcontrolSpec {}
