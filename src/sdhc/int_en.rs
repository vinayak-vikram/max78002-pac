#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Field `CMD_COMP` reader - Command Complete Status Enable."]
pub type CmdCompR = crate::BitReader;
#[doc = "Field `CMD_COMP` writer - Command Complete Status Enable."]
pub type CmdCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMP` reader - Transfer Complete Status Enable."]
pub type TransCompR = crate::BitReader;
#[doc = "Field `TRANS_COMP` writer - Transfer Complete Status Enable."]
pub type TransCompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_GAP` reader - Block Gap Event Status Enable."]
pub type BlkGapR = crate::BitReader;
#[doc = "Field `BLK_GAP` writer - Block Gap Event Status Enable."]
pub type BlkGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA Interrupt Status Enable."]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Interrupt Status Enable."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_WR` reader - Buffer Write Ready Status Enable."]
pub type BufferWrR = crate::BitReader;
#[doc = "Field `BUFFER_WR` writer - Buffer Write Ready Status Enable."]
pub type BufferWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFFER_RD` reader - Buffer Read Ready Status Enable."]
pub type BufferRdR = crate::BitReader;
#[doc = "Field `BUFFER_RD` writer - Buffer Read Ready Status Enable."]
pub type BufferRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERT` reader - Card Insertion Status Enable."]
pub type CardInsertR = crate::BitReader;
#[doc = "Field `CARD_INSERT` writer - Card Insertion Status Enable."]
pub type CardInsertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - Card Removal Status Enable."]
pub type CardRemovalR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - Card Removal Status Enable."]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INT` reader - Card Interrupt Status Enable."]
pub type CardIntR = crate::BitReader;
#[doc = "Field `CARD_INT` writer - Card Interrupt Status Enable."]
pub type CardIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING` reader - Re-Tuning Event Status Enable."]
pub type RetuningR = crate::BitReader;
#[doc = "Field `RETUNING` writer - Re-Tuning Event Status Enable."]
pub type RetuningW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable."]
    #[inline(always)]
    pub fn cmd_comp(&self) -> CmdCompR {
        CmdCompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable."]
    #[inline(always)]
    pub fn trans_comp(&self) -> TransCompR {
        TransCompR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable."]
    #[inline(always)]
    pub fn blk_gap(&self) -> BlkGapR {
        BlkGapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_wr(&self) -> BufferWrR {
        BufferWrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_rd(&self) -> BufferRdR {
        BufferRdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable."]
    #[inline(always)]
    pub fn card_insert(&self) -> CardInsertR {
        CardInsertR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable."]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable."]
    #[inline(always)]
    pub fn card_int(&self) -> CardIntR {
        CardIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable."]
    #[inline(always)]
    pub fn retuning(&self) -> RetuningR {
        RetuningR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable."]
    #[inline(always)]
    pub fn cmd_comp(&mut self) -> CmdCompW<'_, IntEnSpec> {
        CmdCompW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable."]
    #[inline(always)]
    pub fn trans_comp(&mut self) -> TransCompW<'_, IntEnSpec> {
        TransCompW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable."]
    #[inline(always)]
    pub fn blk_gap(&mut self) -> BlkGapW<'_, IntEnSpec> {
        BlkGapW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, IntEnSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_wr(&mut self) -> BufferWrW<'_, IntEnSpec> {
        BufferWrW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable."]
    #[inline(always)]
    pub fn buffer_rd(&mut self) -> BufferRdW<'_, IntEnSpec> {
        BufferRdW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable."]
    #[inline(always)]
    pub fn card_insert(&mut self) -> CardInsertW<'_, IntEnSpec> {
        CardInsertW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Status Enable."]
    #[inline(always)]
    pub fn card_removal(&mut self) -> CardRemovalW<'_, IntEnSpec> {
        CardRemovalW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable."]
    #[inline(always)]
    pub fn card_int(&mut self) -> CardIntW<'_, IntEnSpec> {
        CardIntW::new(self, 8)
    }
    #[doc = "Bit 12 - Re-Tuning Event Status Enable."]
    #[inline(always)]
    pub fn retuning(&mut self) -> RetuningW<'_, IntEnSpec> {
        RetuningW::new(self, 12)
    }
}
#[doc = "Normal Interrupt Status Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for IntEnSpec {}
