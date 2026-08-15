#[doc = "Register `ULPS_CLK_MARK_STATUS` reader"]
pub type R = crate::R<UlpsClkMarkStatusSpec>;
#[doc = "Register `ULPS_CLK_MARK_STATUS` writer"]
pub type W = crate::W<UlpsClkMarkStatusSpec>;
#[doc = "Field `CLK_LANE` reader - Clock Lane."]
pub type ClkLaneR = crate::BitReader;
#[doc = "Field `CLK_LANE` writer - Clock Lane."]
pub type ClkLaneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Lane."]
    #[inline(always)]
    pub fn clk_lane(&self) -> ClkLaneR {
        ClkLaneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Lane."]
    #[inline(always)]
    pub fn clk_lane(&mut self) -> ClkLaneW<'_, UlpsClkMarkStatusSpec> {
        ClkLaneW::new(self, 0)
    }
}
#[doc = "ULPS_CLK_MARK_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_clk_mark_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_clk_mark_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UlpsClkMarkStatusSpec;
impl crate::RegisterSpec for UlpsClkMarkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulps_clk_mark_status::R`](R) reader structure"]
impl crate::Readable for UlpsClkMarkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ulps_clk_mark_status::W`](W) writer structure"]
impl crate::Writable for UlpsClkMarkStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ULPS_CLK_MARK_STATUS to value 0"]
impl crate::Resettable for UlpsClkMarkStatusSpec {}
