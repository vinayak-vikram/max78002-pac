#[doc = "Register `LPWKEN2` reader"]
pub type R = crate::R<Lpwken2Spec>;
#[doc = "Register `LPWKEN2` writer"]
pub type W = crate::W<Lpwken2Spec>;
#[doc = "Field `WAKEEN` reader - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenR = crate::FieldReader;
#[doc = "Field `WAKEEN` writer - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&self) -> WakeenR {
        WakeenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WakeenW<'_, Lpwken2Spec> {
        WakeenW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwken2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwken2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwken2Spec;
impl crate::RegisterSpec for Lpwken2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwken2::R`](R) reader structure"]
impl crate::Readable for Lpwken2Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwken2::W`](W) writer structure"]
impl crate::Writable for Lpwken2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPWKEN2 to value 0"]
impl crate::Resettable for Lpwken2Spec {}
