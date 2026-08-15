#[doc = "Register `FIFO10` reader"]
pub type R = crate::R<Fifo10Spec>;
#[doc = "Register `FIFO10` writer"]
pub type W = crate::W<Fifo10Spec>;
#[doc = "Field `USBHS_FIFO10` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo10R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO10` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo10(&self) -> UsbhsFifo10R {
        UsbhsFifo10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo10(&mut self) -> UsbhsFifo10W<'_, Fifo10Spec> {
        UsbhsFifo10W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo10Spec;
impl crate::RegisterSpec for Fifo10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo10::R`](R) reader structure"]
impl crate::Readable for Fifo10Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo10::W`](W) writer structure"]
impl crate::Writable for Fifo10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO10 to value 0"]
impl crate::Resettable for Fifo10Spec {}
