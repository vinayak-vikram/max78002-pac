#[doc = "Register `VFIFO_PIXEL_NUM` reader"]
pub type R = crate::R<VfifoPixelNumSpec>;
#[doc = "Register `VFIFO_PIXEL_NUM` writer"]
pub type W = crate::W<VfifoPixelNumSpec>;
#[doc = "Field `PIXEL_NUM` reader - Number of pixels per line."]
pub type PixelNumR = crate::FieldReader<u16>;
#[doc = "Field `PIXEL_NUM` writer - Number of pixels per line."]
pub type PixelNumW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Number of pixels per line."]
    #[inline(always)]
    pub fn pixel_num(&self) -> PixelNumR {
        PixelNumR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Number of pixels per line."]
    #[inline(always)]
    pub fn pixel_num(&mut self) -> PixelNumW<'_, VfifoPixelNumSpec> {
        PixelNumW::new(self, 0)
    }
}
#[doc = "Video FIFO CSI Pixel Number Per Line.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_pixel_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_pixel_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoPixelNumSpec;
impl crate::RegisterSpec for VfifoPixelNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_pixel_num::R`](R) reader structure"]
impl crate::Readable for VfifoPixelNumSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_pixel_num::W`](W) writer structure"]
impl crate::Writable for VfifoPixelNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_PIXEL_NUM to value 0"]
impl crate::Resettable for VfifoPixelNumSpec {}
