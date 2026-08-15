#[doc = "Register `MXM_INT_EN` reader"]
pub type R = crate::R<MxmIntEnSpec>;
#[doc = "Register `MXM_INT_EN` writer"]
pub type W = crate::W<MxmIntEnSpec>;
#[doc = "Field `VBUS` reader - VBUS"]
pub type VbusR = crate::BitReader;
#[doc = "Field `VBUS` writer - VBUS"]
pub type VbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOVBUS` reader - NOVBUS"]
pub type NovbusR = crate::BitReader;
#[doc = "Field `NOVBUS` writer - NOVBUS"]
pub type NovbusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VBUS"]
    #[inline(always)]
    pub fn vbus(&self) -> VbusR {
        VbusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NOVBUS"]
    #[inline(always)]
    pub fn novbus(&self) -> NovbusR {
        NovbusR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VbusW<'_, MxmIntEnSpec> {
        VbusW::new(self, 0)
    }
    #[doc = "Bit 1 - NOVBUS"]
    #[inline(always)]
    pub fn novbus(&mut self) -> NovbusW<'_, MxmIntEnSpec> {
        NovbusW::new(self, 1)
    }
}
#[doc = "USB Added Maxim Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MxmIntEnSpec;
impl crate::RegisterSpec for MxmIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mxm_int_en::R`](R) reader structure"]
impl crate::Readable for MxmIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`mxm_int_en::W`](W) writer structure"]
impl crate::Writable for MxmIntEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MXM_INT_EN to value 0"]
impl crate::Resettable for MxmIntEnSpec {}
