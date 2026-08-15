#[doc = "Register `INTRUSBEN` reader"]
pub type R = crate::R<IntrusbenSpec>;
#[doc = "Register `INTRUSBEN` writer"]
pub type W = crate::W<IntrusbenSpec>;
#[doc = "Field `SUSPEND_INT_EN` reader - Suspend detected."]
pub type SuspendIntEnR = crate::BitReader;
#[doc = "Field `SUSPEND_INT_EN` writer - Suspend detected."]
pub type SuspendIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME_INT_EN` reader - Resume detected."]
pub type ResumeIntEnR = crate::BitReader;
#[doc = "Field `RESUME_INT_EN` writer - Resume detected."]
pub type ResumeIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_INT_EN` reader - Bus reset detected."]
pub type ResetIntEnR = crate::BitReader;
#[doc = "Field `RESET_INT_EN` writer - Bus reset detected."]
pub type ResetIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF_INT_EN` reader - Start of Frame."]
pub type SofIntEnR = crate::BitReader;
#[doc = "Field `SOF_INT_EN` writer - Start of Frame."]
pub type SofIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Suspend detected."]
    #[inline(always)]
    pub fn suspend_int_en(&self) -> SuspendIntEnR {
        SuspendIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume detected."]
    #[inline(always)]
    pub fn resume_int_en(&self) -> ResumeIntEnR {
        ResumeIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus reset detected."]
    #[inline(always)]
    pub fn reset_int_en(&self) -> ResetIntEnR {
        ResetIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame."]
    #[inline(always)]
    pub fn sof_int_en(&self) -> SofIntEnR {
        SofIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend detected."]
    #[inline(always)]
    pub fn suspend_int_en(&mut self) -> SuspendIntEnW<'_, IntrusbenSpec> {
        SuspendIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Resume detected."]
    #[inline(always)]
    pub fn resume_int_en(&mut self) -> ResumeIntEnW<'_, IntrusbenSpec> {
        ResumeIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus reset detected."]
    #[inline(always)]
    pub fn reset_int_en(&mut self) -> ResetIntEnW<'_, IntrusbenSpec> {
        ResetIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Frame."]
    #[inline(always)]
    pub fn sof_int_en(&mut self) -> SofIntEnW<'_, IntrusbenSpec> {
        SofIntEnW::new(self, 3)
    }
}
#[doc = "Interrupt enable for common USB interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrusben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrusbenSpec;
impl crate::RegisterSpec for IntrusbenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intrusben::R`](R) reader structure"]
impl crate::Readable for IntrusbenSpec {}
#[doc = "`write(|w| ..)` method takes [`intrusben::W`](W) writer structure"]
impl crate::Writable for IntrusbenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRUSBEN to value 0"]
impl crate::Resettable for IntrusbenSpec {}
