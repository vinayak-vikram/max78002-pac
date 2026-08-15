#[doc = "Register `CFG_P_FIFO_SEND_LEVEL` reader"]
pub type R = crate::R<CfgPFifoSendLevelSpec>;
#[doc = "Register `CFG_P_FIFO_SEND_LEVEL` writer"]
pub type W = crate::W<CfgPFifoSendLevelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_P_FIFO_SEND_LEVEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_p_fifo_send_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_p_fifo_send_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPFifoSendLevelSpec;
impl crate::RegisterSpec for CfgPFifoSendLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_p_fifo_send_level::R`](R) reader structure"]
impl crate::Readable for CfgPFifoSendLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_p_fifo_send_level::W`](W) writer structure"]
impl crate::Writable for CfgPFifoSendLevelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_P_FIFO_SEND_LEVEL to value 0"]
impl crate::Resettable for CfgPFifoSendLevelSpec {}
