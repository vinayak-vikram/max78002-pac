#[doc = "Register `FIFO1` writer"]
pub type W = crate::W<Fifo1Spec>;
impl core::fmt::Debug for crate::generic::Reg<Fifo1Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "FIFO 1 data port.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo1Spec;
impl crate::RegisterSpec for Fifo1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifo1::W`](W) writer structure"]
impl crate::Writable for Fifo1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for Fifo1Spec {}
