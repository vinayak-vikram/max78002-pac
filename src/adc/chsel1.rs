#[doc = "Register `CHSEL1` reader"]
pub type R = crate::R<Chsel1Spec>;
#[doc = "Register `CHSEL1` writer"]
pub type W = crate::W<Chsel1Spec>;
#[doc = "Field `slot4_id` reader - channel assignment for slot 4."]
pub type Slot4IdR = crate::FieldReader;
#[doc = "Field `slot4_id` writer - channel assignment for slot 4."]
pub type Slot4IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot5_id` reader - channel assignment for slot 5."]
pub type Slot5IdR = crate::FieldReader;
#[doc = "Field `slot5_id` writer - channel assignment for slot 5."]
pub type Slot5IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot6_id` reader - channel assignment for slot 6."]
pub type Slot6IdR = crate::FieldReader;
#[doc = "Field `slot6_id` writer - channel assignment for slot 6."]
pub type Slot6IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot7_id` reader - channel assignment for slot 7."]
pub type Slot7IdR = crate::FieldReader;
#[doc = "Field `slot7_id` writer - channel assignment for slot 7."]
pub type Slot7IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 4."]
    #[inline(always)]
    pub fn slot4_id(&self) -> Slot4IdR {
        Slot4IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 5."]
    #[inline(always)]
    pub fn slot5_id(&self) -> Slot5IdR {
        Slot5IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 6."]
    #[inline(always)]
    pub fn slot6_id(&self) -> Slot6IdR {
        Slot6IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 7."]
    #[inline(always)]
    pub fn slot7_id(&self) -> Slot7IdR {
        Slot7IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 4."]
    #[inline(always)]
    pub fn slot4_id(&mut self) -> Slot4IdW<'_, Chsel1Spec> {
        Slot4IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 5."]
    #[inline(always)]
    pub fn slot5_id(&mut self) -> Slot5IdW<'_, Chsel1Spec> {
        Slot5IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 6."]
    #[inline(always)]
    pub fn slot6_id(&mut self) -> Slot6IdW<'_, Chsel1Spec> {
        Slot6IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 7."]
    #[inline(always)]
    pub fn slot7_id(&mut self) -> Slot7IdW<'_, Chsel1Spec> {
        Slot7IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel1Spec;
impl crate::RegisterSpec for Chsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel1::R`](R) reader structure"]
impl crate::Readable for Chsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel1::W`](W) writer structure"]
impl crate::Writable for Chsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL1 to value 0"]
impl crate::Resettable for Chsel1Spec {}
