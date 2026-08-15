#[doc = "Register `M31_PHY_DEBUG_OUT` reader"]
pub type R = crate::R<M31PhyDebugOutSpec>;
#[doc = "Register `M31_PHY_DEBUG_OUT` writer"]
pub type W = crate::W<M31PhyDebugOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M31_PHY_DEBUG_OUT\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_debug_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_debug_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M31PhyDebugOutSpec;
impl crate::RegisterSpec for M31PhyDebugOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m31_phy_debug_out::R`](R) reader structure"]
impl crate::Readable for M31PhyDebugOutSpec {}
#[doc = "`write(|w| ..)` method takes [`m31_phy_debug_out::W`](W) writer structure"]
impl crate::Writable for M31PhyDebugOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M31_PHY_DEBUG_OUT to value 0"]
impl crate::Resettable for M31PhyDebugOutSpec {}
