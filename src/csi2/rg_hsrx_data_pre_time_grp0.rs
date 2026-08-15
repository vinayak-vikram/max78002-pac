#[doc = "Register `RG_HSRX_DATA_PRE_TIME_GRP0` reader"]
pub type R = crate::R<RgHsrxDataPreTimeGrp0Spec>;
#[doc = "Register `RG_HSRX_DATA_PRE_TIME_GRP0` writer"]
pub type W = crate::W<RgHsrxDataPreTimeGrp0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RG_HSRX_DATA_PRE_TIME_GRP0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_hsrx_data_pre_time_grp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_hsrx_data_pre_time_grp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgHsrxDataPreTimeGrp0Spec;
impl crate::RegisterSpec for RgHsrxDataPreTimeGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_hsrx_data_pre_time_grp0::R`](R) reader structure"]
impl crate::Readable for RgHsrxDataPreTimeGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`rg_hsrx_data_pre_time_grp0::W`](W) writer structure"]
impl crate::Writable for RgHsrxDataPreTimeGrp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_HSRX_DATA_PRE_TIME_GRP0 to value 0"]
impl crate::Resettable for RgHsrxDataPreTimeGrp0Spec {}
