#[doc = "Register `HOST_CN_2` reader"]
pub type R = crate::R<HostCn2Spec>;
#[doc = "Register `HOST_CN_2` writer"]
pub type W = crate::W<HostCn2Spec>;
#[doc = "Field `UHS` reader - UHS Mode Select."]
pub type UhsR = crate::FieldReader;
#[doc = "Field `UHS` writer - UHS Mode Select."]
pub type UhsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIGNAL_V1_8` reader - 1.8V Signaling Enable."]
pub type SignalV1_8R = crate::BitReader;
#[doc = "Field `SIGNAL_V1_8` writer - 1.8V Signaling Enable."]
pub type SignalV1_8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVER_STRENGTH` reader - Driver Strength Select."]
pub type DriverStrengthR = crate::FieldReader;
#[doc = "Field `DRIVER_STRENGTH` writer - Driver Strength Select."]
pub type DriverStrengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXCUTE` reader - Execute Tuning."]
pub type ExcuteR = crate::BitReader;
#[doc = "Field `EXCUTE` writer - Execute Tuning."]
pub type ExcuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLING_CLK` reader - Sampling Clock Select."]
pub type SamplingClkR = crate::BitReader;
#[doc = "Field `SAMPLING_CLK` writer - Sampling Clock Select."]
pub type SamplingClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCH_INT` reader - Asynchronous Interrupt Enable."]
pub type AsynchIntR = crate::BitReader;
#[doc = "Field `ASYNCH_INT` writer - Asynchronous Interrupt Enable."]
pub type AsynchIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESET_VAL_EN` reader - Preset Value Enable."]
pub type PresetValEnR = crate::BitReader;
#[doc = "Field `PRESET_VAL_EN` writer - Preset Value Enable."]
pub type PresetValEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - UHS Mode Select."]
    #[inline(always)]
    pub fn uhs(&self) -> UhsR {
        UhsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable."]
    #[inline(always)]
    pub fn signal_v1_8(&self) -> SignalV1_8R {
        SignalV1_8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Driver Strength Select."]
    #[inline(always)]
    pub fn driver_strength(&self) -> DriverStrengthR {
        DriverStrengthR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Execute Tuning."]
    #[inline(always)]
    pub fn excute(&self) -> ExcuteR {
        ExcuteR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sampling Clock Select."]
    #[inline(always)]
    pub fn sampling_clk(&self) -> SamplingClkR {
        SamplingClkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub fn asynch_int(&self) -> AsynchIntR {
        AsynchIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Preset Value Enable."]
    #[inline(always)]
    pub fn preset_val_en(&self) -> PresetValEnR {
        PresetValEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - UHS Mode Select."]
    #[inline(always)]
    pub fn uhs(&mut self) -> UhsW<'_, HostCn2Spec> {
        UhsW::new(self, 0)
    }
    #[doc = "Bit 3 - 1.8V Signaling Enable."]
    #[inline(always)]
    pub fn signal_v1_8(&mut self) -> SignalV1_8W<'_, HostCn2Spec> {
        SignalV1_8W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Driver Strength Select."]
    #[inline(always)]
    pub fn driver_strength(&mut self) -> DriverStrengthW<'_, HostCn2Spec> {
        DriverStrengthW::new(self, 4)
    }
    #[doc = "Bit 6 - Execute Tuning."]
    #[inline(always)]
    pub fn excute(&mut self) -> ExcuteW<'_, HostCn2Spec> {
        ExcuteW::new(self, 6)
    }
    #[doc = "Bit 7 - Sampling Clock Select."]
    #[inline(always)]
    pub fn sampling_clk(&mut self) -> SamplingClkW<'_, HostCn2Spec> {
        SamplingClkW::new(self, 7)
    }
    #[doc = "Bit 14 - Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub fn asynch_int(&mut self) -> AsynchIntW<'_, HostCn2Spec> {
        AsynchIntW::new(self, 14)
    }
    #[doc = "Bit 15 - Preset Value Enable."]
    #[inline(always)]
    pub fn preset_val_en(&mut self) -> PresetValEnW<'_, HostCn2Spec> {
        PresetValEnW::new(self, 15)
    }
}
#[doc = "Host Control 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cn_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cn_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCn2Spec;
impl crate::RegisterSpec for HostCn2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`host_cn_2::R`](R) reader structure"]
impl crate::Readable for HostCn2Spec {}
#[doc = "`write(|w| ..)` method takes [`host_cn_2::W`](W) writer structure"]
impl crate::Writable for HostCn2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_CN_2 to value 0"]
impl crate::Resettable for HostCn2Spec {}
