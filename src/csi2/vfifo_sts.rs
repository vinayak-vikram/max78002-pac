#[doc = "Register `VFIFO_STS` reader"]
pub type R = crate::R<VfifoStsSpec>;
#[doc = "Register `VFIFO_STS` writer"]
pub type W = crate::W<VfifoStsSpec>;
#[doc = "Field `FEMPTY` reader - FIFO empty."]
pub type FemptyR = crate::BitReader;
#[doc = "Field `FEMPTY` writer - FIFO empty."]
pub type FemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHD` reader - FIFO above threshold."]
pub type FthdR = crate::BitReader;
#[doc = "Field `FTHD` writer - FIFO above threshold."]
pub type FthdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFULL` reader - FIFO full."]
pub type FfullR = crate::BitReader;
#[doc = "Field `FFULL` writer - FIFO full."]
pub type FfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` reader - FIFO underrun"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - FIFO underrun"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERRUN` reader - FIFO overrun"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - FIFO overrun"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSYNC` reader - CSI out of sync"]
pub type OutsyncR = crate::BitReader;
#[doc = "Field `OUTSYNC` writer - CSI out of sync"]
pub type OutsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMTERR` reader - CSI Pixel Format Error"]
pub type FmterrR = crate::BitReader;
#[doc = "Field `FMTERR` writer - CSI Pixel Format Error"]
pub type FmterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBWTO` reader - AHB wait time out"]
pub type AhbwtoR = crate::BitReader;
#[doc = "Field `AHBWTO` writer - AHB wait time out"]
pub type AhbwtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - CSI Frame Start"]
pub type FsR = crate::BitReader;
#[doc = "Field `FS` writer - CSI Frame Start"]
pub type FsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - CSI Frame End"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - CSI Frame End"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS` reader - CSI Line Start"]
pub type LsR = crate::BitReader;
#[doc = "Field `LS` writer - CSI Line Start"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LE` reader - CSI Line End"]
pub type LeR = crate::BitReader;
#[doc = "Field `LE` writer - CSI Line End"]
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FELT` reader - FIFO remaining entity count"]
pub type FeltR = crate::FieldReader;
#[doc = "Field `FELT` writer - FIFO remaining entity count"]
pub type FeltW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FMT` reader - CSI pixel format of current transaction"]
pub type FmtR = crate::FieldReader;
#[doc = "Field `FMT` writer - CSI pixel format of current transaction"]
pub type FmtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - FIFO empty."]
    #[inline(always)]
    pub fn fempty(&self) -> FemptyR {
        FemptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO above threshold."]
    #[inline(always)]
    pub fn fthd(&self) -> FthdR {
        FthdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO full."]
    #[inline(always)]
    pub fn ffull(&self) -> FfullR {
        FfullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO underrun"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO overrun"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CSI out of sync"]
    #[inline(always)]
    pub fn outsync(&self) -> OutsyncR {
        OutsyncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSI Pixel Format Error"]
    #[inline(always)]
    pub fn fmterr(&self) -> FmterrR {
        FmterrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB wait time out"]
    #[inline(always)]
    pub fn ahbwto(&self) -> AhbwtoR {
        AhbwtoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI Frame Start"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI Frame End"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CSI Line Start"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CSI Line End"]
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:22 - FIFO remaining entity count"]
    #[inline(always)]
    pub fn felt(&self) -> FeltR {
        FeltR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - CSI pixel format of current transaction"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO empty."]
    #[inline(always)]
    pub fn fempty(&mut self) -> FemptyW<'_, VfifoStsSpec> {
        FemptyW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO above threshold."]
    #[inline(always)]
    pub fn fthd(&mut self) -> FthdW<'_, VfifoStsSpec> {
        FthdW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO full."]
    #[inline(always)]
    pub fn ffull(&mut self) -> FfullW<'_, VfifoStsSpec> {
        FfullW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO underrun"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<'_, VfifoStsSpec> {
        UnderrunW::new(self, 3)
    }
    #[doc = "Bit 4 - FIFO overrun"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, VfifoStsSpec> {
        OverrunW::new(self, 4)
    }
    #[doc = "Bit 5 - CSI out of sync"]
    #[inline(always)]
    pub fn outsync(&mut self) -> OutsyncW<'_, VfifoStsSpec> {
        OutsyncW::new(self, 5)
    }
    #[doc = "Bit 6 - CSI Pixel Format Error"]
    #[inline(always)]
    pub fn fmterr(&mut self) -> FmterrW<'_, VfifoStsSpec> {
        FmterrW::new(self, 6)
    }
    #[doc = "Bit 7 - AHB wait time out"]
    #[inline(always)]
    pub fn ahbwto(&mut self) -> AhbwtoW<'_, VfifoStsSpec> {
        AhbwtoW::new(self, 7)
    }
    #[doc = "Bit 8 - CSI Frame Start"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, VfifoStsSpec> {
        FsW::new(self, 8)
    }
    #[doc = "Bit 9 - CSI Frame End"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<'_, VfifoStsSpec> {
        FeW::new(self, 9)
    }
    #[doc = "Bit 10 - CSI Line Start"]
    #[inline(always)]
    pub fn ls(&mut self) -> LsW<'_, VfifoStsSpec> {
        LsW::new(self, 10)
    }
    #[doc = "Bit 11 - CSI Line End"]
    #[inline(always)]
    pub fn le(&mut self) -> LeW<'_, VfifoStsSpec> {
        LeW::new(self, 11)
    }
    #[doc = "Bits 16:22 - FIFO remaining entity count"]
    #[inline(always)]
    pub fn felt(&mut self) -> FeltW<'_, VfifoStsSpec> {
        FeltW::new(self, 16)
    }
    #[doc = "Bits 24:29 - CSI pixel format of current transaction"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FmtW<'_, VfifoStsSpec> {
        FmtW::new(self, 24)
    }
}
#[doc = "Video FIFO Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoStsSpec;
impl crate::RegisterSpec for VfifoStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_sts::R`](R) reader structure"]
impl crate::Readable for VfifoStsSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_sts::W`](W) writer structure"]
impl crate::Writable for VfifoStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_STS to value 0"]
impl crate::Resettable for VfifoStsSpec {}
