#[doc = "Register `XO32MKS` reader"]
pub type R = crate::R<Xo32mksSpec>;
#[doc = "Register `XO32MKS` writer"]
pub type W = crate::W<Xo32mksSpec>;
#[doc = "Field `CLK` reader - Kick Start XO Counter Setting"]
pub type ClkR = crate::FieldReader;
#[doc = "Field `CLK` writer - Kick Start XO Counter Setting"]
pub type ClkW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Kick Start XO Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Kick Start XO Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVER` reader - Kick Start XO Driver"]
pub type DriverR = crate::FieldReader;
#[doc = "Field `DRIVER` writer - Kick Start XO Driver"]
pub type DriverW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PULSE` reader - Kick Start XO 2X Pulse"]
pub type PulseR = crate::BitReader;
#[doc = "Field `PULSE` writer - Kick Start XO 2X Pulse"]
pub type PulseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Kick Start XO Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: No kick start clock."]
    None = 0,
    #[doc = "1: Test Clock in P1.2 (TMR3\\[22\\]=1)."]
    Test = 1,
    #[doc = "2: Internal secondary oscilator"]
    Iso = 2,
    #[doc = "3: Internal Primary Oscilator"]
    Ipo = 3,
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
#[doc = "Field `CLKSEL` reader - Kick Start XO Clock Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::None,
            1 => Clksel::Test,
            2 => Clksel::Iso,
            3 => Clksel::Ipo,
            _ => unreachable!(),
        }
    }
    #[doc = "No kick start clock."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Clksel::None
    }
    #[doc = "Test Clock in P1.2 (TMR3\\[22\\]=1)."]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == Clksel::Test
    }
    #[doc = "Internal secondary oscilator"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Clksel::Iso
    }
    #[doc = "Internal Primary Oscilator"]
    #[inline(always)]
    pub fn is_ipo(&self) -> bool {
        *self == Clksel::Ipo
    }
}
#[doc = "Field `CLKSEL` writer - Kick Start XO Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No kick start clock."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::None)
    }
    #[doc = "Test Clock in P1.2 (TMR3\\[22\\]=1)."]
    #[inline(always)]
    pub fn test(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Test)
    }
    #[doc = "Internal secondary oscilator"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Iso)
    }
    #[doc = "Internal Primary Oscilator"]
    #[inline(always)]
    pub fn ipo(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ipo)
    }
}
impl R {
    #[doc = "Bits 0:6 - Kick Start XO Counter Setting"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Kick Start XO Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Kick Start XO Driver"]
    #[inline(always)]
    pub fn driver(&self) -> DriverR {
        DriverR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Kick Start XO 2X Pulse"]
    #[inline(always)]
    pub fn pulse(&self) -> PulseR {
        PulseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Kick Start XO Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Kick Start XO Counter Setting"]
    #[inline(always)]
    pub fn clk(&mut self) -> ClkW<'_, Xo32mksSpec> {
        ClkW::new(self, 0)
    }
    #[doc = "Bit 7 - Kick Start XO Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Xo32mksSpec> {
        EnW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Kick Start XO Driver"]
    #[inline(always)]
    pub fn driver(&mut self) -> DriverW<'_, Xo32mksSpec> {
        DriverW::new(self, 8)
    }
    #[doc = "Bit 11 - Kick Start XO 2X Pulse"]
    #[inline(always)]
    pub fn pulse(&mut self) -> PulseW<'_, Xo32mksSpec> {
        PulseW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Kick Start XO Clock Select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, Xo32mksSpec> {
        ClkselW::new(self, 12)
    }
}
#[doc = "RISC-V Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`xo32mks::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo32mks::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xo32mksSpec;
impl crate::RegisterSpec for Xo32mksSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xo32mks::R`](R) reader structure"]
impl crate::Readable for Xo32mksSpec {}
#[doc = "`write(|w| ..)` method takes [`xo32mks::W`](W) writer structure"]
impl crate::Writable for Xo32mksSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XO32MKS to value 0"]
impl crate::Resettable for Xo32mksSpec {}
