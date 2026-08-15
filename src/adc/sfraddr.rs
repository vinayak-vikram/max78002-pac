#[doc = "Register `SFRADDR` reader"]
pub type R = crate::R<SfraddrSpec>;
#[doc = "Register `SFRADDR` writer"]
pub type W = crate::W<SfraddrSpec>;
#[doc = "Field `ADDR` reader - Address to SAR Digital"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address to SAR Digital"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Address to SAR Digital"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address to SAR Digital"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SfraddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "SFR Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfraddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfraddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfraddrSpec;
impl crate::RegisterSpec for SfraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfraddr::R`](R) reader structure"]
impl crate::Readable for SfraddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfraddr::W`](W) writer structure"]
impl crate::Writable for SfraddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRADDR to value 0"]
impl crate::Resettable for SfraddrSpec {}
