#[doc = "Register `OUTCOUNT` reader"]
pub type R = crate::R<OutcountSpec>;
#[doc = "Register `OUTCOUNT` writer"]
pub type W = crate::W<OutcountSpec>;
#[doc = "Field `OUTCOUNT` reader - Read Number of Data Bytes in OUT FIFO. Returns the number of data bytes in the packet that are read next in the OUT FIFO."]
pub type OutcountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Read Number of Data Bytes in OUT FIFO. Returns the number of data bytes in the packet that are read next in the OUT FIFO."]
    #[inline(always)]
    pub fn outcount(&self) -> OutcountR {
        OutcountR::new(self.bits & 0x1fff)
    }
}
impl W {}
#[doc = "Number of received bytes in OUT EPx FIFO (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outcount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutcountSpec;
impl crate::RegisterSpec for OutcountSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`outcount::R`](R) reader structure"]
impl crate::Readable for OutcountSpec {}
#[doc = "`write(|w| ..)` method takes [`outcount::W`](W) writer structure"]
impl crate::Writable for OutcountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTCOUNT to value 0"]
impl crate::Resettable for OutcountSpec {}
