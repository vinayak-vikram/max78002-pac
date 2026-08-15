#[doc = "Register `CFG_VCX_EN` reader"]
pub type R = crate::R<CfgVcxEnSpec>;
#[doc = "Register `CFG_VCX_EN` writer"]
pub type W = crate::W<CfgVcxEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_VCX_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vcx_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vcx_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgVcxEnSpec;
impl crate::RegisterSpec for CfgVcxEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_vcx_en::R`](R) reader structure"]
impl crate::Readable for CfgVcxEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_vcx_en::W`](W) writer structure"]
impl crate::Writable for CfgVcxEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_VCX_EN to value 0"]
impl crate::Resettable for CfgVcxEnSpec {}
