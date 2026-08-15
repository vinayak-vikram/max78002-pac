#[doc = "Register `AON_POWER_READY_N` reader"]
pub type R = crate::R<AonPowerReadyNSpec>;
#[doc = "Register `AON_POWER_READY_N` writer"]
pub type W = crate::W<AonPowerReadyNSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AON_POWER_READY_N.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_power_ready_n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_power_ready_n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonPowerReadyNSpec;
impl crate::RegisterSpec for AonPowerReadyNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_power_ready_n::R`](R) reader structure"]
impl crate::Readable for AonPowerReadyNSpec {}
#[doc = "`write(|w| ..)` method takes [`aon_power_ready_n::W`](W) writer structure"]
impl crate::Writable for AonPowerReadyNSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON_POWER_READY_N to value 0"]
impl crate::Resettable for AonPowerReadyNSpec {}
