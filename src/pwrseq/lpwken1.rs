#[doc = "Register `LPWKEN1` reader"]
pub type R = crate::R<Lpwken1Spec>;
#[doc = "Register `LPWKEN1` writer"]
pub type W = crate::W<Lpwken1Spec>;
#[doc = "Field `WAKEEN` reader - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenR = crate::FieldReader<u16>;
#[doc = "Field `WAKEEN` writer - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&self) -> WakeenR {
        WakeenR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WakeenW<'_, Lpwken1Spec> {
        WakeenW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwken1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwken1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwken1Spec;
impl crate::RegisterSpec for Lpwken1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwken1::R`](R) reader structure"]
impl crate::Readable for Lpwken1Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwken1::W`](W) writer structure"]
impl crate::Writable for Lpwken1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPWKEN1 to value 0"]
impl crate::Resettable for Lpwken1Spec {}
