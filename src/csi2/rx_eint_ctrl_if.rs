#[doc = "Register `RX_EINT_CTRL_IF` reader"]
pub type R = crate::R<RxEintCtrlIfSpec>;
#[doc = "Register `RX_EINT_CTRL_IF` writer"]
pub type W = crate::W<RxEintCtrlIfSpec>;
#[doc = "Field `EECC2` reader - CSI RX ECC 2-bit Error interrupt flag."]
pub type Eecc2R = crate::BitReader;
#[doc = "Field `EECC2` writer - CSI RX ECC 2-bit Error interrupt flag."]
pub type Eecc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EECC1` reader - CSI RX ECC 1-bit Error interrupt flag."]
pub type Eecc1R = crate::BitReader;
#[doc = "Field `EECC1` writer - CSI RX ECC 1-bit Error interrupt flag."]
pub type Eecc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECRC` reader - CSI RX CRC Error interrupt flag."]
pub type EcrcR = crate::BitReader;
#[doc = "Field `ECRC` writer - CSI RX CRC Error interrupt flag."]
pub type EcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EID` reader - CSI RX Packet Header Data ID Error interrupt flag"]
pub type EidR = crate::BitReader;
#[doc = "Field `EID` writer - CSI RX Packet Header Data ID Error interrupt flag"]
pub type EidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTFFOV` reader - CSI RX Packet FIFO Overrun interrupt flag"]
pub type PktffovR = crate::BitReader;
#[doc = "Field `PKTFFOV` writer - CSI RX Packet FIFO Overrun interrupt flag"]
pub type PktffovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ULPSA` reader - CSI Data Lane0 ULPSS Active interrupt flag"]
pub type Dl0ulpsaR = crate::BitReader;
#[doc = "Field `DL0ULPSA` writer - CSI Data Lane0 ULPSS Active interrupt flag"]
pub type Dl0ulpsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1ULPSA` reader - CSI Data Lane1 ULPSS Active interrupt flag"]
pub type Dl1ulpsaR = crate::BitReader;
#[doc = "Field `DL1ULPSA` writer - CSI Data Lane1 ULPSS Active interrupt flag"]
pub type Dl1ulpsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL0ULPSM` reader - CSI Data Lane0 ULPSS Mark interrupt flag"]
pub type Dl0ulpsmR = crate::BitReader;
#[doc = "Field `DL0ULPSM` writer - CSI Data Lane0 ULPSS Mark interrupt flag"]
pub type Dl0ulpsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL1ULPSM` reader - CSI Data Lane1 ULPSS Mark interrupt flag"]
pub type Dl1ulpsmR = crate::BitReader;
#[doc = "Field `DL1ULPSM` writer - CSI Data Lane1 ULPSS Mark interrupt flag"]
pub type Dl1ulpsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL0ULPSA` reader - CSI Clock Lane0 ULPSS Active interrupt flag"]
pub type Cl0ulpsaR = crate::BitReader;
#[doc = "Field `CL0ULPSA` writer - CSI Clock Lane0 ULPSS Active interrupt flag"]
pub type Cl0ulpsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CL0ULPSM` reader - CSI Data Lane0 ULPSS Mark interrupt flag"]
pub type Cl0ulpsmR = crate::BitReader;
#[doc = "Field `CL0ULPSM` writer - CSI Data Lane0 ULPSS Mark interrupt flag"]
pub type Cl0ulpsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSI RX ECC 2-bit Error interrupt flag."]
    #[inline(always)]
    pub fn eecc2(&self) -> Eecc2R {
        Eecc2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSI RX ECC 1-bit Error interrupt flag."]
    #[inline(always)]
    pub fn eecc1(&self) -> Eecc1R {
        Eecc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CSI RX CRC Error interrupt flag."]
    #[inline(always)]
    pub fn ecrc(&self) -> EcrcR {
        EcrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CSI RX Packet Header Data ID Error interrupt flag"]
    #[inline(always)]
    pub fn eid(&self) -> EidR {
        EidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI RX Packet FIFO Overrun interrupt flag"]
    #[inline(always)]
    pub fn pktffov(&self) -> PktffovR {
        PktffovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI Data Lane0 ULPSS Active interrupt flag"]
    #[inline(always)]
    pub fn dl0ulpsa(&self) -> Dl0ulpsaR {
        Dl0ulpsaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI Data Lane1 ULPSS Active interrupt flag"]
    #[inline(always)]
    pub fn dl1ulpsa(&self) -> Dl1ulpsaR {
        Dl1ulpsaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - CSI Data Lane0 ULPSS Mark interrupt flag"]
    #[inline(always)]
    pub fn dl0ulpsm(&self) -> Dl0ulpsmR {
        Dl0ulpsmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CSI Data Lane1 ULPSS Mark interrupt flag"]
    #[inline(always)]
    pub fn dl1ulpsm(&self) -> Dl1ulpsmR {
        Dl1ulpsmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CSI Clock Lane0 ULPSS Active interrupt flag"]
    #[inline(always)]
    pub fn cl0ulpsa(&self) -> Cl0ulpsaR {
        Cl0ulpsaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CSI Data Lane0 ULPSS Mark interrupt flag"]
    #[inline(always)]
    pub fn cl0ulpsm(&self) -> Cl0ulpsmR {
        Cl0ulpsmR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSI RX ECC 2-bit Error interrupt flag."]
    #[inline(always)]
    pub fn eecc2(&mut self) -> Eecc2W<'_, RxEintCtrlIfSpec> {
        Eecc2W::new(self, 0)
    }
    #[doc = "Bit 1 - CSI RX ECC 1-bit Error interrupt flag."]
    #[inline(always)]
    pub fn eecc1(&mut self) -> Eecc1W<'_, RxEintCtrlIfSpec> {
        Eecc1W::new(self, 1)
    }
    #[doc = "Bit 2 - CSI RX CRC Error interrupt flag."]
    #[inline(always)]
    pub fn ecrc(&mut self) -> EcrcW<'_, RxEintCtrlIfSpec> {
        EcrcW::new(self, 2)
    }
    #[doc = "Bit 3 - CSI RX Packet Header Data ID Error interrupt flag"]
    #[inline(always)]
    pub fn eid(&mut self) -> EidW<'_, RxEintCtrlIfSpec> {
        EidW::new(self, 3)
    }
    #[doc = "Bit 4 - CSI RX Packet FIFO Overrun interrupt flag"]
    #[inline(always)]
    pub fn pktffov(&mut self) -> PktffovW<'_, RxEintCtrlIfSpec> {
        PktffovW::new(self, 4)
    }
    #[doc = "Bit 8 - CSI Data Lane0 ULPSS Active interrupt flag"]
    #[inline(always)]
    pub fn dl0ulpsa(&mut self) -> Dl0ulpsaW<'_, RxEintCtrlIfSpec> {
        Dl0ulpsaW::new(self, 8)
    }
    #[doc = "Bit 9 - CSI Data Lane1 ULPSS Active interrupt flag"]
    #[inline(always)]
    pub fn dl1ulpsa(&mut self) -> Dl1ulpsaW<'_, RxEintCtrlIfSpec> {
        Dl1ulpsaW::new(self, 9)
    }
    #[doc = "Bit 12 - CSI Data Lane0 ULPSS Mark interrupt flag"]
    #[inline(always)]
    pub fn dl0ulpsm(&mut self) -> Dl0ulpsmW<'_, RxEintCtrlIfSpec> {
        Dl0ulpsmW::new(self, 12)
    }
    #[doc = "Bit 13 - CSI Data Lane1 ULPSS Mark interrupt flag"]
    #[inline(always)]
    pub fn dl1ulpsm(&mut self) -> Dl1ulpsmW<'_, RxEintCtrlIfSpec> {
        Dl1ulpsmW::new(self, 13)
    }
    #[doc = "Bit 16 - CSI Clock Lane0 ULPSS Active interrupt flag"]
    #[inline(always)]
    pub fn cl0ulpsa(&mut self) -> Cl0ulpsaW<'_, RxEintCtrlIfSpec> {
        Cl0ulpsaW::new(self, 16)
    }
    #[doc = "Bit 17 - CSI Data Lane0 ULPSS Mark interrupt flag"]
    #[inline(always)]
    pub fn cl0ulpsm(&mut self) -> Cl0ulpsmW<'_, RxEintCtrlIfSpec> {
        Cl0ulpsmW::new(self, 17)
    }
}
#[doc = "RX Controller Interrupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_ctrl_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_ctrl_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxEintCtrlIfSpec;
impl crate::RegisterSpec for RxEintCtrlIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_eint_ctrl_if::R`](R) reader structure"]
impl crate::Readable for RxEintCtrlIfSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_eint_ctrl_if::W`](W) writer structure"]
impl crate::Writable for RxEintCtrlIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_EINT_CTRL_IF to value 0"]
impl crate::Resettable for RxEintCtrlIfSpec {}
