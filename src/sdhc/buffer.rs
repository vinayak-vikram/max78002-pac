#[doc = "Register `BUFFER` reader"]
pub type R = crate::R<BufferSpec>;
#[doc = "Register `BUFFER` writer"]
pub type W = crate::W<BufferSpec>;
#[doc = "Field `DATA` reader - Buffer Data."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Buffer Data."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, BufferSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Buffer Data Port.\n\nYou can [`read`](crate::Reg::read) this register and get [`buffer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufferSpec;
impl crate::RegisterSpec for BufferSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffer::R`](R) reader structure"]
impl crate::Readable for BufferSpec {}
#[doc = "`write(|w| ..)` method takes [`buffer::W`](W) writer structure"]
impl crate::Writable for BufferSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFFER to value 0"]
impl crate::Resettable for BufferSpec {}
