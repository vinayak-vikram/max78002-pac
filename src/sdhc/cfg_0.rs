#[doc = "Register `CFG_0` reader"]
pub type R = crate::R<Cfg0Spec>;
#[doc = "Field `TO_CLK_FREQ` reader - Timeout Clock Frequency."]
pub type ToClkFreqR = crate::FieldReader;
#[doc = "Field `TO_CLK_UNIT` reader - Timeout Clock Unit."]
pub type ToClkUnitR = crate::BitReader;
#[doc = "Field `CLK_FREQ` reader - Base Clock Frequency For SD Clock."]
pub type ClkFreqR = crate::FieldReader;
#[doc = "Field `MAX_BLK_LEN` reader - Max Block Length."]
pub type MaxBlkLenR = crate::FieldReader;
#[doc = "Field `BIT_8` reader - 8-bit Support for Embedded Device."]
pub type Bit8R = crate::BitReader;
#[doc = "Field `ADMA2` reader - ADMA2 Support."]
pub type Adma2R = crate::BitReader;
#[doc = "Field `HS` reader - High Speed Support."]
pub type HsR = crate::BitReader;
#[doc = "Field `SDMA` reader - SDMA Support."]
pub type SdmaR = crate::BitReader;
#[doc = "Field `SUSPEND` reader - Suspend/Resume Support."]
pub type SuspendR = crate::BitReader;
#[doc = "Field `V3_3` reader - Voltage Support 3.3V."]
pub type V3_3R = crate::BitReader;
#[doc = "Field `V3_0` reader - Voltage Support 3.0V."]
pub type V3_0R = crate::BitReader;
#[doc = "Field `V1_8` reader - Voltage Support 1.8V."]
pub type V1_8R = crate::BitReader;
#[doc = "Field `BIT_64_SYS_BUS` reader - 64-bit System Bus Support."]
pub type Bit64SysBusR = crate::BitReader;
#[doc = "Field `ASYNC_INT` reader - Asynchronous Interrupt Support."]
pub type AsyncIntR = crate::BitReader;
#[doc = "Field `SLOT_TYPE` reader - Slot Type."]
pub type SlotTypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Frequency."]
    #[inline(always)]
    pub fn to_clk_freq(&self) -> ToClkFreqR {
        ToClkFreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Timeout Clock Unit."]
    #[inline(always)]
    pub fn to_clk_unit(&self) -> ToClkUnitR {
        ToClkUnitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency For SD Clock."]
    #[inline(always)]
    pub fn clk_freq(&self) -> ClkFreqR {
        ClkFreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length."]
    #[inline(always)]
    pub fn max_blk_len(&self) -> MaxBlkLenR {
        MaxBlkLenR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 8-bit Support for Embedded Device."]
    #[inline(always)]
    pub fn bit_8(&self) -> Bit8R {
        Bit8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support."]
    #[inline(always)]
    pub fn adma2(&self) -> Adma2R {
        Adma2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support."]
    #[inline(always)]
    pub fn hs(&self) -> HsR {
        HsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMA Support."]
    #[inline(always)]
    pub fn sdma(&self) -> SdmaR {
        SdmaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support."]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3V."]
    #[inline(always)]
    pub fn v3_3(&self) -> V3_3R {
        V3_3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0V."]
    #[inline(always)]
    pub fn v3_0(&self) -> V3_0R {
        V3_0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8V."]
    #[inline(always)]
    pub fn v1_8(&self) -> V1_8R {
        V1_8R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Bus Support."]
    #[inline(always)]
    pub fn bit_64_sys_bus(&self) -> Bit64SysBusR {
        Bit64SysBusR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support."]
    #[inline(always)]
    pub fn async_int(&self) -> AsyncIntR {
        AsyncIntR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Slot Type."]
    #[inline(always)]
    pub fn slot_type(&self) -> SlotTypeR {
        SlotTypeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Capabilities 0-31.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Spec;
impl crate::RegisterSpec for Cfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_0::R`](R) reader structure"]
impl crate::Readable for Cfg0Spec {}
#[doc = "`reset()` method sets CFG_0 to value 0"]
impl crate::Resettable for Cfg0Spec {}
