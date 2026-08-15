#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Start conversion control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Stop conversions."]
    Stop = 0,
    #[doc = "1: Start conversions."]
    Start = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start conversion control."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::Stop,
            true => Start::Start,
        }
    }
    #[doc = "Stop conversions."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Start::Stop
    }
    #[doc = "Start conversions."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Start::Start
    }
}
#[doc = "Field `START` writer - Start conversion control."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop conversions."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Stop)
    }
    #[doc = "Start conversions."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
}
#[doc = "Trigger mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrigMode {
    #[doc = "0: software trigger mode."]
    Software = 0,
    #[doc = "1: hardware trigger mode."]
    Hardware = 1,
}
impl From<TrigMode> for bool {
    #[inline(always)]
    fn from(variant: TrigMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG_MODE` reader - Trigger mode control."]
pub type TrigModeR = crate::BitReader<TrigMode>;
impl TrigModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TrigMode {
        match self.bits {
            false => TrigMode::Software,
            true => TrigMode::Hardware,
        }
    }
    #[doc = "software trigger mode."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TrigMode::Software
    }
    #[doc = "hardware trigger mode."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == TrigMode::Hardware
    }
}
#[doc = "Field `TRIG_MODE` writer - Trigger mode control."]
pub type TrigModeW<'a, REG> = crate::BitWriter<'a, REG, TrigMode>;
impl<'a, REG> TrigModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "software trigger mode."]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(TrigMode::Software)
    }
    #[doc = "hardware trigger mode."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(TrigMode::Hardware)
    }
}
#[doc = "Conversion mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CnvMode {
    #[doc = "0: Do one conversion sequence."]
    Atomic = 0,
    #[doc = "1: Do continuous conversion sequences."]
    Continuous = 1,
}
impl From<CnvMode> for bool {
    #[inline(always)]
    fn from(variant: CnvMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNV_MODE` reader - Conversion mode control."]
pub type CnvModeR = crate::BitReader<CnvMode>;
impl CnvModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CnvMode {
        match self.bits {
            false => CnvMode::Atomic,
            true => CnvMode::Continuous,
        }
    }
    #[doc = "Do one conversion sequence."]
    #[inline(always)]
    pub fn is_atomic(&self) -> bool {
        *self == CnvMode::Atomic
    }
    #[doc = "Do continuous conversion sequences."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CnvMode::Continuous
    }
}
#[doc = "Field `CNV_MODE` writer - Conversion mode control."]
pub type CnvModeW<'a, REG> = crate::BitWriter<'a, REG, CnvMode>;
impl<'a, REG> CnvModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do one conversion sequence."]
    #[inline(always)]
    pub fn atomic(self) -> &'a mut crate::W<REG> {
        self.variant(CnvMode::Atomic)
    }
    #[doc = "Do continuous conversion sequences."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(CnvMode::Continuous)
    }
}
#[doc = "Sample clock off control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampCkOff {
    #[doc = "0: Sample clock always generated."]
    Always = 0,
    #[doc = "1: Sample clock generated only when converting."]
    CnvOnly = 1,
}
impl From<SampCkOff> for bool {
    #[inline(always)]
    fn from(variant: SampCkOff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMP_CK_OFF` reader - Sample clock off control."]
pub type SampCkOffR = crate::BitReader<SampCkOff>;
impl SampCkOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SampCkOff {
        match self.bits {
            false => SampCkOff::Always,
            true => SampCkOff::CnvOnly,
        }
    }
    #[doc = "Sample clock always generated."]
    #[inline(always)]
    pub fn is_always(&self) -> bool {
        *self == SampCkOff::Always
    }
    #[doc = "Sample clock generated only when converting."]
    #[inline(always)]
    pub fn is_cnv_only(&self) -> bool {
        *self == SampCkOff::CnvOnly
    }
}
#[doc = "Field `SAMP_CK_OFF` writer - Sample clock off control."]
pub type SampCkOffW<'a, REG> = crate::BitWriter<'a, REG, SampCkOff>;
impl<'a, REG> SampCkOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sample clock always generated."]
    #[inline(always)]
    pub fn always(self) -> &'a mut crate::W<REG> {
        self.variant(SampCkOff::Always)
    }
    #[doc = "Sample clock generated only when converting."]
    #[inline(always)]
    pub fn cnv_only(self) -> &'a mut crate::W<REG> {
        self.variant(SampCkOff::CnvOnly)
    }
}
#[doc = "Field `TRIG_SEL` reader - Hardware trigger source select."]
pub type TrigSelR = crate::FieldReader;
#[doc = "Field `TRIG_SEL` writer - Hardware trigger source select."]
pub type TrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Temp sensor select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TsSel {
    #[doc = "0: Temp sensor is not one of the slots in the sequence."]
    Dis = 0,
    #[doc = "1: Temp sensor is one of the slots in the sequence."]
    En = 1,
}
impl From<TsSel> for bool {
    #[inline(always)]
    fn from(variant: TsSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS_SEL` reader - Temp sensor select."]
pub type TsSelR = crate::BitReader<TsSel>;
impl TsSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TsSel {
        match self.bits {
            false => TsSel::Dis,
            true => TsSel::En,
        }
    }
    #[doc = "Temp sensor is not one of the slots in the sequence."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TsSel::Dis
    }
    #[doc = "Temp sensor is one of the slots in the sequence."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TsSel::En
    }
}
#[doc = "Field `TS_SEL` writer - Temp sensor select."]
pub type TsSelW<'a, REG> = crate::BitWriter<'a, REG, TsSel>;
impl<'a, REG> TsSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temp sensor is not one of the slots in the sequence."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TsSel::Dis)
    }
    #[doc = "Temp sensor is one of the slots in the sequence."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TsSel::En)
    }
}
#[doc = "Number of samples to average for each output data code.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avg {
    #[doc = "0: 1 Sample per output code."]
    Avg1 = 0,
    #[doc = "1: 2 Samples per output code."]
    Avg2 = 1,
    #[doc = "2: 4 Samples per output code."]
    Avg4 = 2,
    #[doc = "3: 8 Samples per output code."]
    Avg8 = 3,
    #[doc = "4: 16 Samples per output code."]
    Avg16 = 4,
    #[doc = "5: 32 Samples per output code."]
    Avg32 = 5,
}
impl From<Avg> for u8 {
    #[inline(always)]
    fn from(variant: Avg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avg {
    type Ux = u8;
}
impl crate::IsEnum for Avg {}
#[doc = "Field `AVG` reader - Number of samples to average for each output data code."]
pub type AvgR = crate::FieldReader<Avg>;
impl AvgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Avg> {
        match self.bits {
            0 => Some(Avg::Avg1),
            1 => Some(Avg::Avg2),
            2 => Some(Avg::Avg4),
            3 => Some(Avg::Avg8),
            4 => Some(Avg::Avg16),
            5 => Some(Avg::Avg32),
            _ => None,
        }
    }
    #[doc = "1 Sample per output code."]
    #[inline(always)]
    pub fn is_avg1(&self) -> bool {
        *self == Avg::Avg1
    }
    #[doc = "2 Samples per output code."]
    #[inline(always)]
    pub fn is_avg2(&self) -> bool {
        *self == Avg::Avg2
    }
    #[doc = "4 Samples per output code."]
    #[inline(always)]
    pub fn is_avg4(&self) -> bool {
        *self == Avg::Avg4
    }
    #[doc = "8 Samples per output code."]
    #[inline(always)]
    pub fn is_avg8(&self) -> bool {
        *self == Avg::Avg8
    }
    #[doc = "16 Samples per output code."]
    #[inline(always)]
    pub fn is_avg16(&self) -> bool {
        *self == Avg::Avg16
    }
    #[doc = "32 Samples per output code."]
    #[inline(always)]
    pub fn is_avg32(&self) -> bool {
        *self == Avg::Avg32
    }
}
#[doc = "Field `AVG` writer - Number of samples to average for each output data code."]
pub type AvgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Avg>;
impl<'a, REG> AvgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Sample per output code."]
    #[inline(always)]
    pub fn avg1(self) -> &'a mut crate::W<REG> {
        self.variant(Avg::Avg1)
    }
    #[doc = "2 Samples per output code."]
    #[inline(always)]
    pub fn avg2(self) -> &'a mut crate::W<REG> {
        self.variant(Avg::Avg2)
    }
    #[doc = "4 Samples per output code."]
    #[inline(always)]
    pub fn avg4(self) -> &'a mut crate::W<REG> {
        self.variant(Avg::Avg4)
    }
    #[doc = "8 Samples per output code."]
    #[inline(always)]
    pub fn avg8(self) -> &'a mut crate::W<REG> {
        self.variant(Avg::Avg8)
    }
    #[doc = "16 Samples per output code."]
    #[inline(always)]
    pub fn avg16(self) -> &'a mut crate::W<REG> {
        self.variant(Avg::Avg16)
    }
    #[doc = "32 Samples per output code."]
    #[inline(always)]
    pub fn avg32(self) -> &'a mut crate::W<REG> {
        self.variant(Avg::Avg32)
    }
}
#[doc = "Field `NUM_SLOTS` reader - Number of slots enabled for the conversion sequence"]
pub type NumSlotsR = crate::FieldReader;
#[doc = "Field `NUM_SLOTS` writer - Number of slots enabled for the conversion sequence"]
pub type NumSlotsW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Start conversion control."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger mode control."]
    #[inline(always)]
    pub fn trig_mode(&self) -> TrigModeR {
        TrigModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Conversion mode control."]
    #[inline(always)]
    pub fn cnv_mode(&self) -> CnvModeR {
        CnvModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sample clock off control."]
    #[inline(always)]
    pub fn samp_ck_off(&self) -> SampCkOffR {
        SampCkOffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Hardware trigger source select."]
    #[inline(always)]
    pub fn trig_sel(&self) -> TrigSelR {
        TrigSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Temp sensor select."]
    #[inline(always)]
    pub fn ts_sel(&self) -> TsSelR {
        TsSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Number of samples to average for each output data code."]
    #[inline(always)]
    pub fn avg(&self) -> AvgR {
        AvgR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Number of slots enabled for the conversion sequence"]
    #[inline(always)]
    pub fn num_slots(&self) -> NumSlotsR {
        NumSlotsR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start conversion control."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, Ctrl1Spec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger mode control."]
    #[inline(always)]
    pub fn trig_mode(&mut self) -> TrigModeW<'_, Ctrl1Spec> {
        TrigModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion mode control."]
    #[inline(always)]
    pub fn cnv_mode(&mut self) -> CnvModeW<'_, Ctrl1Spec> {
        CnvModeW::new(self, 2)
    }
    #[doc = "Bit 3 - Sample clock off control."]
    #[inline(always)]
    pub fn samp_ck_off(&mut self) -> SampCkOffW<'_, Ctrl1Spec> {
        SampCkOffW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Hardware trigger source select."]
    #[inline(always)]
    pub fn trig_sel(&mut self) -> TrigSelW<'_, Ctrl1Spec> {
        TrigSelW::new(self, 4)
    }
    #[doc = "Bit 7 - Temp sensor select."]
    #[inline(always)]
    pub fn ts_sel(&mut self) -> TsSelW<'_, Ctrl1Spec> {
        TsSelW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Number of samples to average for each output data code."]
    #[inline(always)]
    pub fn avg(&mut self) -> AvgW<'_, Ctrl1Spec> {
        AvgW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Number of slots enabled for the conversion sequence"]
    #[inline(always)]
    pub fn num_slots(&mut self) -> NumSlotsW<'_, Ctrl1Spec> {
        NumSlotsW::new(self, 16)
    }
}
#[doc = "Control Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {}
