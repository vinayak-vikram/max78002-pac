#[doc = "Register `CHSEL7` reader"]
pub type R = crate::R<Chsel7Spec>;
#[doc = "Register `CHSEL7` writer"]
pub type W = crate::W<Chsel7Spec>;
#[doc = "Field `slot28_id` reader - channel assignment for slot 28."]
pub type Slot28IdR = crate::FieldReader;
#[doc = "Field `slot28_id` writer - channel assignment for slot 28."]
pub type Slot28IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot29_id` reader - channel assignment for slot 29."]
pub type Slot29IdR = crate::FieldReader;
#[doc = "Field `slot29_id` writer - channel assignment for slot 29."]
pub type Slot29IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot30_id` reader - channel assignment for slot 30."]
pub type Slot30IdR = crate::FieldReader;
#[doc = "Field `slot30_id` writer - channel assignment for slot 30."]
pub type Slot30IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot31_id` reader - channel assignment for slot 31."]
pub type Slot31IdR = crate::FieldReader;
#[doc = "Field `slot31_id` writer - channel assignment for slot 31."]
pub type Slot31IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 28."]
    #[inline(always)]
    pub fn slot28_id(&self) -> Slot28IdR {
        Slot28IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 29."]
    #[inline(always)]
    pub fn slot29_id(&self) -> Slot29IdR {
        Slot29IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 30."]
    #[inline(always)]
    pub fn slot30_id(&self) -> Slot30IdR {
        Slot30IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 31."]
    #[inline(always)]
    pub fn slot31_id(&self) -> Slot31IdR {
        Slot31IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 28."]
    #[inline(always)]
    pub fn slot28_id(&mut self) -> Slot28IdW<'_, Chsel7Spec> {
        Slot28IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 29."]
    #[inline(always)]
    pub fn slot29_id(&mut self) -> Slot29IdW<'_, Chsel7Spec> {
        Slot29IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 30."]
    #[inline(always)]
    pub fn slot30_id(&mut self) -> Slot30IdW<'_, Chsel7Spec> {
        Slot30IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 31."]
    #[inline(always)]
    pub fn slot31_id(&mut self) -> Slot31IdW<'_, Chsel7Spec> {
        Slot31IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel7Spec;
impl crate::RegisterSpec for Chsel7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel7::R`](R) reader structure"]
impl crate::Readable for Chsel7Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel7::W`](W) writer structure"]
impl crate::Writable for Chsel7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL7 to value 0"]
impl crate::Resettable for Chsel7Spec {}
