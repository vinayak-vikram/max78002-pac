#[doc = "Register `TS0` reader"]
pub type R = crate::R<Ts0Spec>;
#[doc = "Register `TS0` writer"]
pub type W = crate::W<Ts0Spec>;
#[doc = "Field `GAIN` reader - Unsigned gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
pub type GainR = crate::FieldReader<u16>;
#[doc = "Field `GAIN` writer - Unsigned gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Unsigned gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Unsigned gain for temp sensor normalization Temp degrees C = (ADC result * TS_GAIN) + TS_OFFSET."]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<'_, Ts0Spec> {
        GainW::new(self, 0)
    }
}
#[doc = "Temp Sensor trim0\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ts0Spec;
impl crate::RegisterSpec for Ts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts0::R`](R) reader structure"]
impl crate::Readable for Ts0Spec {}
#[doc = "`write(|w| ..)` method takes [`ts0::W`](W) writer structure"]
impl crate::Writable for Ts0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TS0 to value 0"]
impl crate::Resettable for Ts0Spec {}
