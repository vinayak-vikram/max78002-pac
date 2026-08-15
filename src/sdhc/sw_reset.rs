#[doc = "Register `SW_RESET` reader"]
pub type R = crate::R<SwResetSpec>;
#[doc = "Register `SW_RESET` writer"]
pub type W = crate::W<SwResetSpec>;
#[doc = "Field `RESET_ALL` reader - Software Reset For All."]
pub type ResetAllR = crate::BitReader;
#[doc = "Field `RESET_ALL` writer - Software Reset For All."]
pub type ResetAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_CMD` reader - Software Reset For CMD Line."]
pub type ResetCmdR = crate::BitReader;
#[doc = "Field `RESET_CMD` writer - Software Reset For CMD Line."]
pub type ResetCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_DAT` reader - Software Reset For DAT Line."]
pub type ResetDatR = crate::BitReader;
#[doc = "Field `RESET_DAT` writer - Software Reset For DAT Line."]
pub type ResetDatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset For All."]
    #[inline(always)]
    pub fn reset_all(&self) -> ResetAllR {
        ResetAllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line."]
    #[inline(always)]
    pub fn reset_cmd(&self) -> ResetCmdR {
        ResetCmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line."]
    #[inline(always)]
    pub fn reset_dat(&self) -> ResetDatR {
        ResetDatR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All."]
    #[inline(always)]
    pub fn reset_all(&mut self) -> ResetAllW<'_, SwResetSpec> {
        ResetAllW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD Line."]
    #[inline(always)]
    pub fn reset_cmd(&mut self) -> ResetCmdW<'_, SwResetSpec> {
        ResetCmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset For DAT Line."]
    #[inline(always)]
    pub fn reset_dat(&mut self) -> ResetDatW<'_, SwResetSpec> {
        ResetDatW::new(self, 2)
    }
}
#[doc = "Software Reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwResetSpec;
impl crate::RegisterSpec for SwResetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sw_reset::R`](R) reader structure"]
impl crate::Readable for SwResetSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_reset::W`](W) writer structure"]
impl crate::Writable for SwResetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_RESET to value 0"]
impl crate::Resettable for SwResetSpec {}
