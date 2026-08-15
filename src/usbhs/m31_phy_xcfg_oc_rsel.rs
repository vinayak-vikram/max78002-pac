#[doc = "Register `M31_PHY_XCFG_OC_RSEL` reader"]
pub type R = crate::R<M31PhyXcfgOcRselSpec>;
#[doc = "Register `M31_PHY_XCFG_OC_RSEL` writer"]
pub type W = crate::W<M31PhyXcfgOcRselSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M31_PHY_XCFG_OC_RSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_oc_rsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_oc_rsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M31PhyXcfgOcRselSpec;
impl crate::RegisterSpec for M31PhyXcfgOcRselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m31_phy_xcfg_oc_rsel::R`](R) reader structure"]
impl crate::Readable for M31PhyXcfgOcRselSpec {}
#[doc = "`write(|w| ..)` method takes [`m31_phy_xcfg_oc_rsel::W`](W) writer structure"]
impl crate::Writable for M31PhyXcfgOcRselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M31_PHY_XCFG_OC_RSEL to value 0"]
impl crate::Resettable for M31PhyXcfgOcRselSpec {}
