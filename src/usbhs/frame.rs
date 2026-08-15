#[doc = "Register `FRAME` reader"]
pub type R = crate::R<FrameSpec>;
#[doc = "Register `FRAME` writer"]
pub type W = crate::W<FrameSpec>;
#[doc = "Field `FRAMENUM` reader - Read the last received frame number, that is the 11-bit frame number received in the SOF packet."]
pub type FramenumR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Read the last received frame number, that is the 11-bit frame number received in the SOF packet."]
    #[inline(always)]
    pub fn framenum(&self) -> FramenumR {
        FramenumR::new(self.bits & 0x07ff)
    }
}
impl W {}
#[doc = "Frame number.\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSpec;
impl crate::RegisterSpec for FrameSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FrameSpec {}
#[doc = "`write(|w| ..)` method takes [`frame::W`](W) writer structure"]
impl crate::Writable for FrameSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME to value 0"]
impl crate::Resettable for FrameSpec {}
