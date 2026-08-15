#[doc = "Register `VFIFO_CFG1` reader"]
pub type R = crate::R<VfifoCfg1Spec>;
#[doc = "Register `VFIFO_CFG1` writer"]
pub type W = crate::W<VfifoCfg1Spec>;
#[doc = "Field `AHBWCYC` reader - Maximal AHB Wait Clock Cycles."]
pub type AhbwcycR = crate::FieldReader<u16>;
#[doc = "Field `AHBWCYC` writer - Maximal AHB Wait Clock Cycles."]
pub type AhbwcycW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WAIT_FIRST_FS` reader - WAIT_FIRST_FS."]
pub type WaitFirstFsR = crate::BitReader;
#[doc = "Field `WAIT_FIRST_FS` writer - WAIT_FIRST_FS."]
pub type WaitFirstFsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCU_FRAME_CTRL` reader - ACCU_FRAME_CTRL."]
pub type AccuFrameCtrlR = crate::BitReader;
#[doc = "Field `ACCU_FRAME_CTRL` writer - ACCU_FRAME_CTRL."]
pub type AccuFrameCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCU_LINE_CTRL` reader - ACCU_LINE_CTRL."]
pub type AccuLineCtrlR = crate::BitReader;
#[doc = "Field `ACCU_LINE_CTRL` writer - ACCU_LINE_CTRL."]
pub type AccuLineCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCU_LINE_CNT` reader - ACCU_LINE_CNT."]
pub type AccuLineCntR = crate::BitReader;
#[doc = "Field `ACCU_LINE_CNT` writer - ACCU_LINE_CNT."]
pub type AccuLineCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCU_PIXEL_CNT` reader - ACCU_PIXEL_CNT."]
pub type AccuPixelCntR = crate::BitReader;
#[doc = "Field `ACCU_PIXEL_CNT` writer - ACCU_PIXEL_CNT."]
pub type AccuPixelCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCU_PIXEL_ZERO` reader - ACCU_PIXEL_ZERO."]
pub type AccuPixelZeroR = crate::BitReader;
#[doc = "Field `ACCU_PIXEL_ZERO` writer - ACCU_PIXEL_ZERO."]
pub type AccuPixelZeroW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Maximal AHB Wait Clock Cycles."]
    #[inline(always)]
    pub fn ahbwcyc(&self) -> AhbwcycR {
        AhbwcycR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - WAIT_FIRST_FS."]
    #[inline(always)]
    pub fn wait_first_fs(&self) -> WaitFirstFsR {
        WaitFirstFsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ACCU_FRAME_CTRL."]
    #[inline(always)]
    pub fn accu_frame_ctrl(&self) -> AccuFrameCtrlR {
        AccuFrameCtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ACCU_LINE_CTRL."]
    #[inline(always)]
    pub fn accu_line_ctrl(&self) -> AccuLineCtrlR {
        AccuLineCtrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ACCU_LINE_CNT."]
    #[inline(always)]
    pub fn accu_line_cnt(&self) -> AccuLineCntR {
        AccuLineCntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ACCU_PIXEL_CNT."]
    #[inline(always)]
    pub fn accu_pixel_cnt(&self) -> AccuPixelCntR {
        AccuPixelCntR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ACCU_PIXEL_ZERO."]
    #[inline(always)]
    pub fn accu_pixel_zero(&self) -> AccuPixelZeroR {
        AccuPixelZeroR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Maximal AHB Wait Clock Cycles."]
    #[inline(always)]
    pub fn ahbwcyc(&mut self) -> AhbwcycW<'_, VfifoCfg1Spec> {
        AhbwcycW::new(self, 0)
    }
    #[doc = "Bit 16 - WAIT_FIRST_FS."]
    #[inline(always)]
    pub fn wait_first_fs(&mut self) -> WaitFirstFsW<'_, VfifoCfg1Spec> {
        WaitFirstFsW::new(self, 16)
    }
    #[doc = "Bit 17 - ACCU_FRAME_CTRL."]
    #[inline(always)]
    pub fn accu_frame_ctrl(&mut self) -> AccuFrameCtrlW<'_, VfifoCfg1Spec> {
        AccuFrameCtrlW::new(self, 17)
    }
    #[doc = "Bit 18 - ACCU_LINE_CTRL."]
    #[inline(always)]
    pub fn accu_line_ctrl(&mut self) -> AccuLineCtrlW<'_, VfifoCfg1Spec> {
        AccuLineCtrlW::new(self, 18)
    }
    #[doc = "Bit 19 - ACCU_LINE_CNT."]
    #[inline(always)]
    pub fn accu_line_cnt(&mut self) -> AccuLineCntW<'_, VfifoCfg1Spec> {
        AccuLineCntW::new(self, 19)
    }
    #[doc = "Bit 20 - ACCU_PIXEL_CNT."]
    #[inline(always)]
    pub fn accu_pixel_cnt(&mut self) -> AccuPixelCntW<'_, VfifoCfg1Spec> {
        AccuPixelCntW::new(self, 20)
    }
    #[doc = "Bit 21 - ACCU_PIXEL_ZERO."]
    #[inline(always)]
    pub fn accu_pixel_zero(&mut self) -> AccuPixelZeroW<'_, VfifoCfg1Spec> {
        AccuPixelZeroW::new(self, 21)
    }
}
#[doc = "Video FIFO Configuration Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoCfg1Spec;
impl crate::RegisterSpec for VfifoCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_cfg1::R`](R) reader structure"]
impl crate::Readable for VfifoCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_cfg1::W`](W) writer structure"]
impl crate::Writable for VfifoCfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_CFG1 to value 0"]
impl crate::Resettable for VfifoCfg1Spec {}
