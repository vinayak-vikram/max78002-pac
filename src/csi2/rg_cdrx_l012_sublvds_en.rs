#[doc = "Register `RG_CDRX_L012_SUBLVDS_EN` reader"]
pub type R = crate::R<RgCdrxL012SublvdsEnSpec>;
#[doc = "Register `RG_CDRX_L012_SUBLVDS_EN` writer"]
pub type W = crate::W<RgCdrxL012SublvdsEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RG_CDRX_L012_SUBLVDS_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_l012_sublvds_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_l012_sublvds_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCdrxL012SublvdsEnSpec;
impl crate::RegisterSpec for RgCdrxL012SublvdsEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_cdrx_l012_sublvds_en::R`](R) reader structure"]
impl crate::Readable for RgCdrxL012SublvdsEnSpec {}
#[doc = "`write(|w| ..)` method takes [`rg_cdrx_l012_sublvds_en::W`](W) writer structure"]
impl crate::Writable for RgCdrxL012SublvdsEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CDRX_L012_SUBLVDS_EN to value 0"]
impl crate::Resettable for RgCdrxL012SublvdsEnSpec {}
