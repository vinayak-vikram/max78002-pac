#[doc = "Register `SHARED_BUS` reader"]
pub type R = crate::R<SharedBusSpec>;
#[doc = "Register `SHARED_BUS` writer"]
pub type W = crate::W<SharedBusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SHARED_BUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_bus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_bus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedBusSpec;
impl crate::RegisterSpec for SharedBusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_bus::R`](R) reader structure"]
impl crate::Readable for SharedBusSpec {}
#[doc = "`write(|w| ..)` method takes [`shared_bus::W`](W) writer structure"]
impl crate::Writable for SharedBusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHARED_BUS to value 0"]
impl crate::Resettable for SharedBusSpec {}
