#[doc = "Register `INTRUSB` reader"]
pub type R = crate::R<IntrusbSpec>;
#[doc = "Register `INTRUSB` writer"]
pub type W = crate::W<IntrusbSpec>;
#[doc = "Field `SUSPEND_INT` reader - Suspend detected."]
pub type SuspendIntR = crate::BitReader;
#[doc = "Field `RESUME_INT` reader - Resume detected."]
pub type ResumeIntR = crate::BitReader;
#[doc = "Field `RESET_INT` reader - Bus reset detected."]
pub type ResetIntR = crate::BitReader;
#[doc = "Field `SOF_INT` reader - Start of Frame."]
pub type SofIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Suspend detected."]
    #[inline(always)]
    pub fn suspend_int(&self) -> SuspendIntR {
        SuspendIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume detected."]
    #[inline(always)]
    pub fn resume_int(&self) -> ResumeIntR {
        ResumeIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus reset detected."]
    #[inline(always)]
    pub fn reset_int(&self) -> ResetIntR {
        ResetIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame."]
    #[inline(always)]
    pub fn sof_int(&self) -> SofIntR {
        SofIntR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt register for common USB interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrusb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrusbSpec;
impl crate::RegisterSpec for IntrusbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intrusb::R`](R) reader structure"]
impl crate::Readable for IntrusbSpec {}
#[doc = "`write(|w| ..)` method takes [`intrusb::W`](W) writer structure"]
impl crate::Writable for IntrusbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRUSB to value 0"]
impl crate::Resettable for IntrusbSpec {}
