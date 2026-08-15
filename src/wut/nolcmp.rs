#[doc = "Register `NOLCMP` reader"]
pub type R = crate::R<NolcmpSpec>;
#[doc = "Register `NOLCMP` writer"]
pub type W = crate::W<NolcmpSpec>;
#[doc = "Field `NOLLCMP` reader - Non Overlaping Low Compare."]
pub type NollcmpR = crate::FieldReader;
#[doc = "Field `NOLLCMP` writer - Non Overlaping Low Compare."]
pub type NollcmpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NOLHCMP` reader - Non Overlaping High Compare."]
pub type NolhcmpR = crate::FieldReader;
#[doc = "Field `NOLHCMP` writer - Non Overlaping High Compare."]
pub type NolhcmpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Non Overlaping Low Compare."]
    #[inline(always)]
    pub fn nollcmp(&self) -> NollcmpR {
        NollcmpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Non Overlaping High Compare."]
    #[inline(always)]
    pub fn nolhcmp(&self) -> NolhcmpR {
        NolhcmpR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Non Overlaping Low Compare."]
    #[inline(always)]
    pub fn nollcmp(&mut self) -> NollcmpW<'_, NolcmpSpec> {
        NollcmpW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Non Overlaping High Compare."]
    #[inline(always)]
    pub fn nolhcmp(&mut self) -> NolhcmpW<'_, NolcmpSpec> {
        NolhcmpW::new(self, 8)
    }
}
#[doc = "Non Overlaping Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nolcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nolcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NolcmpSpec;
impl crate::RegisterSpec for NolcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nolcmp::R`](R) reader structure"]
impl crate::Readable for NolcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`nolcmp::W`](W) writer structure"]
impl crate::Writable for NolcmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NOLCMP to value 0"]
impl crate::Resettable for NolcmpSpec {}
