#[doc = "Register `ADCCFG0` reader"]
pub type R = crate::R<Adccfg0Spec>;
#[doc = "Register `ADCCFG0` writer"]
pub type W = crate::W<Adccfg0Spec>;
#[doc = "Field `LP_5K_DIS` reader - Disable 5K divider optionin low power modes"]
pub type Lp5kDisR = crate::BitReader;
#[doc = "Field `LP_5K_DIS` writer - Disable 5K divider optionin low power modes"]
pub type Lp5kDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_50K_DIS` reader - Disable 50K divider optionin low power modes"]
pub type Lp50kDisR = crate::BitReader;
#[doc = "Field `LP_50K_DIS` writer - Disable 50K divider optionin low power modes"]
pub type Lp50kDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_REF` reader - External Reference Select Option"]
pub type ExtRefR = crate::BitReader;
#[doc = "Field `EXT_REF` writer - External Reference Select Option"]
pub type ExtRefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_SEL` reader - Internal Reference Select Option"]
pub type RefSelR = crate::BitReader;
#[doc = "Field `REF_SEL` writer - Internal Reference Select Option"]
pub type RefSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable 5K divider optionin low power modes"]
    #[inline(always)]
    pub fn lp_5k_dis(&self) -> Lp5kDisR {
        Lp5kDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable 50K divider optionin low power modes"]
    #[inline(always)]
    pub fn lp_50k_dis(&self) -> Lp50kDisR {
        Lp50kDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reference Select Option"]
    #[inline(always)]
    pub fn ext_ref(&self) -> ExtRefR {
        ExtRefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Reference Select Option"]
    #[inline(always)]
    pub fn ref_sel(&self) -> RefSelR {
        RefSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable 5K divider optionin low power modes"]
    #[inline(always)]
    pub fn lp_5k_dis(&mut self) -> Lp5kDisW<'_, Adccfg0Spec> {
        Lp5kDisW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable 50K divider optionin low power modes"]
    #[inline(always)]
    pub fn lp_50k_dis(&mut self) -> Lp50kDisW<'_, Adccfg0Spec> {
        Lp50kDisW::new(self, 1)
    }
    #[doc = "Bit 2 - External Reference Select Option"]
    #[inline(always)]
    pub fn ext_ref(&mut self) -> ExtRefW<'_, Adccfg0Spec> {
        ExtRefW::new(self, 2)
    }
    #[doc = "Bit 3 - Internal Reference Select Option"]
    #[inline(always)]
    pub fn ref_sel(&mut self) -> RefSelW<'_, Adccfg0Spec> {
        RefSelW::new(self, 3)
    }
}
#[doc = "ADC Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccfg0Spec;
impl crate::RegisterSpec for Adccfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccfg0::R`](R) reader structure"]
impl crate::Readable for Adccfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`adccfg0::W`](W) writer structure"]
impl crate::Writable for Adccfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCFG0 to value 0"]
impl crate::Resettable for Adccfg0Spec {}
