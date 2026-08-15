#[doc = "Register `ADMA_ER` reader"]
pub type R = crate::R<AdmaErSpec>;
#[doc = "Register `ADMA_ER` writer"]
pub type W = crate::W<AdmaErSpec>;
#[doc = "Field `STATE` reader - ADMA Error State."]
pub type StateR = crate::FieldReader;
#[doc = "Field `STATE` writer - ADMA Error State."]
pub type StateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEN_MISMATCH` reader - ADMA Length Mismatch Error."]
pub type LenMismatchR = crate::BitReader;
#[doc = "Field `LEN_MISMATCH` writer - ADMA Length Mismatch Error."]
pub type LenMismatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - ADMA Error State."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error."]
    #[inline(always)]
    pub fn len_mismatch(&self) -> LenMismatchR {
        LenMismatchR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMA Error State."]
    #[inline(always)]
    pub fn state(&mut self) -> StateW<'_, AdmaErSpec> {
        StateW::new(self, 0)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error."]
    #[inline(always)]
    pub fn len_mismatch(&mut self) -> LenMismatchW<'_, AdmaErSpec> {
        LenMismatchW::new(self, 2)
    }
}
#[doc = "ADMA Error Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaErSpec;
impl crate::RegisterSpec for AdmaErSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adma_er::R`](R) reader structure"]
impl crate::Readable for AdmaErSpec {}
#[doc = "`write(|w| ..)` method takes [`adma_er::W`](W) writer structure"]
impl crate::Writable for AdmaErSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADMA_ER to value 0"]
impl crate::Resettable for AdmaErSpec {}
