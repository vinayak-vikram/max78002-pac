#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: Select HCLK."]
    Hclk = 0,
    #[doc = "1: Select CLK_ADC0."]
    ClkAdc0 = 1,
    #[doc = "2: Select CLK_ADC1."]
    ClkAdc1 = 2,
    #[doc = "3: Select CLK_ADC2."]
    ClkAdc2 = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock source select."]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Hclk,
            1 => Clksel::ClkAdc0,
            2 => Clksel::ClkAdc1,
            3 => Clksel::ClkAdc2,
            _ => unreachable!(),
        }
    }
    #[doc = "Select HCLK."]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == Clksel::Hclk
    }
    #[doc = "Select CLK_ADC0."]
    #[inline(always)]
    pub fn is_clk_adc0(&self) -> bool {
        *self == Clksel::ClkAdc0
    }
    #[doc = "Select CLK_ADC1."]
    #[inline(always)]
    pub fn is_clk_adc1(&self) -> bool {
        *self == Clksel::ClkAdc1
    }
    #[doc = "Select CLK_ADC2."]
    #[inline(always)]
    pub fn is_clk_adc2(&self) -> bool {
        *self == Clksel::ClkAdc2
    }
}
#[doc = "Field `CLKSEL` writer - Clock source select."]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select HCLK."]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hclk)
    }
    #[doc = "Select CLK_ADC0."]
    #[inline(always)]
    pub fn clk_adc0(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::ClkAdc0)
    }
    #[doc = "Select CLK_ADC1."]
    #[inline(always)]
    pub fn clk_adc1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::ClkAdc1)
    }
    #[doc = "Select CLK_ADC2."]
    #[inline(always)]
    pub fn clk_adc2(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::ClkAdc2)
    }
}
#[doc = "Clock divider control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: Divide by 2."]
    Div2 = 0,
    #[doc = "1: Divide by 4."]
    Div4 = 1,
    #[doc = "2: Divide by 8."]
    Div8 = 2,
    #[doc = "3: Divide by 16."]
    Div16 = 3,
    #[doc = "4: Divide by 1."]
    Div1 = 4,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - Clock divider control."]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv> {
        match self.bits {
            0 => Some(Clkdiv::Div2),
            1 => Some(Clkdiv::Div4),
            2 => Some(Clkdiv::Div8),
            3 => Some(Clkdiv::Div16),
            4 => Some(Clkdiv::Div1),
            _ => None,
        }
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Clkdiv::Div2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Clkdiv::Div4
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Clkdiv::Div8
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Clkdiv::Div16
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Clkdiv::Div1
    }
}
#[doc = "Field `CLKDIV` writer - Clock divider control."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkdiv>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div16)
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source select."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Clock divider control."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source select."]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, ClkctrlSpec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Clock divider control."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, ClkctrlSpec> {
        ClkdivW::new(self, 4)
    }
}
#[doc = "Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {}
