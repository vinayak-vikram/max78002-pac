#[doc = "Register `MAX_CURR_CFG` reader"]
pub type R = crate::R<MaxCurrCfgSpec>;
#[doc = "Field `V3_3` reader - Maximum Current for 3.3V."]
pub type V3_3R = crate::FieldReader;
#[doc = "Field `V3_0` reader - Maximum Current for 3.0V."]
pub type V3_0R = crate::FieldReader;
#[doc = "Field `V1_8` reader - Maximum Current for 1.8V."]
pub type V1_8R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V."]
    #[inline(always)]
    pub fn v3_3(&self) -> V3_3R {
        V3_3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V."]
    #[inline(always)]
    pub fn v3_0(&self) -> V3_0R {
        V3_0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V."]
    #[inline(always)]
    pub fn v1_8(&self) -> V1_8R {
        V1_8R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities.\n\nYou can [`read`](crate::Reg::read) this register and get [`max_curr_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxCurrCfgSpec;
impl crate::RegisterSpec for MaxCurrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_curr_cfg::R`](R) reader structure"]
impl crate::Readable for MaxCurrCfgSpec {}
#[doc = "`reset()` method sets MAX_CURR_CFG to value 0"]
impl crate::Resettable for MaxCurrCfgSpec {}
