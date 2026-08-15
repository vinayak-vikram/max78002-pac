#[doc = "Register `FIFO4` reader"]
pub type R = crate::R<Fifo4Spec>;
#[doc = "Register `FIFO4` writer"]
pub type W = crate::W<Fifo4Spec>;
#[doc = "Field `USBHS_FIFO4` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo4R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO4` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo4(&self) -> UsbhsFifo4R {
        UsbhsFifo4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo4(&mut self) -> UsbhsFifo4W<'_, Fifo4Spec> {
        UsbhsFifo4W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo4Spec;
impl crate::RegisterSpec for Fifo4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo4::R`](R) reader structure"]
impl crate::Readable for Fifo4Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo4::W`](W) writer structure"]
impl crate::Writable for Fifo4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO4 to value 0"]
impl crate::Resettable for Fifo4Spec {}
