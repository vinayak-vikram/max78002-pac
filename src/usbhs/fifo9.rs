#[doc = "Register `FIFO9` reader"]
pub type R = crate::R<Fifo9Spec>;
#[doc = "Register `FIFO9` writer"]
pub type W = crate::W<Fifo9Spec>;
#[doc = "Field `USBHS_FIFO9` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo9R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO9` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo9W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo9(&self) -> UsbhsFifo9R {
        UsbhsFifo9R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo9(&mut self) -> UsbhsFifo9W<'_, Fifo9Spec> {
        UsbhsFifo9W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo9Spec;
impl crate::RegisterSpec for Fifo9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo9::R`](R) reader structure"]
impl crate::Readable for Fifo9Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo9::W`](W) writer structure"]
impl crate::Writable for Fifo9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO9 to value 0"]
impl crate::Resettable for Fifo9Spec {}
