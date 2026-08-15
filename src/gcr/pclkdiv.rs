#[doc = "Register `PCLKDIV` reader"]
pub type R = crate::R<PclkdivSpec>;
#[doc = "Register `PCLKDIV` writer"]
pub type W = crate::W<PclkdivSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdioclkdiv {
    #[doc = "0: 48 MHz"]
    IpoDiv2 = 0,
    #[doc = "1: 24 MHz"]
    IpoDiv4 = 1,
}
impl From<Sdioclkdiv> for bool {
    #[inline(always)]
    fn from(variant: Sdioclkdiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIOCLKDIV` reader - "]
pub type SdioclkdivR = crate::BitReader<Sdioclkdiv>;
impl SdioclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdioclkdiv {
        match self.bits {
            false => Sdioclkdiv::IpoDiv2,
            true => Sdioclkdiv::IpoDiv4,
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_ipo_div2(&self) -> bool {
        *self == Sdioclkdiv::IpoDiv2
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_ipo_div4(&self) -> bool {
        *self == Sdioclkdiv::IpoDiv4
    }
}
#[doc = "Field `SDIOCLKDIV` writer - "]
pub type SdioclkdivW<'a, REG> = crate::BitWriter<'a, REG, Sdioclkdiv>;
impl<'a, REG> SdioclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn ipo_div2(self) -> &'a mut crate::W<REG> {
        self.variant(Sdioclkdiv::IpoDiv2)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn ipo_div4(self) -> &'a mut crate::W<REG> {
        self.variant(Sdioclkdiv::IpoDiv4)
    }
}
#[doc = "CNN Clock Divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cnnclkdiv {
    #[doc = "0: `0`"]
    Div2 = 0,
    #[doc = "1: `1`"]
    Div4 = 1,
    #[doc = "2: `10`"]
    Div8 = 2,
    #[doc = "3: `11`"]
    Div16 = 3,
    #[doc = "4: `100`"]
    Div1 = 4,
}
impl From<Cnnclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Cnnclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cnnclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Cnnclkdiv {}
#[doc = "Field `CNNCLKDIV` reader - CNN Clock Divider."]
pub type CnnclkdivR = crate::FieldReader<Cnnclkdiv>;
impl CnnclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cnnclkdiv> {
        match self.bits {
            0 => Some(Cnnclkdiv::Div2),
            1 => Some(Cnnclkdiv::Div4),
            2 => Some(Cnnclkdiv::Div8),
            3 => Some(Cnnclkdiv::Div16),
            4 => Some(Cnnclkdiv::Div1),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cnnclkdiv::Div2
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cnnclkdiv::Div4
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cnnclkdiv::Div8
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cnnclkdiv::Div16
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cnnclkdiv::Div1
    }
}
#[doc = "Field `CNNCLKDIV` writer - CNN Clock Divider."]
pub type CnnclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cnnclkdiv>;
impl<'a, REG> CnnclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclkdiv::Div2)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclkdiv::Div4)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclkdiv::Div8)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclkdiv::Div16)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclkdiv::Div1)
    }
}
#[doc = "CNN Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cnnclksel {
    #[doc = "0: `0`"]
    Pclk = 0,
    #[doc = "1: `1`"]
    Iso = 1,
    #[doc = "3: `11`"]
    Ipll = 3,
}
impl From<Cnnclksel> for u8 {
    #[inline(always)]
    fn from(variant: Cnnclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cnnclksel {
    type Ux = u8;
}
impl crate::IsEnum for Cnnclksel {}
#[doc = "Field `CNNCLKSEL` reader - CNN Clock Select."]
pub type CnnclkselR = crate::FieldReader<Cnnclksel>;
impl CnnclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cnnclksel> {
        match self.bits {
            0 => Some(Cnnclksel::Pclk),
            1 => Some(Cnnclksel::Iso),
            3 => Some(Cnnclksel::Ipll),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == Cnnclksel::Pclk
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Cnnclksel::Iso
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ipll(&self) -> bool {
        *self == Cnnclksel::Ipll
    }
}
#[doc = "Field `CNNCLKSEL` writer - CNN Clock Select."]
pub type CnnclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cnnclksel>;
impl<'a, REG> CnnclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclksel::Pclk)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclksel::Iso)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ipll(self) -> &'a mut crate::W<REG> {
        self.variant(Cnnclksel::Ipll)
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sdioclkdiv(&self) -> SdioclkdivR {
        SdioclkdivR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 14:16 - CNN Clock Divider."]
    #[inline(always)]
    pub fn cnnclkdiv(&self) -> CnnclkdivR {
        CnnclkdivR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18 - CNN Clock Select."]
    #[inline(always)]
    pub fn cnnclksel(&self) -> CnnclkselR {
        CnnclkselR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sdioclkdiv(&mut self) -> SdioclkdivW<'_, PclkdivSpec> {
        SdioclkdivW::new(self, 7)
    }
    #[doc = "Bits 14:16 - CNN Clock Divider."]
    #[inline(always)]
    pub fn cnnclkdiv(&mut self) -> CnnclkdivW<'_, PclkdivSpec> {
        CnnclkdivW::new(self, 14)
    }
    #[doc = "Bits 17:18 - CNN Clock Select."]
    #[inline(always)]
    pub fn cnnclksel(&mut self) -> CnnclkselW<'_, PclkdivSpec> {
        CnnclkselW::new(self, 17)
    }
}
#[doc = "Peripheral Clock Divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PclkdivSpec;
impl crate::RegisterSpec for PclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdiv::R`](R) reader structure"]
impl crate::Readable for PclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`pclkdiv::W`](W) writer structure"]
impl crate::Writable for PclkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCLKDIV to value 0x01"]
impl crate::Resettable for PclkdivSpec {
    const RESET_VALUE: u32 = 0x01;
}
