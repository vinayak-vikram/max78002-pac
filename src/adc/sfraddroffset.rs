#[doc = "Register `SFRADDROFFSET` reader"]
pub type R = crate::R<SfraddroffsetSpec>;
#[doc = "Register `SFRADDROFFSET` writer"]
pub type W = crate::W<SfraddroffsetSpec>;
#[doc = "Field `OFFSET` reader - Address Offset for SAR Digital"]
pub type OffsetR = crate::FieldReader;
#[doc = "Field `OFFSET` writer - Address Offset for SAR Digital"]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Address Offset for SAR Digital"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address Offset for SAR Digital"]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<'_, SfraddroffsetSpec> {
        OffsetW::new(self, 0)
    }
}
#[doc = "SFR Address Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfraddroffset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfraddroffset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfraddroffsetSpec;
impl crate::RegisterSpec for SfraddroffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfraddroffset::R`](R) reader structure"]
impl crate::Readable for SfraddroffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`sfraddroffset::W`](W) writer structure"]
impl crate::Writable for SfraddroffsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRADDROFFSET to value 0"]
impl crate::Resettable for SfraddroffsetSpec {}
