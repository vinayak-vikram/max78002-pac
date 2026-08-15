#[doc = "Register `VFIFO_FRAME_STS` reader"]
pub type R = crate::R<VfifoFrameStsSpec>;
#[doc = "Register `VFIFO_FRAME_STS` writer"]
pub type W = crate::W<VfifoFrameStsSpec>;
#[doc = "Field `FRAME_STATE` reader - Frame State."]
pub type FrameStateR = crate::FieldReader;
#[doc = "Field `FRAME_STATE` writer - Frame State."]
pub type FrameStateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ERROR_CODE` reader - Error Codes."]
pub type ErrorCodeR = crate::FieldReader;
#[doc = "Field `ERROR_CODE` writer - Error Codes."]
pub type ErrorCodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Frame State."]
    #[inline(always)]
    pub fn frame_state(&self) -> FrameStateR {
        FrameStateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Error Codes."]
    #[inline(always)]
    pub fn error_code(&self) -> ErrorCodeR {
        ErrorCodeR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frame State."]
    #[inline(always)]
    pub fn frame_state(&mut self) -> FrameStateW<'_, VfifoFrameStsSpec> {
        FrameStateW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Error Codes."]
    #[inline(always)]
    pub fn error_code(&mut self) -> ErrorCodeW<'_, VfifoFrameStsSpec> {
        ErrorCodeW::new(self, 3)
    }
}
#[doc = "Video FIFO Frame Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_frame_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_frame_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoFrameStsSpec;
impl crate::RegisterSpec for VfifoFrameStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_frame_sts::R`](R) reader structure"]
impl crate::Readable for VfifoFrameStsSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_frame_sts::W`](W) writer structure"]
impl crate::Writable for VfifoFrameStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_FRAME_STS to value 0"]
impl crate::Resettable for VfifoFrameStsSpec {}
