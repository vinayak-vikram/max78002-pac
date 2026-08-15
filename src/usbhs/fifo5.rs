#[doc = "Register `FIFO5` reader"]
pub type R = crate::R<Fifo5Spec>;
#[doc = "Register `FIFO5` writer"]
pub type W = crate::W<Fifo5Spec>;
#[doc = "Field `USBHS_FIFO5` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo5R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO5` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo5(&self) -> UsbhsFifo5R {
        UsbhsFifo5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo5(&mut self) -> UsbhsFifo5W<'_, Fifo5Spec> {
        UsbhsFifo5W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo5Spec;
impl crate::RegisterSpec for Fifo5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo5::R`](R) reader structure"]
impl crate::Readable for Fifo5Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo5::W`](W) writer structure"]
impl crate::Writable for Fifo5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO5 to value 0"]
impl crate::Resettable for Fifo5Spec {}
