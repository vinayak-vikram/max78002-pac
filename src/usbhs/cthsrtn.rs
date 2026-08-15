#[doc = "Register `CTHSRTN` reader"]
pub type R = crate::R<CthsrtnSpec>;
#[doc = "Register `CTHSRTN` writer"]
pub type W = crate::W<CthsrtnSpec>;
#[doc = "Field `C_T_HSTRN` reader - High Speed Resume Delay Clock Cycles. This configures the delay from when the RESUME state on the bus ends, the when the USBHS resumes normal operation."]
pub type CTHstrnR = crate::FieldReader<u16>;
#[doc = "Field `C_T_HSTRN` writer - High Speed Resume Delay Clock Cycles. This configures the delay from when the RESUME state on the bus ends, the when the USBHS resumes normal operation."]
pub type CTHstrnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - High Speed Resume Delay Clock Cycles. This configures the delay from when the RESUME state on the bus ends, the when the USBHS resumes normal operation."]
    #[inline(always)]
    pub fn c_t_hstrn(&self) -> CTHstrnR {
        CTHstrnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - High Speed Resume Delay Clock Cycles. This configures the delay from when the RESUME state on the bus ends, the when the USBHS resumes normal operation."]
    #[inline(always)]
    pub fn c_t_hstrn(&mut self) -> CTHstrnW<'_, CthsrtnSpec> {
        CTHstrnW::new(self, 0)
    }
}
#[doc = "Sets delay between HS resume to UTM normal operating mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`cthsrtn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cthsrtn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CthsrtnSpec;
impl crate::RegisterSpec for CthsrtnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cthsrtn::R`](R) reader structure"]
impl crate::Readable for CthsrtnSpec {}
#[doc = "`write(|w| ..)` method takes [`cthsrtn::W`](W) writer structure"]
impl crate::Writable for CthsrtnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTHSRTN to value 0"]
impl crate::Resettable for CthsrtnSpec {}
