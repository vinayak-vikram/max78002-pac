#[doc = "Register `CFG_1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Field `SDR50` reader - SDR50 Support."]
pub type Sdr50R = crate::BitReader;
#[doc = "Field `SDR104` reader - SDR104 Support."]
pub type Sdr104R = crate::BitReader;
#[doc = "Field `DDR50` reader - DDR50 Support."]
pub type Ddr50R = crate::BitReader;
#[doc = "Field `DRIVER_A` reader - Driver Type A Support."]
pub type DriverAR = crate::BitReader;
#[doc = "Field `DRIVER_C` reader - Driver Type C Support."]
pub type DriverCR = crate::BitReader;
#[doc = "Field `DRIVER_D` reader - Driver Type D Support."]
pub type DriverDR = crate::BitReader;
#[doc = "Field `TIMER_CNT_TUNING` reader - Timer Count for Re-Tuning."]
pub type TimerCntTuningR = crate::FieldReader;
#[doc = "Field `TUNING_SDR50` reader - Use Tuning for SDR50."]
pub type TuningSdr50R = crate::BitReader;
#[doc = "Field `RETUNING` reader - Re-Tuning Modes."]
pub type RetuningR = crate::FieldReader;
#[doc = "Field `CLK_MULTI` reader - Clock Multiplier."]
pub type ClkMultiR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SDR50 Support."]
    #[inline(always)]
    pub fn sdr50(&self) -> Sdr50R {
        Sdr50R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support."]
    #[inline(always)]
    pub fn sdr104(&self) -> Sdr104R {
        Sdr104R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support."]
    #[inline(always)]
    pub fn ddr50(&self) -> Ddr50R {
        Ddr50R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type A Support."]
    #[inline(always)]
    pub fn driver_a(&self) -> DriverAR {
        DriverAR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support."]
    #[inline(always)]
    pub fn driver_c(&self) -> DriverCR {
        DriverCR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support."]
    #[inline(always)]
    pub fn driver_d(&self) -> DriverDR {
        DriverDR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning."]
    #[inline(always)]
    pub fn timer_cnt_tuning(&self) -> TimerCntTuningR {
        TimerCntTuningR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50."]
    #[inline(always)]
    pub fn tuning_sdr50(&self) -> TuningSdr50R {
        TuningSdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Re-Tuning Modes."]
    #[inline(always)]
    pub fn retuning(&self) -> RetuningR {
        RetuningR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier."]
    #[inline(always)]
    pub fn clk_multi(&self) -> ClkMultiR {
        ClkMultiR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Capabilities 32-63.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`reset()` method sets CFG_1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
