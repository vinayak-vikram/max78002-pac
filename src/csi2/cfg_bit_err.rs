#[doc = "Register `CFG_BIT_ERR` reader"]
pub type R = crate::R<CfgBitErrSpec>;
#[doc = "Register `CFG_BIT_ERR` writer"]
pub type W = crate::W<CfgBitErrSpec>;
#[doc = "Field `MBE` reader - Multiple bit ECC error."]
pub type MbeR = crate::BitReader;
#[doc = "Field `MBE` writer - Multiple bit ECC error."]
pub type MbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBE` reader - Single bit ECC error."]
pub type SbeR = crate::BitReader;
#[doc = "Field `SBE` writer - Single bit ECC error."]
pub type SbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEADER` reader - Header bit location of single bit ECC error."]
pub type HeaderR = crate::FieldReader;
#[doc = "Field `HEADER` writer - Header bit location of single bit ECC error."]
pub type HeaderW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CRC` reader - CRC error."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC error."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_ERR_SEND_LVL` reader - Video Error Send Level."]
pub type VidErrSendLvlR = crate::BitReader;
#[doc = "Field `VID_ERR_SEND_LVL` writer - Video Error Send Level."]
pub type VidErrSendLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VID_ERR_FIFO_WR_OV` reader - Video Error Fifo Overflow."]
pub type VidErrFifoWrOvR = crate::BitReader;
#[doc = "Field `VID_ERR_FIFO_WR_OV` writer - Video Error Fifo Overflow."]
pub type VidErrFifoWrOvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Multiple bit ECC error."]
    #[inline(always)]
    pub fn mbe(&self) -> MbeR {
        MbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single bit ECC error."]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Header bit location of single bit ECC error."]
    #[inline(always)]
    pub fn header(&self) -> HeaderR {
        HeaderR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - CRC error."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Video Error Send Level."]
    #[inline(always)]
    pub fn vid_err_send_lvl(&self) -> VidErrSendLvlR {
        VidErrSendLvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Video Error Fifo Overflow."]
    #[inline(always)]
    pub fn vid_err_fifo_wr_ov(&self) -> VidErrFifoWrOvR {
        VidErrFifoWrOvR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multiple bit ECC error."]
    #[inline(always)]
    pub fn mbe(&mut self) -> MbeW<'_, CfgBitErrSpec> {
        MbeW::new(self, 0)
    }
    #[doc = "Bit 1 - Single bit ECC error."]
    #[inline(always)]
    pub fn sbe(&mut self) -> SbeW<'_, CfgBitErrSpec> {
        SbeW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Header bit location of single bit ECC error."]
    #[inline(always)]
    pub fn header(&mut self) -> HeaderW<'_, CfgBitErrSpec> {
        HeaderW::new(self, 2)
    }
    #[doc = "Bit 7 - CRC error."]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, CfgBitErrSpec> {
        CrcW::new(self, 7)
    }
    #[doc = "Bit 8 - Video Error Send Level."]
    #[inline(always)]
    pub fn vid_err_send_lvl(&mut self) -> VidErrSendLvlW<'_, CfgBitErrSpec> {
        VidErrSendLvlW::new(self, 8)
    }
    #[doc = "Bit 9 - Video Error Fifo Overflow."]
    #[inline(always)]
    pub fn vid_err_fifo_wr_ov(&mut self) -> VidErrFifoWrOvW<'_, CfgBitErrSpec> {
        VidErrFifoWrOvW::new(self, 9)
    }
}
#[doc = "CFG_BIT_ERR.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_bit_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_bit_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgBitErrSpec;
impl crate::RegisterSpec for CfgBitErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_bit_err::R`](R) reader structure"]
impl crate::Readable for CfgBitErrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_bit_err::W`](W) writer structure"]
impl crate::Writable for CfgBitErrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_BIT_ERR to value 0"]
impl crate::Resettable for CfgBitErrSpec {}
