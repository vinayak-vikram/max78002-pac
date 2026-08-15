#[doc = "Register `INCSRU` reader"]
pub type R = crate::R<IncsruSpec>;
#[doc = "Register `INCSRU` writer"]
pub type W = crate::W<IncsruSpec>;
#[doc = "Double Packet Buffering Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpktbufdis {
    #[doc = "0: Enable Double packet buffering."]
    En = 0,
    #[doc = "1: Disable Double Packet Buffering."]
    Dis = 1,
}
impl From<Dpktbufdis> for bool {
    #[inline(always)]
    fn from(variant: Dpktbufdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPKTBUFDIS` reader - Double Packet Buffering Disable"]
pub type DpktbufdisR = crate::BitReader<Dpktbufdis>;
impl DpktbufdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpktbufdis {
        match self.bits {
            false => Dpktbufdis::En,
            true => Dpktbufdis::Dis,
        }
    }
    #[doc = "Enable Double packet buffering."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dpktbufdis::En
    }
    #[doc = "Disable Double Packet Buffering."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dpktbufdis::Dis
    }
}
#[doc = "Field `DPKTBUFDIS` writer - Double Packet Buffering Disable"]
pub type DpktbufdisW<'a, REG> = crate::BitWriter<'a, REG, Dpktbufdis>;
impl<'a, REG> DpktbufdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable Double packet buffering."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dpktbufdis::En)
    }
    #[doc = "Disable Double Packet Buffering."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dpktbufdis::Dis)
    }
}
#[doc = "Force In Data - Toggle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frcdatatog {
    #[doc = "0: Toggle data-toglge only when an ACK is received."]
    Received = 0,
    #[doc = "1: Toggle data-toggle regardless of ACK."]
    Dontcare = 1,
}
impl From<Frcdatatog> for bool {
    #[inline(always)]
    fn from(variant: Frcdatatog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRCDATATOG` reader - Force In Data - Toggle"]
pub type FrcdatatogR = crate::BitReader<Frcdatatog>;
impl FrcdatatogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frcdatatog {
        match self.bits {
            false => Frcdatatog::Received,
            true => Frcdatatog::Dontcare,
        }
    }
    #[doc = "Toggle data-toglge only when an ACK is received."]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == Frcdatatog::Received
    }
    #[doc = "Toggle data-toggle regardless of ACK."]
    #[inline(always)]
    pub fn is_dontcare(&self) -> bool {
        *self == Frcdatatog::Dontcare
    }
}
#[doc = "Field `FRCDATATOG` writer - Force In Data - Toggle"]
pub type FrcdatatogW<'a, REG> = crate::BitWriter<'a, REG, Frcdatatog>;
impl<'a, REG> FrcdatatogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Toggle data-toglge only when an ACK is received."]
    #[inline(always)]
    pub fn received(self) -> &'a mut crate::W<REG> {
        self.variant(Frcdatatog::Received)
    }
    #[doc = "Toggle data-toggle regardless of ACK."]
    #[inline(always)]
    pub fn dontcare(self) -> &'a mut crate::W<REG> {
        self.variant(Frcdatatog::Dontcare)
    }
}
#[doc = "Endpoint Direction Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Endpoint direction is OUT."]
    Out = 0,
    #[doc = "1: Endpoint direction is IN."]
    In = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Endpoint Direction Mode."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Out,
            true => Mode::In,
        }
    }
    #[doc = "Endpoint direction is OUT."]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == Mode::Out
    }
    #[doc = "Endpoint direction is IN."]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == Mode::In
    }
}
#[doc = "Field `MODE` writer - Endpoint Direction Mode."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Endpoint direction is OUT."]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Out)
    }
    #[doc = "Endpoint direction is IN."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::In)
    }
}
#[doc = "Isochronous Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso {
    #[doc = "0: Enable IN Bulk and IN interrupt transfers."]
    Interrupt = 0,
    #[doc = "1: Enable IN Isochronous transfers."]
    Isochronous = 1,
}
impl From<Iso> for bool {
    #[inline(always)]
    fn from(variant: Iso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO` reader - Isochronous Transfer Enable"]
pub type IsoR = crate::BitReader<Iso>;
impl IsoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso {
        match self.bits {
            false => Iso::Interrupt,
            true => Iso::Isochronous,
        }
    }
    #[doc = "Enable IN Bulk and IN interrupt transfers."]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Iso::Interrupt
    }
    #[doc = "Enable IN Isochronous transfers."]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == Iso::Isochronous
    }
}
#[doc = "Field `ISO` writer - Isochronous Transfer Enable"]
pub type IsoW<'a, REG> = crate::BitWriter<'a, REG, Iso>;
impl<'a, REG> IsoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable IN Bulk and IN interrupt transfers."]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Iso::Interrupt)
    }
    #[doc = "Enable IN Isochronous transfers."]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut crate::W<REG> {
        self.variant(Iso::Isochronous)
    }
}
#[doc = "Auto Set inpktrdy.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoset {
    #[doc = "0: USBHS_INCSRL_inpktrdy must be set by firmware."]
    Set = 0,
    #[doc = "1: USBHS_INCSRL_inpktrdy is automatically set."]
    Auto = 1,
}
impl From<Autoset> for bool {
    #[inline(always)]
    fn from(variant: Autoset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOSET` reader - Auto Set inpktrdy."]
pub type AutosetR = crate::BitReader<Autoset>;
impl AutosetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoset {
        match self.bits {
            false => Autoset::Set,
            true => Autoset::Auto,
        }
    }
    #[doc = "USBHS_INCSRL_inpktrdy must be set by firmware."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Autoset::Set
    }
    #[doc = "USBHS_INCSRL_inpktrdy is automatically set."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Autoset::Auto
    }
}
#[doc = "Field `AUTOSET` writer - Auto Set inpktrdy."]
pub type AutosetW<'a, REG> = crate::BitWriter<'a, REG, Autoset>;
impl<'a, REG> AutosetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USBHS_INCSRL_inpktrdy must be set by firmware."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Autoset::Set)
    }
    #[doc = "USBHS_INCSRL_inpktrdy is automatically set."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Autoset::Auto)
    }
}
impl R {
    #[doc = "Bit 1 - Double Packet Buffering Disable"]
    #[inline(always)]
    pub fn dpktbufdis(&self) -> DpktbufdisR {
        DpktbufdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Force In Data - Toggle"]
    #[inline(always)]
    pub fn frcdatatog(&self) -> FrcdatatogR {
        FrcdatatogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint Direction Mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Isochronous Transfer Enable"]
    #[inline(always)]
    pub fn iso(&self) -> IsoR {
        IsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto Set inpktrdy."]
    #[inline(always)]
    pub fn autoset(&self) -> AutosetR {
        AutosetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Double Packet Buffering Disable"]
    #[inline(always)]
    pub fn dpktbufdis(&mut self) -> DpktbufdisW<'_, IncsruSpec> {
        DpktbufdisW::new(self, 1)
    }
    #[doc = "Bit 3 - Force In Data - Toggle"]
    #[inline(always)]
    pub fn frcdatatog(&mut self) -> FrcdatatogW<'_, IncsruSpec> {
        FrcdatatogW::new(self, 3)
    }
    #[doc = "Bit 5 - Endpoint Direction Mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, IncsruSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 6 - Isochronous Transfer Enable"]
    #[inline(always)]
    pub fn iso(&mut self) -> IsoW<'_, IncsruSpec> {
        IsoW::new(self, 6)
    }
    #[doc = "Bit 7 - Auto Set inpktrdy."]
    #[inline(always)]
    pub fn autoset(&mut self) -> AutosetW<'_, IncsruSpec> {
        AutosetW::new(self, 7)
    }
}
#[doc = "Control status upper register for INx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`incsru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`incsru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IncsruSpec;
impl crate::RegisterSpec for IncsruSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`incsru::R`](R) reader structure"]
impl crate::Readable for IncsruSpec {}
#[doc = "`write(|w| ..)` method takes [`incsru::W`](W) writer structure"]
impl crate::Writable for IncsruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INCSRU to value 0"]
impl crate::Resettable for IncsruSpec {}
