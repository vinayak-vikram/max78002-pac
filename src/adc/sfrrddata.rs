#[doc = "Register `SFRRDDATA` reader"]
pub type R = crate::R<SfrrddataSpec>;
#[doc = "Register `SFRRDDATA` writer"]
pub type W = crate::W<SfrrddataSpec>;
#[doc = "Field `DATA` reader - DATA from SAR Digital"]
pub type DataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DATA from SAR Digital"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "SFR Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrrddata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrrddata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrrddataSpec;
impl crate::RegisterSpec for SfrrddataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfrrddata::R`](R) reader structure"]
impl crate::Readable for SfrrddataSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrrddata::W`](W) writer structure"]
impl crate::Writable for SfrrddataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRRDDATA to value 0"]
impl crate::Resettable for SfrrddataSpec {}
