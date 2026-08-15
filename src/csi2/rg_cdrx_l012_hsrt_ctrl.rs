#[doc = "Register `RG_CDRX_L012_HSRT_CTRL` reader"]
pub type R = crate::R<RgCdrxL012HsrtCtrlSpec>;
#[doc = "Register `RG_CDRX_L012_HSRT_CTRL` writer"]
pub type W = crate::W<RgCdrxL012HsrtCtrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RG_CDRX_L012_HSRT_CTRL.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_l012_hsrt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_l012_hsrt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCdrxL012HsrtCtrlSpec;
impl crate::RegisterSpec for RgCdrxL012HsrtCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_cdrx_l012_hsrt_ctrl::R`](R) reader structure"]
impl crate::Readable for RgCdrxL012HsrtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rg_cdrx_l012_hsrt_ctrl::W`](W) writer structure"]
impl crate::Writable for RgCdrxL012HsrtCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CDRX_L012_HSRT_CTRL to value 0"]
impl crate::Resettable for RgCdrxL012HsrtCtrlSpec {}
