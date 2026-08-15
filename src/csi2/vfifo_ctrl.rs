#[doc = "Register `VFIFO_CTRL` reader"]
pub type R = crate::R<VfifoCtrlSpec>;
#[doc = "Register `VFIFO_CTRL` writer"]
pub type W = crate::W<VfifoCtrlSpec>;
#[doc = "Video FIFO Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoen {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` reader - Video FIFO Enable."]
pub type FifoenR = crate::BitReader<Fifoen>;
impl FifoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoen {
        match self.bits {
            false => Fifoen::Dis,
            true => Fifoen::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Fifoen::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Fifoen::En
    }
}
#[doc = "Field `FIFOEN` writer - Video FIFO Enable."]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG, Fifoen>;
impl<'a, REG> FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::En)
    }
}
#[doc = "Field `FLUSH` reader - Write 1 to flush FIFO contents."]
pub type FlushR = crate::BitReader;
#[doc = "Field `FLUSH` writer - Write 1 to flush FIFO contents."]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THD` reader - FIFO Threshold."]
pub type ThdR = crate::FieldReader;
#[doc = "Field `THD` writer - FIFO Threshold."]
pub type ThdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Video FIFO Enable."]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to flush FIFO contents."]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:14 - FIFO Threshold."]
    #[inline(always)]
    pub fn thd(&self) -> ThdR {
        ThdR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Video FIFO Enable."]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, VfifoCtrlSpec> {
        FifoenW::new(self, 0)
    }
    #[doc = "Bit 4 - Write 1 to flush FIFO contents."]
    #[inline(always)]
    pub fn flush(&mut self) -> FlushW<'_, VfifoCtrlSpec> {
        FlushW::new(self, 4)
    }
    #[doc = "Bits 8:14 - FIFO Threshold."]
    #[inline(always)]
    pub fn thd(&mut self) -> ThdW<'_, VfifoCtrlSpec> {
        ThdW::new(self, 8)
    }
}
#[doc = "Video FIFO Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoCtrlSpec;
impl crate::RegisterSpec for VfifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ctrl::R`](R) reader structure"]
impl crate::Readable for VfifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ctrl::W`](W) writer structure"]
impl crate::Writable for VfifoCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_CTRL to value 0"]
impl crate::Resettable for VfifoCtrlSpec {}
