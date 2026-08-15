#[doc = "Register `PPI_TURNAROUND_CFG` reader"]
pub type R = crate::R<PpiTurnaroundCfgSpec>;
#[doc = "Register `PPI_TURNAROUND_CFG` writer"]
pub type W = crate::W<PpiTurnaroundCfgSpec>;
#[doc = "Field `DL0TAREQ` reader - CSI Data Lane0 turn around request."]
pub type Dl0tareqR = crate::BitReader;
#[doc = "Field `DL0TAREQ` writer - CSI Data Lane0 turn around request."]
pub type Dl0tareqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0TADIS` reader - CSI Data Lane0 turn around disable."]
pub type Dl0tadisR = crate::BitReader;
#[doc = "Field `DL0TADIS` writer - CSI Data Lane0 turn around disable."]
pub type Dl0tadisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0FRCRX` reader - CSI Data Lane0 force RX mode."]
pub type Dl0frcrxR = crate::BitReader;
#[doc = "Field `DL0FRCRX` writer - CSI Data Lane0 force RX mode."]
pub type Dl0frcrxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSI Data Lane0 turn around request."]
    #[inline(always)]
    pub fn dl0tareq(&self) -> Dl0tareqR {
        Dl0tareqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSI Data Lane0 turn around disable."]
    #[inline(always)]
    pub fn dl0tadis(&self) -> Dl0tadisR {
        Dl0tadisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI Data Lane0 force RX mode."]
    #[inline(always)]
    pub fn dl0frcrx(&self) -> Dl0frcrxR {
        Dl0frcrxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSI Data Lane0 turn around request."]
    #[inline(always)]
    pub fn dl0tareq(&mut self) -> Dl0tareqW<'_, PpiTurnaroundCfgSpec> {
        Dl0tareqW::new(self, 0)
    }
    #[doc = "Bit 1 - CSI Data Lane0 turn around disable."]
    #[inline(always)]
    pub fn dl0tadis(&mut self) -> Dl0tadisW<'_, PpiTurnaroundCfgSpec> {
        Dl0tadisW::new(self, 1)
    }
    #[doc = "Bit 2 - CSI Data Lane0 force RX mode."]
    #[inline(always)]
    pub fn dl0frcrx(&mut self) -> Dl0frcrxW<'_, PpiTurnaroundCfgSpec> {
        Dl0frcrxW::new(self, 2)
    }
}
#[doc = "DPHY PPI Turn-Around Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_turnaround_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_turnaround_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiTurnaroundCfgSpec;
impl crate::RegisterSpec for PpiTurnaroundCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_turnaround_cfg::R`](R) reader structure"]
impl crate::Readable for PpiTurnaroundCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_turnaround_cfg::W`](W) writer structure"]
impl crate::Writable for PpiTurnaroundCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_TURNAROUND_CFG to value 0"]
impl crate::Resettable for PpiTurnaroundCfgSpec {}
