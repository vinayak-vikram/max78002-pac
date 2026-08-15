#[doc = "Register `VFIFO_CFG0` reader"]
pub type R = crate::R<VfifoCfg0Spec>;
#[doc = "Register `VFIFO_CFG0` writer"]
pub type W = crate::W<VfifoCfg0Spec>;
#[doc = "Field `VC` reader - CSI Virtual Channel."]
pub type VcR = crate::FieldReader;
#[doc = "Field `VC` writer - CSI Virtual Channel."]
pub type VcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "DMA Mode, the condition to trigger DMA request..\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmamode {
    #[doc = "0: No DMA."]
    NoDma = 0,
    #[doc = "1: Immediately send DMA request."]
    DmaReq = 1,
    #[doc = "2: Wait for FIFO above threshold."]
    FifoThd = 2,
    #[doc = "3: Wait for FIFO is full."]
    FifoFull = 3,
}
impl From<Dmamode> for u8 {
    #[inline(always)]
    fn from(variant: Dmamode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmamode {
    type Ux = u8;
}
impl crate::IsEnum for Dmamode {}
#[doc = "Field `DMAMODE` reader - DMA Mode, the condition to trigger DMA request.."]
pub type DmamodeR = crate::FieldReader<Dmamode>;
impl DmamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamode {
        match self.bits {
            0 => Dmamode::NoDma,
            1 => Dmamode::DmaReq,
            2 => Dmamode::FifoThd,
            3 => Dmamode::FifoFull,
            _ => unreachable!(),
        }
    }
    #[doc = "No DMA."]
    #[inline(always)]
    pub fn is_no_dma(&self) -> bool {
        *self == Dmamode::NoDma
    }
    #[doc = "Immediately send DMA request."]
    #[inline(always)]
    pub fn is_dma_req(&self) -> bool {
        *self == Dmamode::DmaReq
    }
    #[doc = "Wait for FIFO above threshold."]
    #[inline(always)]
    pub fn is_fifo_thd(&self) -> bool {
        *self == Dmamode::FifoThd
    }
    #[doc = "Wait for FIFO is full."]
    #[inline(always)]
    pub fn is_fifo_full(&self) -> bool {
        *self == Dmamode::FifoFull
    }
}
#[doc = "Field `DMAMODE` writer - DMA Mode, the condition to trigger DMA request.."]
pub type DmamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmamode, crate::Safe>;
impl<'a, REG> DmamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA."]
    #[inline(always)]
    pub fn no_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::NoDma)
    }
    #[doc = "Immediately send DMA request."]
    #[inline(always)]
    pub fn dma_req(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::DmaReq)
    }
    #[doc = "Wait for FIFO above threshold."]
    #[inline(always)]
    pub fn fifo_thd(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::FifoThd)
    }
    #[doc = "Wait for FIFO is full."]
    #[inline(always)]
    pub fn fifo_full(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::FifoFull)
    }
}
#[doc = "Field `AHBWAIT` reader - AHB Wait Enable."]
pub type AhbwaitR = crate::BitReader;
#[doc = "Field `AHBWAIT` writer - AHB Wait Enable."]
pub type AhbwaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFORM` reader - FIFO Read Mode."]
pub type FiformR = crate::BitReader;
#[doc = "Field `FIFORM` writer - FIFO Read Mode."]
pub type FiformW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRDE` reader - Error Detection Enable."]
pub type ErrdeR = crate::BitReader;
#[doc = "Field `ERRDE` writer - Error Detection Enable."]
pub type ErrdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWM` reader - Full Band Width mode."]
pub type FbwmR = crate::BitReader;
#[doc = "Field `FBWM` writer - Full Band Width mode."]
pub type FbwmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CSI Virtual Channel."]
    #[inline(always)]
    pub fn vc(&self) -> VcR {
        VcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:7 - DMA Mode, the condition to trigger DMA request.."]
    #[inline(always)]
    pub fn dmamode(&self) -> DmamodeR {
        DmamodeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - AHB Wait Enable."]
    #[inline(always)]
    pub fn ahbwait(&self) -> AhbwaitR {
        AhbwaitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO Read Mode."]
    #[inline(always)]
    pub fn fiform(&self) -> FiformR {
        FiformR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error Detection Enable."]
    #[inline(always)]
    pub fn errde(&self) -> ErrdeR {
        ErrdeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Full Band Width mode."]
    #[inline(always)]
    pub fn fbwm(&self) -> FbwmR {
        FbwmR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSI Virtual Channel."]
    #[inline(always)]
    pub fn vc(&mut self) -> VcW<'_, VfifoCfg0Spec> {
        VcW::new(self, 0)
    }
    #[doc = "Bits 6:7 - DMA Mode, the condition to trigger DMA request.."]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DmamodeW<'_, VfifoCfg0Spec> {
        DmamodeW::new(self, 6)
    }
    #[doc = "Bit 8 - AHB Wait Enable."]
    #[inline(always)]
    pub fn ahbwait(&mut self) -> AhbwaitW<'_, VfifoCfg0Spec> {
        AhbwaitW::new(self, 8)
    }
    #[doc = "Bit 9 - FIFO Read Mode."]
    #[inline(always)]
    pub fn fiform(&mut self) -> FiformW<'_, VfifoCfg0Spec> {
        FiformW::new(self, 9)
    }
    #[doc = "Bit 10 - Error Detection Enable."]
    #[inline(always)]
    pub fn errde(&mut self) -> ErrdeW<'_, VfifoCfg0Spec> {
        ErrdeW::new(self, 10)
    }
    #[doc = "Bit 11 - Full Band Width mode."]
    #[inline(always)]
    pub fn fbwm(&mut self) -> FbwmW<'_, VfifoCfg0Spec> {
        FbwmW::new(self, 11)
    }
}
#[doc = "Video FIFO Configuration Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoCfg0Spec;
impl crate::RegisterSpec for VfifoCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_cfg0::R`](R) reader structure"]
impl crate::Readable for VfifoCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_cfg0::W`](W) writer structure"]
impl crate::Writable for VfifoCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_CFG0 to value 0"]
impl crate::Resettable for VfifoCfg0Spec {}
