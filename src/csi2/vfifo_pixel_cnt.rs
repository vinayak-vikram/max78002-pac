#[doc = "Register `VFIFO_PIXEL_CNT` reader"]
pub type R = crate::R<VfifoPixelCntSpec>;
#[doc = "Register `VFIFO_PIXEL_CNT` writer"]
pub type W = crate::W<VfifoPixelCntSpec>;
#[doc = "Field `PIXEL_CNT` reader - Number of received pixels in current line in a frame."]
pub type PixelCntR = crate::FieldReader<u16>;
#[doc = "Field `PIXEL_CNT` writer - Number of received pixels in current line in a frame."]
pub type PixelCntW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Number of received pixels in current line in a frame."]
    #[inline(always)]
    pub fn pixel_cnt(&self) -> PixelCntR {
        PixelCntR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of received pixels in current line in a frame."]
    #[inline(always)]
    pub fn pixel_cnt(&mut self) -> PixelCntW<'_, VfifoPixelCntSpec> {
        PixelCntW::new(self, 0)
    }
}
#[doc = "Video FIFO CSI Pixel Count.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_pixel_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_pixel_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoPixelCntSpec;
impl crate::RegisterSpec for VfifoPixelCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_pixel_cnt::R`](R) reader structure"]
impl crate::Readable for VfifoPixelCntSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_pixel_cnt::W`](W) writer structure"]
impl crate::Writable for VfifoPixelCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_PIXEL_CNT to value 0"]
impl crate::Resettable for VfifoPixelCntSpec {}
