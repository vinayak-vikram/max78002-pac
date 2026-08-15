#[doc = "Register `M31_PHY_XCFG_LOCK_RANGE_MAX` reader"]
pub type R = crate::R<M31PhyXcfgLockRangeMaxSpec>;
#[doc = "Register `M31_PHY_XCFG_LOCK_RANGE_MAX` writer"]
pub type W = crate::W<M31PhyXcfgLockRangeMaxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M31_PHY_XCFG_LOCK_RANGE_MAX\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_lock_range_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_lock_range_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M31PhyXcfgLockRangeMaxSpec;
impl crate::RegisterSpec for M31PhyXcfgLockRangeMaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m31_phy_xcfg_lock_range_max::R`](R) reader structure"]
impl crate::Readable for M31PhyXcfgLockRangeMaxSpec {}
#[doc = "`write(|w| ..)` method takes [`m31_phy_xcfg_lock_range_max::W`](W) writer structure"]
impl crate::Writable for M31PhyXcfgLockRangeMaxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M31_PHY_XCFG_LOCK_RANGE_MAX to value 0"]
impl crate::Resettable for M31PhyXcfgLockRangeMaxSpec {}
