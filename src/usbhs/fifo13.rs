#[doc = "Register `FIFO13` reader"]
pub type R = crate::R<Fifo13Spec>;
#[doc = "Register `FIFO13` writer"]
pub type W = crate::W<Fifo13Spec>;
#[doc = "Field `USBHS_FIFO13` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo13R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO13` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo13(&self) -> UsbhsFifo13R {
        UsbhsFifo13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo13(&mut self) -> UsbhsFifo13W<'_, Fifo13Spec> {
        UsbhsFifo13W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo13Spec;
impl crate::RegisterSpec for Fifo13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo13::R`](R) reader structure"]
impl crate::Readable for Fifo13Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo13::W`](W) writer structure"]
impl crate::Writable for Fifo13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO13 to value 0"]
impl crate::Resettable for Fifo13Spec {}
