#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Field `EN_SUSPENDM` reader - Enable SUSPENDM signal."]
pub type EnSuspendmR = crate::BitReader;
#[doc = "Field `EN_SUSPENDM` writer - Enable SUSPENDM signal."]
pub type EnSuspendmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND` reader - Suspend mode detected."]
pub type SuspendR = crate::BitReader;
#[doc = "Field `RESUME` reader - Generate resume signaling."]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Generate resume signaling."]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Bus reset detected."]
pub type ResetR = crate::BitReader;
#[doc = "Field `HS_MODE` reader - High-speed mode detected."]
pub type HsModeR = crate::BitReader;
#[doc = "Field `HS_ENABLE` reader - High-speed mode enable."]
pub type HsEnableR = crate::BitReader;
#[doc = "Field `HS_ENABLE` writer - High-speed mode enable."]
pub type HsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTCONN` reader - Softconn."]
pub type SoftconnR = crate::BitReader;
#[doc = "Field `SOFTCONN` writer - Softconn."]
pub type SoftconnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO_UPDATE` reader - Wait for SOF during Isochronous xfers."]
pub type IsoUpdateR = crate::BitReader;
#[doc = "Field `ISO_UPDATE` writer - Wait for SOF during Isochronous xfers."]
pub type IsoUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable SUSPENDM signal."]
    #[inline(always)]
    pub fn en_suspendm(&self) -> EnSuspendmR {
        EnSuspendmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend mode detected."]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate resume signaling."]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus reset detected."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - High-speed mode detected."]
    #[inline(always)]
    pub fn hs_mode(&self) -> HsModeR {
        HsModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-speed mode enable."]
    #[inline(always)]
    pub fn hs_enable(&self) -> HsEnableR {
        HsEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Softconn."]
    #[inline(always)]
    pub fn softconn(&self) -> SoftconnR {
        SoftconnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait for SOF during Isochronous xfers."]
    #[inline(always)]
    pub fn iso_update(&self) -> IsoUpdateR {
        IsoUpdateR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable SUSPENDM signal."]
    #[inline(always)]
    pub fn en_suspendm(&mut self) -> EnSuspendmW<'_, PowerSpec> {
        EnSuspendmW::new(self, 0)
    }
    #[doc = "Bit 2 - Generate resume signaling."]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, PowerSpec> {
        ResumeW::new(self, 2)
    }
    #[doc = "Bit 5 - High-speed mode enable."]
    #[inline(always)]
    pub fn hs_enable(&mut self) -> HsEnableW<'_, PowerSpec> {
        HsEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - Softconn."]
    #[inline(always)]
    pub fn softconn(&mut self) -> SoftconnW<'_, PowerSpec> {
        SoftconnW::new(self, 6)
    }
    #[doc = "Bit 7 - Wait for SOF during Isochronous xfers."]
    #[inline(always)]
    pub fn iso_update(&mut self) -> IsoUpdateW<'_, PowerSpec> {
        IsoUpdateW::new(self, 7)
    }
}
#[doc = "Power management register.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for PowerSpec {}
