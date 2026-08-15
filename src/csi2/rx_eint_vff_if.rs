#[doc = "Register `RX_EINT_VFF_IF` reader"]
pub type R = crate::R<RxEintVffIfSpec>;
#[doc = "Register `RX_EINT_VFF_IF` writer"]
pub type W = crate::W<RxEintVffIfSpec>;
#[doc = "Field `FNEMPTY` reader - Video FIFO not empty interrupt flag."]
pub type FnemptyR = crate::BitReader;
#[doc = "Field `FNEMPTY` writer - Video FIFO not empty interrupt flag."]
pub type FnemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHD` reader - Video FIFO above threshold interrupt flag."]
pub type FthdR = crate::BitReader;
#[doc = "Field `FTHD` writer - Video FIFO above threshold interrupt flag."]
pub type FthdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFULL` reader - Video FIFO full interrupt flag."]
pub type FfullR = crate::BitReader;
#[doc = "Field `FFULL` writer - Video FIFO full interrupt flag."]
pub type FfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` reader - Video FIFO underrun interrupt flag"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Video FIFO underrun interrupt flag"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Video FIFO overrun interrupt flag"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Video FIFO overrun interrupt flag"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSYNC` reader - CSI out of sync interrupt flag"]
pub type OutsyncR = crate::BitReader;
#[doc = "Field `OUTSYNC` writer - CSI out of sync interrupt flag"]
pub type OutsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMTERR` reader - CSI Pixel Format Error interrupt flag"]
pub type FmterrR = crate::BitReader;
#[doc = "Field `FMTERR` writer - CSI Pixel Format Error interrupt flag"]
pub type FmterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBWTO` reader - AHB wait time out interrupt flag"]
pub type AhbwtoR = crate::BitReader;
#[doc = "Field `AHBWTO` writer - AHB wait time out interrupt flag"]
pub type AhbwtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - CSI Frame Start interrupt flag"]
pub type FsR = crate::BitReader;
#[doc = "Field `FS` writer - CSI Frame Start interrupt flag"]
pub type FsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - CSI Frame End interrupt flag"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - CSI Frame End interrupt flag"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - CSI Line Start interrupt flag"]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - CSI Line Start interrupt flag"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - CSI Line End interrupt flag"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - CSI Line End interrupt flag"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_OVR` reader - Raw FIFO Overrun Interrupt Enable"]
pub type RawOvrR = crate::BitReader;
#[doc = "Field `RAW_OVR` writer - Raw FIFO Overrun Interrupt Enable"]
pub type RawOvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_AHBERR` reader - Raw AHB Error Interrupt Enable"]
pub type RawAhberrR = crate::BitReader;
#[doc = "Field `RAW_AHBERR` writer - Raw AHB Error Interrupt Enable"]
pub type RawAhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_RDTO` reader - AHBM_RDTO"]
pub type AhbmRdtoR = crate::BitReader;
#[doc = "Field `AHBM_RDTO` writer - AHBM_RDTO"]
pub type AhbmRdtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_IDTO` reader - AHBM_IDTO"]
pub type AhbmIdtoR = crate::BitReader;
#[doc = "Field `AHBM_IDTO` writer - AHBM_IDTO"]
pub type AhbmIdtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBM_MAX` reader - AHBM_MAX"]
pub type AhbmMaxR = crate::BitReader;
#[doc = "Field `AHBM_MAX` writer - AHBM_MAX"]
pub type AhbmMaxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Video FIFO not empty interrupt flag."]
    #[inline(always)]
    pub fn fnempty(&self) -> FnemptyR {
        FnemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Video FIFO above threshold interrupt flag."]
    #[inline(always)]
    pub fn fthd(&self) -> FthdR {
        FthdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Video FIFO full interrupt flag."]
    #[inline(always)]
    pub fn ffull(&self) -> FfullR {
        FfullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Video FIFO underrun interrupt flag"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Video FIFO overrun interrupt flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CSI out of sync interrupt flag"]
    #[inline(always)]
    pub fn outsync(&self) -> OutsyncR {
        OutsyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSI Pixel Format Error interrupt flag"]
    #[inline(always)]
    pub fn fmterr(&self) -> FmterrR {
        FmterrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB wait time out interrupt flag"]
    #[inline(always)]
    pub fn ahbwto(&self) -> AhbwtoR {
        AhbwtoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI Frame Start interrupt flag"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI Frame End interrupt flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CSI Line Start interrupt flag"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CSI Line End interrupt flag"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn raw_ovr(&self) -> RawOvrR {
        RawOvrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw AHB Error Interrupt Enable"]
    #[inline(always)]
    pub fn raw_ahberr(&self) -> RawAhberrR {
        RawAhberrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - AHBM_RDTO"]
    #[inline(always)]
    pub fn ahbm_rdto(&self) -> AhbmRdtoR {
        AhbmRdtoR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AHBM_IDTO"]
    #[inline(always)]
    pub fn ahbm_idto(&self) -> AhbmIdtoR {
        AhbmIdtoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AHBM_MAX"]
    #[inline(always)]
    pub fn ahbm_max(&self) -> AhbmMaxR {
        AhbmMaxR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Video FIFO not empty interrupt flag."]
    #[inline(always)]
    pub fn fnempty(&mut self) -> FnemptyW<'_, RxEintVffIfSpec> {
        FnemptyW::new(self, 0)
    }
    #[doc = "Bit 1 - Video FIFO above threshold interrupt flag."]
    #[inline(always)]
    pub fn fthd(&mut self) -> FthdW<'_, RxEintVffIfSpec> {
        FthdW::new(self, 1)
    }
    #[doc = "Bit 2 - Video FIFO full interrupt flag."]
    #[inline(always)]
    pub fn ffull(&mut self) -> FfullW<'_, RxEintVffIfSpec> {
        FfullW::new(self, 2)
    }
    #[doc = "Bit 3 - Video FIFO underrun interrupt flag"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<'_, RxEintVffIfSpec> {
        UnderrunW::new(self, 3)
    }
    #[doc = "Bit 4 - Video FIFO overrun interrupt flag"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, RxEintVffIfSpec> {
        OverrunW::new(self, 4)
    }
    #[doc = "Bit 5 - CSI out of sync interrupt flag"]
    #[inline(always)]
    pub fn outsync(&mut self) -> OutsyncW<'_, RxEintVffIfSpec> {
        OutsyncW::new(self, 5)
    }
    #[doc = "Bit 6 - CSI Pixel Format Error interrupt flag"]
    #[inline(always)]
    pub fn fmterr(&mut self) -> FmterrW<'_, RxEintVffIfSpec> {
        FmterrW::new(self, 6)
    }
    #[doc = "Bit 7 - AHB wait time out interrupt flag"]
    #[inline(always)]
    pub fn ahbwto(&mut self) -> AhbwtoW<'_, RxEintVffIfSpec> {
        AhbwtoW::new(self, 7)
    }
    #[doc = "Bit 8 - CSI Frame Start interrupt flag"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, RxEintVffIfSpec> {
        FsW::new(self, 8)
    }
    #[doc = "Bit 9 - CSI Frame End interrupt flag"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, RxEintVffIfSpec> {
        FeW::new(self, 9)
    }
    #[doc = "Bit 10 - CSI Line Start interrupt flag"]
    #[inline(always)]
    pub fn ls(&mut self) -> LsW<'_, RxEintVffIfSpec> {
        LsW::new(self, 10)
    }
    #[doc = "Bit 11 - CSI Line End interrupt flag"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<'_, RxEintVffIfSpec> {
        LeW::new(self, 11)
    }
    #[doc = "Bit 12 - Raw FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn raw_ovr(&mut self) -> RawOvrW<'_, RxEintVffIfSpec> {
        RawOvrW::new(self, 12)
    }
    #[doc = "Bit 13 - Raw AHB Error Interrupt Enable"]
    #[inline(always)]
    pub fn raw_ahberr(&mut self) -> RawAhberrW<'_, RxEintVffIfSpec> {
        RawAhberrW::new(self, 13)
    }
    #[doc = "Bit 24 - AHBM_RDTO"]
    #[inline(always)]
    pub fn ahbm_rdto(&mut self) -> AhbmRdtoW<'_, RxEintVffIfSpec> {
        AhbmRdtoW::new(self, 24)
    }
    #[doc = "Bit 25 - AHBM_IDTO"]
    #[inline(always)]
    pub fn ahbm_idto(&mut self) -> AhbmIdtoW<'_, RxEintVffIfSpec> {
        AhbmIdtoW::new(self, 25)
    }
    #[doc = "Bit 26 - AHBM_MAX"]
    #[inline(always)]
    pub fn ahbm_max(&mut self) -> AhbmMaxW<'_, RxEintVffIfSpec> {
        AhbmMaxW::new(self, 26)
    }
}
#[doc = "RX Video FIFO Interrupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_vff_if::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_vff_if::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxEintVffIfSpec;
impl crate::RegisterSpec for RxEintVffIfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_eint_vff_if::R`](R) reader structure"]
impl crate::Readable for RxEintVffIfSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_eint_vff_if::W`](W) writer structure"]
impl crate::Writable for RxEintVffIfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_EINT_VFF_IF to value 0"]
impl crate::Resettable for RxEintVffIfSpec {}
