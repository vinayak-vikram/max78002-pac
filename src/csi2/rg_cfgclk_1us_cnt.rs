#[doc = "Register `RG_CFGCLK_1US_CNT` reader"]
pub type R = crate::R<RgCfgclk1usCntSpec>;
#[doc = "Register `RG_CFGCLK_1US_CNT` writer"]
pub type W = crate::W<RgCfgclk1usCntSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RG_CFGCLK_1US_CNT.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cfgclk_1us_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cfgclk_1us_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCfgclk1usCntSpec;
impl crate::RegisterSpec for RgCfgclk1usCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_cfgclk_1us_cnt::R`](R) reader structure"]
impl crate::Readable for RgCfgclk1usCntSpec {}
#[doc = "`write(|w| ..)` method takes [`rg_cfgclk_1us_cnt::W`](W) writer structure"]
impl crate::Writable for RgCfgclk1usCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CFGCLK_1US_CNT to value 0"]
impl crate::Resettable for RgCfgclk1usCntSpec {}
