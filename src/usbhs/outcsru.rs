#[doc = "Register `OUTCSRU` reader"]
pub type R = crate::R<OutcsruSpec>;
#[doc = "Register `OUTCSRU` writer"]
pub type W = crate::W<OutcsruSpec>;
#[doc = "Field `INCOMPRX` reader - "]
pub type IncomprxR = crate::BitReader;
#[doc = "Field `DPKTBUFDIS` reader - "]
pub type DpktbufdisR = crate::BitReader;
#[doc = "Field `DPKTBUFDIS` writer - "]
pub type DpktbufdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISNYET` reader - "]
pub type DisnyetR = crate::BitReader;
#[doc = "Field `DISNYET` writer - "]
pub type DisnyetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISO` reader - "]
pub type IsoR = crate::BitReader;
#[doc = "Field `ISO` writer - "]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCLEAR` reader - "]
pub type AutoclearR = crate::BitReader;
#[doc = "Field `AUTOCLEAR` writer - "]
pub type AutoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn incomprx(&self) -> IncomprxR {
        IncomprxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dpktbufdis(&self) -> DpktbufdisR {
        DpktbufdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disnyet(&self) -> DisnyetR {
        DisnyetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn autoclear(&self) -> AutoclearR {
        AutoclearR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dpktbufdis(&mut self) -> DpktbufdisW<'_, OutcsruSpec> {
        DpktbufdisW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn disnyet(&mut self) -> DisnyetW<'_, OutcsruSpec> {
        DisnyetW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn iso(&mut self) -> IsoW<'_, OutcsruSpec> {
        IsoW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn autoclear(&mut self) -> AutoclearW<'_, OutcsruSpec> {
        AutoclearW::new(self, 7)
    }
}
#[doc = "Control status upper register for OUTx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outcsru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcsru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutcsruSpec;
impl crate::RegisterSpec for OutcsruSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`outcsru::R`](R) reader structure"]
impl crate::Readable for OutcsruSpec {}
#[doc = "`write(|w| ..)` method takes [`outcsru::W`](W) writer structure"]
impl crate::Writable for OutcsruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTCSRU to value 0"]
impl crate::Resettable for OutcsruSpec {}
