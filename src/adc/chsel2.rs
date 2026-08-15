#[doc = "Register `CHSEL2` reader"]
pub type R = crate::R<Chsel2Spec>;
#[doc = "Register `CHSEL2` writer"]
pub type W = crate::W<Chsel2Spec>;
#[doc = "Field `slot8_id` reader - channel assignment for slot 8."]
pub type Slot8IdR = crate::FieldReader;
#[doc = "Field `slot8_id` writer - channel assignment for slot 8."]
pub type Slot8IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot9_id` reader - channel assignment for slot 9."]
pub type Slot9IdR = crate::FieldReader;
#[doc = "Field `slot9_id` writer - channel assignment for slot 9."]
pub type Slot9IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot10_id` reader - channel assignment for slot 10."]
pub type Slot10IdR = crate::FieldReader;
#[doc = "Field `slot10_id` writer - channel assignment for slot 10."]
pub type Slot10IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot11_id` reader - channel assignment for slot 11."]
pub type Slot11IdR = crate::FieldReader;
#[doc = "Field `slot11_id` writer - channel assignment for slot 11."]
pub type Slot11IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 8."]
    #[inline(always)]
    pub fn slot8_id(&self) -> Slot8IdR {
        Slot8IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 9."]
    #[inline(always)]
    pub fn slot9_id(&self) -> Slot9IdR {
        Slot9IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 10."]
    #[inline(always)]
    pub fn slot10_id(&self) -> Slot10IdR {
        Slot10IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 11."]
    #[inline(always)]
    pub fn slot11_id(&self) -> Slot11IdR {
        Slot11IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 8."]
    #[inline(always)]
    pub fn slot8_id(&mut self) -> Slot8IdW<'_, Chsel2Spec> {
        Slot8IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 9."]
    #[inline(always)]
    pub fn slot9_id(&mut self) -> Slot9IdW<'_, Chsel2Spec> {
        Slot9IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 10."]
    #[inline(always)]
    pub fn slot10_id(&mut self) -> Slot10IdW<'_, Chsel2Spec> {
        Slot10IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 11."]
    #[inline(always)]
    pub fn slot11_id(&mut self) -> Slot11IdW<'_, Chsel2Spec> {
        Slot11IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel2Spec;
impl crate::RegisterSpec for Chsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel2::R`](R) reader structure"]
impl crate::Readable for Chsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel2::W`](W) writer structure"]
impl crate::Writable for Chsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL2 to value 0"]
impl crate::Resettable for Chsel2Spec {}
