#[doc = "Register `CFG_DATABUS16_SEL` reader"]
pub type R = crate::R<CfgDatabus16SelSpec>;
#[doc = "Register `CFG_DATABUS16_SEL` writer"]
pub type W = crate::W<CfgDatabus16SelSpec>;
#[doc = "Field `EN` reader - Enable 16-bit data bus."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable 16-bit data bus."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable 16-bit data bus."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable 16-bit data bus."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CfgDatabus16SelSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "CFG_DATABUS16_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_databus16_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_databus16_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDatabus16SelSpec;
impl crate::RegisterSpec for CfgDatabus16SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_databus16_sel::R`](R) reader structure"]
impl crate::Readable for CfgDatabus16SelSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_databus16_sel::W`](W) writer structure"]
impl crate::Writable for CfgDatabus16SelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DATABUS16_SEL to value 0"]
impl crate::Resettable for CfgDatabus16SelSpec {}
