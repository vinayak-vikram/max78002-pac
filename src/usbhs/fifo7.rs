#[doc = "Register `FIFO7` reader"]
pub type R = crate::R<Fifo7Spec>;
#[doc = "Register `FIFO7` writer"]
pub type W = crate::W<Fifo7Spec>;
#[doc = "Field `USBHS_FIFO7` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo7R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO7` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo7(&self) -> UsbhsFifo7R {
        UsbhsFifo7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo7(&mut self) -> UsbhsFifo7W<'_, Fifo7Spec> {
        UsbhsFifo7W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo7Spec;
impl crate::RegisterSpec for Fifo7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo7::R`](R) reader structure"]
impl crate::Readable for Fifo7Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo7::W`](W) writer structure"]
impl crate::Writable for Fifo7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO7 to value 0"]
impl crate::Resettable for Fifo7Spec {}
