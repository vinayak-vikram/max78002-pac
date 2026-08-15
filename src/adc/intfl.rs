#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `READY` reader - ADC is ready."]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - ADC is ready."]
pub type ReadyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ABORT` reader - Conversion start is aborted."]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Conversion start is aborted."]
pub type AbortW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `START_DET` reader - Conversion start is detected."]
pub type StartDetR = crate::BitReader;
#[doc = "Field `START_DET` writer - Conversion start is detected."]
pub type StartDetW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SEQ_STARTED` reader - "]
pub type SeqStartedR = crate::BitReader;
#[doc = "Field `SEQ_STARTED` writer - "]
pub type SeqStartedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SEQ_DONE` reader - "]
pub type SeqDoneR = crate::BitReader;
#[doc = "Field `SEQ_DONE` writer - "]
pub type SeqDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CONV_DONE` reader - "]
pub type ConvDoneR = crate::BitReader;
#[doc = "Field `CONV_DONE` writer - "]
pub type ConvDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLIPPED` reader - "]
pub type ClippedR = crate::BitReader;
#[doc = "Field `CLIPPED` writer - "]
pub type ClippedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_LVL` reader - "]
pub type FifoLvlR = crate::BitReader;
#[doc = "Field `FIFO_LVL` writer - "]
pub type FifoLvlW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_UFL` reader - "]
pub type FifoUflR = crate::BitReader;
#[doc = "Field `FIFO_UFL` writer - "]
pub type FifoUflW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FIFO_OFL` reader - "]
pub type FifoOflR = crate::BitReader;
#[doc = "Field `FIFO_OFL` writer - "]
pub type FifoOflW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC is ready."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Conversion start is aborted."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Conversion start is detected."]
    #[inline(always)]
    pub fn start_det(&self) -> StartDetR {
        StartDetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn seq_started(&self) -> SeqStartedR {
        SeqStartedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn seq_done(&self) -> SeqDoneR {
        SeqDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn conv_done(&self) -> ConvDoneR {
        ConvDoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clipped(&self) -> ClippedR {
        ClippedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fifo_lvl(&self) -> FifoLvlR {
        FifoLvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fifo_ufl(&self) -> FifoUflR {
        FifoUflR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fifo_ofl(&self) -> FifoOflR {
        FifoOflR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC is ready."]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<'_, IntflSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 2 - Conversion start is aborted."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<'_, IntflSpec> {
        AbortW::new(self, 2)
    }
    #[doc = "Bit 3 - Conversion start is detected."]
    #[inline(always)]
    pub fn start_det(&mut self) -> StartDetW<'_, IntflSpec> {
        StartDetW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn seq_started(&mut self) -> SeqStartedW<'_, IntflSpec> {
        SeqStartedW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn seq_done(&mut self) -> SeqDoneW<'_, IntflSpec> {
        SeqDoneW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn conv_done(&mut self) -> ConvDoneW<'_, IntflSpec> {
        ConvDoneW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clipped(&mut self) -> ClippedW<'_, IntflSpec> {
        ClippedW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fifo_lvl(&mut self) -> FifoLvlW<'_, IntflSpec> {
        FifoLvlW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fifo_ufl(&mut self) -> FifoUflW<'_, IntflSpec> {
        FifoUflW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fifo_ofl(&mut self) -> FifoOflW<'_, IntflSpec> {
        FifoOflW::new(self, 10)
    }
}
#[doc = "Interrupt Flags Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for IntflSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07fd;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {}
