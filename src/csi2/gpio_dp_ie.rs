#[doc = "Register `GPIO_DP_IE` reader"]
pub type R = crate::R<GpioDpIeSpec>;
#[doc = "Register `GPIO_DP_IE` writer"]
pub type W = crate::W<GpioDpIeSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GPIO_DP_IE.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dp_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dp_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDpIeSpec;
impl crate::RegisterSpec for GpioDpIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_dp_ie::R`](R) reader structure"]
impl crate::Readable for GpioDpIeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_dp_ie::W`](W) writer structure"]
impl crate::Writable for GpioDpIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DP_IE to value 0"]
impl crate::Resettable for GpioDpIeSpec {}
