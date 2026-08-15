#[doc = "Register `FIFO14` reader"]
pub type R = crate::R<Fifo14Spec>;
#[doc = "Register `FIFO14` writer"]
pub type W = crate::W<Fifo14Spec>;
#[doc = "Field `USBHS_FIFO14` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo14R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO14` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo14(&self) -> UsbhsFifo14R {
        UsbhsFifo14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo14(&mut self) -> UsbhsFifo14W<'_, Fifo14Spec> {
        UsbhsFifo14W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo14Spec;
impl crate::RegisterSpec for Fifo14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo14::R`](R) reader structure"]
impl crate::Readable for Fifo14Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo14::W`](W) writer structure"]
impl crate::Writable for Fifo14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO14 to value 0"]
impl crate::Resettable for Fifo14Spec {}
