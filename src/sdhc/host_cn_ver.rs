#[doc = "Register `HOST_CN_VER` reader"]
pub type R = crate::R<HostCnVerSpec>;
#[doc = "Register `HOST_CN_VER` writer"]
pub type W = crate::W<HostCnVerSpec>;
#[doc = "Field `SPEC_VER` reader - Specification Version Number."]
pub type SpecVerR = crate::FieldReader;
#[doc = "Field `SPEC_VER` writer - Specification Version Number."]
pub type SpecVerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VEND_VER` reader - Vendor Version Number."]
pub type VendVerR = crate::FieldReader;
#[doc = "Field `VEND_VER` writer - Vendor Version Number."]
pub type VendVerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specification Version Number."]
    #[inline(always)]
    pub fn spec_ver(&self) -> SpecVerR {
        SpecVerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version Number."]
    #[inline(always)]
    pub fn vend_ver(&self) -> VendVerR {
        VendVerR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specification Version Number."]
    #[inline(always)]
    pub fn spec_ver(&mut self) -> SpecVerW<'_, HostCnVerSpec> {
        SpecVerW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Vendor Version Number."]
    #[inline(always)]
    pub fn vend_ver(&mut self) -> VendVerW<'_, HostCnVerSpec> {
        VendVerW::new(self, 8)
    }
}
#[doc = "Host Controller Version.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cn_ver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cn_ver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCnVerSpec;
impl crate::RegisterSpec for HostCnVerSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`host_cn_ver::R`](R) reader structure"]
impl crate::Readable for HostCnVerSpec {}
#[doc = "`write(|w| ..)` method takes [`host_cn_ver::W`](W) writer structure"]
impl crate::Writable for HostCnVerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_CN_VER to value 0"]
impl crate::Resettable for HostCnVerSpec {}
