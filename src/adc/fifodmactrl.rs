#[doc = "Register `FIFODMACTRL` reader"]
pub type R = crate::R<FifodmactrlSpec>;
#[doc = "Register `FIFODMACTRL` writer"]
pub type W = crate::W<FifodmactrlSpec>;
#[doc = "DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaEn {
    #[doc = "0: Disable DMA."]
    Dis = 0,
    #[doc = "1: Enable DMA."]
    En = 1,
}
impl From<DmaEn> for bool {
    #[inline(always)]
    fn from(variant: DmaEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_EN` reader - DMA Enable."]
pub type DmaEnR = crate::BitReader<DmaEn>;
impl DmaEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaEn {
        match self.bits {
            false => DmaEn::Dis,
            true => DmaEn::En,
        }
    }
    #[doc = "Disable DMA."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DmaEn::Dis
    }
    #[doc = "Enable DMA."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DmaEn::En
    }
}
#[doc = "Field `DMA_EN` writer - DMA Enable."]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG, DmaEn>;
impl<'a, REG> DmaEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEn::Dis)
    }
    #[doc = "Enable DMA."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEn::En)
    }
}
#[doc = "FIFO Flush.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flush {
    #[doc = "0: Normal FIFO operation."]
    Normal = 0,
    #[doc = "1: Flush FIFO."]
    Flush = 1,
}
impl From<Flush> for bool {
    #[inline(always)]
    fn from(variant: Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLUSH` reader - FIFO Flush."]
pub type FlushR = crate::BitReader<Flush>;
impl FlushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flush {
        match self.bits {
            false => Flush::Normal,
            true => Flush::Flush,
        }
    }
    #[doc = "Normal FIFO operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Flush::Normal
    }
    #[doc = "Flush FIFO."]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Flush::Flush
    }
}
#[doc = "Field `FLUSH` writer - FIFO Flush."]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG, Flush>;
impl<'a, REG> FlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal FIFO operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Normal)
    }
    #[doc = "Flush FIFO."]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Flush::Flush)
    }
}
#[doc = "DATA format control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataFormat {
    #[doc = "0: Data and Status in FIFO."]
    DataStatus = 0,
    #[doc = "1: Only Data in FIFO."]
    DataOnly = 1,
    #[doc = "2: Only Raw Data in FIFO."]
    RawDataOnly = 2,
}
impl From<DataFormat> for u8 {
    #[inline(always)]
    fn from(variant: DataFormat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataFormat {
    type Ux = u8;
}
impl crate::IsEnum for DataFormat {}
#[doc = "Field `DATA_FORMAT` reader - DATA format control."]
pub type DataFormatR = crate::FieldReader<DataFormat>;
impl DataFormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DataFormat> {
        match self.bits {
            0 => Some(DataFormat::DataStatus),
            1 => Some(DataFormat::DataOnly),
            2 => Some(DataFormat::RawDataOnly),
            _ => None,
        }
    }
    #[doc = "Data and Status in FIFO."]
    #[inline(always)]
    pub fn is_data_status(&self) -> bool {
        *self == DataFormat::DataStatus
    }
    #[doc = "Only Data in FIFO."]
    #[inline(always)]
    pub fn is_data_only(&self) -> bool {
        *self == DataFormat::DataOnly
    }
    #[doc = "Only Raw Data in FIFO."]
    #[inline(always)]
    pub fn is_raw_data_only(&self) -> bool {
        *self == DataFormat::RawDataOnly
    }
}
#[doc = "Field `DATA_FORMAT` writer - DATA format control."]
pub type DataFormatW<'a, REG> = crate::FieldWriter<'a, REG, 2, DataFormat>;
impl<'a, REG> DataFormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data and Status in FIFO."]
    #[inline(always)]
    pub fn data_status(self) -> &'a mut crate::W<REG> {
        self.variant(DataFormat::DataStatus)
    }
    #[doc = "Only Data in FIFO."]
    #[inline(always)]
    pub fn data_only(self) -> &'a mut crate::W<REG> {
        self.variant(DataFormat::DataOnly)
    }
    #[doc = "Only Raw Data in FIFO."]
    #[inline(always)]
    pub fn raw_data_only(self) -> &'a mut crate::W<REG> {
        self.variant(DataFormat::RawDataOnly)
    }
}
#[doc = "Field `THRESH` reader - FIFO Threshold. These bits define the FIFO interrupt threshold."]
pub type ThreshR = crate::FieldReader;
#[doc = "Field `THRESH` writer - FIFO Threshold. These bits define the FIFO interrupt threshold."]
pub type ThreshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - DMA Enable."]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Flush."]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DATA format control."]
    #[inline(always)]
    pub fn data_format(&self) -> DataFormatR {
        DataFormatR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:15 - FIFO Threshold. These bits define the FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thresh(&self) -> ThreshR {
        ThreshR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable."]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, FifodmactrlSpec> {
        DmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Flush."]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<'_, FifodmactrlSpec> {
        FlushW::new(self, 1)
    }
    #[doc = "Bits 2:3 - DATA format control."]
    #[inline(always)]
    pub fn data_format(&mut self) -> DataFormatW<'_, FifodmactrlSpec> {
        DataFormatW::new(self, 2)
    }
    #[doc = "Bits 8:15 - FIFO Threshold. These bits define the FIFO interrupt threshold."]
    #[inline(always)]
    pub fn thresh(&mut self) -> ThreshW<'_, FifodmactrlSpec> {
        ThreshW::new(self, 8)
    }
}
#[doc = "FIFO and DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifodmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodmactrlSpec;
impl crate::RegisterSpec for FifodmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodmactrl::R`](R) reader structure"]
impl crate::Readable for FifodmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifodmactrl::W`](W) writer structure"]
impl crate::Writable for FifodmactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFODMACTRL to value 0"]
impl crate::Resettable for FifodmactrlSpec {}
