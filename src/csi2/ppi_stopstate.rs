#[doc = "Register `PPI_STOPSTATE` reader"]
pub type R = crate::R<PpiStopstateSpec>;
#[doc = "Register `PPI_STOPSTATE` writer"]
pub type W = crate::W<PpiStopstateSpec>;
#[doc = "Field `DL0STOP` reader - CSI Data Lane0 Stop State."]
pub type Dl0stopR = crate::BitReader;
#[doc = "Field `DL0STOP` writer - CSI Data Lane0 Stop State."]
pub type Dl0stopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1STOP` reader - CSI Data Lane1 Stop State."]
pub type Dl1stopR = crate::BitReader;
#[doc = "Field `DL1STOP` writer - CSI Data Lane1 Stop State."]
pub type Dl1stopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL0STOP` reader - CSI Clock Lane0 Stop State."]
pub type Cl0stopR = crate::BitReader;
#[doc = "Field `CL0STOP` writer - CSI Clock Lane0 Stop State."]
pub type Cl0stopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSI Data Lane0 Stop State."]
    #[inline(always)]
    pub fn dl0stop(&self) -> Dl0stopR {
        Dl0stopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSI Data Lane1 Stop State."]
    #[inline(always)]
    pub fn dl1stop(&self) -> Dl1stopR {
        Dl1stopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI Clock Lane0 Stop State."]
    #[inline(always)]
    pub fn cl0stop(&self) -> Cl0stopR {
        Cl0stopR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSI Data Lane0 Stop State."]
    #[inline(always)]
    pub fn dl0stop(&mut self) -> Dl0stopW<'_, PpiStopstateSpec> {
        Dl0stopW::new(self, 0)
    }
    #[doc = "Bit 1 - CSI Data Lane1 Stop State."]
    #[inline(always)]
    pub fn dl1stop(&mut self) -> Dl1stopW<'_, PpiStopstateSpec> {
        Dl1stopW::new(self, 1)
    }
    #[doc = "Bit 2 - CSI Clock Lane0 Stop State."]
    #[inline(always)]
    pub fn cl0stop(&mut self) -> Cl0stopW<'_, PpiStopstateSpec> {
        Cl0stopW::new(self, 2)
    }
}
#[doc = "DPHY PPI Stop State Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_stopstate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_stopstate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpiStopstateSpec;
impl crate::RegisterSpec for PpiStopstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppi_stopstate::R`](R) reader structure"]
impl crate::Readable for PpiStopstateSpec {}
#[doc = "`write(|w| ..)` method takes [`ppi_stopstate::W`](W) writer structure"]
impl crate::Writable for PpiStopstateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPI_STOPSTATE to value 0"]
impl crate::Resettable for PpiStopstateSpec {}
