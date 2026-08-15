#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Write. This bit is automatically cleared after the operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wr {
    #[doc = "0: No operation/complete."]
    Complete = 0,
    #[doc = "1: Start operation."]
    Start = 1,
}
impl From<Wr> for bool {
    #[inline(always)]
    fn from(variant: Wr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR` reader - Write. This bit is automatically cleared after the operation."]
pub type WrR = crate::BitReader<Wr>;
impl WrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wr {
        match self.bits {
            false => Wr::Complete,
            true => Wr::Start,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Wr::Complete
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Wr::Start
    }
}
#[doc = "Field `WR` writer - Write. This bit is automatically cleared after the operation."]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG, Wr>;
impl<'a, REG> WrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(Wr::Complete)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Wr::Start)
    }
}
#[doc = "Mass Erase. This bit is automatically cleared after the operation."]
pub use Wr as Me;
#[doc = "Page Erase. This bit is automatically cleared after the operation."]
pub use Wr as Pge;
#[doc = "Field `ME` reader - Mass Erase. This bit is automatically cleared after the operation."]
pub use WrR as MeR;
#[doc = "Field `PGE` reader - Page Erase. This bit is automatically cleared after the operation."]
pub use WrR as PgeR;
#[doc = "Field `ME` writer - Mass Erase. This bit is automatically cleared after the operation."]
pub use WrW as MeW;
#[doc = "Field `PGE` writer - Page Erase. This bit is automatically cleared after the operation."]
pub use WrW as PgeW;
#[doc = "Data Width. This bits selects write data width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdth {
    #[doc = "0: 128-bit."]
    Size128 = 0,
    #[doc = "1: 32-bit."]
    Size32 = 1,
}
impl From<Wdth> for bool {
    #[inline(always)]
    fn from(variant: Wdth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTH` reader - Data Width. This bits selects write data width."]
pub type WdthR = crate::BitReader<Wdth>;
impl WdthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdth {
        match self.bits {
            false => Wdth::Size128,
            true => Wdth::Size32,
        }
    }
    #[doc = "128-bit."]
    #[inline(always)]
    pub fn is_size128(&self) -> bool {
        *self == Wdth::Size128
    }
    #[doc = "32-bit."]
    #[inline(always)]
    pub fn is_size32(&self) -> bool {
        *self == Wdth::Size32
    }
}
#[doc = "Field `WDTH` writer - Data Width. This bits selects write data width."]
pub type WdthW<'a, REG> = crate::BitWriter<'a, REG, Wdth>;
impl<'a, REG> WdthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "128-bit."]
    #[inline(always)]
    pub fn size128(self) -> &'a mut crate::W<REG> {
        self.variant(Wdth::Size128)
    }
    #[doc = "32-bit."]
    #[inline(always)]
    pub fn size32(self) -> &'a mut crate::W<REG> {
        self.variant(Wdth::Size32)
    }
}
#[doc = "Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EraseCode {
    #[doc = "0: No operation."]
    Nop = 0,
    #[doc = "85: Enable Page Erase."]
    ErasePage = 85,
    #[doc = "170: Enable Mass Erase. The debug port must be enabled."]
    EraseAll = 170,
}
impl From<EraseCode> for u8 {
    #[inline(always)]
    fn from(variant: EraseCode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EraseCode {
    type Ux = u8;
}
impl crate::IsEnum for EraseCode {}
#[doc = "Field `ERASE_CODE` reader - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
pub type EraseCodeR = crate::FieldReader<EraseCode>;
impl EraseCodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EraseCode> {
        match self.bits {
            0 => Some(EraseCode::Nop),
            85 => Some(EraseCode::ErasePage),
            170 => Some(EraseCode::EraseAll),
            _ => None,
        }
    }
    #[doc = "No operation."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == EraseCode::Nop
    }
    #[doc = "Enable Page Erase."]
    #[inline(always)]
    pub fn is_erase_page(&self) -> bool {
        *self == EraseCode::ErasePage
    }
    #[doc = "Enable Mass Erase. The debug port must be enabled."]
    #[inline(always)]
    pub fn is_erase_all(&self) -> bool {
        *self == EraseCode::EraseAll
    }
}
#[doc = "Field `ERASE_CODE` writer - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
pub type EraseCodeW<'a, REG> = crate::FieldWriter<'a, REG, 8, EraseCode>;
impl<'a, REG> EraseCodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No operation."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(EraseCode::Nop)
    }
    #[doc = "Enable Page Erase."]
    #[inline(always)]
    pub fn erase_page(self) -> &'a mut crate::W<REG> {
        self.variant(EraseCode::ErasePage)
    }
    #[doc = "Enable Mass Erase. The debug port must be enabled."]
    #[inline(always)]
    pub fn erase_all(self) -> &'a mut crate::W<REG> {
        self.variant(EraseCode::EraseAll)
    }
}
#[doc = "Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pend {
    #[doc = "0: Idle."]
    Idle = 0,
    #[doc = "1: Busy."]
    Busy = 1,
}
impl From<Pend> for bool {
    #[inline(always)]
    fn from(variant: Pend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEND` reader - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
pub type PendR = crate::BitReader<Pend>;
impl PendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pend {
        match self.bits {
            false => Pend::Idle,
            true => Pend::Busy,
        }
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Pend::Idle
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Pend::Busy
    }
}
#[doc = "Field `LVE` reader - Low Voltage enable."]
pub type LveR = crate::BitReader;
#[doc = "Field `LVE` writer - Low Voltage enable."]
pub type LveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Unlock {
    #[doc = "2: Flash Unlocked."]
    Unlocked = 2,
    #[doc = "3: Flash Locked."]
    Locked = 3,
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(variant: Unlock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Unlock {
    type Ux = u8;
}
impl crate::IsEnum for Unlock {}
#[doc = "Field `UNLOCK` reader - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
pub type UnlockR = crate::FieldReader<Unlock>;
impl UnlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Unlock> {
        match self.bits {
            2 => Some(Unlock::Unlocked),
            3 => Some(Unlock::Locked),
            _ => None,
        }
    }
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Unlock::Unlocked
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Unlock::Locked
    }
}
#[doc = "Field `UNLOCK` writer - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
pub type UnlockW<'a, REG> = crate::FieldWriter<'a, REG, 4, Unlock>;
impl<'a, REG> UnlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash Unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Unlock::Unlocked)
    }
    #[doc = "Flash Locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Unlock::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn me(&self) -> MeR {
        MeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn pge(&self) -> PgeR {
        PgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Width. This bits selects write data width."]
    #[inline(always)]
    pub fn wdth(&self) -> WdthR {
        WdthR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    pub fn erase_code(&self) -> EraseCodeR {
        EraseCodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flash Pending. When Flash operation is in progress (busy), Flash reads and writes will fail. When PEND is set, write to all Flash registers, with exception of the Flash interrupt register, are ignored."]
    #[inline(always)]
    pub fn pend(&self) -> PendR {
        PendR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    pub fn lve(&self) -> LveR {
        LveR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn wr(&mut self) -> WrW<'_, CtrlSpec> {
        WrW::new(self, 0)
    }
    #[doc = "Bit 1 - Mass Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn me(&mut self) -> MeW<'_, CtrlSpec> {
        MeW::new(self, 1)
    }
    #[doc = "Bit 2 - Page Erase. This bit is automatically cleared after the operation."]
    #[inline(always)]
    pub fn pge(&mut self) -> PgeW<'_, CtrlSpec> {
        PgeW::new(self, 2)
    }
    #[doc = "Bit 4 - Data Width. This bits selects write data width."]
    #[inline(always)]
    pub fn wdth(&mut self) -> WdthW<'_, CtrlSpec> {
        WdthW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Erase Code. The ERASE_CODE must be set up property before erase operation can be initiated. These bits are automatically cleared after the operation is complete."]
    #[inline(always)]
    pub fn erase_code(&mut self) -> EraseCodeW<'_, CtrlSpec> {
        EraseCodeW::new(self, 8)
    }
    #[doc = "Bit 25 - Low Voltage enable."]
    #[inline(always)]
    pub fn lve(&mut self) -> LveW<'_, CtrlSpec> {
        LveW::new(self, 25)
    }
    #[doc = "Bits 28:31 - Flash Unlock. The correct unlock code must be written to these four bits before any Flash write or erase operation is allowed."]
    #[inline(always)]
    pub fn unlock(&mut self) -> UnlockW<'_, CtrlSpec> {
        UnlockW::new(self, 28)
    }
}
#[doc = "Flash Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
