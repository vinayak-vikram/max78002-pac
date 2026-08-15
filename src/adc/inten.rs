#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `READY` reader - ADC is ready."]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - ADC is ready."]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - Conversion start is aborted."]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Conversion start is aborted."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_DET` reader - Conversion start is detected."]
pub type StartDetR = crate::BitReader;
#[doc = "Field `START_DET` writer - Conversion start is detected."]
pub type StartDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_STARTED` reader - "]
pub type SeqStartedR = crate::BitReader;
#[doc = "Field `SEQ_STARTED` writer - "]
pub type SeqStartedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_DONE` reader - "]
pub type SeqDoneR = crate::BitReader;
#[doc = "Field `SEQ_DONE` writer - "]
pub type SeqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONV_DONE` reader - "]
pub type ConvDoneR = crate::BitReader;
#[doc = "Field `CONV_DONE` writer - "]
pub type ConvDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLIPPED` reader - "]
pub type ClippedR = crate::BitReader;
#[doc = "Field `CLIPPED` writer - "]
pub type ClippedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_LVL` reader - "]
pub type FifoLvlR = crate::BitReader;
#[doc = "Field `FIFO_LVL` writer - "]
pub type FifoLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_UFL` reader - "]
pub type FifoUflR = crate::BitReader;
#[doc = "Field `FIFO_UFL` writer - "]
pub type FifoUflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_OFL` reader - "]
pub type FifoOflR = crate::BitReader;
#[doc = "Field `FIFO_OFL` writer - "]
pub type FifoOflW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn ready(&mut self) -> ReadyW<'_, IntenSpec> {
        ReadyW::new(self, 0)
    }
    #[doc = "Bit 2 - Conversion start is aborted."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<'_, IntenSpec> {
        AbortW::new(self, 2)
    }
    #[doc = "Bit 3 - Conversion start is detected."]
    #[inline(always)]
    pub fn start_det(&mut self) -> StartDetW<'_, IntenSpec> {
        StartDetW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn seq_started(&mut self) -> SeqStartedW<'_, IntenSpec> {
        SeqStartedW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn seq_done(&mut self) -> SeqDoneW<'_, IntenSpec> {
        SeqDoneW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn conv_done(&mut self) -> ConvDoneW<'_, IntenSpec> {
        ConvDoneW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clipped(&mut self) -> ClippedW<'_, IntenSpec> {
        ClippedW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fifo_lvl(&mut self) -> FifoLvlW<'_, IntenSpec> {
        FifoLvlW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fifo_ufl(&mut self) -> FifoUflW<'_, IntenSpec> {
        FifoUflW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fifo_ofl(&mut self) -> FifoOflW<'_, IntenSpec> {
        FifoOflW::new(self, 10)
    }
}
#[doc = "Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
