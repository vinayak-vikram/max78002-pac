#[doc = "Register `FORCE_EVENT_INT_STAT` reader"]
pub type R = crate::R<ForceEventIntStatSpec>;
#[doc = "Register `FORCE_EVENT_INT_STAT` writer"]
pub type W = crate::W<ForceEventIntStatSpec>;
#[doc = "Field `CMD_TO` reader - Force Event for Command Timeout Error."]
pub type CmdToR = crate::BitReader;
#[doc = "Field `CMD_CRC` reader - Force Event for Command CRC Error."]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_END_BIT` reader - Force Event for Command End Bit Error."]
pub type CmdEndBitR = crate::BitReader;
#[doc = "Field `CMD_INDEX` reader - Force Event for Command Index Error."]
pub type CmdIndexR = crate::BitReader;
#[doc = "Field `DATA_TO` reader - Force Event for Data Timeout Error."]
pub type DataToR = crate::BitReader;
#[doc = "Field `DATA_CRC` reader - Force Event for Data CRC Error."]
pub type DataCrcR = crate::BitReader;
#[doc = "Field `DATA_END_BIT` reader - Force Event for Data End Bit Error."]
pub type DataEndBitR = crate::BitReader;
#[doc = "Field `CURR_LIMIT` reader - Force Event for Current Limit Error."]
pub type CurrLimitR = crate::BitReader;
#[doc = "Field `AUTO_CMD` reader - Force Event for Auto CMD Error."]
pub type AutoCmdR = crate::BitReader;
#[doc = "Field `ADMA` reader - Force Event for ADMA Error."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - Force Event for ADMA Error."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR` writer - Force Event for Vendor Specific Error Status."]
pub type VendorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Force Event for Command Timeout Error."]
    #[inline(always)]
    pub fn cmd_to(&self) -> CmdToR {
        CmdToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error."]
    #[inline(always)]
    pub fn cmd_end_bit(&self) -> CmdEndBitR {
        CmdEndBitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error."]
    #[inline(always)]
    pub fn data_to(&self) -> DataToR {
        DataToR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error."]
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error."]
    #[inline(always)]
    pub fn data_end_bit(&self) -> DataEndBitR {
        DataEndBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error."]
    #[inline(always)]
    pub fn curr_limit(&self) -> CurrLimitR {
        CurrLimitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AutoCmdR {
        AutoCmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Force Event for ADMA Error."]
    #[inline(always)]
    pub fn adma(&mut self) -> AdmaW<'_, ForceEventIntStatSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bits 12:14 - Force Event for Vendor Specific Error Status."]
    #[inline(always)]
    pub fn vendor(&mut self) -> VendorW<'_, ForceEventIntStatSpec> {
        VendorW::new(self, 12)
    }
}
#[doc = "Force Event for Error Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`force_event_int_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_event_int_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceEventIntStatSpec;
impl crate::RegisterSpec for ForceEventIntStatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`force_event_int_stat::R`](R) reader structure"]
impl crate::Readable for ForceEventIntStatSpec {}
#[doc = "`write(|w| ..)` method takes [`force_event_int_stat::W`](W) writer structure"]
impl crate::Writable for ForceEventIntStatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FORCE_EVENT_INT_STAT to value 0"]
impl crate::Resettable for ForceEventIntStatSpec {}
