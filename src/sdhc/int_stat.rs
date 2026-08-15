#[doc = "Register `INT_STAT` reader"]
pub type R = crate::R<IntStatSpec>;
#[doc = "Register `INT_STAT` writer"]
pub type W = crate::W<IntStatSpec>;
#[doc = "Field `CMD_COMP` reader - Command Complete."]
pub type CmdCompR = crate::BitReader;
#[doc = "Field `CMD_COMP` writer - Command Complete."]
pub type CmdCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMP` reader - Transfer Complete."]
pub type TransCompR = crate::BitReader;
#[doc = "Field `TRANS_COMP` writer - Transfer Complete."]
pub type TransCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_GAP_EVENT` reader - Block Gap Event."]
pub type BlkGapEventR = crate::BitReader;
#[doc = "Field `BLK_GAP_EVENT` writer - Block Gap Event."]
pub type BlkGapEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA Interrupt."]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Interrupt."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_WR_READY` reader - Buffer Write Ready."]
pub type BuffWrReadyR = crate::BitReader;
#[doc = "Field `BUFF_WR_READY` writer - Buffer Write Ready."]
pub type BuffWrReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_RD_READY` reader - Buffer Read Ready."]
pub type BuffRdReadyR = crate::BitReader;
#[doc = "Field `BUFF_RD_READY` writer - Buffer Read Ready."]
pub type BuffRdReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION` reader - Card Insertion."]
pub type CardInsertionR = crate::BitReader;
#[doc = "Field `CARD_INSERTION` writer - Card Insertion."]
pub type CardInsertionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - Card Removal."]
pub type CardRemovalR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - Card Removal."]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTR` reader - Card Interrupt."]
pub type CardIntrR = crate::BitReader;
#[doc = "Field `CARD_INTR` writer - Card Interrupt."]
pub type CardIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING` reader - Re-Tuning Event."]
pub type RetuningR = crate::BitReader;
#[doc = "Field `RETUNING` writer - Re-Tuning Event."]
pub type RetuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INTR` reader - Error Interrupt."]
pub type ErrIntrR = crate::BitReader;
#[doc = "Field `ERR_INTR` writer - Error Interrupt."]
pub type ErrIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete."]
    #[inline(always)]
    pub fn cmd_comp(&self) -> CmdCompR {
        CmdCompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete."]
    #[inline(always)]
    pub fn trans_comp(&self) -> TransCompR {
        TransCompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event."]
    #[inline(always)]
    pub fn blk_gap_event(&self) -> BlkGapEventR {
        BlkGapEventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready."]
    #[inline(always)]
    pub fn buff_wr_ready(&self) -> BuffWrReadyR {
        BuffWrReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready."]
    #[inline(always)]
    pub fn buff_rd_ready(&self) -> BuffRdReadyR {
        BuffRdReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion."]
    #[inline(always)]
    pub fn card_insertion(&self) -> CardInsertionR {
        CardInsertionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal."]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt."]
    #[inline(always)]
    pub fn card_intr(&self) -> CardIntrR {
        CardIntrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event."]
    #[inline(always)]
    pub fn retuning(&self) -> RetuningR {
        RetuningR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt."]
    #[inline(always)]
    pub fn err_intr(&self) -> ErrIntrR {
        ErrIntrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete."]
    #[inline(always)]
    pub fn cmd_comp(&mut self) -> CmdCompW<'_, IntStatSpec> {
        CmdCompW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete."]
    #[inline(always)]
    pub fn trans_comp(&mut self) -> TransCompW<'_, IntStatSpec> {
        TransCompW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event."]
    #[inline(always)]
    pub fn blk_gap_event(&mut self) -> BlkGapEventW<'_, IntStatSpec> {
        BlkGapEventW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, IntStatSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready."]
    #[inline(always)]
    pub fn buff_wr_ready(&mut self) -> BuffWrReadyW<'_, IntStatSpec> {
        BuffWrReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready."]
    #[inline(always)]
    pub fn buff_rd_ready(&mut self) -> BuffRdReadyW<'_, IntStatSpec> {
        BuffRdReadyW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion."]
    #[inline(always)]
    pub fn card_insertion(&mut self) -> CardInsertionW<'_, IntStatSpec> {
        CardInsertionW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal."]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CardRemovalW<'_, IntStatSpec> {
        CardRemovalW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt."]
    #[inline(always)]
    pub fn card_intr(&mut self) -> CardIntrW<'_, IntStatSpec> {
        CardIntrW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event."]
    #[inline(always)]
    pub fn retuning(&mut self) -> RetuningW<'_, IntStatSpec> {
        RetuningW::new(self, 12)
    }
    #[doc = "Bit 15 - Error Interrupt."]
    #[inline(always)]
    pub fn err_intr(&mut self) -> ErrIntrW<'_, IntStatSpec> {
        ErrIntrW::new(self, 15)
    }
}
#[doc = "Normal Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatSpec;
impl crate::RegisterSpec for IntStatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_stat::R`](R) reader structure"]
impl crate::Readable for IntStatSpec {}
#[doc = "`write(|w| ..)` method takes [`int_stat::W`](W) writer structure"]
impl crate::Writable for IntStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_STAT to value 0"]
impl crate::Resettable for IntStatSpec {}
