#[doc = "Register `SAMPCLKCTRL` reader"]
pub type R = crate::R<SampclkctrlSpec>;
#[doc = "Register `SAMPCLKCTRL` writer"]
pub type W = crate::W<SampclkctrlSpec>;
#[doc = "Field `TRACK_CNT` reader - Number of cycles for SAMPLE_CLK high time."]
pub type TrackCntR = crate::FieldReader;
#[doc = "Field `TRACK_CNT` writer - Number of cycles for SAMPLE_CLK high time."]
pub type TrackCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IDLE_CNT` reader - Number of cycles for SAMPLE_CLK low time."]
pub type IdleCntR = crate::FieldReader<u16>;
#[doc = "Field `IDLE_CNT` writer - Number of cycles for SAMPLE_CLK low time."]
pub type IdleCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Number of cycles for SAMPLE_CLK high time."]
    #[inline(always)]
    pub fn track_cnt(&self) -> TrackCntR {
        TrackCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Number of cycles for SAMPLE_CLK low time."]
    #[inline(always)]
    pub fn idle_cnt(&self) -> IdleCntR {
        IdleCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of cycles for SAMPLE_CLK high time."]
    #[inline(always)]
    pub fn track_cnt(&mut self) -> TrackCntW<'_, SampclkctrlSpec> {
        TrackCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of cycles for SAMPLE_CLK low time."]
    #[inline(always)]
    pub fn idle_cnt(&mut self) -> IdleCntW<'_, SampclkctrlSpec> {
        IdleCntW::new(self, 16)
    }
}
#[doc = "Sample Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sampclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampclkctrlSpec;
impl crate::RegisterSpec for SampclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sampclkctrl::R`](R) reader structure"]
impl crate::Readable for SampclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sampclkctrl::W`](W) writer structure"]
impl crate::Writable for SampclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPCLKCTRL to value 0"]
impl crate::Resettable for SampclkctrlSpec {}
