#[doc = "Register `LCNT` reader"]
pub type R = crate::R<LcntSpec>;
#[doc = "Register `LCNT` writer"]
pub type W = crate::W<LcntSpec>;
#[doc = "Field `last` reader - Index of the last layer to execute."]
pub type LastR = crate::FieldReader;
#[doc = "Field `last` writer - Index of the last layer to execute."]
pub type LastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `start` reader - Index of the first layer to execute."]
pub type StartR = crate::FieldReader;
#[doc = "Field `start` writer - Index of the first layer to execute."]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Index of the last layer to execute."]
    #[inline(always)]
    pub fn last(&self) -> LastR {
        LastR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Index of the first layer to execute."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Index of the last layer to execute."]
    #[inline(always)]
    pub fn last(&mut self) -> LastW<'_, LcntSpec> {
        LastW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Index of the first layer to execute."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, LcntSpec> {
        StartW::new(self, 8)
    }
}
#[doc = "Layer count.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcntSpec;
impl crate::RegisterSpec for LcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcnt::R`](R) reader structure"]
impl crate::Readable for LcntSpec {}
#[doc = "`write(|w| ..)` method takes [`lcnt::W`](W) writer structure"]
impl crate::Writable for LcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCNT to value 0"]
impl crate::Resettable for LcntSpec {}
