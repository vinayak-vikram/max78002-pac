#[doc = "Register `FIFO12` reader"]
pub type R = crate::R<Fifo12Spec>;
#[doc = "Register `FIFO12` writer"]
pub type W = crate::W<Fifo12Spec>;
#[doc = "Field `USBHS_FIFO12` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo12R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO12` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo12(&self) -> UsbhsFifo12R {
        UsbhsFifo12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo12(&mut self) -> UsbhsFifo12W<'_, Fifo12Spec> {
        UsbhsFifo12W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo12Spec;
impl crate::RegisterSpec for Fifo12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo12::R`](R) reader structure"]
impl crate::Readable for Fifo12Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo12::W`](W) writer structure"]
impl crate::Writable for Fifo12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO12 to value 0"]
impl crate::Resettable for Fifo12Spec {}
