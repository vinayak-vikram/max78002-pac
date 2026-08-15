#[doc = "Register `FIFO8` reader"]
pub type R = crate::R<Fifo8Spec>;
#[doc = "Register `FIFO8` writer"]
pub type W = crate::W<Fifo8Spec>;
#[doc = "Field `USBHS_FIFO8` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo8R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO8` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo8(&self) -> UsbhsFifo8R {
        UsbhsFifo8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo8(&mut self) -> UsbhsFifo8W<'_, Fifo8Spec> {
        UsbhsFifo8W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo8Spec;
impl crate::RegisterSpec for Fifo8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo8::R`](R) reader structure"]
impl crate::Readable for Fifo8Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo8::W`](W) writer structure"]
impl crate::Writable for Fifo8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO8 to value 0"]
impl crate::Resettable for Fifo8Spec {}
