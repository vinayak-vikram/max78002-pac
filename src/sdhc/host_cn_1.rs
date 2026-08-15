#[doc = "Register `HOST_CN_1` reader"]
pub type R = crate::R<HostCn1Spec>;
#[doc = "Register `HOST_CN_1` writer"]
pub type W = crate::W<HostCn1Spec>;
#[doc = "Field `LED_CN` reader - LED Control."]
pub type LedCnR = crate::BitReader;
#[doc = "Field `LED_CN` writer - LED Control."]
pub type LedCnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TRANSFER_WIDTH` reader - Data Transfer Width."]
pub type DataTransferWidthR = crate::BitReader;
#[doc = "Field `DATA_TRANSFER_WIDTH` writer - Data Transfer Width."]
pub type DataTransferWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_EN` reader - High Speed Enable."]
pub type HsEnR = crate::BitReader;
#[doc = "Field `HS_EN` writer - High Speed Enable."]
pub type HsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SELECT` reader - DMA Select."]
pub type DmaSelectR = crate::FieldReader;
#[doc = "Field `DMA_SELECT` writer - DMA Select."]
pub type DmaSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXT_DATA_TRANSFER_WIDTH` reader - Extended Data Transfer Width."]
pub type ExtDataTransferWidthR = crate::BitReader;
#[doc = "Field `EXT_DATA_TRANSFER_WIDTH` writer - Extended Data Transfer Width."]
pub type ExtDataTransferWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DETECT_TEST` reader - Card Detect Test Level."]
pub type CardDetectTestR = crate::BitReader;
#[doc = "Field `CARD_DETECT_TEST` writer - Card Detect Test Level."]
pub type CardDetectTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DETECT_SIGNAL` reader - Card Detect Signal Selection."]
pub type CardDetectSignalR = crate::BitReader;
#[doc = "Field `CARD_DETECT_SIGNAL` writer - Card Detect Signal Selection."]
pub type CardDetectSignalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LED Control."]
    #[inline(always)]
    pub fn led_cn(&self) -> LedCnR {
        LedCnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfer Width."]
    #[inline(always)]
    pub fn data_transfer_width(&self) -> DataTransferWidthR {
        DataTransferWidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - High Speed Enable."]
    #[inline(always)]
    pub fn hs_en(&self) -> HsEnR {
        HsEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - DMA Select."]
    #[inline(always)]
    pub fn dma_select(&self) -> DmaSelectR {
        DmaSelectR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width."]
    #[inline(always)]
    pub fn ext_data_transfer_width(&self) -> ExtDataTransferWidthR {
        ExtDataTransferWidthR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Detect Test Level."]
    #[inline(always)]
    pub fn card_detect_test(&self) -> CardDetectTestR {
        CardDetectTestR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection."]
    #[inline(always)]
    pub fn card_detect_signal(&self) -> CardDetectSignalR {
        CardDetectSignalR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED Control."]
    #[inline(always)]
    pub fn led_cn(&mut self) -> LedCnW<'_, HostCn1Spec> {
        LedCnW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfer Width."]
    #[inline(always)]
    pub fn data_transfer_width(&mut self) -> DataTransferWidthW<'_, HostCn1Spec> {
        DataTransferWidthW::new(self, 1)
    }
    #[doc = "Bit 2 - High Speed Enable."]
    #[inline(always)]
    pub fn hs_en(&mut self) -> HsEnW<'_, HostCn1Spec> {
        HsEnW::new(self, 2)
    }
    #[doc = "Bits 3:4 - DMA Select."]
    #[inline(always)]
    pub fn dma_select(&mut self) -> DmaSelectW<'_, HostCn1Spec> {
        DmaSelectW::new(self, 3)
    }
    #[doc = "Bit 5 - Extended Data Transfer Width."]
    #[inline(always)]
    pub fn ext_data_transfer_width(&mut self) -> ExtDataTransferWidthW<'_, HostCn1Spec> {
        ExtDataTransferWidthW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Detect Test Level."]
    #[inline(always)]
    pub fn card_detect_test(&mut self) -> CardDetectTestW<'_, HostCn1Spec> {
        CardDetectTestW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Detect Signal Selection."]
    #[inline(always)]
    pub fn card_detect_signal(&mut self) -> CardDetectSignalW<'_, HostCn1Spec> {
        CardDetectSignalW::new(self, 7)
    }
}
#[doc = "Host Control 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cn_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cn_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCn1Spec;
impl crate::RegisterSpec for HostCn1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`host_cn_1::R`](R) reader structure"]
impl crate::Readable for HostCn1Spec {}
#[doc = "`write(|w| ..)` method takes [`host_cn_1::W`](W) writer structure"]
impl crate::Writable for HostCn1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_CN_1 to value 0"]
impl crate::Resettable for HostCn1Spec {}
