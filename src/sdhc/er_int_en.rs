#[doc = "Register `ER_INT_EN` reader"]
pub type R = crate::R<ErIntEnSpec>;
#[doc = "Register `ER_INT_EN` writer"]
pub type W = crate::W<ErIntEnSpec>;
#[doc = "Field `CMD_TO` reader - Command Timeout Error Status Enable."]
pub type CmdToR = crate::BitReader;
#[doc = "Field `CMD_TO` writer - Command Timeout Error Status Enable."]
pub type CmdToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC` reader - Command CRC Error Status Enable."]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_CRC` writer - Command CRC Error Status Enable."]
pub type CmdCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT` reader - Command End Bit Error Status Enable."]
pub type CmdEndBitR = crate::BitReader;
#[doc = "Field `CMD_END_BIT` writer - Command End Bit Error Status Enable."]
pub type CmdEndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX` reader - Command Index Error Status Enable."]
pub type CmdIdxR = crate::BitReader;
#[doc = "Field `CMD_IDX` writer - Command Index Error Status Enable."]
pub type CmdIdxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TO` reader - Data Timeout Error Status Enable."]
pub type DataToR = crate::BitReader;
#[doc = "Field `DATA_TO` writer - Data Timeout Error Status Enable."]
pub type DataToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC` reader - Data CRC Error Status Enable."]
pub type DataCrcR = crate::BitReader;
#[doc = "Field `DATA_CRC` writer - Data CRC Error Status Enable."]
pub type DataCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT` reader - Data End Bit Error Status Enable."]
pub type DataEndBitR = crate::BitReader;
#[doc = "Field `DATA_END_BIT` writer - Data End Bit Error Status Enable."]
pub type DataEndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD` reader - Auto CMD Error Status Enable."]
pub type AutoCmdR = crate::BitReader;
#[doc = "Field `AUTO_CMD` writer - Auto CMD Error Status Enable."]
pub type AutoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - ADMA Error Status Enable."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - ADMA Error Status Enable."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING` reader - Tuning Error Status Enable."]
pub type TuningR = crate::BitReader;
#[doc = "Field `TUNING` writer - Tuning Error Status Enable."]
pub type TuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR` reader - Vendor Specific Error Status Enable."]
pub type VendorR = crate::BitReader;
#[doc = "Field `VENDOR` writer - Vendor Specific Error Status Enable."]
pub type VendorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Status Enable."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CmdToR {
        CmdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Status Enable."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CmdEndBitR {
        CmdEndBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable."]
    #[inline(always)]
    pub fn cmd_idx(&self) -> CmdIdxR {
        CmdIdxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable."]
    #[inline(always)]
    pub fn data_to(&self) -> DataToR {
        DataToR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable."]
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DataEndBitR {
        DataEndBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Status Enable."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AutoCmdR {
        AutoCmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Status Enable."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tuning Error Status Enable."]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Vendor Specific Error Status Enable."]
    #[inline(always)]
    pub fn vendor(&self) -> VendorR {
        VendorR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Status Enable."]
    #[inline(always)]
    pub fn cmd_to(&mut self) -> CmdToW<'_, ErIntEnSpec> {
        CmdToW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error Status Enable."]
    #[inline(always)]
    pub fn cmd_crc(&mut self) -> CmdCrcW<'_, ErIntEnSpec> {
        CmdCrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable."]
    #[inline(always)]
    pub fn cmd_end_bit(&mut self) -> CmdEndBitW<'_, ErIntEnSpec> {
        CmdEndBitW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable."]
    #[inline(always)]
    pub fn cmd_idx(&mut self) -> CmdIdxW<'_, ErIntEnSpec> {
        CmdIdxW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable."]
    #[inline(always)]
    pub fn data_to(&mut self) -> DataToW<'_, ErIntEnSpec> {
        DataToW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable."]
    #[inline(always)]
    pub fn data_crc(&mut self) -> DataCrcW<'_, ErIntEnSpec> {
        DataCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable."]
    #[inline(always)]
    pub fn data_end_bit(&mut self) -> DataEndBitW<'_, ErIntEnSpec> {
        DataEndBitW::new(self, 6)
    }
    #[doc = "Bit 8 - Auto CMD Error Status Enable."]
    #[inline(always)]
    pub fn auto_cmd(&mut self) -> AutoCmdW<'_, ErIntEnSpec> {
        AutoCmdW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error Status Enable."]
    #[inline(always)]
    pub fn adma(&mut self) -> AdmaW<'_, ErIntEnSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 10 - Tuning Error Status Enable."]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<'_, ErIntEnSpec> {
        TuningW::new(self, 10)
    }
    #[doc = "Bit 12 - Vendor Specific Error Status Enable."]
    #[inline(always)]
    pub fn vendor(&mut self) -> VendorW<'_, ErIntEnSpec> {
        VendorW::new(self, 12)
    }
}
#[doc = "Error Interrupt Status Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErIntEnSpec;
impl crate::RegisterSpec for ErIntEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`er_int_en::R`](R) reader structure"]
impl crate::Readable for ErIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`er_int_en::W`](W) writer structure"]
impl crate::Writable for ErIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ER_INT_EN to value 0"]
impl crate::Resettable for ErIntEnSpec {}
