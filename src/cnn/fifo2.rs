#[doc = "Register `FIFO2` writer"]
pub type W = crate::W<Fifo2Spec>;
impl core::fmt::Debug for crate::generic::Reg<Fifo2Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "FIFO 2 data port.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo2Spec;
impl crate::RegisterSpec for Fifo2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifo2::W`](W) writer structure"]
impl crate::Writable for Fifo2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO2 to value 0"]
impl crate::Resettable for Fifo2Spec {}
