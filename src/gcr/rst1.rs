#[doc = "Register `RST1` reader"]
pub type R = crate::R<Rst1Spec>;
#[doc = "Register `RST1` writer"]
pub type W = crate::W<Rst1Spec>;
#[doc = "I2C1 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetRead {
    #[doc = "0: Reset complete."]
    ResetDone = 0,
    #[doc = "1: Starts reset or indicates reset in progress."]
    Busy = 1,
}
impl From<ResetRead> for bool {
    #[inline(always)]
    fn from(variant: ResetRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` reader - I2C1 Reset."]
pub type I2c1R = crate::BitReader<ResetRead>;
impl I2c1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetRead {
        match self.bits {
            false => ResetRead::ResetDone,
            true => ResetRead::Busy,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == ResetRead::ResetDone
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == ResetRead::Busy
    }
}
#[doc = "Field `I2C1` writer - I2C1 Reset."]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG, ResetRead>;
impl<'a, REG> I2c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(ResetRead::ResetDone)
    }
    #[doc = "Starts reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(ResetRead::Busy)
    }
}
#[doc = "Field `PT` reader - PT Reset."]
pub use I2c1R as PtR;
#[doc = "Field `SDHC` reader - SDHC Reset."]
pub use I2c1R as SdhcR;
#[doc = "Field `OWM` reader - OWM Reset."]
pub use I2c1R as OwmR;
#[doc = "Field `CRC` reader - CRC Reset."]
pub use I2c1R as CrcR;
#[doc = "Field `AES` reader - AES Reset."]
pub use I2c1R as AesR;
#[doc = "Field `SPI0` reader - SPI 0 Reset."]
pub use I2c1R as Spi0R;
#[doc = "Field `CSI2PHY` reader - CSI2 PHY Reset."]
pub use I2c1R as Csi2phyR;
#[doc = "Field `SMPHR` reader - SMPHR Reset."]
pub use I2c1R as SmphrR;
#[doc = "Field `I2S` reader - I2S Reset."]
pub use I2c1R as I2sR;
#[doc = "Field `I2C2` reader - I2C2 Reset."]
pub use I2c1R as I2c2R;
#[doc = "Field `DVS` reader - DVS Reset."]
pub use I2c1R as DvsR;
#[doc = "Field `SIMO` reader - SIMO Reset."]
pub use I2c1R as SimoR;
#[doc = "Field `PCIF` reader - PCIF Reset."]
pub use I2c1R as PcifR;
#[doc = "Field `CSI2` reader - CSI2 Reset."]
pub use I2c1R as Csi2R;
#[doc = "Field `CPU1` reader - CPU1 Reset."]
pub use I2c1R as Cpu1R;
#[doc = "Field `PT` writer - PT Reset."]
pub use I2c1W as PtW;
#[doc = "Field `SDHC` writer - SDHC Reset."]
pub use I2c1W as SdhcW;
#[doc = "Field `OWM` writer - OWM Reset."]
pub use I2c1W as OwmW;
#[doc = "Field `CRC` writer - CRC Reset."]
pub use I2c1W as CrcW;
#[doc = "Field `AES` writer - AES Reset."]
pub use I2c1W as AesW;
#[doc = "Field `SPI0` writer - SPI 0 Reset."]
pub use I2c1W as Spi0W;
#[doc = "Field `CSI2PHY` writer - CSI2 PHY Reset."]
pub use I2c1W as Csi2phyW;
#[doc = "Field `SMPHR` writer - SMPHR Reset."]
pub use I2c1W as SmphrW;
#[doc = "Field `I2S` writer - I2S Reset."]
pub use I2c1W as I2sW;
#[doc = "Field `I2C2` writer - I2C2 Reset."]
pub use I2c1W as I2c2W;
#[doc = "Field `DVS` writer - DVS Reset."]
pub use I2c1W as DvsW;
#[doc = "Field `SIMO` writer - SIMO Reset."]
pub use I2c1W as SimoW;
#[doc = "Field `PCIF` writer - PCIF Reset."]
pub use I2c1W as PcifW;
#[doc = "Field `CSI2` writer - CSI2 Reset."]
pub use I2c1W as Csi2W;
#[doc = "Field `CPU1` writer - CPU1 Reset."]
pub use I2c1W as Cpu1W;
impl R {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - SDHC Reset."]
    #[inline(always)]
    pub fn sdhc(&self) -> SdhcR {
        SdhcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OWM Reset."]
    #[inline(always)]
    pub fn owm(&self) -> OwmR {
        OwmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC Reset."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AES Reset."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI 0 Reset."]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - CSI2 PHY Reset."]
    #[inline(always)]
    pub fn csi2phy(&self) -> Csi2phyR {
        Csi2phyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SmphrR {
        SmphrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - I2S Reset."]
    #[inline(always)]
    pub fn i2s(&self) -> I2sR {
        I2sR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C2 Reset."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - DVS Reset."]
    #[inline(always)]
    pub fn dvs(&self) -> DvsR {
        DvsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SIMO Reset."]
    #[inline(always)]
    pub fn simo(&self) -> SimoR {
        SimoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PCIF Reset."]
    #[inline(always)]
    pub fn pcif(&self) -> PcifR {
        PcifR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CSI2 Reset."]
    #[inline(always)]
    pub fn csi2(&self) -> Csi2R {
        Csi2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Reset."]
    #[inline(always)]
    pub fn cpu1(&self) -> Cpu1R {
        Cpu1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Reset."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<'_, Rst1Spec> {
        I2c1W::new(self, 0)
    }
    #[doc = "Bit 1 - PT Reset."]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<'_, Rst1Spec> {
        PtW::new(self, 1)
    }
    #[doc = "Bit 6 - SDHC Reset."]
    #[inline(always)]
    pub fn sdhc(&mut self) -> SdhcW<'_, Rst1Spec> {
        SdhcW::new(self, 6)
    }
    #[doc = "Bit 7 - OWM Reset."]
    #[inline(always)]
    pub fn owm(&mut self) -> OwmW<'_, Rst1Spec> {
        OwmW::new(self, 7)
    }
    #[doc = "Bit 9 - CRC Reset."]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, Rst1Spec> {
        CrcW::new(self, 9)
    }
    #[doc = "Bit 10 - AES Reset."]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<'_, Rst1Spec> {
        AesW::new(self, 10)
    }
    #[doc = "Bit 11 - SPI 0 Reset."]
    #[inline(always)]
    pub fn spi0(&mut self) -> Spi0W<'_, Rst1Spec> {
        Spi0W::new(self, 11)
    }
    #[doc = "Bit 14 - CSI2 PHY Reset."]
    #[inline(always)]
    pub fn csi2phy(&mut self) -> Csi2phyW<'_, Rst1Spec> {
        Csi2phyW::new(self, 14)
    }
    #[doc = "Bit 16 - SMPHR Reset."]
    #[inline(always)]
    pub fn smphr(&mut self) -> SmphrW<'_, Rst1Spec> {
        SmphrW::new(self, 16)
    }
    #[doc = "Bit 19 - I2S Reset."]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2sW<'_, Rst1Spec> {
        I2sW::new(self, 19)
    }
    #[doc = "Bit 20 - I2C2 Reset."]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<'_, Rst1Spec> {
        I2c2W::new(self, 20)
    }
    #[doc = "Bit 24 - DVS Reset."]
    #[inline(always)]
    pub fn dvs(&mut self) -> DvsW<'_, Rst1Spec> {
        DvsW::new(self, 24)
    }
    #[doc = "Bit 25 - SIMO Reset."]
    #[inline(always)]
    pub fn simo(&mut self) -> SimoW<'_, Rst1Spec> {
        SimoW::new(self, 25)
    }
    #[doc = "Bit 26 - PCIF Reset."]
    #[inline(always)]
    pub fn pcif(&mut self) -> PcifW<'_, Rst1Spec> {
        PcifW::new(self, 26)
    }
    #[doc = "Bit 27 - CSI2 Reset."]
    #[inline(always)]
    pub fn csi2(&mut self) -> Csi2W<'_, Rst1Spec> {
        Csi2W::new(self, 27)
    }
    #[doc = "Bit 31 - CPU1 Reset."]
    #[inline(always)]
    pub fn cpu1(&mut self) -> Cpu1W<'_, Rst1Spec> {
        Cpu1W::new(self, 31)
    }
}
#[doc = "Reset 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst1Spec;
impl crate::RegisterSpec for Rst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst1::R`](R) reader structure"]
impl crate::Readable for Rst1Spec {}
#[doc = "`write(|w| ..)` method takes [`rst1::W`](W) writer structure"]
impl crate::Writable for Rst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST1 to value 0"]
impl crate::Resettable for Rst1Spec {}
