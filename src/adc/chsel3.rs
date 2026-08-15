#[doc = "Register `CHSEL3` reader"]
pub type R = crate::R<Chsel3Spec>;
#[doc = "Register `CHSEL3` writer"]
pub type W = crate::W<Chsel3Spec>;
#[doc = "Field `slot12_id` reader - channel assignment for slot 12."]
pub type Slot12IdR = crate::FieldReader;
#[doc = "Field `slot12_id` writer - channel assignment for slot 12."]
pub type Slot12IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot13_id` reader - channel assignment for slot 13."]
pub type Slot13IdR = crate::FieldReader;
#[doc = "Field `slot13_id` writer - channel assignment for slot 13."]
pub type Slot13IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot14_id` reader - channel assignment for slot 14."]
pub type Slot14IdR = crate::FieldReader;
#[doc = "Field `slot14_id` writer - channel assignment for slot 14."]
pub type Slot14IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot15_id` reader - channel assignment for slot 15."]
pub type Slot15IdR = crate::FieldReader;
#[doc = "Field `slot15_id` writer - channel assignment for slot 15."]
pub type Slot15IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 12."]
    #[inline(always)]
    pub fn slot12_id(&self) -> Slot12IdR {
        Slot12IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 13."]
    #[inline(always)]
    pub fn slot13_id(&self) -> Slot13IdR {
        Slot13IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 14."]
    #[inline(always)]
    pub fn slot14_id(&self) -> Slot14IdR {
        Slot14IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 15."]
    #[inline(always)]
    pub fn slot15_id(&self) -> Slot15IdR {
        Slot15IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 12."]
    #[inline(always)]
    pub fn slot12_id(&mut self) -> Slot12IdW<'_, Chsel3Spec> {
        Slot12IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 13."]
    #[inline(always)]
    pub fn slot13_id(&mut self) -> Slot13IdW<'_, Chsel3Spec> {
        Slot13IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 14."]
    #[inline(always)]
    pub fn slot14_id(&mut self) -> Slot14IdW<'_, Chsel3Spec> {
        Slot14IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 15."]
    #[inline(always)]
    pub fn slot15_id(&mut self) -> Slot15IdW<'_, Chsel3Spec> {
        Slot15IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel3Spec;
impl crate::RegisterSpec for Chsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel3::R`](R) reader structure"]
impl crate::Readable for Chsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel3::W`](W) writer structure"]
impl crate::Writable for Chsel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL3 to value 0"]
impl crate::Resettable for Chsel3Spec {}
