#[doc = "Register `LPCN` reader"]
pub type R = crate::R<LpcnSpec>;
#[doc = "Register `LPCN` writer"]
pub type W = crate::W<LpcnSpec>;
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret0 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 0 retention."]
    En = 1,
}
impl From<Ramret0> for bool {
    #[inline(always)]
    fn from(variant: Ramret0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET0` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret0R = crate::BitReader<Ramret0>;
impl Ramret0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret0 {
        match self.bits {
            false => Ramret0::Dis,
            true => Ramret0::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret0::Dis
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret0::En
    }
}
#[doc = "Field `RAMRET0` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret0W<'a, REG> = crate::BitWriter<'a, REG, Ramret0>;
impl<'a, REG> Ramret0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret0::Dis)
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret0::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret1 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 1 retention."]
    En = 1,
}
impl From<Ramret1> for bool {
    #[inline(always)]
    fn from(variant: Ramret1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET1` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret1R = crate::BitReader<Ramret1>;
impl Ramret1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret1 {
        match self.bits {
            false => Ramret1::Dis,
            true => Ramret1::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret1::Dis
    }
    #[doc = "Enable System RAM 1 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret1::En
    }
}
#[doc = "Field `RAMRET1` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret1W<'a, REG> = crate::BitWriter<'a, REG, Ramret1>;
impl<'a, REG> Ramret1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret1::Dis)
    }
    #[doc = "Enable System RAM 1 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret1::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret2 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 2 retention."]
    En = 1,
}
impl From<Ramret2> for bool {
    #[inline(always)]
    fn from(variant: Ramret2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET2` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret2R = crate::BitReader<Ramret2>;
impl Ramret2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret2 {
        match self.bits {
            false => Ramret2::Dis,
            true => Ramret2::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret2::Dis
    }
    #[doc = "Enable System RAM 2 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret2::En
    }
}
#[doc = "Field `RAMRET2` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret2W<'a, REG> = crate::BitWriter<'a, REG, Ramret2>;
impl<'a, REG> Ramret2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret2::Dis)
    }
    #[doc = "Enable System RAM 2 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret2::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret3 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    En = 1,
}
impl From<Ramret3> for bool {
    #[inline(always)]
    fn from(variant: Ramret3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET3` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret3R = crate::BitReader<Ramret3>;
impl Ramret3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret3 {
        match self.bits {
            false => Ramret3::Dis,
            true => Ramret3::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret3::Dis
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret3::En
    }
}
#[doc = "Field `RAMRET3` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret3W<'a, REG> = crate::BitWriter<'a, REG, Ramret3>;
impl<'a, REG> Ramret3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret3::Dis)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret3::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret4 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    En = 1,
}
impl From<Ramret4> for bool {
    #[inline(always)]
    fn from(variant: Ramret4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET4` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret4R = crate::BitReader<Ramret4>;
impl Ramret4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret4 {
        match self.bits {
            false => Ramret4::Dis,
            true => Ramret4::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret4::Dis
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret4::En
    }
}
#[doc = "Field `RAMRET4` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret4W<'a, REG> = crate::BitWriter<'a, REG, Ramret4>;
impl<'a, REG> Ramret4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret4::Dis)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret4::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret5 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    En = 1,
}
impl From<Ramret5> for bool {
    #[inline(always)]
    fn from(variant: Ramret5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET5` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret5R = crate::BitReader<Ramret5>;
impl Ramret5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret5 {
        match self.bits {
            false => Ramret5::Dis,
            true => Ramret5::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret5::Dis
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret5::En
    }
}
#[doc = "Field `RAMRET5` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret5W<'a, REG> = crate::BitWriter<'a, REG, Ramret5>;
impl<'a, REG> Ramret5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret5::Dis)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret5::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret6 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    En = 1,
}
impl From<Ramret6> for bool {
    #[inline(always)]
    fn from(variant: Ramret6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET6` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret6R = crate::BitReader<Ramret6>;
impl Ramret6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret6 {
        match self.bits {
            false => Ramret6::Dis,
            true => Ramret6::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret6::Dis
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret6::En
    }
}
#[doc = "Field `RAMRET6` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret6W<'a, REG> = crate::BitWriter<'a, REG, Ramret6>;
impl<'a, REG> Ramret6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret6::Dis)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret6::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret7 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    En = 1,
}
impl From<Ramret7> for bool {
    #[inline(always)]
    fn from(variant: Ramret7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET7` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret7R = crate::BitReader<Ramret7>;
impl Ramret7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret7 {
        match self.bits {
            false => Ramret7::Dis,
            true => Ramret7::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret7::Dis
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret7::En
    }
}
#[doc = "Field `RAMRET7` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret7W<'a, REG> = crate::BitWriter<'a, REG, Ramret7>;
impl<'a, REG> Ramret7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret7::Dis)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret7::En)
    }
}
#[doc = "Field `ISOCLK_SELECT` reader - 0 = PCLK 1= ISO CLK use for RISV in Low power mode"]
pub type IsoclkSelectR = crate::BitReader;
#[doc = "Field `ISOCLK_SELECT` writer - 0 = PCLK 1= ISO CLK use for RISV in Low power mode"]
pub type IsoclkSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_ENTRY_DIS` reader - Fast Low Power mode entry disable"]
pub type FastEntryDisR = crate::BitReader;
#[doc = "Field `FAST_ENTRY_DIS` writer - Fast Low Power mode entry disable"]
pub type FastEntryDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bandgap OFF. This controls the System Bandgap in DeepSleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgoff {
    #[doc = "0: Bandgap is always ON."]
    On = 0,
    #[doc = "1: Bandgap is OFF in DeepSleep mode (default)."]
    Off = 1,
}
impl From<Bgoff> for bool {
    #[inline(always)]
    fn from(variant: Bgoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGOFF` reader - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
pub type BgoffR = crate::BitReader<Bgoff>;
impl BgoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgoff {
        match self.bits {
            false => Bgoff::On,
            true => Bgoff::Off,
        }
    }
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Bgoff::On
    }
    #[doc = "Bandgap is OFF in DeepSleep mode (default)."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Bgoff::Off
    }
}
#[doc = "Field `BGOFF` writer - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
pub type BgoffW<'a, REG> = crate::BitWriter<'a, REG, Bgoff>;
impl<'a, REG> BgoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Bgoff::On)
    }
    #[doc = "Bandgap is OFF in DeepSleep mode (default)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Bgoff::Off)
    }
}
#[doc = "Field `WKRST` reader - Reset wakeup status registers"]
pub type WkrstR = crate::BitReader;
#[doc = "Field `WKRST` writer - Reset wakeup status registers"]
pub type WkrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret0(&self) -> Ramret0R {
        Ramret0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret1(&self) -> Ramret1R {
        Ramret1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret2(&self) -> Ramret2R {
        Ramret2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret3(&self) -> Ramret3R {
        Ramret3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret4(&self) -> Ramret4R {
        Ramret4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret5(&self) -> Ramret5R {
        Ramret5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret6(&self) -> Ramret6R {
        Ramret6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret7(&self) -> Ramret7R {
        Ramret7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = PCLK 1= ISO CLK use for RISV in Low power mode"]
    #[inline(always)]
    pub fn isoclk_select(&self) -> IsoclkSelectR {
        IsoclkSelectR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Low Power mode entry disable"]
    #[inline(always)]
    pub fn fast_entry_dis(&self) -> FastEntryDisR {
        FastEntryDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bgoff(&self) -> BgoffR {
        BgoffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset wakeup status registers"]
    #[inline(always)]
    pub fn wkrst(&self) -> WkrstR {
        WkrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret0(&mut self) -> Ramret0W<'_, LpcnSpec> {
        Ramret0W::new(self, 0)
    }
    #[doc = "Bit 1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret1(&mut self) -> Ramret1W<'_, LpcnSpec> {
        Ramret1W::new(self, 1)
    }
    #[doc = "Bit 2 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret2(&mut self) -> Ramret2W<'_, LpcnSpec> {
        Ramret2W::new(self, 2)
    }
    #[doc = "Bit 3 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret3(&mut self) -> Ramret3W<'_, LpcnSpec> {
        Ramret3W::new(self, 3)
    }
    #[doc = "Bit 4 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret4(&mut self) -> Ramret4W<'_, LpcnSpec> {
        Ramret4W::new(self, 4)
    }
    #[doc = "Bit 5 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret5(&mut self) -> Ramret5W<'_, LpcnSpec> {
        Ramret5W::new(self, 5)
    }
    #[doc = "Bit 6 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret6(&mut self) -> Ramret6W<'_, LpcnSpec> {
        Ramret6W::new(self, 6)
    }
    #[doc = "Bit 7 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret7(&mut self) -> Ramret7W<'_, LpcnSpec> {
        Ramret7W::new(self, 7)
    }
    #[doc = "Bit 8 - 0 = PCLK 1= ISO CLK use for RISV in Low power mode"]
    #[inline(always)]
    pub fn isoclk_select(&mut self) -> IsoclkSelectW<'_, LpcnSpec> {
        IsoclkSelectW::new(self, 8)
    }
    #[doc = "Bit 9 - Fast Low Power mode entry disable"]
    #[inline(always)]
    pub fn fast_entry_dis(&mut self) -> FastEntryDisW<'_, LpcnSpec> {
        FastEntryDisW::new(self, 9)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bgoff(&mut self) -> BgoffW<'_, LpcnSpec> {
        BgoffW::new(self, 11)
    }
    #[doc = "Bit 31 - Reset wakeup status registers"]
    #[inline(always)]
    pub fn wkrst(&mut self) -> WkrstW<'_, LpcnSpec> {
        WkrstW::new(self, 31)
    }
}
#[doc = "Low Power Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpcnSpec;
impl crate::RegisterSpec for LpcnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcn::R`](R) reader structure"]
impl crate::Readable for LpcnSpec {}
#[doc = "`write(|w| ..)` method takes [`lpcn::W`](W) writer structure"]
impl crate::Writable for LpcnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCN to value 0"]
impl crate::Resettable for LpcnSpec {}
