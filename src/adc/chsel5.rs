#[doc = "Register `CHSEL5` reader"]
pub type R = crate::R<Chsel5Spec>;
#[doc = "Register `CHSEL5` writer"]
pub type W = crate::W<Chsel5Spec>;
#[doc = "Field `slot20_id` reader - channel assignment for slot 20."]
pub type Slot20IdR = crate::FieldReader;
#[doc = "Field `slot20_id` writer - channel assignment for slot 20."]
pub type Slot20IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot21_id` reader - channel assignment for slot 21."]
pub type Slot21IdR = crate::FieldReader;
#[doc = "Field `slot21_id` writer - channel assignment for slot 21."]
pub type Slot21IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot22_id` reader - channel assignment for slot 22."]
pub type Slot22IdR = crate::FieldReader;
#[doc = "Field `slot22_id` writer - channel assignment for slot 22."]
pub type Slot22IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot23_id` reader - channel assignment for slot 23."]
pub type Slot23IdR = crate::FieldReader;
#[doc = "Field `slot23_id` writer - channel assignment for slot 23."]
pub type Slot23IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 20."]
    #[inline(always)]
    pub fn slot20_id(&self) -> Slot20IdR {
        Slot20IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 21."]
    #[inline(always)]
    pub fn slot21_id(&self) -> Slot21IdR {
        Slot21IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 22."]
    #[inline(always)]
    pub fn slot22_id(&self) -> Slot22IdR {
        Slot22IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 23."]
    #[inline(always)]
    pub fn slot23_id(&self) -> Slot23IdR {
        Slot23IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 20."]
    #[inline(always)]
    pub fn slot20_id(&mut self) -> Slot20IdW<'_, Chsel5Spec> {
        Slot20IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 21."]
    #[inline(always)]
    pub fn slot21_id(&mut self) -> Slot21IdW<'_, Chsel5Spec> {
        Slot21IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 22."]
    #[inline(always)]
    pub fn slot22_id(&mut self) -> Slot22IdW<'_, Chsel5Spec> {
        Slot22IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 23."]
    #[inline(always)]
    pub fn slot23_id(&mut self) -> Slot23IdW<'_, Chsel5Spec> {
        Slot23IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel5Spec;
impl crate::RegisterSpec for Chsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel5::R`](R) reader structure"]
impl crate::Readable for Chsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel5::W`](W) writer structure"]
impl crate::Writable for Chsel5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL5 to value 0"]
impl crate::Resettable for Chsel5Spec {}
