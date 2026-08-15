#[doc = "Register `CHSEL6` reader"]
pub type R = crate::R<Chsel6Spec>;
#[doc = "Register `CHSEL6` writer"]
pub type W = crate::W<Chsel6Spec>;
#[doc = "Field `slot24_id` reader - channel assignment for slot 24."]
pub type Slot24IdR = crate::FieldReader;
#[doc = "Field `slot24_id` writer - channel assignment for slot 24."]
pub type Slot24IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot25_id` reader - channel assignment for slot 25."]
pub type Slot25IdR = crate::FieldReader;
#[doc = "Field `slot25_id` writer - channel assignment for slot 25."]
pub type Slot25IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot26_id` reader - channel assignment for slot 26."]
pub type Slot26IdR = crate::FieldReader;
#[doc = "Field `slot26_id` writer - channel assignment for slot 26."]
pub type Slot26IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot27_id` reader - channel assignment for slot 27."]
pub type Slot27IdR = crate::FieldReader;
#[doc = "Field `slot27_id` writer - channel assignment for slot 27."]
pub type Slot27IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 24."]
    #[inline(always)]
    pub fn slot24_id(&self) -> Slot24IdR {
        Slot24IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 25."]
    #[inline(always)]
    pub fn slot25_id(&self) -> Slot25IdR {
        Slot25IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 26."]
    #[inline(always)]
    pub fn slot26_id(&self) -> Slot26IdR {
        Slot26IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 27."]
    #[inline(always)]
    pub fn slot27_id(&self) -> Slot27IdR {
        Slot27IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 24."]
    #[inline(always)]
    pub fn slot24_id(&mut self) -> Slot24IdW<'_, Chsel6Spec> {
        Slot24IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 25."]
    #[inline(always)]
    pub fn slot25_id(&mut self) -> Slot25IdW<'_, Chsel6Spec> {
        Slot25IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 26."]
    #[inline(always)]
    pub fn slot26_id(&mut self) -> Slot26IdW<'_, Chsel6Spec> {
        Slot26IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 27."]
    #[inline(always)]
    pub fn slot27_id(&mut self) -> Slot27IdW<'_, Chsel6Spec> {
        Slot27IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel6Spec;
impl crate::RegisterSpec for Chsel6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel6::R`](R) reader structure"]
impl crate::Readable for Chsel6Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel6::W`](W) writer structure"]
impl crate::Writable for Chsel6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL6 to value 0"]
impl crate::Resettable for Chsel6Spec {}
