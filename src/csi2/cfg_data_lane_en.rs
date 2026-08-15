#[doc = "Register `CFG_DATA_LANE_EN` reader"]
pub type R = crate::R<CfgDataLaneEnSpec>;
#[doc = "Register `CFG_DATA_LANE_EN` writer"]
pub type W = crate::W<CfgDataLaneEnSpec>;
#[doc = "Field `EN` reader - Enable data lane setting for controller."]
pub type EnR = crate::FieldReader;
#[doc = "Field `EN` writer - Enable data lane setting for controller."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Enable data lane setting for controller."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable data lane setting for controller."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CfgDataLaneEnSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "CFG_DATA_LANE_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_lane_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_lane_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDataLaneEnSpec;
impl crate::RegisterSpec for CfgDataLaneEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data_lane_en::R`](R) reader structure"]
impl crate::Readable for CfgDataLaneEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_data_lane_en::W`](W) writer structure"]
impl crate::Writable for CfgDataLaneEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DATA_LANE_EN to value 0"]
impl crate::Resettable for CfgDataLaneEnSpec {}
