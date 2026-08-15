#[doc = "Register `HWVERS` reader"]
pub type R = crate::R<HwversSpec>;
#[doc = "Register `HWVERS` writer"]
pub type W = crate::W<HwversSpec>;
#[doc = "Field `USBHS_HWVERS` reader - USBHS Register."]
pub type UsbhsHwversR = crate::FieldReader<u16>;
#[doc = "Field `USBHS_HWVERS` writer - USBHS Register."]
pub type UsbhsHwversW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - USBHS Register."]
    #[inline(always)]
    pub fn usbhs_hwvers(&self) -> UsbhsHwversR {
        UsbhsHwversR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - USBHS Register."]
    #[inline(always)]
    pub fn usbhs_hwvers(&mut self) -> UsbhsHwversW<'_, HwversSpec> {
        UsbhsHwversW::new(self, 0)
    }
}
#[doc = "HWVERS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwversSpec;
impl crate::RegisterSpec for HwversSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hwvers::R`](R) reader structure"]
impl crate::Readable for HwversSpec {}
#[doc = "`write(|w| ..)` method takes [`hwvers::W`](W) writer structure"]
impl crate::Writable for HwversSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HWVERS to value 0"]
impl crate::Resettable for HwversSpec {}
