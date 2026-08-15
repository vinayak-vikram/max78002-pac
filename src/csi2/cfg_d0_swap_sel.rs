#[doc = "Register `CFG_D0_SWAP_SEL` reader"]
pub type R = crate::R<CfgD0SwapSelSpec>;
#[doc = "Register `CFG_D0_SWAP_SEL` writer"]
pub type W = crate::W<CfgD0SwapSelSpec>;
#[doc = "Control Source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: PAD_CDRX_L0."]
    PadCdrxL0 = 0,
    #[doc = "1: PAD_CDRX_L1."]
    PadCdrxL1 = 1,
    #[doc = "2: PAD_CDRX_L2."]
    PadCdrxL2 = 2,
    #[doc = "3: PAD_CDRX_L3."]
    PadCdrxL3 = 3,
    #[doc = "4: PAD_CDRX_L4."]
    PadCdrxL4 = 4,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - Control Source."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            0 => Some(Src::PadCdrxL0),
            1 => Some(Src::PadCdrxL1),
            2 => Some(Src::PadCdrxL2),
            3 => Some(Src::PadCdrxL3),
            4 => Some(Src::PadCdrxL4),
            _ => None,
        }
    }
    #[doc = "PAD_CDRX_L0."]
    #[inline(always)]
    pub fn is_pad_cdrx_l0(&self) -> bool {
        *self == Src::PadCdrxL0
    }
    #[doc = "PAD_CDRX_L1."]
    #[inline(always)]
    pub fn is_pad_cdrx_l1(&self) -> bool {
        *self == Src::PadCdrxL1
    }
    #[doc = "PAD_CDRX_L2."]
    #[inline(always)]
    pub fn is_pad_cdrx_l2(&self) -> bool {
        *self == Src::PadCdrxL2
    }
    #[doc = "PAD_CDRX_L3."]
    #[inline(always)]
    pub fn is_pad_cdrx_l3(&self) -> bool {
        *self == Src::PadCdrxL3
    }
    #[doc = "PAD_CDRX_L4."]
    #[inline(always)]
    pub fn is_pad_cdrx_l4(&self) -> bool {
        *self == Src::PadCdrxL4
    }
}
#[doc = "Field `SRC` writer - Control Source."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PAD_CDRX_L0."]
    #[inline(always)]
    pub fn pad_cdrx_l0(self) -> &'a mut crate::W<REG> {
        self.variant(Src::PadCdrxL0)
    }
    #[doc = "PAD_CDRX_L1."]
    #[inline(always)]
    pub fn pad_cdrx_l1(self) -> &'a mut crate::W<REG> {
        self.variant(Src::PadCdrxL1)
    }
    #[doc = "PAD_CDRX_L2."]
    #[inline(always)]
    pub fn pad_cdrx_l2(self) -> &'a mut crate::W<REG> {
        self.variant(Src::PadCdrxL2)
    }
    #[doc = "PAD_CDRX_L3."]
    #[inline(always)]
    pub fn pad_cdrx_l3(self) -> &'a mut crate::W<REG> {
        self.variant(Src::PadCdrxL3)
    }
    #[doc = "PAD_CDRX_L4."]
    #[inline(always)]
    pub fn pad_cdrx_l4(self) -> &'a mut crate::W<REG> {
        self.variant(Src::PadCdrxL4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Control Source."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Control Source."]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, CfgD0SwapSelSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "CFG_D0_SWAP_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_d0_swap_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_d0_swap_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgD0SwapSelSpec;
impl crate::RegisterSpec for CfgD0SwapSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_d0_swap_sel::R`](R) reader structure"]
impl crate::Readable for CfgD0SwapSelSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_d0_swap_sel::W`](W) writer structure"]
impl crate::Writable for CfgD0SwapSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_D0_SWAP_SEL to value 0"]
impl crate::Resettable for CfgD0SwapSelSpec {}
