#[doc = "Register `PRESENT` reader"]
pub type R = crate::R<PresentSpec>;
#[doc = "Field `CMD` reader - Command Inhibit (CMD)."]
pub type CmdR = crate::BitReader;
#[doc = "Field `DAT` reader - Command Inhibit (DAT)."]
pub type DatR = crate::BitReader;
#[doc = "Field `DAT_LINE_ACTIVE` reader - DAT Line Active."]
pub type DatLineActiveR = crate::BitReader;
#[doc = "Field `RETUNING` reader - Re-Tuning Request."]
pub type RetuningR = crate::BitReader;
#[doc = "Field `WRITE_TRANSFER` reader - Write Transfer Active."]
pub type WriteTransferR = crate::BitReader;
#[doc = "Field `READ_TRANSFER` reader - Read Transfer Active."]
pub type ReadTransferR = crate::BitReader;
#[doc = "Field `BUFFER_WRITE` reader - Buffer Write Enable."]
pub type BufferWriteR = crate::BitReader;
#[doc = "Field `BUFFER_READ` reader - Buffer Read Enable."]
pub type BufferReadR = crate::BitReader;
#[doc = "Field `CARD_INSERTED` reader - Card Inserted."]
pub type CardInsertedR = crate::BitReader;
#[doc = "Field `CARD_STATE` reader - Card State Stable."]
pub type CardStateR = crate::BitReader;
#[doc = "Field `CARD_DETECT` reader - Card Detect Pin Level."]
pub type CardDetectR = crate::BitReader;
#[doc = "Field `WP` reader - Write Protect Switch Pin Level."]
pub type WpR = crate::BitReader;
#[doc = "Field `DAT_SIGNAL_LEVEL` reader - DAT\\[3:0\\] Line Signal Level."]
pub type DatSignalLevelR = crate::FieldReader;
#[doc = "Field `CMD_SIGNAL_LEVEL` reader - CMD Line Signal Level."]
pub type CmdSignalLevelR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)."]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)."]
    #[inline(always)]
    pub fn dat(&self) -> DatR {
        DatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active."]
    #[inline(always)]
    pub fn dat_line_active(&self) -> DatLineActiveR {
        DatLineActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Re-Tuning Request."]
    #[inline(always)]
    pub fn retuning(&self) -> RetuningR {
        RetuningR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active."]
    #[inline(always)]
    pub fn write_transfer(&self) -> WriteTransferR {
        WriteTransferR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active."]
    #[inline(always)]
    pub fn read_transfer(&self) -> ReadTransferR {
        ReadTransferR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable."]
    #[inline(always)]
    pub fn buffer_write(&self) -> BufferWriteR {
        BufferWriteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable."]
    #[inline(always)]
    pub fn buffer_read(&self) -> BufferReadR {
        BufferReadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted."]
    #[inline(always)]
    pub fn card_inserted(&self) -> CardInsertedR {
        CardInsertedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Card State Stable."]
    #[inline(always)]
    pub fn card_state(&self) -> CardStateR {
        CardStateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level."]
    #[inline(always)]
    pub fn card_detect(&self) -> CardDetectR {
        CardDetectR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level."]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - DAT\\[3:0\\] Line Signal Level."]
    #[inline(always)]
    pub fn dat_signal_level(&self) -> DatSignalLevelR {
        DatSignalLevelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - CMD Line Signal Level."]
    #[inline(always)]
    pub fn cmd_signal_level(&self) -> CmdSignalLevelR {
        CmdSignalLevelR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Present State.\n\nYou can [`read`](crate::Reg::read) this register and get [`present::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresentSpec;
impl crate::RegisterSpec for PresentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`present::R`](R) reader structure"]
impl crate::Readable for PresentSpec {}
#[doc = "`reset()` method sets PRESENT to value 0"]
impl crate::Resettable for PresentSpec {}
