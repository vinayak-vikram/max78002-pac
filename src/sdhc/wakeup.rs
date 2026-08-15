#[doc = "Register `WAKEUP` reader"]
pub type R = crate::R<WakeupSpec>;
#[doc = "Register `WAKEUP` writer"]
pub type W = crate::W<WakeupSpec>;
#[doc = "Field `CARD_INT` reader - Wakeup Event Enable On Card Interrupt."]
pub type CardIntR = crate::BitReader;
#[doc = "Field `CARD_INT` writer - Wakeup Event Enable On Card Interrupt."]
pub type CardIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INS` reader - Wakeup Event Enable On SD Card Insertion."]
pub type CardInsR = crate::BitReader;
#[doc = "Field `CARD_INS` writer - Wakeup Event Enable On SD Card Insertion."]
pub type CardInsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REM` reader - Wakeup Event Enable On SD Card Removal."]
pub type CardRemR = crate::BitReader;
#[doc = "Field `CARD_REM` writer - Wakeup Event Enable On SD Card Removal."]
pub type CardRemW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt."]
    #[inline(always)]
    pub fn card_int(&self) -> CardIntR {
        CardIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion."]
    #[inline(always)]
    pub fn card_ins(&self) -> CardInsR {
        CardInsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal."]
    #[inline(always)]
    pub fn card_rem(&self) -> CardRemR {
        CardRemR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt."]
    #[inline(always)]
    pub fn card_int(&mut self) -> CardIntW<'_, WakeupSpec> {
        CardIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion."]
    #[inline(always)]
    pub fn card_ins(&mut self) -> CardInsW<'_, WakeupSpec> {
        CardInsW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal."]
    #[inline(always)]
    pub fn card_rem(&mut self) -> CardRemW<'_, WakeupSpec> {
        CardRemW::new(self, 2)
    }
}
#[doc = "Wakeup Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WakeupSpec;
impl crate::RegisterSpec for WakeupSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wakeup::R`](R) reader structure"]
impl crate::Readable for WakeupSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeup::W`](W) writer structure"]
impl crate::Writable for WakeupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP to value 0"]
impl crate::Resettable for WakeupSpec {}
