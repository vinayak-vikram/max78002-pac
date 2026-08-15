#[doc = "Register `ER_INT_SIGNAL` reader"]
pub type R = crate::R<ErIntSignalSpec>;
#[doc = "Register `ER_INT_SIGNAL` writer"]
pub type W = crate::W<ErIntSignalSpec>;
#[doc = "Field `CMD_TO` reader - Command Timeout Error Signal Enable."]
pub type CmdToR = crate::BitReader;
#[doc = "Field `CMD_TO` writer - Command Timeout Error Signal Enable."]
pub type CmdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC` reader - Command CRC Error Signal Enable."]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_CRC` writer - Command CRC Error Signal Enable."]
pub type CmdCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT` reader - Command End Bit Error Signal Enable."]
pub type CmdEndBitR = crate::BitReader;
#[doc = "Field `CMD_END_BIT` writer - Command End Bit Error Signal Enable."]
pub type CmdEndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX` reader - Command Index Error Signal Enable."]
pub type CmdIdxR = crate::BitReader;
#[doc = "Field `CMD_IDX` writer - Command Index Error Signal Enable."]
pub type CmdIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TO` reader - Data Timeout Error Signal Enable."]
pub type DataToR = crate::BitReader;
#[doc = "Field `DATA_TO` writer - Data Timeout Error Signal Enable."]
pub type DataToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC` reader - Data CRC Error Signal Enable."]
pub type DataCrcR = crate::BitReader;
#[doc = "Field `DATA_CRC` writer - Data CRC Error Signal Enable."]
pub type DataCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT` reader - Data End Bit Error Signal Enable."]
pub type DataEndBitR = crate::BitReader;
#[doc = "Field `DATA_END_BIT` writer - Data End Bit Error Signal Enable."]
pub type DataEndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_LIM` reader - Current Limit Error Signal Enable."]
pub type CurrLimR = crate::BitReader;
#[doc = "Field `CURR_LIM` writer - Current Limit Error Signal Enable."]
pub type CurrLimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD` reader - Auto CMD Error Signal Enable."]
pub type AutoCmdR = crate::BitReader;
#[doc = "Field `AUTO_CMD` writer - Auto CMD Error Signal Enable."]
pub type AutoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - ADMA Error Signal Enable."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - ADMA Error Signal Enable."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING` reader - Tuning Error Signal Enable."]
pub type TuningR = crate::BitReader;
#[doc = "Field `TUNING` writer - Tuning Error Signal Enable."]
pub type TuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAR_RESP` reader - Target Response Error Signal Enable."]
pub type TarRespR = crate::BitReader;
#[doc = "Field `TAR_RESP` writer - Target Response Error Signal Enable."]
pub type TarRespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CmdToR {
        CmdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CmdEndBitR {
        CmdEndBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_idx(&self) -> CmdIdxR {
        CmdIdxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn data_to(&self) -> DataToR {
        DataToR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable."]
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DataEndBitR {
        DataEndBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable."]
    #[inline(always)]
    pub fn curr_lim(&self) -> CurrLimR {
        CurrLimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AutoCmdR {
        AutoCmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tuning Error Signal Enable."]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable."]
    #[inline(always)]
    pub fn tar_resp(&self) -> TarRespR {
        TarRespR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_to(&mut self) -> CmdToW<'_, ErIntSignalSpec> {
        CmdToW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_crc(&mut self) -> CmdCrcW<'_, ErIntSignalSpec> {
        CmdCrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_end_bit(&mut self) -> CmdEndBitW<'_, ErIntSignalSpec> {
        CmdEndBitW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable."]
    #[inline(always)]
    pub fn cmd_idx(&mut self) -> CmdIdxW<'_, ErIntSignalSpec> {
        CmdIdxW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable."]
    #[inline(always)]
    pub fn data_to(&mut self) -> DataToW<'_, ErIntSignalSpec> {
        DataToW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable."]
    #[inline(always)]
    pub fn data_crc(&mut self) -> DataCrcW<'_, ErIntSignalSpec> {
        DataCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable."]
    #[inline(always)]
    pub fn data_end_bit(&mut self) -> DataEndBitW<'_, ErIntSignalSpec> {
        DataEndBitW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable."]
    #[inline(always)]
    pub fn curr_lim(&mut self) -> CurrLimW<'_, ErIntSignalSpec> {
        CurrLimW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable."]
    #[inline(always)]
    pub fn auto_cmd(&mut self) -> AutoCmdW<'_, ErIntSignalSpec> {
        AutoCmdW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable."]
    #[inline(always)]
    pub fn adma(&mut self) -> AdmaW<'_, ErIntSignalSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 10 - Tuning Error Signal Enable."]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, ErIntSignalSpec> {
        TuningW::new(self, 10)
    }
    #[doc = "Bit 12 - Target Response Error Signal Enable."]
    #[inline(always)]
    pub fn tar_resp(&mut self) -> TarRespW<'_, ErIntSignalSpec> {
        TarRespW::new(self, 12)
    }
}
#[doc = "Error Interrupt Signal Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_int_signal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_int_signal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErIntSignalSpec;
impl crate::RegisterSpec for ErIntSignalSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`er_int_signal::R`](R) reader structure"]
impl crate::Readable for ErIntSignalSpec {}
#[doc = "`write(|w| ..)` method takes [`er_int_signal::W`](W) writer structure"]
impl crate::Writable for ErIntSignalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ER_INT_SIGNAL to value 0"]
impl crate::Resettable for ErIntSignalSpec {}
