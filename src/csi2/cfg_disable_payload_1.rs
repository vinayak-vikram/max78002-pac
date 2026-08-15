#[doc = "Register `CFG_DISABLE_PAYLOAD_1` reader"]
pub type R = crate::R<CfgDisablePayload1Spec>;
#[doc = "Register `CFG_DISABLE_PAYLOAD_1` writer"]
pub type W = crate::W<CfgDisablePayload1Spec>;
#[doc = "Field `USR_DEF_TYPE30` reader - User defined type 0x30."]
pub type UsrDefType30R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE30` writer - User defined type 0x30."]
pub type UsrDefType30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE31` reader - User defined type 0x31."]
pub type UsrDefType31R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE31` writer - User defined type 0x31."]
pub type UsrDefType31W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE32` reader - User defined type 0x32."]
pub type UsrDefType32R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE32` writer - User defined type 0x32."]
pub type UsrDefType32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE33` reader - User defined type 0x33."]
pub type UsrDefType33R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE33` writer - User defined type 0x33."]
pub type UsrDefType33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE34` reader - User defined type 0x34."]
pub type UsrDefType34R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE34` writer - User defined type 0x34."]
pub type UsrDefType34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE35` reader - User defined type 0x35."]
pub type UsrDefType35R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE35` writer - User defined type 0x35."]
pub type UsrDefType35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE36` reader - User defined type 0x36."]
pub type UsrDefType36R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE36` writer - User defined type 0x36."]
pub type UsrDefType36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USR_DEF_TYPE37` reader - User defined type 0x37."]
pub type UsrDefType37R = crate::BitReader;
#[doc = "Field `USR_DEF_TYPE37` writer - User defined type 0x37."]
pub type UsrDefType37W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User defined type 0x30."]
    #[inline(always)]
    pub fn usr_def_type30(&self) -> UsrDefType30R {
        UsrDefType30R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User defined type 0x31."]
    #[inline(always)]
    pub fn usr_def_type31(&self) -> UsrDefType31R {
        UsrDefType31R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User defined type 0x32."]
    #[inline(always)]
    pub fn usr_def_type32(&self) -> UsrDefType32R {
        UsrDefType32R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User defined type 0x33."]
    #[inline(always)]
    pub fn usr_def_type33(&self) -> UsrDefType33R {
        UsrDefType33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - User defined type 0x34."]
    #[inline(always)]
    pub fn usr_def_type34(&self) -> UsrDefType34R {
        UsrDefType34R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - User defined type 0x35."]
    #[inline(always)]
    pub fn usr_def_type35(&self) -> UsrDefType35R {
        UsrDefType35R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - User defined type 0x36."]
    #[inline(always)]
    pub fn usr_def_type36(&self) -> UsrDefType36R {
        UsrDefType36R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - User defined type 0x37."]
    #[inline(always)]
    pub fn usr_def_type37(&self) -> UsrDefType37R {
        UsrDefType37R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User defined type 0x30."]
    #[inline(always)]
    pub fn usr_def_type30(&mut self) -> UsrDefType30W<'_, CfgDisablePayload1Spec> {
        UsrDefType30W::new(self, 0)
    }
    #[doc = "Bit 1 - User defined type 0x31."]
    #[inline(always)]
    pub fn usr_def_type31(&mut self) -> UsrDefType31W<'_, CfgDisablePayload1Spec> {
        UsrDefType31W::new(self, 1)
    }
    #[doc = "Bit 2 - User defined type 0x32."]
    #[inline(always)]
    pub fn usr_def_type32(&mut self) -> UsrDefType32W<'_, CfgDisablePayload1Spec> {
        UsrDefType32W::new(self, 2)
    }
    #[doc = "Bit 3 - User defined type 0x33."]
    #[inline(always)]
    pub fn usr_def_type33(&mut self) -> UsrDefType33W<'_, CfgDisablePayload1Spec> {
        UsrDefType33W::new(self, 3)
    }
    #[doc = "Bit 4 - User defined type 0x34."]
    #[inline(always)]
    pub fn usr_def_type34(&mut self) -> UsrDefType34W<'_, CfgDisablePayload1Spec> {
        UsrDefType34W::new(self, 4)
    }
    #[doc = "Bit 5 - User defined type 0x35."]
    #[inline(always)]
    pub fn usr_def_type35(&mut self) -> UsrDefType35W<'_, CfgDisablePayload1Spec> {
        UsrDefType35W::new(self, 5)
    }
    #[doc = "Bit 6 - User defined type 0x36."]
    #[inline(always)]
    pub fn usr_def_type36(&mut self) -> UsrDefType36W<'_, CfgDisablePayload1Spec> {
        UsrDefType36W::new(self, 6)
    }
    #[doc = "Bit 7 - User defined type 0x37."]
    #[inline(always)]
    pub fn usr_def_type37(&mut self) -> UsrDefType37W<'_, CfgDisablePayload1Spec> {
        UsrDefType37W::new(self, 7)
    }
}
#[doc = "CFG_DISABLE_PAYLOAD_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_disable_payload_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_disable_payload_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDisablePayload1Spec;
impl crate::RegisterSpec for CfgDisablePayload1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_disable_payload_1::R`](R) reader structure"]
impl crate::Readable for CfgDisablePayload1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_disable_payload_1::W`](W) writer structure"]
impl crate::Writable for CfgDisablePayload1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_DISABLE_PAYLOAD_1 to value 0"]
impl crate::Resettable for CfgDisablePayload1Spec {}
