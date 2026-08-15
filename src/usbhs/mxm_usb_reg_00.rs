#[doc = "Register `MXM_USB_REG_00` reader"]
pub type R = crate::R<MxmUsbReg00Spec>;
#[doc = "Register `MXM_USB_REG_00` writer"]
pub type W = crate::W<MxmUsbReg00Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "MXM_USB_REG_00\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_usb_reg_00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_usb_reg_00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MxmUsbReg00Spec;
impl crate::RegisterSpec for MxmUsbReg00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mxm_usb_reg_00::R`](R) reader structure"]
impl crate::Readable for MxmUsbReg00Spec {}
#[doc = "`write(|w| ..)` method takes [`mxm_usb_reg_00::W`](W) writer structure"]
impl crate::Writable for MxmUsbReg00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MXM_USB_REG_00 to value 0"]
impl crate::Resettable for MxmUsbReg00Spec {}
