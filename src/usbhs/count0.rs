#[doc = "Register `COUNT0` reader"]
pub type R = crate::R<Count0Spec>;
#[doc = "Register `COUNT0` writer"]
pub type W = crate::W<Count0Spec>;
#[doc = "Field `COUNT0` reader - Read Number of Data Bytes in the Endpoint 0 FIFO. Returns the number of data bytes in the endpoint 0 FIFO. This value changes as contents of the FIFO change. The value is only valued when USBHS_OUTSCRL_outpktrdy = 1"]
pub type Count0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Read Number of Data Bytes in the Endpoint 0 FIFO. Returns the number of data bytes in the endpoint 0 FIFO. This value changes as contents of the FIFO change. The value is only valued when USBHS_OUTSCRL_outpktrdy = 1"]
    #[inline(always)]
    pub fn count0(&self) -> Count0R {
        Count0R::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Number of received bytes in EP 0 FIFO (INDEX == 0).\n\nYou can [`read`](crate::Reg::read) this register and get [`count0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Count0Spec;
impl crate::RegisterSpec for Count0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`count0::R`](R) reader structure"]
impl crate::Readable for Count0Spec {}
#[doc = "`write(|w| ..)` method takes [`count0::W`](W) writer structure"]
impl crate::Writable for Count0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNT0 to value 0"]
impl crate::Resettable for Count0Spec {}
