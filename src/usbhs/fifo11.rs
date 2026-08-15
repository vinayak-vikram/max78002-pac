#[doc = "Register `FIFO11` reader"]
pub type R = crate::R<Fifo11Spec>;
#[doc = "Register `FIFO11` writer"]
pub type W = crate::W<Fifo11Spec>;
#[doc = "Field `USBHS_FIFO11` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo11R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO11` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo11(&self) -> UsbhsFifo11R {
        UsbhsFifo11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo11(&mut self) -> UsbhsFifo11W<'_, Fifo11Spec> {
        UsbhsFifo11W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo11Spec;
impl crate::RegisterSpec for Fifo11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo11::R`](R) reader structure"]
impl crate::Readable for Fifo11Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo11::W`](W) writer structure"]
impl crate::Writable for Fifo11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO11 to value 0"]
impl crate::Resettable for Fifo11Spec {}
