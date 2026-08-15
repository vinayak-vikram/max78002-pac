#[doc = "Register `CFG_CLK_LANE_EN` reader"]
pub type R = crate::R<CfgClkLaneEnSpec>;
#[doc = "Register `CFG_CLK_LANE_EN` writer"]
pub type W = crate::W<CfgClkLaneEnSpec>;
#[doc = "Field `EN` reader - Enable lane clock setting for controller."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable lane clock setting for controller."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable lane clock setting for controller."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable lane clock setting for controller."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CfgClkLaneEnSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "CFG_CLK_LANE_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_clk_lane_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_clk_lane_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgClkLaneEnSpec;
impl crate::RegisterSpec for CfgClkLaneEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_clk_lane_en::R`](R) reader structure"]
impl crate::Readable for CfgClkLaneEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_clk_lane_en::W`](W) writer structure"]
impl crate::Writable for CfgClkLaneEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_CLK_LANE_EN to value 0"]
impl crate::Resettable for CfgClkLaneEnSpec {}
