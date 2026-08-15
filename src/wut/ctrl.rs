#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Timer Mode Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tmode {
    #[doc = "0: One Shot Mode."]
    OneShot = 0,
    #[doc = "1: Continuous Mode."]
    Continuous = 1,
    #[doc = "2: Counter Mode."]
    Counter = 2,
    #[doc = "3: PWM Mode."]
    Pwm = 3,
    #[doc = "4: Capture Mode."]
    Capture = 4,
    #[doc = "5: Compare Mode."]
    Compare = 5,
    #[doc = "6: Gated Mode."]
    Gated = 6,
    #[doc = "7: Capture/Compare Mode."]
    CaptureCompare = 7,
}
impl From<Tmode> for u8 {
    #[inline(always)]
    fn from(variant: Tmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tmode {
    type Ux = u8;
}
impl crate::IsEnum for Tmode {}
#[doc = "Field `TMODE` reader - Timer Mode Select."]
pub type TmodeR = crate::FieldReader<Tmode>;
impl TmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmode {
        match self.bits {
            0 => Tmode::OneShot,
            1 => Tmode::Continuous,
            2 => Tmode::Counter,
            3 => Tmode::Pwm,
            4 => Tmode::Capture,
            5 => Tmode::Compare,
            6 => Tmode::Gated,
            7 => Tmode::CaptureCompare,
            _ => unreachable!(),
        }
    }
    #[doc = "One Shot Mode."]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == Tmode::OneShot
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Tmode::Continuous
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Tmode::Counter
    }
    #[doc = "PWM Mode."]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Tmode::Pwm
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Tmode::Capture
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == Tmode::Compare
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == Tmode::Gated
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == Tmode::CaptureCompare
    }
}
#[doc = "Field `TMODE` writer - Timer Mode Select."]
pub type TmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tmode, crate::Safe>;
impl<'a, REG> TmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One Shot Mode."]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::OneShot)
    }
    #[doc = "Continuous Mode."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Continuous)
    }
    #[doc = "Counter Mode."]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Counter)
    }
    #[doc = "PWM Mode."]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Pwm)
    }
    #[doc = "Capture Mode."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Capture)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Compare)
    }
    #[doc = "Gated Mode."]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::Gated)
    }
    #[doc = "Capture/Compare Mode."]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(Tmode::CaptureCompare)
    }
}
#[doc = "Timer Prescaler Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pres {
    #[doc = "0: Divide by 1 (PRES3 = 0)."]
    Div1 = 0,
    #[doc = "1: Divide by 2 (PRES3 = 0)."]
    Div2 = 1,
    #[doc = "2: Divide by 4 (PRES3 = 0)."]
    Div4 = 2,
    #[doc = "3: Divide by 8 (PRES3 = 0)."]
    Div8 = 3,
    #[doc = "4: Divide by 16 (PRES3 = 0)."]
    Div16 = 4,
    #[doc = "5: Divide by 32 (PRES3 = 0)."]
    Div32 = 5,
    #[doc = "6: Divide by 64 (PRES3 = 0)."]
    Div64 = 6,
    #[doc = "7: Divide by 128 (PRES3 = 0)."]
    Div128 = 7,
}
impl From<Pres> for u8 {
    #[inline(always)]
    fn from(variant: Pres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pres {
    type Ux = u8;
}
impl crate::IsEnum for Pres {}
#[doc = "Field `PRES` reader - Timer Prescaler Select."]
pub type PresR = crate::FieldReader<Pres>;
impl PresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pres {
        match self.bits {
            0 => Pres::Div1,
            1 => Pres::Div2,
            2 => Pres::Div4,
            3 => Pres::Div8,
            4 => Pres::Div16,
            5 => Pres::Div32,
            6 => Pres::Div64,
            7 => Pres::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Pres::Div1
    }
    #[doc = "Divide by 2 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pres::Div2
    }
    #[doc = "Divide by 4 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pres::Div4
    }
    #[doc = "Divide by 8 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pres::Div8
    }
    #[doc = "Divide by 16 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Pres::Div16
    }
    #[doc = "Divide by 32 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Pres::Div32
    }
    #[doc = "Divide by 64 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Pres::Div64
    }
    #[doc = "Divide by 128 (PRES3 = 0)."]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Pres::Div128
    }
}
#[doc = "Field `PRES` writer - Timer Prescaler Select."]
pub type PresW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pres, crate::Safe>;
impl<'a, REG> PresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div1)
    }
    #[doc = "Divide by 2 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div2)
    }
    #[doc = "Divide by 4 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div4)
    }
    #[doc = "Divide by 8 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div8)
    }
    #[doc = "Divide by 16 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div16)
    }
    #[doc = "Divide by 32 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div32)
    }
    #[doc = "Divide by 64 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div64)
    }
    #[doc = "Divide by 128 (PRES3 = 0)."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Pres::Div128)
    }
}
#[doc = "Field `TPOL` reader - Timer pOLARITY."]
pub type TpolR = crate::BitReader;
#[doc = "Field `TPOL` writer - Timer pOLARITY."]
pub type TpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: `0`"]
    TimerDis = 0,
    #[doc = "1: `1`"]
    TimerEn = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Timer Enable."]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::TimerDis,
            true => Ten::TimerEn,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_timer_dis(&self) -> bool {
        *self == Ten::TimerDis
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_timer_en(&self) -> bool {
        *self == Ten::TimerEn
    }
}
#[doc = "Field `TEN` writer - Timer Enable."]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn timer_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::TimerDis)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn timer_en(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::TimerEn)
    }
}
#[doc = "Timer Prescaler Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pres3 {
    #[doc = "0: Prescaler bit 3 clear: PRES selects divide by 1 to 128."]
    Low = 0,
    #[doc = "1: Prescaler bit 3 set: PRES selects divide by 256 to 4096."]
    High = 1,
}
impl From<Pres3> for bool {
    #[inline(always)]
    fn from(variant: Pres3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRES3` reader - Timer Prescaler Select."]
pub type Pres3R = crate::BitReader<Pres3>;
impl Pres3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pres3 {
        match self.bits {
            false => Pres3::Low,
            true => Pres3::High,
        }
    }
    #[doc = "Prescaler bit 3 clear: PRES selects divide by 1 to 128."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Pres3::Low
    }
    #[doc = "Prescaler bit 3 set: PRES selects divide by 256 to 4096."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Pres3::High
    }
}
#[doc = "Field `PRES3` writer - Timer Prescaler Select."]
pub type Pres3W<'a, REG> = crate::BitWriter<'a, REG, Pres3>;
impl<'a, REG> Pres3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler bit 3 clear: PRES selects divide by 1 to 128."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Pres3::Low)
    }
    #[doc = "Prescaler bit 3 set: PRES selects divide by 256 to 4096."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Pres3::High)
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer Mode Select."]
    #[inline(always)]
    pub fn tmode(&self) -> TmodeR {
        TmodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Timer Prescaler Select."]
    #[inline(always)]
    pub fn pres(&self) -> PresR {
        PresR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Timer pOLARITY."]
    #[inline(always)]
    pub fn tpol(&self) -> TpolR {
        TpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer Prescaler Select."]
    #[inline(always)]
    pub fn pres3(&self) -> Pres3R {
        Pres3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer Mode Select."]
    #[inline(always)]
    pub fn tmode(&mut self) -> TmodeW<'_, CtrlSpec> {
        TmodeW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Timer Prescaler Select."]
    #[inline(always)]
    pub fn pres(&mut self) -> PresW<'_, CtrlSpec> {
        PresW::new(self, 3)
    }
    #[doc = "Bit 6 - Timer pOLARITY."]
    #[inline(always)]
    pub fn tpol(&mut self) -> TpolW<'_, CtrlSpec> {
        TpolW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer Enable."]
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<'_, CtrlSpec> {
        TenW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer Prescaler Select."]
    #[inline(always)]
    pub fn pres3(&mut self) -> Pres3W<'_, CtrlSpec> {
        Pres3W::new(self, 8)
    }
}
#[doc = "Wakeup Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
