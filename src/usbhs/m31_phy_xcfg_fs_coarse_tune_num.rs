#[doc = "Register `M31_PHY_XCFG_FS_COARSE_TUNE_NUM` reader"]
pub type R = crate::R<M31PhyXcfgFsCoarseTuneNumSpec>;
#[doc = "Register `M31_PHY_XCFG_FS_COARSE_TUNE_NUM` writer"]
pub type W = crate::W<M31PhyXcfgFsCoarseTuneNumSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M31_PHY_XCFG_FS_COARSE_TUNE_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_fs_coarse_tune_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_fs_coarse_tune_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M31PhyXcfgFsCoarseTuneNumSpec;
impl crate::RegisterSpec for M31PhyXcfgFsCoarseTuneNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m31_phy_xcfg_fs_coarse_tune_num::R`](R) reader structure"]
impl crate::Readable for M31PhyXcfgFsCoarseTuneNumSpec {}
#[doc = "`write(|w| ..)` method takes [`m31_phy_xcfg_fs_coarse_tune_num::W`](W) writer structure"]
impl crate::Writable for M31PhyXcfgFsCoarseTuneNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M31_PHY_XCFG_FS_COARSE_TUNE_NUM to value 0"]
impl crate::Resettable for M31PhyXcfgFsCoarseTuneNumSpec {}
