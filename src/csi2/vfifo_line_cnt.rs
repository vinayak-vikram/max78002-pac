#[doc = "Register `VFIFO_LINE_CNT` reader"]
pub type R = crate::R<VfifoLineCntSpec>;
#[doc = "Register `VFIFO_LINE_CNT` writer"]
pub type W = crate::W<VfifoLineCntSpec>;
#[doc = "Field `LINE_CNT` reader - Number of received lines in current frame."]
pub type LineCntR = crate::FieldReader<u16>;
#[doc = "Field `LINE_CNT` writer - Number of received lines in current frame."]
pub type LineCntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of received lines in current frame."]
    #[inline(always)]
    pub fn line_cnt(&self) -> LineCntR {
        LineCntR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of received lines in current frame."]
    #[inline(always)]
    pub fn line_cnt(&mut self) -> LineCntW<'_, VfifoLineCntSpec> {
        LineCntW::new(self, 0)
    }
}
#[doc = "Video FIFO CSI Line Count.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_line_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_line_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoLineCntSpec;
impl crate::RegisterSpec for VfifoLineCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_line_cnt::R`](R) reader structure"]
impl crate::Readable for VfifoLineCntSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_line_cnt::W`](W) writer structure"]
impl crate::Writable for VfifoLineCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_LINE_CNT to value 0"]
impl crate::Resettable for VfifoLineCntSpec {}
