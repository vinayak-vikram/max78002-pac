#[doc = "Register `FIFO3` writer"]
pub type W = crate::W<Fifo3Spec>;
impl core::fmt::Debug for crate::generic::Reg<Fifo3Spec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "FIFO 3 data port.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo3Spec;
impl crate::RegisterSpec for Fifo3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifo3::W`](W) writer structure"]
impl crate::Writable for Fifo3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO3 to value 0"]
impl crate::Resettable for Fifo3Spec {}
