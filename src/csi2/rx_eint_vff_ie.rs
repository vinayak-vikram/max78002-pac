#[doc = "Register `RX_EINT_VFF_IE` reader"]
pub type R = crate::R<RxEintVffIeSpec>;
#[doc = "Register `RX_EINT_VFF_IE` writer"]
pub type W = crate::W<RxEintVffIeSpec>;
#[doc = "Field `FNEMPTY` reader - Video FIFO not empty interrupt enable."]
pub type FnemptyR = crate::BitReader;
#[doc = "Field `FNEMPTY` writer - Video FIFO not empty interrupt enable."]
pub type FnemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHD` reader - Video FIFO above threshold interrupt enable."]
pub type FthdR = crate::BitReader;
#[doc = "Field `FTHD` writer - Video FIFO above threshold interrupt enable."]
pub type FthdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFULL` reader - Video FIFO full interrupt enable."]
pub type FfullR = crate::BitReader;
#[doc = "Field `FFULL` writer - Video FIFO full interrupt enable."]
pub type FfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` reader - Video FIFO underrun interrupt enable"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Video FIFO underrun interrupt enable"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - Video FIFO overrun interrupt enable"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Video FIFO overrun interrupt enable"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSYNC` reader - CSI out of sync interrupt enable"]
pub type OutsyncR = crate::BitReader;
#[doc = "Field `OUTSYNC` writer - CSI out of sync interrupt enable"]
pub type OutsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMTERR` reader - CSI Pixel Format Error interrupt enable"]
pub type FmterrR = crate::BitReader;
#[doc = "Field `FMTERR` writer - CSI Pixel Format Error interrupt enable"]
pub type FmterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBWTO` reader - AHB wait time out interrupt enable"]
pub type AhbwtoR = crate::BitReader;
#[doc = "Field `AHBWTO` writer - AHB wait time out interrupt enable"]
pub type AhbwtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - CSI Frame Start interrupt enable"]
pub type FsR = crate::BitReader;
#[doc = "Field `FS` writer - CSI Frame Start interrupt enable"]
pub type FsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - CSI Frame End interrupt enable"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - CSI Frame End interrupt enable"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - CSI Line Start interrupt enable"]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - CSI Line Start interrupt enable"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - CSI Line End interrupt enable"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - CSI Line End interrupt enable"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_OVR` reader - Raw FIFO Overrun Interrupt Enable"]
pub type RawOvrR = crate::BitReader;
#[doc = "Field `RAW_OVR` writer - Raw FIFO Overrun Interrupt Enable"]
pub type RawOvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_AHBERR` reader - Raw AHB Error Interrupt Enable"]
pub type RawAhberrR = crate::BitReader;
#[doc = "Field `RAW_AHBERR` writer - Raw AHB Error Interrupt Enable"]
pub type RawAhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNEMP_MD` reader - Video FIFO not empty detection mode"]
pub type FnempMdR = crate::BitReader;
#[doc = "Field `FNEMP_MD` writer - Video FIFO not empty detection mode"]
pub type FnempMdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHD_MD` reader - Video FIFO threshold detection mode"]
pub type FthdMdR = crate::BitReader;
#[doc = "Field `FTHD_MD` writer - Video FIFO threshold detection mode"]
pub type FthdMdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFUL_MD` reader - Video FIFO full detection mode"]
pub type FfulMdR = crate::BitReader;
#[doc = "Field `FFUL_MD` writer - Video FIFO full detection mode"]
pub type FfulMdW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Video FIFO not empty interrupt enable."]
    #[inline(always)]
    pub fn fnempty(&self) -> FnemptyR {
        FnemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Video FIFO above threshold interrupt enable."]
    #[inline(always)]
    pub fn fthd(&self) -> FthdR {
        FthdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Video FIFO full interrupt enable."]
    #[inline(always)]
    pub fn ffull(&self) -> FfullR {
        FfullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Video FIFO underrun interrupt enable"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Video FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CSI out of sync interrupt enable"]
    #[inline(always)]
    pub fn outsync(&self) -> OutsyncR {
        OutsyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSI Pixel Format Error interrupt enable"]
    #[inline(always)]
    pub fn fmterr(&self) -> FmterrR {
        FmterrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB wait time out interrupt enable"]
    #[inline(always)]
    pub fn ahbwto(&self) -> AhbwtoR {
        AhbwtoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI Frame Start interrupt enable"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI Frame End interrupt enable"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CSI Line Start interrupt enable"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CSI Line End interrupt enable"]
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
    #[doc = "Bit 16 - Video FIFO not empty detection mode"]
    #[inline(always)]
    pub fn fnemp_md(&self) -> FnempMdR {
        FnempMdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Video FIFO threshold detection mode"]
    #[inline(always)]
    pub fn fthd_md(&self) -> FthdMdR {
        FthdMdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Video FIFO full detection mode"]
    #[inline(always)]
    pub fn fful_md(&self) -> FfulMdR {
        FfulMdR::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bit 0 - Video FIFO not empty interrupt enable."]
    #[inline(always)]
    pub fn fnempty(&mut self) -> FnemptyW<'_, RxEintVffIeSpec> {
        FnemptyW::new(self, 0)
    }
    #[doc = "Bit 1 - Video FIFO above threshold interrupt enable."]
    #[inline(always)]
    pub fn fthd(&mut self) -> FthdW<'_, RxEintVffIeSpec> {
        FthdW::new(self, 1)
    }
    #[doc = "Bit 2 - Video FIFO full interrupt enable."]
    #[inline(always)]
    pub fn ffull(&mut self) -> FfullW<'_, RxEintVffIeSpec> {
        FfullW::new(self, 2)
    }
    #[doc = "Bit 3 - Video FIFO underrun interrupt enable"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<'_, RxEintVffIeSpec> {
        UnderrunW::new(self, 3)
    }
    #[doc = "Bit 4 - Video FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, RxEintVffIeSpec> {
        OverrunW::new(self, 4)
    }
    #[doc = "Bit 5 - CSI out of sync interrupt enable"]
    #[inline(always)]
    pub fn outsync(&mut self) -> OutsyncW<'_, RxEintVffIeSpec> {
        OutsyncW::new(self, 5)
    }
    #[doc = "Bit 6 - CSI Pixel Format Error interrupt enable"]
    #[inline(always)]
    pub fn fmterr(&mut self) -> FmterrW<'_, RxEintVffIeSpec> {
        FmterrW::new(self, 6)
    }
    #[doc = "Bit 7 - AHB wait time out interrupt enable"]
    #[inline(always)]
    pub fn ahbwto(&mut self) -> AhbwtoW<'_, RxEintVffIeSpec> {
        AhbwtoW::new(self, 7)
    }
    #[doc = "Bit 8 - CSI Frame Start interrupt enable"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, RxEintVffIeSpec> {
        FsW::new(self, 8)
    }
    #[doc = "Bit 9 - CSI Frame End interrupt enable"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, RxEintVffIeSpec> {
        FeW::new(self, 9)
    }
    #[doc = "Bit 10 - CSI Line Start interrupt enable"]
    #[inline(always)]
    pub fn ls(&mut self) -> LsW<'_, RxEintVffIeSpec> {
        LsW::new(self, 10)
    }
    #[doc = "Bit 11 - CSI Line End interrupt enable"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<'_, RxEintVffIeSpec> {
        LeW::new(self, 11)
    }
    #[doc = "Bit 12 - Raw FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn raw_ovr(&mut self) -> RawOvrW<'_, RxEintVffIeSpec> {
        RawOvrW::new(self, 12)
    }
    #[doc = "Bit 13 - Raw AHB Error Interrupt Enable"]
    #[inline(always)]
    pub fn raw_ahberr(&mut self) -> RawAhberrW<'_, RxEintVffIeSpec> {
        RawAhberrW::new(self, 13)
    }
    #[doc = "Bit 16 - Video FIFO not empty detection mode"]
    #[inline(always)]
    pub fn fnemp_md(&mut self) -> FnempMdW<'_, RxEintVffIeSpec> {
        FnempMdW::new(self, 16)
    }
    #[doc = "Bit 17 - Video FIFO threshold detection mode"]
    #[inline(always)]
    pub fn fthd_md(&mut self) -> FthdMdW<'_, RxEintVffIeSpec> {
        FthdMdW::new(self, 17)
    }
    #[doc = "Bit 18 - Video FIFO full detection mode"]
    #[inline(always)]
    pub fn fful_md(&mut self) -> FfulMdW<'_, RxEintVffIeSpec> {
        FfulMdW::new(self, 18)
    }
    #[doc = "Bit 24 - AHBM_RDTO"]
    #[inline(always)]
    pub fn ahbm_rdto(&mut self) -> AhbmRdtoW<'_, RxEintVffIeSpec> {
        AhbmRdtoW::new(self, 24)
    }
    #[doc = "Bit 25 - AHBM_IDTO"]
    #[inline(always)]
    pub fn ahbm_idto(&mut self) -> AhbmIdtoW<'_, RxEintVffIeSpec> {
        AhbmIdtoW::new(self, 25)
    }
    #[doc = "Bit 26 - AHBM_MAX"]
    #[inline(always)]
    pub fn ahbm_max(&mut self) -> AhbmMaxW<'_, RxEintVffIeSpec> {
        AhbmMaxW::new(self, 26)
    }
}
#[doc = "RX Video FIFO Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_vff_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_vff_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxEintVffIeSpec;
impl crate::RegisterSpec for RxEintVffIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_eint_vff_ie::R`](R) reader structure"]
impl crate::Readable for RxEintVffIeSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_eint_vff_ie::W`](W) writer structure"]
impl crate::Writable for RxEintVffIeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_EINT_VFF_IE to value 0"]
impl crate::Resettable for RxEintVffIeSpec {}
