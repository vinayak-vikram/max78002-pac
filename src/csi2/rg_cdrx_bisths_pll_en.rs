#[doc = "Register `RG_CDRX_BISTHS_PLL_EN` reader"]
pub type R = crate::R<RgCdrxBisthsPllEnSpec>;
#[doc = "Register `RG_CDRX_BISTHS_PLL_EN` writer"]
pub type W = crate::W<RgCdrxBisthsPllEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RG_CDRX_BISTHS_PLL_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_bisths_pll_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_bisths_pll_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCdrxBisthsPllEnSpec;
impl crate::RegisterSpec for RgCdrxBisthsPllEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_cdrx_bisths_pll_en::R`](R) reader structure"]
impl crate::Readable for RgCdrxBisthsPllEnSpec {}
#[doc = "`write(|w| ..)` method takes [`rg_cdrx_bisths_pll_en::W`](W) writer structure"]
impl crate::Writable for RgCdrxBisthsPllEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CDRX_BISTHS_PLL_EN to value 0"]
impl crate::Resettable for RgCdrxBisthsPllEnSpec {}
