#[doc = "Register `CFG_PPI_16_EN` reader"]
pub type R = crate::R<CfgPpi16EnSpec>;
#[doc = "Register `CFG_PPI_16_EN` writer"]
pub type W = crate::W<CfgPpi16EnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_PPI_16_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_ppi_16_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_ppi_16_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPpi16EnSpec;
impl crate::RegisterSpec for CfgPpi16EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_ppi_16_en::R`](R) reader structure"]
impl crate::Readable for CfgPpi16EnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_ppi_16_en::W`](W) writer structure"]
impl crate::Writable for CfgPpi16EnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_PPI_16_EN to value 0"]
impl crate::Resettable for CfgPpi16EnSpec {}
