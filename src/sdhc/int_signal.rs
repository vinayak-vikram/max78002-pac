#[doc = "Register `INT_SIGNAL` reader"]
pub type R = crate::R<IntSignalSpec>;
#[doc = "Register `INT_SIGNAL` writer"]
pub type W = crate::W<IntSignalSpec>;
#[doc = "Field `CMD_COMP` reader - Command Complete Signal Enable."]
pub type CmdCompR = crate::BitReader;
#[doc = "Field `CMD_COMP` writer - Command Complete Signal Enable."]
pub type CmdCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMP` reader - Transfer Complete Signal Enable."]
pub type TransCompR = crate::BitReader;
#[doc = "Field `TRANS_COMP` writer - Transfer Complete Signal Enable."]
pub type TransCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_GAP` reader - Block Gap Event Signal Enable."]
pub type BlkGapR = crate::BitReader;
#[doc = "Field `BLK_GAP` writer - Block Gap Event Signal Enable."]
pub type BlkGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA Interrupt Signal Enable."]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Interrupt Signal Enable."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_WR` reader - Buffer Write Ready Signal Enable."]
pub type BufferWrR = crate::BitReader;
#[doc = "Field `BUFFER_WR` writer - Buffer Write Ready Signal Enable."]
pub type BufferWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_RD` reader - Buffer Read Ready Signal Enable."]
pub type BufferRdR = crate::BitReader;
#[doc = "Field `BUFFER_RD` writer - Buffer Read Ready Signal Enable."]
pub type BufferRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERT` reader - Card Insertion Signal Enable."]
pub type CardInsertR = crate::BitReader;
#[doc = "Field `CARD_INSERT` writer - Card Insertion Signal Enable."]
pub type CardInsertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - Card Removal Signal Enable."]
pub type CardRemovalR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - Card Removal Signal Enable."]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INT` reader - Card Interrupt Signal Enable."]
pub type CardIntR = crate::BitReader;
#[doc = "Field `CARD_INT` writer - Card Interrupt Signal Enable."]
pub type CardIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING` reader - Re-Tuning Event Signal Enable."]
pub type RetuningR = crate::BitReader;
#[doc = "Field `RETUNING` writer - Re-Tuning Event Signal Enable."]
pub type RetuningW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable."]
    #[inline(always)]
    pub fn cmd_comp(&self) -> CmdCompR {
        CmdCompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable."]
    #[inline(always)]
    pub fn trans_comp(&self) -> TransCompR {
        TransCompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable."]
    #[inline(always)]
    pub fn blk_gap(&self) -> BlkGapR {
        BlkGapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable."]
    #[inline(always)]
    pub fn buffer_wr(&self) -> BufferWrR {
        BufferWrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable."]
    #[inline(always)]
    pub fn buffer_rd(&self) -> BufferRdR {
        BufferRdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable."]
    #[inline(always)]
    pub fn card_insert(&self) -> CardInsertR {
        CardInsertR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable."]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable."]
    #[inline(always)]
    pub fn card_int(&self) -> CardIntR {
        CardIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable."]
    #[inline(always)]
    pub fn retuning(&self) -> RetuningR {
        RetuningR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable."]
    #[inline(always)]
    pub fn cmd_comp(&mut self) -> CmdCompW<'_, IntSignalSpec> {
        CmdCompW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable."]
    #[inline(always)]
    pub fn trans_comp(&mut self) -> TransCompW<'_, IntSignalSpec> {
        TransCompW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable."]
    #[inline(always)]
    pub fn blk_gap(&mut self) -> BlkGapW<'_, IntSignalSpec> {
        BlkGapW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, IntSignalSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable."]
    #[inline(always)]
    pub fn buffer_wr(&mut self) -> BufferWrW<'_, IntSignalSpec> {
        BufferWrW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable."]
    #[inline(always)]
    pub fn buffer_rd(&mut self) -> BufferRdW<'_, IntSignalSpec> {
        BufferRdW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable."]
    #[inline(always)]
    pub fn card_insert(&mut self) -> CardInsertW<'_, IntSignalSpec> {
        CardInsertW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable."]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CardRemovalW<'_, IntSignalSpec> {
        CardRemovalW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable."]
    #[inline(always)]
    pub fn card_int(&mut self) -> CardIntW<'_, IntSignalSpec> {
        CardIntW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event Signal Enable."]
    #[inline(always)]
    pub fn retuning(&mut self) -> RetuningW<'_, IntSignalSpec> {
        RetuningW::new(self, 12)
    }
}
#[doc = "Normal Interrupt Signal Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_signal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_signal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSignalSpec;
impl crate::RegisterSpec for IntSignalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_signal::R`](R) reader structure"]
impl crate::Readable for IntSignalSpec {}
#[doc = "`write(|w| ..)` method takes [`int_signal::W`](W) writer structure"]
impl crate::Writable for IntSignalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_SIGNAL to value 0"]
impl crate::Resettable for IntSignalSpec {}
