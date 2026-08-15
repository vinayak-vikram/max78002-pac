#[doc = "Register `CHSEL4` reader"]
pub type R = crate::R<Chsel4Spec>;
#[doc = "Register `CHSEL4` writer"]
pub type W = crate::W<Chsel4Spec>;
#[doc = "Field `slot16_id` reader - channel assignment for slot 16."]
pub type Slot16IdR = crate::FieldReader;
#[doc = "Field `slot16_id` writer - channel assignment for slot 16."]
pub type Slot16IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot17_id` reader - channel assignment for slot 17."]
pub type Slot17IdR = crate::FieldReader;
#[doc = "Field `slot17_id` writer - channel assignment for slot 17."]
pub type Slot17IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot18_id` reader - channel assignment for slot 18."]
pub type Slot18IdR = crate::FieldReader;
#[doc = "Field `slot18_id` writer - channel assignment for slot 18."]
pub type Slot18IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `slot19_id` reader - channel assignment for slot 19."]
pub type Slot19IdR = crate::FieldReader;
#[doc = "Field `slot19_id` writer - channel assignment for slot 19."]
pub type Slot19IdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - channel assignment for slot 16."]
    #[inline(always)]
    pub fn slot16_id(&self) -> Slot16IdR {
        Slot16IdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 17."]
    #[inline(always)]
    pub fn slot17_id(&self) -> Slot17IdR {
        Slot17IdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 18."]
    #[inline(always)]
    pub fn slot18_id(&self) -> Slot18IdR {
        Slot18IdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 19."]
    #[inline(always)]
    pub fn slot19_id(&self) -> Slot19IdR {
        Slot19IdR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - channel assignment for slot 16."]
    #[inline(always)]
    pub fn slot16_id(&mut self) -> Slot16IdW<'_, Chsel4Spec> {
        Slot16IdW::new(self, 0)
    }
    #[doc = "Bits 8:12 - channel assignment for slot 17."]
    #[inline(always)]
    pub fn slot17_id(&mut self) -> Slot17IdW<'_, Chsel4Spec> {
        Slot17IdW::new(self, 8)
    }
    #[doc = "Bits 16:20 - channel assignment for slot 18."]
    #[inline(always)]
    pub fn slot18_id(&mut self) -> Slot18IdW<'_, Chsel4Spec> {
        Slot18IdW::new(self, 16)
    }
    #[doc = "Bits 24:28 - channel assignment for slot 19."]
    #[inline(always)]
    pub fn slot19_id(&mut self) -> Slot19IdW<'_, Chsel4Spec> {
        Slot19IdW::new(self, 24)
    }
}
#[doc = "Channel Select Register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chsel4Spec;
impl crate::RegisterSpec for Chsel4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chsel4::R`](R) reader structure"]
impl crate::Readable for Chsel4Spec {}
#[doc = "`write(|w| ..)` method takes [`chsel4::W`](W) writer structure"]
impl crate::Writable for Chsel4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSEL4 to value 0"]
impl crate::Resettable for Chsel4Spec {}
