#[doc = "Register `TRANS` reader"]
pub type R = crate::R<TransSpec>;
#[doc = "Register `TRANS` writer"]
pub type W = crate::W<TransSpec>;
#[doc = "DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: `1`"]
    DmaTransfer = 1,
    #[doc = "0: `0`"]
    NonDmaTransfer = 0,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_EN` reader - DMA Enable."]
pub type DmaEnR = crate::BitReader<Enable>;
impl DmaEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            true => Enable::DmaTransfer,
            false => Enable::NonDmaTransfer,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_dma_transfer(&self) -> bool {
        *self == Enable::DmaTransfer
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_non_dma_transfer(&self) -> bool {
        *self == Enable::NonDmaTransfer
    }
}
#[doc = "Field `DMA_EN` writer - DMA Enable."]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> DmaEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn dma_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::DmaTransfer)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn non_dma_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::NonDmaTransfer)
    }
}
#[doc = "Block Count Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Count {
    #[doc = "1: `1`"]
    Enable = 1,
    #[doc = "0: `0`"]
    Disable = 0,
}
impl From<Count> for bool {
    #[inline(always)]
    fn from(variant: Count) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLK_CNT_EN` reader - Block Count Enable."]
pub type BlkCntEnR = crate::BitReader<Count>;
impl BlkCntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Count {
        match self.bits {
            true => Count::Enable,
            false => Count::Disable,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Count::Enable
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Count::Disable
    }
}
#[doc = "Field `BLK_CNT_EN` writer - Block Count Enable."]
pub type BlkCntEnW<'a, REG> = crate::BitWriter<'a, REG, Count>;
impl<'a, REG> BlkCntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Count::Enable)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Count::Disable)
    }
}
#[doc = "Auto CMD Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "0: `0`"]
    Disable = 0,
    #[doc = "1: `1`"]
    Cmd12 = 1,
    #[doc = "2: `10`"]
    Cmd23 = 2,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `AUTO_CMD_EN` reader - Auto CMD Enable."]
pub type AutoCmdEnR = crate::FieldReader<Cmd>;
impl AutoCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmd> {
        match self.bits {
            0 => Some(Cmd::Disable),
            1 => Some(Cmd::Cmd12),
            2 => Some(Cmd::Cmd23),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd::Disable
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_cmd12(&self) -> bool {
        *self == Cmd::Cmd12
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_cmd23(&self) -> bool {
        *self == Cmd::Cmd23
    }
}
#[doc = "Field `AUTO_CMD_EN` writer - Auto CMD Enable."]
pub type AutoCmdEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd>;
impl<'a, REG> AutoCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Disable)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cmd12(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Cmd12)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn cmd23(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Cmd23)
    }
}
#[doc = "Data Transfer Direction Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "1: `1`"]
    Read = 1,
    #[doc = "0: `0`"]
    Write = 0,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_WRITE` reader - Data Transfer Direction Select."]
pub type ReadWriteR = crate::BitReader<Read>;
impl ReadWriteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            true => Read::Read,
            false => Read::Write,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Read::Read
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Read::Write
    }
}
#[doc = "Field `READ_WRITE` writer - Data Transfer Direction Select."]
pub type ReadWriteW<'a, REG> = crate::BitWriter<'a, REG, Read>;
impl<'a, REG> ReadWriteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Read)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Read::Write)
    }
}
#[doc = "Multi / Single Block Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Multi {
    #[doc = "1: `1`"]
    Enable = 1,
    #[doc = "0: `0`"]
    Disable = 0,
}
impl From<Multi> for bool {
    #[inline(always)]
    fn from(variant: Multi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MULTI` reader - Multi / Single Block Select."]
pub type MultiR = crate::BitReader<Multi>;
impl MultiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Multi {
        match self.bits {
            true => Multi::Enable,
            false => Multi::Disable,
        }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Multi::Enable
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Multi::Disable
    }
}
#[doc = "Field `MULTI` writer - Multi / Single Block Select."]
pub type MultiW<'a, REG> = crate::BitWriter<'a, REG, Multi>;
impl<'a, REG> MultiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Multi::Enable)
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Multi::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable."]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable."]
    #[inline(always)]
    pub fn blk_cnt_en(&self) -> BlkCntEnR {
        BlkCntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Auto CMD Enable."]
    #[inline(always)]
    pub fn auto_cmd_en(&self) -> AutoCmdEnR {
        AutoCmdEnR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select."]
    #[inline(always)]
    pub fn read_write(&self) -> ReadWriteR {
        ReadWriteR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi / Single Block Select."]
    #[inline(always)]
    pub fn multi(&self) -> MultiR {
        MultiR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable."]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<'_, TransSpec> {
        DmaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Block Count Enable."]
    #[inline(always)]
    pub fn blk_cnt_en(&mut self) -> BlkCntEnW<'_, TransSpec> {
        BlkCntEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Auto CMD Enable."]
    #[inline(always)]
    pub fn auto_cmd_en(&mut self) -> AutoCmdEnW<'_, TransSpec> {
        AutoCmdEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select."]
    #[inline(always)]
    pub fn read_write(&mut self) -> ReadWriteW<'_, TransSpec> {
        ReadWriteW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi / Single Block Select."]
    #[inline(always)]
    pub fn multi(&mut self) -> MultiW<'_, TransSpec> {
        MultiW::new(self, 5)
    }
}
#[doc = "Transfer Mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`trans::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trans::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransSpec;
impl crate::RegisterSpec for TransSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`trans::R`](R) reader structure"]
impl crate::Readable for TransSpec {}
#[doc = "`write(|w| ..)` method takes [`trans::W`](W) writer structure"]
impl crate::Writable for TransSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRANS to value 0"]
impl crate::Resettable for TransSpec {}
