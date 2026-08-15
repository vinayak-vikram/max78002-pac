#[doc = "Register `FIFO0` reader"]
pub type R = crate::R<Fifo0Spec>;
#[doc = "Register `FIFO0` writer"]
pub type W = crate::W<Fifo0Spec>;
#[doc = "Field `USBHS_FIFO0` reader - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo0R = crate::FieldReader<u32>;
#[doc = "Field `USBHS_FIFO0` writer - USBHS Endpoint FIFO Read/Write Register."]
pub type UsbhsFifo0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo0(&self) -> UsbhsFifo0R {
        UsbhsFifo0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USBHS Endpoint FIFO Read/Write Register."]
    #[inline(always)]
    pub fn usbhs_fifo0(&mut self) -> UsbhsFifo0W<'_, Fifo0Spec> {
        UsbhsFifo0W::new(self, 0)
    }
}
#[doc = "Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo0Spec;
impl crate::RegisterSpec for Fifo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo0::R`](R) reader structure"]
impl crate::Readable for Fifo0Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo0::W`](W) writer structure"]
impl crate::Writable for Fifo0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO0 to value 0"]
impl crate::Resettable for Fifo0Spec {}
