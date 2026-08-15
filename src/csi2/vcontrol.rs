#[doc = "Register `VCONTROL` reader"]
pub type R = crate::R<VcontrolSpec>;
#[doc = "Register `VCONTROL` writer"]
pub type W = crate::W<VcontrolSpec>;
#[doc = "Field `NORMAL_MODE` reader - NORMAL_MODE."]
pub type NormalModeR = crate::BitReader;
#[doc = "Field `NORMAL_MODE` writer - NORMAL_MODE."]
pub type NormalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_RX_DC_TEST` reader - LP_RX_DC_TEST."]
pub type LpRxDcTestR = crate::BitReader;
#[doc = "Field `LP_RX_DC_TEST` writer - LP_RX_DC_TEST."]
pub type LpRxDcTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_RX_DC_1` reader - LP_RX_DC_1."]
pub type LpRxDc1R = crate::BitReader;
#[doc = "Field `LP_RX_DC_1` writer - LP_RX_DC_1."]
pub type LpRxDc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_RX_DC_0` reader - LP_RX_DC_0."]
pub type LpRxDc0R = crate::BitReader;
#[doc = "Field `LP_RX_DC_0` writer - LP_RX_DC_0."]
pub type LpRxDc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_SEN_1` reader - CAL_SEN_1."]
pub type CalSen1R = crate::BitReader;
#[doc = "Field `CAL_SEN_1` writer - CAL_SEN_1."]
pub type CalSen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL_SEN_0` reader - CAL_SEN_0."]
pub type CalSen0R = crate::BitReader;
#[doc = "Field `CAL_SEN_0` writer - CAL_SEN_0."]
pub type CalSen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRT_0` reader - HSRT_0."]
pub type Hsrt0R = crate::BitReader;
#[doc = "Field `HSRT_0` writer - HSRT_0."]
pub type Hsrt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRT_1` reader - HSRT_1."]
pub type Hsrt1R = crate::BitReader;
#[doc = "Field `HSRT_1` writer - HSRT_1."]
pub type Hsrt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_RX_PARTBERT` reader - LP_RX_PARTBERT."]
pub type LpRxPartbertR = crate::BitReader;
#[doc = "Field `LP_RX_PARTBERT` writer - LP_RX_PARTBERT."]
pub type LpRxPartbertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_INT_LOOPBACK` reader - HS_INT_LOOPBACK."]
pub type HsIntLoopbackR = crate::BitReader;
#[doc = "Field `HS_INT_LOOPBACK` writer - HS_INT_LOOPBACK."]
pub type HsIntLoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_RX_PARTBERT` reader - HS_RX_PARTBERT."]
pub type HsRxPartbertR = crate::BitReader;
#[doc = "Field `HS_RX_PARTBERT` writer - HS_RX_PARTBERT."]
pub type HsRxPartbertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_RX_PRBS9` reader - HS_RX_PRBS9."]
pub type HsRxPrbs9R = crate::BitReader;
#[doc = "Field `HS_RX_PRBS9` writer - HS_RX_PRBS9."]
pub type HsRxPrbs9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPEND_MODE` reader - SUSPEND_MODE."]
pub type SuspendModeR = crate::BitReader;
#[doc = "Field `SUSPEND_MODE` writer - SUSPEND_MODE."]
pub type SuspendModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NORMAL_MODE."]
    #[inline(always)]
    pub fn normal_mode(&self) -> NormalModeR {
        NormalModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LP_RX_DC_TEST."]
    #[inline(always)]
    pub fn lp_rx_dc_test(&self) -> LpRxDcTestR {
        LpRxDcTestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LP_RX_DC_1."]
    #[inline(always)]
    pub fn lp_rx_dc_1(&self) -> LpRxDc1R {
        LpRxDc1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP_RX_DC_0."]
    #[inline(always)]
    pub fn lp_rx_dc_0(&self) -> LpRxDc0R {
        LpRxDc0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CAL_SEN_1."]
    #[inline(always)]
    pub fn cal_sen_1(&self) -> CalSen1R {
        CalSen1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CAL_SEN_0."]
    #[inline(always)]
    pub fn cal_sen_0(&self) -> CalSen0R {
        CalSen0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - HSRT_0."]
    #[inline(always)]
    pub fn hsrt_0(&self) -> Hsrt0R {
        Hsrt0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HSRT_1."]
    #[inline(always)]
    pub fn hsrt_1(&self) -> Hsrt1R {
        Hsrt1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - LP_RX_PARTBERT."]
    #[inline(always)]
    pub fn lp_rx_partbert(&self) -> LpRxPartbertR {
        LpRxPartbertR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HS_INT_LOOPBACK."]
    #[inline(always)]
    pub fn hs_int_loopback(&self) -> HsIntLoopbackR {
        HsIntLoopbackR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 27 - HS_RX_PARTBERT."]
    #[inline(always)]
    pub fn hs_rx_partbert(&self) -> HsRxPartbertR {
        HsRxPartbertR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - HS_RX_PRBS9."]
    #[inline(always)]
    pub fn hs_rx_prbs9(&self) -> HsRxPrbs9R {
        HsRxPrbs9R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - SUSPEND_MODE."]
    #[inline(always)]
    pub fn suspend_mode(&self) -> SuspendModeR {
        SuspendModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NORMAL_MODE."]
    #[inline(always)]
    pub fn normal_mode(&mut self) -> NormalModeW<'_, VcontrolSpec> {
        NormalModeW::new(self, 0)
    }
    #[doc = "Bit 1 - LP_RX_DC_TEST."]
    #[inline(always)]
    pub fn lp_rx_dc_test(&mut self) -> LpRxDcTestW<'_, VcontrolSpec> {
        LpRxDcTestW::new(self, 1)
    }
    #[doc = "Bit 2 - LP_RX_DC_1."]
    #[inline(always)]
    pub fn lp_rx_dc_1(&mut self) -> LpRxDc1W<'_, VcontrolSpec> {
        LpRxDc1W::new(self, 2)
    }
    #[doc = "Bit 3 - LP_RX_DC_0."]
    #[inline(always)]
    pub fn lp_rx_dc_0(&mut self) -> LpRxDc0W<'_, VcontrolSpec> {
        LpRxDc0W::new(self, 3)
    }
    #[doc = "Bit 4 - CAL_SEN_1."]
    #[inline(always)]
    pub fn cal_sen_1(&mut self) -> CalSen1W<'_, VcontrolSpec> {
        CalSen1W::new(self, 4)
    }
    #[doc = "Bit 5 - CAL_SEN_0."]
    #[inline(always)]
    pub fn cal_sen_0(&mut self) -> CalSen0W<'_, VcontrolSpec> {
        CalSen0W::new(self, 5)
    }
    #[doc = "Bit 7 - HSRT_0."]
    #[inline(always)]
    pub fn hsrt_0(&mut self) -> Hsrt0W<'_, VcontrolSpec> {
        Hsrt0W::new(self, 7)
    }
    #[doc = "Bit 8 - HSRT_1."]
    #[inline(always)]
    pub fn hsrt_1(&mut self) -> Hsrt1W<'_, VcontrolSpec> {
        Hsrt1W::new(self, 8)
    }
    #[doc = "Bit 10 - LP_RX_PARTBERT."]
    #[inline(always)]
    pub fn lp_rx_partbert(&mut self) -> LpRxPartbertW<'_, VcontrolSpec> {
        LpRxPartbertW::new(self, 10)
    }
    #[doc = "Bit 11 - HS_INT_LOOPBACK."]
    #[inline(always)]
    pub fn hs_int_loopback(&mut self) -> HsIntLoopbackW<'_, VcontrolSpec> {
        HsIntLoopbackW::new(self, 11)
    }
    #[doc = "Bit 27 - HS_RX_PARTBERT."]
    #[inline(always)]
    pub fn hs_rx_partbert(&mut self) -> HsRxPartbertW<'_, VcontrolSpec> {
        HsRxPartbertW::new(self, 27)
    }
    #[doc = "Bit 28 - HS_RX_PRBS9."]
    #[inline(always)]
    pub fn hs_rx_prbs9(&mut self) -> HsRxPrbs9W<'_, VcontrolSpec> {
        HsRxPrbs9W::new(self, 28)
    }
    #[doc = "Bit 31 - SUSPEND_MODE."]
    #[inline(always)]
    pub fn suspend_mode(&mut self) -> SuspendModeW<'_, VcontrolSpec> {
        SuspendModeW::new(self, 31)
    }
}
#[doc = "PMA_RDY.\n\nYou can [`read`](crate::Reg::read) this register and get [`vcontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VcontrolSpec;
impl crate::RegisterSpec for VcontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vcontrol::R`](R) reader structure"]
impl crate::Readable for VcontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`vcontrol::W`](W) writer structure"]
impl crate::Writable for VcontrolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VCONTROL to value 0"]
impl crate::Resettable for VcontrolSpec {}
