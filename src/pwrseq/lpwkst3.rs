#[doc = "Register `LPWKST3` reader"]
pub type R = crate::R<Lpwkst3Spec>;
#[doc = "Register `LPWKST3` writer"]
pub type W = crate::W<Lpwkst3Spec>;
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestR = crate::FieldReader;
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&self) -> WakestR {
        WakestR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&mut self) -> WakestW<'_, Lpwkst3Spec> {
        WakestW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwkst3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwkst3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwkst3Spec;
impl crate::RegisterSpec for Lpwkst3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwkst3::R`](R) reader structure"]
impl crate::Readable for Lpwkst3Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwkst3::W`](W) writer structure"]
impl crate::Writable for Lpwkst3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPWKST3 to value 0"]
impl crate::Resettable for Lpwkst3Spec {}
