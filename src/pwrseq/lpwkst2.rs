#[doc = "Register `LPWKST2` reader"]
pub type R = crate::R<Lpwkst2Spec>;
#[doc = "Register `LPWKST2` writer"]
pub type W = crate::W<Lpwkst2Spec>;
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestR = crate::FieldReader;
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&self) -> WakestR {
        WakestR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&mut self) -> WakestW<'_, Lpwkst2Spec> {
        WakestW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwkst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwkst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwkst2Spec;
impl crate::RegisterSpec for Lpwkst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwkst2::R`](R) reader structure"]
impl crate::Readable for Lpwkst2Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwkst2::W`](W) writer structure"]
impl crate::Writable for Lpwkst2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPWKST2 to value 0"]
impl crate::Resettable for Lpwkst2Spec {}
