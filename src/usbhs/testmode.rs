#[doc = "Register `TESTMODE` reader"]
pub type R = crate::R<TestmodeSpec>;
#[doc = "Register `TESTMODE` writer"]
pub type W = crate::W<TestmodeSpec>;
#[doc = "Field `TEST_SE0_NAK` reader - Respond to any valid IN token with NAK."]
pub type TestSe0NakR = crate::BitReader;
#[doc = "Field `TEST_SE0_NAK` writer - Respond to any valid IN token with NAK."]
pub type TestSe0NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_J` reader - Force USB to continuous J state."]
pub type TestJR = crate::BitReader;
#[doc = "Field `TEST_J` writer - Force USB to continuous J state."]
pub type TestJW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_K` reader - Force USB to continuous K state."]
pub type TestKR = crate::BitReader;
#[doc = "Field `TEST_K` writer - Force USB to continuous K state."]
pub type TestKW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_PKT` reader - Transmit fixed test packet."]
pub type TestPktR = crate::BitReader;
#[doc = "Field `TEST_PKT` writer - Transmit fixed test packet."]
pub type TestPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HS` reader - Force USB to High-speed after reset."]
pub type ForceHsR = crate::BitReader;
#[doc = "Field `FORCE_HS` writer - Force USB to High-speed after reset."]
pub type ForceHsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_FS` reader - Force USB to Full-speed after reset."]
pub type ForceFsR = crate::BitReader;
#[doc = "Field `FORCE_FS` writer - Force USB to Full-speed after reset."]
pub type ForceFsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Respond to any valid IN token with NAK."]
    #[inline(always)]
    pub fn test_se0_nak(&self) -> TestSe0NakR {
        TestSe0NakR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force USB to continuous J state."]
    #[inline(always)]
    pub fn test_j(&self) -> TestJR {
        TestJR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force USB to continuous K state."]
    #[inline(always)]
    pub fn test_k(&self) -> TestKR {
        TestKR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit fixed test packet."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TestPktR {
        TestPktR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force USB to High-speed after reset."]
    #[inline(always)]
    pub fn force_hs(&self) -> ForceHsR {
        ForceHsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force USB to Full-speed after reset."]
    #[inline(always)]
    pub fn force_fs(&self) -> ForceFsR {
        ForceFsR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Respond to any valid IN token with NAK."]
    #[inline(always)]
    pub fn test_se0_nak(&mut self) -> TestSe0NakW<'_, TestmodeSpec> {
        TestSe0NakW::new(self, 0)
    }
    #[doc = "Bit 1 - Force USB to continuous J state."]
    #[inline(always)]
    pub fn test_j(&mut self) -> TestJW<'_, TestmodeSpec> {
        TestJW::new(self, 1)
    }
    #[doc = "Bit 2 - Force USB to continuous K state."]
    #[inline(always)]
    pub fn test_k(&mut self) -> TestKW<'_, TestmodeSpec> {
        TestKW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit fixed test packet."]
    #[inline(always)]
    pub fn test_pkt(&mut self) -> TestPktW<'_, TestmodeSpec> {
        TestPktW::new(self, 3)
    }
    #[doc = "Bit 4 - Force USB to High-speed after reset."]
    #[inline(always)]
    pub fn force_hs(&mut self) -> ForceHsW<'_, TestmodeSpec> {
        ForceHsW::new(self, 4)
    }
    #[doc = "Bit 5 - Force USB to Full-speed after reset."]
    #[inline(always)]
    pub fn force_fs(&mut self) -> ForceFsW<'_, TestmodeSpec> {
        ForceFsW::new(self, 5)
    }
}
#[doc = "USB 2.0 test mode enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`testmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestmodeSpec;
impl crate::RegisterSpec for TestmodeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`testmode::R`](R) reader structure"]
impl crate::Readable for TestmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`testmode::W`](W) writer structure"]
impl crate::Writable for TestmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TESTMODE to value 0"]
impl crate::Resettable for TestmodeSpec {}
