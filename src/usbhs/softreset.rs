#[doc = "Register `SOFTRESET` reader"]
pub type R = crate::R<SoftresetSpec>;
#[doc = "Register `SOFTRESET` writer"]
pub type W = crate::W<SoftresetSpec>;
#[doc = "Field `RSTS` reader - "]
pub type RstsR = crate::BitReader;
#[doc = "Field `RSTS` writer - "]
pub type RstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTXS` reader - "]
pub type RstxsR = crate::BitReader;
#[doc = "Field `RSTXS` writer - "]
pub type RstxsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsts(&self) -> RstsR {
        RstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstxs(&self) -> RstxsR {
        RstxsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsts(&mut self) -> RstsW<'_, SoftresetSpec> {
        RstsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstxs(&mut self) -> RstxsW<'_, SoftresetSpec> {
        RstxsW::new(self, 1)
    }
}
#[doc = "Software reset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`softreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftresetSpec;
impl crate::RegisterSpec for SoftresetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`softreset::R`](R) reader structure"]
impl crate::Readable for SoftresetSpec {}
#[doc = "`write(|w| ..)` method takes [`softreset::W`](W) writer structure"]
impl crate::Writable for SoftresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOFTRESET to value 0"]
impl crate::Resettable for SoftresetSpec {}
