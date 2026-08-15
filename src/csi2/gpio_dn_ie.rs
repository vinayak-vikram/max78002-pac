#[doc = "Register `GPIO_DN_IE` reader"]
pub type R = crate::R<GpioDnIeSpec>;
#[doc = "Register `GPIO_DN_IE` writer"]
pub type W = crate::W<GpioDnIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GPIO_DN_IE.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dn_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dn_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDnIeSpec;
impl crate::RegisterSpec for GpioDnIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_dn_ie::R`](R) reader structure"]
impl crate::Readable for GpioDnIeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_dn_ie::W`](W) writer structure"]
impl crate::Writable for GpioDnIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DN_IE to value 0"]
impl crate::Resettable for GpioDnIeSpec {}
