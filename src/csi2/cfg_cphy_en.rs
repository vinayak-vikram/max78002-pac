#[doc = "Register `CFG_CPHY_EN` reader"]
pub type R = crate::R<CfgCphyEnSpec>;
#[doc = "Register `CFG_CPHY_EN` writer"]
pub type W = crate::W<CfgCphyEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_CPHY_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cphy_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cphy_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCphyEnSpec;
impl crate::RegisterSpec for CfgCphyEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cphy_en::R`](R) reader structure"]
impl crate::Readable for CfgCphyEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cphy_en::W`](W) writer structure"]
impl crate::Writable for CfgCphyEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_CPHY_EN to value 0"]
impl crate::Resettable for CfgCphyEnSpec {}
