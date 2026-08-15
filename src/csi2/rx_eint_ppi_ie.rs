#[doc = "Register `RX_EINT_PPI_IE` reader"]
pub type R = crate::R<RxEintPpiIeSpec>;
#[doc = "Register `RX_EINT_PPI_IE` writer"]
pub type W = crate::W<RxEintPpiIeSpec>;
#[doc = "Field `DL0STOP` reader - DPHY Data Lane0 Stop State (ppi_stopstate_lan0) interrupt enable."]
pub type Dl0stopR = crate::BitReader;
#[doc = "Field `DL0STOP` writer - DPHY Data Lane0 Stop State (ppi_stopstate_lan0) interrupt enable."]
pub type Dl0stopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1STOP` reader - DPHY Data Lane1 Stop State (ppi_stopstate_lan1) interrupt enable."]
pub type Dl1stopR = crate::BitReader;
#[doc = "Field `DL1STOP` writer - DPHY Data Lane1 Stop State (ppi_stopstate_lan1) interrupt enable."]
pub type Dl1stopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL0STOP` reader - DPHY Clock Lane0 Stop State (ppi_stopstate_clk0) interrupt enable."]
pub type Cl0stopR = crate::BitReader;
#[doc = "Field `CL0STOP` writer - DPHY Clock Lane0 Stop State (ppi_stopstate_clk0) interrupt enable."]
pub type Cl0stopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ECONT0` reader - DPHY Data Lane0 LP0 Contention Error (ppi_errcontentionp0_lan0) interrupt enable"]
pub type Dl0econt0R = crate::BitReader;
#[doc = "Field `DL0ECONT0` writer - DPHY Data Lane0 LP0 Contention Error (ppi_errcontentionp0_lan0) interrupt enable"]
pub type Dl0econt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ECONT1` reader - DPHY Data Lane0 LP1 Contention Error (ppi_errcontentionp1_lan0) interrupt enable"]
pub type Dl0econt1R = crate::BitReader;
#[doc = "Field `DL0ECONT1` writer - DPHY Data Lane0 LP1 Contention Error (ppi_errcontentionp1_lan0) interrupt enable"]
pub type Dl0econt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ESOT` reader - DPHY Data Lane0 Start-of-Transmission (SoT) Error (ppi_errsoths_lan0) interrupt enable"]
pub type Dl0esotR = crate::BitReader;
#[doc = "Field `DL0ESOT` writer - DPHY Data Lane0 Start-of-Transmission (SoT) Error (ppi_errsoths_lan0) interrupt enable"]
pub type Dl0esotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1ESOT` reader - DPHY Data Lane1 Start-of-Transmission (SoT) Error (ppi_errsoths_lan1) interrupt enable"]
pub type Dl1esotR = crate::BitReader;
#[doc = "Field `DL1ESOT` writer - DPHY Data Lane1 Start-of-Transmission (SoT) Error (ppi_errsoths_lan1) interrupt enable"]
pub type Dl1esotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ESOTS` reader - DPHY Data Lane0 SOT Synchronization Error (ppi_errsotsynchs_lan0) interrupt enable"]
pub type Dl0esotsR = crate::BitReader;
#[doc = "Field `DL0ESOTS` writer - DPHY Data Lane0 SOT Synchronization Error (ppi_errsotsynchs_lan0) interrupt enable"]
pub type Dl0esotsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1ESOTS` reader - DPHY Data Lane1 SOT Synchronization Error (ppi_errsotsynchs_lan1) interrupt enable"]
pub type Dl1esotsR = crate::BitReader;
#[doc = "Field `DL1ESOTS` writer - DPHY Data Lane1 SOT Synchronization Error (ppi_errsotsynchs_lan1) interrupt enable"]
pub type Dl1esotsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0EESC` reader - DPHY Data Lane0 Escape Entry Error (ppi_erresc_lan0) interrupt enable"]
pub type Dl0eescR = crate::BitReader;
#[doc = "Field `DL0EESC` writer - DPHY Data Lane0 Escape Entry Error (ppi_erresc_lan0) interrupt enable"]
pub type Dl0eescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1EESC` reader - DPHY Data Lane1 Escape Entry Error (ppi_erresc_lan1) interrupt enable"]
pub type Dl1eescR = crate::BitReader;
#[doc = "Field `DL1EESC` writer - DPHY Data Lane1 Escape Entry Error (ppi_erresc_lan1) interrupt enable"]
pub type Dl1eescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ESESC` reader - DPHY Data Lane0 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
pub type Dl0esescR = crate::BitReader;
#[doc = "Field `DL0ESESC` writer - DPHY Data Lane0 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
pub type Dl0esescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1ESESC` reader - DPHY Data Lane1 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
pub type Dl1esescR = crate::BitReader;
#[doc = "Field `DL1ESESC` writer - DPHY Data Lane1 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
pub type Dl1esescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ECTL` reader - DPHY Data Lane0 Control Error (ppi_errcontrol_lan0) interrupt enable"]
pub type Dl0ectlR = crate::BitReader;
#[doc = "Field `DL0ECTL` writer - DPHY Data Lane0 Control Error (ppi_errcontrol_lan0) interrupt enable"]
pub type Dl0ectlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1ECTL` reader - DPHY Data Lane1 Control Error (ppi_errcontrol_lan0) interrupt enable"]
pub type Dl1ectlR = crate::BitReader;
#[doc = "Field `DL1ECTL` writer - DPHY Data Lane1 Control Error (ppi_errcontrol_lan0) interrupt enable"]
pub type Dl1ectlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DPHY Data Lane0 Stop State (ppi_stopstate_lan0) interrupt enable."]
    #[inline(always)]
    pub fn dl0stop(&self) -> Dl0stopR {
        Dl0stopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DPHY Data Lane1 Stop State (ppi_stopstate_lan1) interrupt enable."]
    #[inline(always)]
    pub fn dl1stop(&self) -> Dl1stopR {
        Dl1stopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DPHY Clock Lane0 Stop State (ppi_stopstate_clk0) interrupt enable."]
    #[inline(always)]
    pub fn cl0stop(&self) -> Cl0stopR {
        Cl0stopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DPHY Data Lane0 LP0 Contention Error (ppi_errcontentionp0_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0econt0(&self) -> Dl0econt0R {
        Dl0econt0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DPHY Data Lane0 LP1 Contention Error (ppi_errcontentionp1_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0econt1(&self) -> Dl0econt1R {
        Dl0econt1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DPHY Data Lane0 Start-of-Transmission (SoT) Error (ppi_errsoths_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0esot(&self) -> Dl0esotR {
        Dl0esotR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DPHY Data Lane1 Start-of-Transmission (SoT) Error (ppi_errsoths_lan1) interrupt enable"]
    #[inline(always)]
    pub fn dl1esot(&self) -> Dl1esotR {
        Dl1esotR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - DPHY Data Lane0 SOT Synchronization Error (ppi_errsotsynchs_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0esots(&self) -> Dl0esotsR {
        Dl0esotsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DPHY Data Lane1 SOT Synchronization Error (ppi_errsotsynchs_lan1) interrupt enable"]
    #[inline(always)]
    pub fn dl1esots(&self) -> Dl1esotsR {
        Dl1esotsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DPHY Data Lane0 Escape Entry Error (ppi_erresc_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0eesc(&self) -> Dl0eescR {
        Dl0eescR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPHY Data Lane1 Escape Entry Error (ppi_erresc_lan1) interrupt enable"]
    #[inline(always)]
    pub fn dl1eesc(&self) -> Dl1eescR {
        Dl1eescR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - DPHY Data Lane0 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0esesc(&self) -> Dl0esescR {
        Dl0esescR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DPHY Data Lane1 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl1esesc(&self) -> Dl1esescR {
        Dl1esescR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - DPHY Data Lane0 Control Error (ppi_errcontrol_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0ectl(&self) -> Dl0ectlR {
        Dl0ectlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DPHY Data Lane1 Control Error (ppi_errcontrol_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl1ectl(&self) -> Dl1ectlR {
        Dl1ectlR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPHY Data Lane0 Stop State (ppi_stopstate_lan0) interrupt enable."]
    #[inline(always)]
    pub fn dl0stop(&mut self) -> Dl0stopW<'_, RxEintPpiIeSpec> {
        Dl0stopW::new(self, 0)
    }
    #[doc = "Bit 1 - DPHY Data Lane1 Stop State (ppi_stopstate_lan1) interrupt enable."]
    #[inline(always)]
    pub fn dl1stop(&mut self) -> Dl1stopW<'_, RxEintPpiIeSpec> {
        Dl1stopW::new(self, 1)
    }
    #[doc = "Bit 4 - DPHY Clock Lane0 Stop State (ppi_stopstate_clk0) interrupt enable."]
    #[inline(always)]
    pub fn cl0stop(&mut self) -> Cl0stopW<'_, RxEintPpiIeSpec> {
        Cl0stopW::new(self, 4)
    }
    #[doc = "Bit 6 - DPHY Data Lane0 LP0 Contention Error (ppi_errcontentionp0_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0econt0(&mut self) -> Dl0econt0W<'_, RxEintPpiIeSpec> {
        Dl0econt0W::new(self, 6)
    }
    #[doc = "Bit 7 - DPHY Data Lane0 LP1 Contention Error (ppi_errcontentionp1_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0econt1(&mut self) -> Dl0econt1W<'_, RxEintPpiIeSpec> {
        Dl0econt1W::new(self, 7)
    }
    #[doc = "Bit 8 - DPHY Data Lane0 Start-of-Transmission (SoT) Error (ppi_errsoths_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0esot(&mut self) -> Dl0esotW<'_, RxEintPpiIeSpec> {
        Dl0esotW::new(self, 8)
    }
    #[doc = "Bit 9 - DPHY Data Lane1 Start-of-Transmission (SoT) Error (ppi_errsoths_lan1) interrupt enable"]
    #[inline(always)]
    pub fn dl1esot(&mut self) -> Dl1esotW<'_, RxEintPpiIeSpec> {
        Dl1esotW::new(self, 9)
    }
    #[doc = "Bit 12 - DPHY Data Lane0 SOT Synchronization Error (ppi_errsotsynchs_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0esots(&mut self) -> Dl0esotsW<'_, RxEintPpiIeSpec> {
        Dl0esotsW::new(self, 12)
    }
    #[doc = "Bit 13 - DPHY Data Lane1 SOT Synchronization Error (ppi_errsotsynchs_lan1) interrupt enable"]
    #[inline(always)]
    pub fn dl1esots(&mut self) -> Dl1esotsW<'_, RxEintPpiIeSpec> {
        Dl1esotsW::new(self, 13)
    }
    #[doc = "Bit 16 - DPHY Data Lane0 Escape Entry Error (ppi_erresc_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0eesc(&mut self) -> Dl0eescW<'_, RxEintPpiIeSpec> {
        Dl0eescW::new(self, 16)
    }
    #[doc = "Bit 17 - DPHY Data Lane1 Escape Entry Error (ppi_erresc_lan1) interrupt enable"]
    #[inline(always)]
    pub fn dl1eesc(&mut self) -> Dl1eescW<'_, RxEintPpiIeSpec> {
        Dl1eescW::new(self, 17)
    }
    #[doc = "Bit 20 - DPHY Data Lane0 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0esesc(&mut self) -> Dl0esescW<'_, RxEintPpiIeSpec> {
        Dl0esescW::new(self, 20)
    }
    #[doc = "Bit 21 - DPHY Data Lane1 Low-Power Data Transmission Synchronization Error (ppi_errsyncesc_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl1esesc(&mut self) -> Dl1esescW<'_, RxEintPpiIeSpec> {
        Dl1esescW::new(self, 21)
    }
    #[doc = "Bit 24 - DPHY Data Lane0 Control Error (ppi_errcontrol_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl0ectl(&mut self) -> Dl0ectlW<'_, RxEintPpiIeSpec> {
        Dl0ectlW::new(self, 24)
    }
    #[doc = "Bit 25 - DPHY Data Lane1 Control Error (ppi_errcontrol_lan0) interrupt enable"]
    #[inline(always)]
    pub fn dl1ectl(&mut self) -> Dl1ectlW<'_, RxEintPpiIeSpec> {
        Dl1ectlW::new(self, 25)
    }
}
#[doc = "RX D-PHY Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_ppi_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_ppi_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxEintPpiIeSpec;
impl crate::RegisterSpec for RxEintPpiIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_eint_ppi_ie::R`](R) reader structure"]
impl crate::Readable for RxEintPpiIeSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_eint_ppi_ie::W`](W) writer structure"]
impl crate::Writable for RxEintPpiIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_EINT_PPI_IE to value 0"]
impl crate::Resettable for RxEintPpiIeSpec {}
