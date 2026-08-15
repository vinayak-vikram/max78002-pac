#[doc = "Register `CFG_DPDN_SWAP` reader"]
pub type R = crate::R<CfgDpdnSwapSpec>;
#[doc = "Register `CFG_DPDN_SWAP` writer"]
pub type W = crate::W<CfgDpdnSwapSpec>;
#[doc = "Field `SWAP_DATA_LANE0` reader - SWAP_DATA_LANE0."]
pub type SwapDataLane0R = crate::BitReader;
#[doc = "Field `SWAP_DATA_LANE0` writer - SWAP_DATA_LANE0."]
pub type SwapDataLane0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_DATA_LANE1` reader - SWAP_DATA_LANE1."]
pub type SwapDataLane1R = crate::BitReader;
#[doc = "Field `SWAP_DATA_LANE1` writer - SWAP_DATA_LANE1."]
pub type SwapDataLane1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_DATA_LANE2` reader - SWAP_DATA_LANE2."]
pub type SwapDataLane2R = crate::BitReader;
#[doc = "Field `SWAP_DATA_LANE2` writer - SWAP_DATA_LANE2."]
pub type SwapDataLane2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_DATA_LANE3` reader - SWAP_DATA_LANE3."]
pub type SwapDataLane3R = crate::BitReader;
#[doc = "Field `SWAP_DATA_LANE3` writer - SWAP_DATA_LANE3."]
pub type SwapDataLane3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_CLK_LANE` reader - SWAP_CLK_LANE."]
pub type SwapClkLaneR = crate::BitReader;
#[doc = "Field `SWAP_CLK_LANE` writer - SWAP_CLK_LANE."]
pub type SwapClkLaneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SWAP_DATA_LANE0."]
    #[inline(always)]
    pub fn swap_data_lane0(&self) -> SwapDataLane0R {
        SwapDataLane0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWAP_DATA_LANE1."]
    #[inline(always)]
    pub fn swap_data_lane1(&self) -> SwapDataLane1R {
        SwapDataLane1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWAP_DATA_LANE2."]
    #[inline(always)]
    pub fn swap_data_lane2(&self) -> SwapDataLane2R {
        SwapDataLane2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SWAP_DATA_LANE3."]
    #[inline(always)]
    pub fn swap_data_lane3(&self) -> SwapDataLane3R {
        SwapDataLane3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SWAP_CLK_LANE."]
    #[inline(always)]
    pub fn swap_clk_lane(&self) -> SwapClkLaneR {
        SwapClkLaneR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWAP_DATA_LANE0."]
    #[inline(always)]
    pub fn swap_data_lane0(&mut self) -> SwapDataLane0W<'_, CfgDpdnSwapSpec> {
        SwapDataLane0W::new(self, 0)
    }
    #[doc = "Bit 1 - SWAP_DATA_LANE1."]
    #[inline(always)]
    pub fn swap_data_lane1(&mut self) -> SwapDataLane1W<'_, CfgDpdnSwapSpec> {
        SwapDataLane1W::new(self, 1)
    }
    #[doc = "Bit 2 - SWAP_DATA_LANE2."]
    #[inline(always)]
    pub fn swap_data_lane2(&mut self) -> SwapDataLane2W<'_, CfgDpdnSwapSpec> {
        SwapDataLane2W::new(self, 2)
    }
    #[doc = "Bit 3 - SWAP_DATA_LANE3."]
    #[inline(always)]
    pub fn swap_data_lane3(&mut self) -> SwapDataLane3W<'_, CfgDpdnSwapSpec> {
        SwapDataLane3W::new(self, 3)
    }
    #[doc = "Bit 4 - SWAP_CLK_LANE."]
    #[inline(always)]
    pub fn swap_clk_lane(&mut self) -> SwapClkLaneW<'_, CfgDpdnSwapSpec> {
        SwapClkLaneW::new(self, 4)
    }
}
#[doc = "CFG_DPDN_SWAP.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dpdn_swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dpdn_swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDpdnSwapSpec;
impl crate::RegisterSpec for CfgDpdnSwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dpdn_swap::R`](R) reader structure"]
impl crate::Readable for CfgDpdnSwapSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dpdn_swap::W`](W) writer structure"]
impl crate::Writable for CfgDpdnSwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DPDN_SWAP to value 0"]
impl crate::Resettable for CfgDpdnSwapSpec {}
