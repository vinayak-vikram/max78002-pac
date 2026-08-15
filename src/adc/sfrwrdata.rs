#[doc = "Register `SFRWRDATA` reader"]
pub type R = crate::R<SfrwrdataSpec>;
#[doc = "Register `SFRWRDATA` writer"]
pub type W = crate::W<SfrwrdataSpec>;
#[doc = "Field `DATA` reader - DATA to SAR Digital"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - DATA to SAR Digital"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA to SAR Digital"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA to SAR Digital"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SfrwrdataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "SFR Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrwrdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrwrdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrwrdataSpec;
impl crate::RegisterSpec for SfrwrdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfrwrdata::R`](R) reader structure"]
impl crate::Readable for SfrwrdataSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrwrdata::W`](W) writer structure"]
impl crate::Writable for SfrwrdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRWRDATA to value 0"]
impl crate::Resettable for SfrwrdataSpec {}
