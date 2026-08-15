#[doc = "Register `CLK_CN` reader"]
pub type R = crate::R<ClkCnSpec>;
#[doc = "Register `CLK_CN` writer"]
pub type W = crate::W<ClkCnSpec>;
#[doc = "Field `INTERNAL_CLK_EN` reader - Internal Clock Enable."]
pub type InternalClkEnR = crate::BitReader;
#[doc = "Field `INTERNAL_CLK_EN` writer - Internal Clock Enable."]
pub type InternalClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTERNAL_CLK_STABLE` reader - Internal Clock Stable."]
pub type InternalClkStableR = crate::BitReader;
#[doc = "Field `SD_CLK_EN` reader - SD Clock Enable."]
pub type SdClkEnR = crate::BitReader;
#[doc = "Field `SD_CLK_EN` writer - SD Clock Enable."]
pub type SdClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GEN_SEL` reader - Clock Generator Select."]
pub type ClkGenSelR = crate::BitReader;
#[doc = "Field `UPPER_SDCLK_FREQ_SEL` reader - Upper Bits of SDCLK Frequency Select."]
pub type UpperSdclkFreqSelR = crate::FieldReader;
#[doc = "Field `UPPER_SDCLK_FREQ_SEL` writer - Upper Bits of SDCLK Frequency Select."]
pub type UpperSdclkFreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDCLK_FREQ_SEL` reader - SDCLK Frequency Select."]
pub type SdclkFreqSelR = crate::FieldReader;
#[doc = "Field `SDCLK_FREQ_SEL` writer - SDCLK Frequency Select."]
pub type SdclkFreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Internal Clock Enable."]
    #[inline(always)]
    pub fn internal_clk_en(&self) -> InternalClkEnR {
        InternalClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Clock Stable."]
    #[inline(always)]
    pub fn internal_clk_stable(&self) -> InternalClkStableR {
        InternalClkStableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SD Clock Enable."]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SdClkEnR {
        SdClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Generator Select."]
    #[inline(always)]
    pub fn clk_gen_sel(&self) -> ClkGenSelR {
        ClkGenSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select."]
    #[inline(always)]
    pub fn upper_sdclk_freq_sel(&self) -> UpperSdclkFreqSelR {
        UpperSdclkFreqSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select."]
    #[inline(always)]
    pub fn sdclk_freq_sel(&self) -> SdclkFreqSelR {
        SdclkFreqSelR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Clock Enable."]
    #[inline(always)]
    pub fn internal_clk_en(&mut self) -> InternalClkEnW<'_, ClkCnSpec> {
        InternalClkEnW::new(self, 0)
    }
    #[doc = "Bit 2 - SD Clock Enable."]
    #[inline(always)]
    pub fn sd_clk_en(&mut self) -> SdClkEnW<'_, ClkCnSpec> {
        SdClkEnW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Upper Bits of SDCLK Frequency Select."]
    #[inline(always)]
    pub fn upper_sdclk_freq_sel(&mut self) -> UpperSdclkFreqSelW<'_, ClkCnSpec> {
        UpperSdclkFreqSelW::new(self, 6)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select."]
    #[inline(always)]
    pub fn sdclk_freq_sel(&mut self) -> SdclkFreqSelW<'_, ClkCnSpec> {
        SdclkFreqSelW::new(self, 8)
    }
}
#[doc = "Clock Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCnSpec;
impl crate::RegisterSpec for ClkCnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clk_cn::R`](R) reader structure"]
impl crate::Readable for ClkCnSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_cn::W`](W) writer structure"]
impl crate::Writable for ClkCnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CN to value 0"]
impl crate::Resettable for ClkCnSpec {}
