#[doc = "Register `PRESET_3` reader"]
pub type R = crate::R<Preset3Spec>;
#[doc = "Field `SDCLK_FREQ` reader - SDCLK Frequency Select Value."]
pub type SdclkFreqR = crate::FieldReader<u16>;
#[doc = "Field `CLK_GEN` reader - Clock Generator Select Value."]
pub type ClkGenR = crate::BitReader;
#[doc = "Field `DRIVER_STRENGTH` reader - Driver Strength Select Value."]
pub type DriverStrengthR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - SDCLK Frequency Select Value."]
    #[inline(always)]
    pub fn sdclk_freq(&self) -> SdclkFreqR {
        SdclkFreqR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Clock Generator Select Value."]
    #[inline(always)]
    pub fn clk_gen(&self) -> ClkGenR {
        ClkGenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value."]
    #[inline(always)]
    pub fn driver_strength(&self) -> DriverStrengthR {
        DriverStrengthR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Preset Value for SDR12.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preset3Spec;
impl crate::RegisterSpec for Preset3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`preset_3::R`](R) reader structure"]
impl crate::Readable for Preset3Spec {}
#[doc = "`reset()` method sets PRESET_3 to value 0"]
impl crate::Resettable for Preset3Spec {}
