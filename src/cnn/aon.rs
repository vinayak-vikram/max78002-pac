#[doc = "Register `AON` reader"]
pub type R = crate::R<AonSpec>;
#[doc = "Register `AON` writer"]
pub type W = crate::W<AonSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Always-on domain control. Written as zero during init.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSpec;
impl crate::RegisterSpec for AonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon::R`](R) reader structure"]
impl crate::Readable for AonSpec {}
#[doc = "`write(|w| ..)` method takes [`aon::W`](W) writer structure"]
impl crate::Writable for AonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON to value 0"]
impl crate::Resettable for AonSpec {}
