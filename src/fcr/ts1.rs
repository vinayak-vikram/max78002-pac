#[doc = "Register `TS1` reader"]
pub type R = crate::R<Ts1Spec>;
#[doc = "Register `TS1` writer"]
pub type W = crate::W<Ts1Spec>;
#[doc = "Field `OFFSET` reader - Signed gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
pub type OffsetR = crate::FieldReader<u16>;
#[doc = "Field `OFFSET` writer - Signed gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
pub type OffsetW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TS_OFFSET_SIGN` reader - Sign extension of TS_OFFSET\\[13:0\\]"]
pub type TsOffsetSignR = crate::FieldReader<u32>;
#[doc = "Field `TS_OFFSET_SIGN` writer - Sign extension of TS_OFFSET\\[13:0\\]"]
pub type TsOffsetSignW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:13 - Signed gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - Sign extension of TS_OFFSET\\[13:0\\]"]
    #[inline(always)]
    pub fn ts_offset_sign(&self) -> TsOffsetSignR {
        TsOffsetSignR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - Signed gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
    #[inline(always)]
    pub fn offset(&mut self) -> OffsetW<'_, Ts1Spec> {
        OffsetW::new(self, 0)
    }
    #[doc = "Bits 14:31 - Sign extension of TS_OFFSET\\[13:0\\]"]
    #[inline(always)]
    pub fn ts_offset_sign(&mut self) -> TsOffsetSignW<'_, Ts1Spec> {
        TsOffsetSignW::new(self, 14)
    }
}
#[doc = "Temp Sensor trim1\n\nYou can [`read`](crate::Reg::read) this register and get [`ts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts1Spec;
impl crate::RegisterSpec for Ts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts1::R`](R) reader structure"]
impl crate::Readable for Ts1Spec {}
#[doc = "`write(|w| ..)` method takes [`ts1::W`](W) writer structure"]
impl crate::Writable for Ts1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TS1 to value 0"]
impl crate::Resettable for Ts1Spec {}
