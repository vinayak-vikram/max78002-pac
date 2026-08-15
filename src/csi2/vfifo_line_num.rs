#[doc = "Register `VFIFO_LINE_NUM` reader"]
pub type R = crate::R<VfifoLineNumSpec>;
#[doc = "Register `VFIFO_LINE_NUM` writer"]
pub type W = crate::W<VfifoLineNumSpec>;
#[doc = "Field `LINE_NUM` reader - Number of lines per frame."]
pub type LineNumR = crate::FieldReader<u16>;
#[doc = "Field `LINE_NUM` writer - Number of lines per frame."]
pub type LineNumW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Number of lines per frame."]
    #[inline(always)]
    pub fn line_num(&self) -> LineNumR {
        LineNumR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of lines per frame."]
    #[inline(always)]
    pub fn line_num(&mut self) -> LineNumW<'_, VfifoLineNumSpec> {
        LineNumW::new(self, 0)
    }
}
#[doc = "Video FIFO CSI Line Number Per Frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_line_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_line_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoLineNumSpec;
impl crate::RegisterSpec for VfifoLineNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_line_num::R`](R) reader structure"]
impl crate::Readable for VfifoLineNumSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_line_num::W`](W) writer structure"]
impl crate::Writable for VfifoLineNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_LINE_NUM to value 0"]
impl crate::Resettable for VfifoLineNumSpec {}
