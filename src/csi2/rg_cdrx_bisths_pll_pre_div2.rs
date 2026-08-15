#[doc = "Register `RG_CDRX_BISTHS_PLL_PRE_DIV2` reader"]
pub type R = crate::R<RgCdrxBisthsPllPreDiv2Spec>;
#[doc = "Register `RG_CDRX_BISTHS_PLL_PRE_DIV2` writer"]
pub type W = crate::W<RgCdrxBisthsPllPreDiv2Spec>;
#[doc = "RXMODE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxmode {
    #[doc = "0: CSI RX Mode."]
    Csi = 0,
    #[doc = "1: DSI RX Mode."]
    Dsi = 1,
}
impl From<Rxmode> for bool {
    #[inline(always)]
    fn from(variant: Rxmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXMODE` reader - RXMODE."]
pub type RxmodeR = crate::BitReader<Rxmode>;
impl RxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxmode {
        match self.bits {
            false => Rxmode::Csi,
            true => Rxmode::Dsi,
        }
    }
    #[doc = "CSI RX Mode."]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == Rxmode::Csi
    }
    #[doc = "DSI RX Mode."]
    #[inline(always)]
    pub fn is_dsi(&self) -> bool {
        *self == Rxmode::Dsi
    }
}
#[doc = "Field `RXMODE` writer - RXMODE."]
pub type RxmodeW<'a, REG> = crate::BitWriter<'a, REG, Rxmode>;
impl<'a, REG> RxmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSI RX Mode."]
    #[inline(always)]
    pub fn csi(self) -> &'a mut crate::W<REG> {
        self.variant(Rxmode::Csi)
    }
    #[doc = "DSI RX Mode."]
    #[inline(always)]
    pub fn dsi(self) -> &'a mut crate::W<REG> {
        self.variant(Rxmode::Dsi)
    }
}
impl R {
    #[doc = "Bit 0 - RXMODE."]
    #[inline(always)]
    pub fn rxmode(&self) -> RxmodeR {
        RxmodeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXMODE."]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RxmodeW<'_, RgCdrxBisthsPllPreDiv2Spec> {
        RxmodeW::new(self, 0)
    }
}
#[doc = "RG_CDRX_BISTHS_PLL_PRE_DIV2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_bisths_pll_pre_div2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_bisths_pll_pre_div2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgCdrxBisthsPllPreDiv2Spec;
impl crate::RegisterSpec for RgCdrxBisthsPllPreDiv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rg_cdrx_bisths_pll_pre_div2::R`](R) reader structure"]
impl crate::Readable for RgCdrxBisthsPllPreDiv2Spec {}
#[doc = "`write(|w| ..)` method takes [`rg_cdrx_bisths_pll_pre_div2::W`](W) writer structure"]
impl crate::Writable for RgCdrxBisthsPllPreDiv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RG_CDRX_BISTHS_PLL_PRE_DIV2 to value 0"]
impl crate::Resettable for RgCdrxBisthsPllPreDiv2Spec {}
