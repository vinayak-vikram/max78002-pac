#[doc = "Register `FIFO15` reader"]
pub type R = crate::R<Fifo15Spec>;
#[doc = "Register `FIFO15` writer"]
pub type W = crate::W<Fifo15Spec>;
#[doc = "Field `USBHS_FIFO15` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo15R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO15` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo15(&self) -> UsbhsFifo15R {
        UsbhsFifo15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo15(&mut self) -> UsbhsFifo15W<'_, Fifo15Spec> {
        UsbhsFifo15W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo15Spec;
impl crate::RegisterSpec for Fifo15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo15::R`](R) reader structure"]
impl crate::Readable for Fifo15Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo15::W`](W) writer structure"]
impl crate::Writable for Fifo15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO15 to value 0"]
impl crate::Resettable for Fifo15Spec {}
