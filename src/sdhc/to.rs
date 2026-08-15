#[doc = "Register `TO` reader"]
pub type R = crate::R<ToSpec>;
#[doc = "Register `TO` writer"]
pub type W = crate::W<ToSpec>;
#[doc = "Field `DATA_COUNT_VALUE` reader - Data Timeout Counter Value."]
pub type DataCountValueR = crate::FieldReader;
#[doc = "Field `DATA_COUNT_VALUE` writer - Data Timeout Counter Value."]
pub type DataCountValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Data Timeout Counter Value."]
    #[inline(always)]
    pub fn data_count_value(&self) -> DataCountValueR {
        DataCountValueR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Timeout Counter Value."]
    #[inline(always)]
    pub fn data_count_value(&mut self) -> DataCountValueW<'_, ToSpec> {
        DataCountValueW::new(self, 0)
    }
}
#[doc = "Timeout Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToSpec;
impl crate::RegisterSpec for ToSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`to::R`](R) reader structure"]
impl crate::Readable for ToSpec {}
#[doc = "`write(|w| ..)` method takes [`to::W`](W) writer structure"]
impl crate::Writable for ToSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TO to value 0"]
impl crate::Resettable for ToSpec {}
