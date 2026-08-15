#[doc = "Register `FIFO1` reader"]
pub type R = crate::R<Fifo1Spec>;
#[doc = "Register `FIFO1` writer"]
pub type W = crate::W<Fifo1Spec>;
#[doc = "Field `USBHS_FIFO1` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo1R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO1` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo1(&self) -> UsbhsFifo1R {
        UsbhsFifo1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo1(&mut self) -> UsbhsFifo1W<'_, Fifo1Spec> {
        UsbhsFifo1W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo1Spec;
impl crate::RegisterSpec for Fifo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo1::R`](R) reader structure"]
impl crate::Readable for Fifo1Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo1::W`](W) writer structure"]
impl crate::Writable for Fifo1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO1 to value 0"]
impl crate::Resettable for Fifo1Spec {}
