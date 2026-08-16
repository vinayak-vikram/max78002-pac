#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `en` reader - Enable. During arming this is set on the non-master quadrants and left clear on the master; the master receives it in the final start write."]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - Enable. During arming this is set on the non-master quadrants and left clear on the master; the master receives it in the final start write."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rdy_sel` reader - APB ready wait select."]
pub type RdySelR = crate::FieldReader;
#[doc = "Field `rdy_sel` writer - APB ready wait select."]
pub type RdySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `clk_en` reader - Clock enable. Must be set before any other quadrant access."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `clk_en` writer - Clock enable. Must be set before any other quadrant access."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ext_rdy` reader - External ready."]
pub type ExtRdyR = crate::BitReader;
#[doc = "Field `ext_rdy` writer - External ready."]
pub type ExtRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `no_pipeline` reader - Disable the pipeline. Note the inverted sense: set this to select non-pipelined mode, which is limited to 50MHz."]
pub type NoPipelineR = crate::BitReader;
#[doc = "Field `no_pipeline` writer - Disable the pipeline. Note the inverted sense: set this to select non-pipelined mode, which is limited to 50MHz."]
pub type NoPipelineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stream_chw` reader - Start layer is both streaming and CHW format."]
pub type StreamChwR = crate::BitReader;
#[doc = "Field `stream_chw` writer - Start layer is both streaming and CHW format."]
pub type StreamChwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `apbclkena` reader - Keep the APB clock enabled so registers remain readable while the state machine runs. Required when using the snoop registers."]
pub type ApbclkenaR = crate::BitReader;
#[doc = "Field `apbclkena` writer - Keep the APB clock enabled so registers remain readable while the state machine runs. Required when using the snoop registers."]
pub type ApbclkenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `oneshot` reader - One-shot mode."]
pub type OneshotR = crate::BitReader;
#[doc = "Field `oneshot` writer - One-shot mode."]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `master` reader - Index of the master quadrant."]
pub type MasterR = crate::FieldReader;
#[doc = "Field `master` writer - Index of the master quadrant."]
pub type MasterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ext_sync` reader - External sync. Set on every quadrant during arming, and set globally when a FIFO is in use."]
pub type ExtSyncR = crate::BitReader;
#[doc = "Field `ext_sync` writer - External sync. Set on every quadrant during arming, and set globally when a FIFO is in use."]
pub type ExtSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irq` reader - Completion flag. Write zero to acknowledge."]
pub type IrqR = crate::BitReader;
#[doc = "Field `irq` writer - Completion flag. Write zero to acknowledge."]
pub type IrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pool_rnd` reader - Average pool rounding."]
pub type PoolRndR = crate::BitReader;
#[doc = "Field `pool_rnd` writer - Average pool rounding."]
pub type PoolRndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stream_en` reader - Any layer uses streaming."]
pub type StreamEnR = crate::BitReader;
#[doc = "Field `stream_en` writer - Any layer uses streaming."]
pub type StreamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_en` reader - FIFO enabled for this quadrant."]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `fifo_en` writer - FIFO enabled for this quadrant."]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mlat_en` reader - Mlator enable. Used only on the mlator unload path."]
pub type MlatEnR = crate::BitReader;
#[doc = "Field `mlat_en` writer - Mlator enable. Used only on the mlator unload path."]
pub type MlatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mlat_sel` reader - Mlator byte select; each byte maps to a channel."]
pub type MlatSelR = crate::FieldReader;
#[doc = "Field `mlat_sel` writer - Mlator byte select; each byte maps to a channel."]
pub type MlatSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `stream_fifo` reader - Set only when FIFO and streaming are both active."]
pub type StreamFifoR = crate::BitReader;
#[doc = "Field `stream_fifo` writer - Set only when FIFO and streaming are both active."]
pub type StreamFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mexpress` reader - Compressed kernel load format."]
pub type MexpressR = crate::BitReader;
#[doc = "Field `mexpress` writer - Compressed kernel load format."]
pub type MexpressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `simple1b` reader - Simple 1-bit weight mode."]
pub type Simple1bR = crate::BitReader;
#[doc = "Field `simple1b` writer - Simple 1-bit weight mode."]
pub type Simple1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fast_fifo` reader - Source is the fast FIFO rather than the APB FIFO."]
pub type FastFifoR = crate::BitReader;
#[doc = "Field `fast_fifo` writer - Source is the fast FIFO rather than the APB FIFO."]
pub type FastFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fifo_group` reader - Fast FIFO group mode."]
pub type FifoGroupR = crate::BitReader;
#[doc = "Field `fifo_group` writer - Fast FIFO group mode."]
pub type FifoGroupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bin_weights` reader - Binary weights present in the network."]
pub type BinWeightsR = crate::BitReader;
#[doc = "Field `bin_weights` writer - Binary weights present in the network."]
pub type BinWeightsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `qupac` reader - Replicate the input map across all four quadrants."]
pub type QupacR = crate::BitReader;
#[doc = "Field `qupac` writer - Replicate the input map across all four quadrants."]
pub type QupacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable. During arming this is set on the non-master quadrants and left clear on the master; the master receives it in the final start write."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - APB ready wait select."]
    #[inline(always)]
    pub fn rdy_sel(&self) -> RdySelR {
        RdySelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Clock enable. Must be set before any other quadrant access."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External ready."]
    #[inline(always)]
    pub fn ext_rdy(&self) -> ExtRdyR {
        ExtRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable the pipeline. Note the inverted sense: set this to select non-pipelined mode, which is limited to 50MHz."]
    #[inline(always)]
    pub fn no_pipeline(&self) -> NoPipelineR {
        NoPipelineR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start layer is both streaming and CHW format."]
    #[inline(always)]
    pub fn stream_chw(&self) -> StreamChwR {
        StreamChwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Keep the APB clock enabled so registers remain readable while the state machine runs. Required when using the snoop registers."]
    #[inline(always)]
    pub fn apbclkena(&self) -> ApbclkenaR {
        ApbclkenaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - One-shot mode."]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Index of the master quadrant."]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - External sync. Set on every quadrant during arming, and set globally when a FIFO is in use."]
    #[inline(always)]
    pub fn ext_sync(&self) -> ExtSyncR {
        ExtSyncR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Completion flag. Write zero to acknowledge."]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Average pool rounding."]
    #[inline(always)]
    pub fn pool_rnd(&self) -> PoolRndR {
        PoolRndR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Any layer uses streaming."]
    #[inline(always)]
    pub fn stream_en(&self) -> StreamEnR {
        StreamEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - FIFO enabled for this quadrant."]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mlator enable. Used only on the mlator unload path."]
    #[inline(always)]
    pub fn mlat_en(&self) -> MlatEnR {
        MlatEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Mlator byte select; each byte maps to a channel."]
    #[inline(always)]
    pub fn mlat_sel(&self) -> MlatSelR {
        MlatSelR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Set only when FIFO and streaming are both active."]
    #[inline(always)]
    pub fn stream_fifo(&self) -> StreamFifoR {
        StreamFifoR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Compressed kernel load format."]
    #[inline(always)]
    pub fn mexpress(&self) -> MexpressR {
        MexpressR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Simple 1-bit weight mode."]
    #[inline(always)]
    pub fn simple1b(&self) -> Simple1bR {
        Simple1bR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Source is the fast FIFO rather than the APB FIFO."]
    #[inline(always)]
    pub fn fast_fifo(&self) -> FastFifoR {
        FastFifoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fast FIFO group mode."]
    #[inline(always)]
    pub fn fifo_group(&self) -> FifoGroupR {
        FifoGroupR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Binary weights present in the network."]
    #[inline(always)]
    pub fn bin_weights(&self) -> BinWeightsR {
        BinWeightsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Replicate the input map across all four quadrants."]
    #[inline(always)]
    pub fn qupac(&self) -> QupacR {
        QupacR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. During arming this is set on the non-master quadrants and left clear on the master; the master receives it in the final start write."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - APB ready wait select."]
    #[inline(always)]
    pub fn rdy_sel(&mut self) -> RdySelW<'_, CtlSpec> {
        RdySelW::new(self, 1)
    }
    #[doc = "Bit 3 - Clock enable. Must be set before any other quadrant access."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, CtlSpec> {
        ClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - External ready."]
    #[inline(always)]
    pub fn ext_rdy(&mut self) -> ExtRdyW<'_, CtlSpec> {
        ExtRdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable the pipeline. Note the inverted sense: set this to select non-pipelined mode, which is limited to 50MHz."]
    #[inline(always)]
    pub fn no_pipeline(&mut self) -> NoPipelineW<'_, CtlSpec> {
        NoPipelineW::new(self, 5)
    }
    #[doc = "Bit 6 - Start layer is both streaming and CHW format."]
    #[inline(always)]
    pub fn stream_chw(&mut self) -> StreamChwW<'_, CtlSpec> {
        StreamChwW::new(self, 6)
    }
    #[doc = "Bit 7 - Keep the APB clock enabled so registers remain readable while the state machine runs. Required when using the snoop registers."]
    #[inline(always)]
    pub fn apbclkena(&mut self) -> ApbclkenaW<'_, CtlSpec> {
        ApbclkenaW::new(self, 7)
    }
    #[doc = "Bit 8 - One-shot mode."]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<'_, CtlSpec> {
        OneshotW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Index of the master quadrant."]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<'_, CtlSpec> {
        MasterW::new(self, 9)
    }
    #[doc = "Bit 11 - External sync. Set on every quadrant during arming, and set globally when a FIFO is in use."]
    #[inline(always)]
    pub fn ext_sync(&mut self) -> ExtSyncW<'_, CtlSpec> {
        ExtSyncW::new(self, 11)
    }
    #[doc = "Bit 12 - Completion flag. Write zero to acknowledge."]
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<'_, CtlSpec> {
        IrqW::new(self, 12)
    }
    #[doc = "Bit 13 - Average pool rounding."]
    #[inline(always)]
    pub fn pool_rnd(&mut self) -> PoolRndW<'_, CtlSpec> {
        PoolRndW::new(self, 13)
    }
    #[doc = "Bit 14 - Any layer uses streaming."]
    #[inline(always)]
    pub fn stream_en(&mut self) -> StreamEnW<'_, CtlSpec> {
        StreamEnW::new(self, 14)
    }
    #[doc = "Bit 15 - FIFO enabled for this quadrant."]
    #[inline(always)]
    pub fn fifo_en(&mut self) -> FifoEnW<'_, CtlSpec> {
        FifoEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Mlator enable. Used only on the mlator unload path."]
    #[inline(always)]
    pub fn mlat_en(&mut self) -> MlatEnW<'_, CtlSpec> {
        MlatEnW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Mlator byte select; each byte maps to a channel."]
    #[inline(always)]
    pub fn mlat_sel(&mut self) -> MlatSelW<'_, CtlSpec> {
        MlatSelW::new(self, 17)
    }
    #[doc = "Bit 19 - Set only when FIFO and streaming are both active."]
    #[inline(always)]
    pub fn stream_fifo(&mut self) -> StreamFifoW<'_, CtlSpec> {
        StreamFifoW::new(self, 19)
    }
    #[doc = "Bit 20 - Compressed kernel load format."]
    #[inline(always)]
    pub fn mexpress(&mut self) -> MexpressW<'_, CtlSpec> {
        MexpressW::new(self, 20)
    }
    #[doc = "Bit 21 - Simple 1-bit weight mode."]
    #[inline(always)]
    pub fn simple1b(&mut self) -> Simple1bW<'_, CtlSpec> {
        Simple1bW::new(self, 21)
    }
    #[doc = "Bit 22 - Source is the fast FIFO rather than the APB FIFO."]
    #[inline(always)]
    pub fn fast_fifo(&mut self) -> FastFifoW<'_, CtlSpec> {
        FastFifoW::new(self, 22)
    }
    #[doc = "Bit 23 - Fast FIFO group mode."]
    #[inline(always)]
    pub fn fifo_group(&mut self) -> FifoGroupW<'_, CtlSpec> {
        FifoGroupW::new(self, 23)
    }
    #[doc = "Bit 30 - Binary weights present in the network."]
    #[inline(always)]
    pub fn bin_weights(&mut self) -> BinWeightsW<'_, CtlSpec> {
        BinWeightsW::new(self, 30)
    }
    #[doc = "Bit 31 - Replicate the input map across all four quadrants."]
    #[inline(always)]
    pub fn qupac(&mut self) -> QupacW<'_, CtlSpec> {
        QupacW::new(self, 31)
    }
}
#[doc = "Quadrant control. Bits other than those named below are written only as part of documented composite values.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
