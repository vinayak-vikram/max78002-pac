#[doc = "Register `ER_INT_STAT` reader"]
pub type R = crate::R<ErIntStatSpec>;
#[doc = "Register `ER_INT_STAT` writer"]
pub type W = crate::W<ErIntStatSpec>;
#[doc = "Field `CMD_TO` reader - Command Timeout Error."]
pub type CmdToR = crate::BitReader;
#[doc = "Field `CMD_TO` writer - Command Timeout Error."]
pub type CmdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC` reader - Command CRC Error."]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_CRC` writer - Command CRC Error."]
pub type CmdCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT` reader - Command End Bit Error."]
pub type CmdEndBitR = crate::BitReader;
#[doc = "Field `CMD_END_BIT` writer - Command End Bit Error."]
pub type CmdEndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX` reader - Command Index Error."]
pub type CmdIdxR = crate::BitReader;
#[doc = "Field `CMD_IDX` writer - Command Index Error."]
pub type CmdIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TO` reader - Data Timeout Error."]
pub type DataToR = crate::BitReader;
#[doc = "Field `DATA_TO` writer - Data Timeout Error."]
pub type DataToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC` reader - Data CRC Error."]
pub type DataCrcR = crate::BitReader;
#[doc = "Field `DATA_CRC` writer - Data CRC Error."]
pub type DataCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT` reader - Data End Bit Error."]
pub type DataEndBitR = crate::BitReader;
#[doc = "Field `DATA_END_BIT` writer - Data End Bit Error."]
pub type DataEndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURRENT_LIMIT` reader - Current Limit Error."]
pub type CurrentLimitR = crate::BitReader;
#[doc = "Field `CURRENT_LIMIT` writer - Current Limit Error."]
pub type CurrentLimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_12` reader - Auto CMD Error."]
pub type AutoCmd12R = crate::BitReader;
#[doc = "Field `AUTO_CMD_12` writer - Auto CMD Error."]
pub type AutoCmd12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - ADMA Error."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - ADMA Error."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - DMA Error."]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Error."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CmdToR {
        CmdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CmdEndBitR {
        CmdEndBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error."]
    #[inline(always)]
    pub fn cmd_idx(&self) -> CmdIdxR {
        CmdIdxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error."]
    #[inline(always)]
    pub fn data_to(&self) -> DataToR {
        DataToR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error."]
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DataEndBitR {
        DataEndBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error."]
    #[inline(always)]
    pub fn current_limit(&self) -> CurrentLimitR {
        CurrentLimitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd_12(&self) -> AutoCmd12R {
        AutoCmd12R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Error."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error."]
    #[inline(always)]
    pub fn cmd_to(&mut self) -> CmdToW<'_, ErIntStatSpec> {
        CmdToW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&mut self) -> CmdCrcW<'_, ErIntStatSpec> {
        CmdCrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_end_bit(&mut self) -> CmdEndBitW<'_, ErIntStatSpec> {
        CmdEndBitW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error."]
    #[inline(always)]
    pub fn cmd_idx(&mut self) -> CmdIdxW<'_, ErIntStatSpec> {
        CmdIdxW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error."]
    #[inline(always)]
    pub fn data_to(&mut self) -> DataToW<'_, ErIntStatSpec> {
        DataToW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error."]
    #[inline(always)]
    pub fn data_crc(&mut self) -> DataCrcW<'_, ErIntStatSpec> {
        DataCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error."]
    #[inline(always)]
    pub fn data_end_bit(&mut self) -> DataEndBitW<'_, ErIntStatSpec> {
        DataEndBitW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error."]
    #[inline(always)]
    pub fn current_limit(&mut self) -> CurrentLimitW<'_, ErIntStatSpec> {
        CurrentLimitW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd_12(&mut self) -> AutoCmd12W<'_, ErIntStatSpec> {
        AutoCmd12W::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error."]
    #[inline(always)]
    pub fn adma(&mut self) -> AdmaW<'_, ErIntStatSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 12 - DMA Error."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, ErIntStatSpec> {
        DmaW::new(self, 12)
    }
}
#[doc = "Error Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_int_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_int_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErIntStatSpec;
impl crate::RegisterSpec for ErIntStatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`er_int_stat::R`](R) reader structure"]
impl crate::Readable for ErIntStatSpec {}
#[doc = "`write(|w| ..)` method takes [`er_int_stat::W`](W) writer structure"]
impl crate::Writable for ErIntStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ER_INT_STAT to value 0"]
impl crate::Resettable for ErIntStatSpec {}
