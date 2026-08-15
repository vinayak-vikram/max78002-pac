#[doc = "Register `IRQ_ENABLE` reader"]
pub type R = crate::R<IrqEnableSpec>;
#[doc = "Register `IRQ_ENABLE` writer"]
pub type W = crate::W<IrqEnableSpec>;
#[doc = "Field `CRC` reader - CRC error."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC error."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBE` reader - Single bit ECC error."]
pub type SbeR = crate::BitReader;
#[doc = "Field `SBE` writer - Single bit ECC error."]
pub type SbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBE` reader - Multiple bit ECC error."]
pub type MbeR = crate::BitReader;
#[doc = "Field `MBE` writer - Multiple bit ECC error."]
pub type MbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPS_ACTIVE` reader - ULPS active status change."]
pub type UlpsActiveR = crate::BitReader;
#[doc = "Field `ULPS_ACTIVE` writer - ULPS active status change."]
pub type UlpsActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULPS_MARK_ACTIVE` reader - ULPS mark active status change."]
pub type UlpsMarkActiveR = crate::BitReader;
#[doc = "Field `ULPS_MARK_ACTIVE` writer - ULPS mark active status change."]
pub type UlpsMarkActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_ERR_SEND_LVL` reader - Video Error Send Level."]
pub type VidErrSendLvlR = crate::BitReader;
#[doc = "Field `VID_ERR_SEND_LVL` writer - Video Error Send Level."]
pub type VidErrSendLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_ERR_FIFO_WR_OV` reader - Video Error Fifo Overflow."]
pub type VidErrFifoWrOvR = crate::BitReader;
#[doc = "Field `VID_ERR_FIFO_WR_OV` writer - Video Error Fifo Overflow."]
pub type VidErrFifoWrOvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC error."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single bit ECC error."]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple bit ECC error."]
    #[inline(always)]
    pub fn mbe(&self) -> MbeR {
        MbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULPS active status change."]
    #[inline(always)]
    pub fn ulps_active(&self) -> UlpsActiveR {
        UlpsActiveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ULPS mark active status change."]
    #[inline(always)]
    pub fn ulps_mark_active(&self) -> UlpsMarkActiveR {
        UlpsMarkActiveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Video Error Send Level."]
    #[inline(always)]
    pub fn vid_err_send_lvl(&self) -> VidErrSendLvlR {
        VidErrSendLvlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Video Error Fifo Overflow."]
    #[inline(always)]
    pub fn vid_err_fifo_wr_ov(&self) -> VidErrFifoWrOvR {
        VidErrFifoWrOvR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC error."]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, IrqEnableSpec> {
        CrcW::new(self, 0)
    }
    #[doc = "Bit 1 - Single bit ECC error."]
    #[inline(always)]
    pub fn sbe(&mut self) -> SbeW<'_, IrqEnableSpec> {
        SbeW::new(self, 1)
    }
    #[doc = "Bit 2 - Multiple bit ECC error."]
    #[inline(always)]
    pub fn mbe(&mut self) -> MbeW<'_, IrqEnableSpec> {
        MbeW::new(self, 2)
    }
    #[doc = "Bit 3 - ULPS active status change."]
    #[inline(always)]
    pub fn ulps_active(&mut self) -> UlpsActiveW<'_, IrqEnableSpec> {
        UlpsActiveW::new(self, 3)
    }
    #[doc = "Bit 4 - ULPS mark active status change."]
    #[inline(always)]
    pub fn ulps_mark_active(&mut self) -> UlpsMarkActiveW<'_, IrqEnableSpec> {
        UlpsMarkActiveW::new(self, 4)
    }
    #[doc = "Bit 5 - Video Error Send Level."]
    #[inline(always)]
    pub fn vid_err_send_lvl(&mut self) -> VidErrSendLvlW<'_, IrqEnableSpec> {
        VidErrSendLvlW::new(self, 5)
    }
    #[doc = "Bit 6 - Video Error Fifo Overflow."]
    #[inline(always)]
    pub fn vid_err_fifo_wr_ov(&mut self) -> VidErrFifoWrOvW<'_, IrqEnableSpec> {
        VidErrFifoWrOvW::new(self, 6)
    }
}
#[doc = "IRQ_ENABLE.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnableSpec;
impl crate::RegisterSpec for IrqEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enable::R`](R) reader structure"]
impl crate::Readable for IrqEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure"]
impl crate::Writable for IrqEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENABLE to value 0"]
impl crate::Resettable for IrqEnableSpec {}
