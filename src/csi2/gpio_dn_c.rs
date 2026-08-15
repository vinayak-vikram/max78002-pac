#[doc = "Register `GPIO_DN_C` reader"]
pub type R = crate::R<GpioDnCSpec>;
#[doc = "Register `GPIO_DN_C` writer"]
pub type W = crate::W<GpioDnCSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GPIO_DN_C.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dn_c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dn_c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDnCSpec;
impl crate::RegisterSpec for GpioDnCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_dn_c::R`](R) reader structure"]
impl crate::Readable for GpioDnCSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_dn_c::W`](W) writer structure"]
impl crate::Writable for GpioDnCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_DN_C to value 0"]
impl crate::Resettable for GpioDnCSpec {}
