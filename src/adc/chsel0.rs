#[doc = "Register `CHSEL0` reader"]
pub type R = crate::R<Chsel0Spec>;
#[doc = "Register `CHSEL0` writer"]
pub type W = crate::W<Chsel0Spec>;
#[doc = "Field `slot0_id` reader - channel assignment for slot 0."]
pub type Slot0IdR = crate::FieldReader;
#[doc = "Field `slot0_id` writer - channel assignment for slot 0."]
pub type Slot0IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot1_id` reader - channel assignment for slot 1."]
pub type Slot1IdR = crate::FieldReader;
#[doc = "Field `slot1_id` writer - channel assignment for slot 1."]
pub type Slot1IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot2_id` reader - channel assignment for slot 2."]
pub type Slot2IdR = crate::FieldReader;
#[doc = "Field `slot2_id` writer - channel assignment for slot 2."]
pub type Slot2IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot3_id` reader - channel assignment for slot 3."]
pub type Slot3IdR = crate::FieldReader;
#[doc = "Field `slot3_id` writer - channel assignment for slot 3."]
pub type Slot3IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 0."]
    #[inline(always)]
    pub fn slot0_id(&self) -> Slot0IdR {
        Slot0IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 1."]
    #[inline(always)]
    pub fn slot1_id(&self) -> Slot1IdR {
        Slot1IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 2."]
    #[inline(always)]
    pub fn slot2_id(&self) -> Slot2IdR {
        Slot2IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 3."]
    #[inline(always)]
    pub fn slot3_id(&self) -> Slot3IdR {
        Slot3IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 0."]
    #[inline(always)]
    pub fn slot0_id(&mut self) -> Slot0IdW<'_, Chsel0Spec> {
        Slot0IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 1."]
    #[inline(always)]
    pub fn slot1_id(&mut self) -> Slot1IdW<'_, Chsel0Spec> {
        Slot1IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 2."]
    #[inline(always)]
    pub fn slot2_id(&mut self) -> Slot2IdW<'_, Chsel0Spec> {
        Slot2IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 3."]
    #[inline(always)]
    pub fn slot3_id(&mut self) -> Slot3IdW<'_, Chsel0Spec> {
        Slot3IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel0Spec;
impl crate::RegisterSpec for Chsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel0::R`](R) reader structure"]
impl crate::Readable for Chsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel0::W`](W) writer structure"]
impl crate::Writable for Chsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL0 to value 0"]
impl crate::Resettable for Chsel0Spec {}
