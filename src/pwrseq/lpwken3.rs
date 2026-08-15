#[doc = "Register `LPWKEN3` reader"]
pub type R = crate::R<Lpwken3Spec>;
#[doc = "Register `LPWKEN3` writer"]
pub type W = crate::W<Lpwken3Spec>;
#[doc = "Field `WAKEEN` reader - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenR = crate::FieldReader;
#[doc = "Field `WAKEEN` writer - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&self) -> WakeenR {
        WakeenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WakeenW<'_, Lpwken3Spec> {
        WakeenW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwken3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwken3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwken3Spec;
impl crate::RegisterSpec for Lpwken3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwken3::R`](R) reader structure"]
impl crate::Readable for Lpwken3Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwken3::W`](W) writer structure"]
impl crate::Writable for Lpwken3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPWKEN3 to value 0"]
impl crate::Resettable for Lpwken3Spec {}
