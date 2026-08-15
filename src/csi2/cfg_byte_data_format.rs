#[doc = "Register `CFG_BYTE_DATA_FORMAT` reader"]
pub type R = crate::R<CfgByteDataFormatSpec>;
#[doc = "Register `CFG_BYTE_DATA_FORMAT` writer"]
pub type W = crate::W<CfgByteDataFormatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_BYTE_DATA_FORMAT.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_byte_data_format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_byte_data_format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgByteDataFormatSpec;
impl crate::RegisterSpec for CfgByteDataFormatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_byte_data_format::R`](R) reader structure"]
impl crate::Readable for CfgByteDataFormatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_byte_data_format::W`](W) writer structure"]
impl crate::Writable for CfgByteDataFormatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_BYTE_DATA_FORMAT to value 0"]
impl crate::Resettable for CfgByteDataFormatSpec {}
