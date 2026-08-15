#[doc = "Register `CFG_NUM_LANES` reader"]
pub type R = crate::R<CfgNumLanesSpec>;
#[doc = "Register `CFG_NUM_LANES` writer"]
pub type W = crate::W<CfgNumLanesSpec>;
#[doc = "Field `LANES` reader - Num Lanes for RX controller."]
pub type LanesR = crate::FieldReader;
#[doc = "Field `LANES` writer - Num Lanes for RX controller."]
pub type LanesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Num Lanes for RX controller."]
    #[inline(always)]
    pub fn lanes(&self) -> LanesR {
        LanesR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Num Lanes for RX controller."]
    #[inline(always)]
    pub fn lanes(&mut self) -> LanesW<'_, CfgNumLanesSpec> {
        LanesW::new(self, 0)
    }
}
#[doc = "CFG_NUM_LANES.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_num_lanes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_num_lanes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgNumLanesSpec;
impl crate::RegisterSpec for CfgNumLanesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_num_lanes::R`](R) reader structure"]
impl crate::Readable for CfgNumLanesSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_num_lanes::W`](W) writer structure"]
impl crate::Writable for CfgNumLanesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_NUM_LANES to value 0"]
impl crate::Resettable for CfgNumLanesSpec {}
