#[doc = "Register `FIFO6` reader"]
pub type R = crate::R<Fifo6Spec>;
#[doc = "Register `FIFO6` writer"]
pub type W = crate::W<Fifo6Spec>;
#[doc = "Field `USBHS_FIFO6` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo6R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO6` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo6(&self) -> UsbhsFifo6R {
        UsbhsFifo6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo6(&mut self) -> UsbhsFifo6W<'_, Fifo6Spec> {
        UsbhsFifo6W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo6Spec;
impl crate::RegisterSpec for Fifo6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo6::R`](R) reader structure"]
impl crate::Readable for Fifo6Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo6::W`](W) writer structure"]
impl crate::Writable for Fifo6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO6 to value 0"]
impl crate::Resettable for Fifo6Spec {}
