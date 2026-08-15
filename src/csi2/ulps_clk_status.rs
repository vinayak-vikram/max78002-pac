#[doc = "Register `ULPS_CLK_STATUS` reader"]
pub type R = crate::R<UlpsClkStatusSpec>;
#[doc = "Register `ULPS_CLK_STATUS` writer"]
pub type W = crate::W<UlpsClkStatusSpec>;
#[doc = "Field `FIFO` reader - FIFO Read/Write register."]
pub type FifoR = crate::BitReader;
#[doc = "Field `FIFO` writer - FIFO Read/Write register."]
pub type FifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO Read/Write register."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Read/Write register."]
    #[inline(always)]
    pub fn fifo(&mut self) -> FifoW<'_, UlpsClkStatusSpec> {
        FifoW::new(self, 0)
    }
}
#[doc = "ULPS_CLK_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_clk_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_clk_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UlpsClkStatusSpec;
impl crate::RegisterSpec for UlpsClkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulps_clk_status::R`](R) reader structure"]
impl crate::Readable for UlpsClkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ulps_clk_status::W`](W) writer structure"]
impl crate::Writable for UlpsClkStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ULPS_CLK_STATUS to value 0"]
impl crate::Resettable for UlpsClkStatusSpec {}
