#[doc = "Register `LPWKST1` reader"]
pub type R = crate::R<Lpwkst1Spec>;
#[doc = "Register `LPWKST1` writer"]
pub type W = crate::W<Lpwkst1Spec>;
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestR = crate::FieldReader<u16>;
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&self) -> WakestR {
        WakestR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&mut self) -> WakestW<'_, Lpwkst1Spec> {
        WakestW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwkst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwkst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwkst1Spec;
impl crate::RegisterSpec for Lpwkst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwkst1::R`](R) reader structure"]
impl crate::Readable for Lpwkst1Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwkst1::W`](W) writer structure"]
impl crate::Writable for Lpwkst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPWKST1 to value 0"]
impl crate::Resettable for Lpwkst1Spec {}
