#[doc = "Register `INTROUTEN` reader"]
pub type R = crate::R<IntroutenSpec>;
#[doc = "Register `INTROUTEN` writer"]
pub type W = crate::W<IntroutenSpec>;
#[doc = "Field `EP1_OUT_INT_EN` reader - Endpoint 1 interrupt."]
pub type Ep1OutIntEnR = crate::BitReader;
#[doc = "Field `EP1_OUT_INT_EN` writer - Endpoint 1 interrupt."]
pub type Ep1OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_OUT_INT_EN` reader - Endpoint 2 interrupt."]
pub type Ep2OutIntEnR = crate::BitReader;
#[doc = "Field `EP2_OUT_INT_EN` writer - Endpoint 2 interrupt."]
pub type Ep2OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_OUT_INT_EN` reader - Endpoint 3 interrupt."]
pub type Ep3OutIntEnR = crate::BitReader;
#[doc = "Field `EP3_OUT_INT_EN` writer - Endpoint 3 interrupt."]
pub type Ep3OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_OUT_INT_EN` reader - Endpoint 4 interrupt."]
pub type Ep4OutIntEnR = crate::BitReader;
#[doc = "Field `EP4_OUT_INT_EN` writer - Endpoint 4 interrupt."]
pub type Ep4OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_OUT_INT_EN` reader - Endpoint 5 interrupt."]
pub type Ep5OutIntEnR = crate::BitReader;
#[doc = "Field `EP5_OUT_INT_EN` writer - Endpoint 5 interrupt."]
pub type Ep5OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_OUT_INT_EN` reader - Endpoint 6 interrupt."]
pub type Ep6OutIntEnR = crate::BitReader;
#[doc = "Field `EP6_OUT_INT_EN` writer - Endpoint 6 interrupt."]
pub type Ep6OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_OUT_INT_EN` reader - Endpoint 7 interrupt."]
pub type Ep7OutIntEnR = crate::BitReader;
#[doc = "Field `EP7_OUT_INT_EN` writer - Endpoint 7 interrupt."]
pub type Ep7OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_OUT_INT_EN` reader - Endpoint 8 interrupt."]
pub type Ep8OutIntEnR = crate::BitReader;
#[doc = "Field `EP8_OUT_INT_EN` writer - Endpoint 8 interrupt."]
pub type Ep8OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP9_OUT_INT_EN` reader - Endpoint 9 interrupt."]
pub type Ep9OutIntEnR = crate::BitReader;
#[doc = "Field `EP9_OUT_INT_EN` writer - Endpoint 9 interrupt."]
pub type Ep9OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP10_OUT_INT_EN` reader - Endpoint 10 interrupt."]
pub type Ep10OutIntEnR = crate::BitReader;
#[doc = "Field `EP10_OUT_INT_EN` writer - Endpoint 10 interrupt."]
pub type Ep10OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP11_OUT_INT_EN` reader - Endpoint 11 interrupt."]
pub type Ep11OutIntEnR = crate::BitReader;
#[doc = "Field `EP11_OUT_INT_EN` writer - Endpoint 11 interrupt."]
pub type Ep11OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP12_OUT_INT_EN` reader - Endpoint 12 interrupt."]
pub type Ep12OutIntEnR = crate::BitReader;
#[doc = "Field `EP12_OUT_INT_EN` writer - Endpoint 12 interrupt."]
pub type Ep12OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP13_OUT_INT_EN` reader - Endpoint 13 interrupt."]
pub type Ep13OutIntEnR = crate::BitReader;
#[doc = "Field `EP13_OUT_INT_EN` writer - Endpoint 13 interrupt."]
pub type Ep13OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP14_OUT_INT_EN` reader - Endpoint 14 interrupt."]
pub type Ep14OutIntEnR = crate::BitReader;
#[doc = "Field `EP14_OUT_INT_EN` writer - Endpoint 14 interrupt."]
pub type Ep14OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP15_OUT_INT_EN` reader - Endpoint 15 interrupt."]
pub type Ep15OutIntEnR = crate::BitReader;
#[doc = "Field `EP15_OUT_INT_EN` writer - Endpoint 15 interrupt."]
pub type Ep15OutIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_out_int_en(&self) -> Ep1OutIntEnR {
        Ep1OutIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_out_int_en(&self) -> Ep2OutIntEnR {
        Ep2OutIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_out_int_en(&self) -> Ep3OutIntEnR {
        Ep3OutIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_out_int_en(&self) -> Ep4OutIntEnR {
        Ep4OutIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_out_int_en(&self) -> Ep5OutIntEnR {
        Ep5OutIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_out_int_en(&self) -> Ep6OutIntEnR {
        Ep6OutIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_out_int_en(&self) -> Ep7OutIntEnR {
        Ep7OutIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_out_int_en(&self) -> Ep8OutIntEnR {
        Ep8OutIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_out_int_en(&self) -> Ep9OutIntEnR {
        Ep9OutIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_out_int_en(&self) -> Ep10OutIntEnR {
        Ep10OutIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_out_int_en(&self) -> Ep11OutIntEnR {
        Ep11OutIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_out_int_en(&self) -> Ep12OutIntEnR {
        Ep12OutIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_out_int_en(&self) -> Ep13OutIntEnR {
        Ep13OutIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_out_int_en(&self) -> Ep14OutIntEnR {
        Ep14OutIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_out_int_en(&self) -> Ep15OutIntEnR {
        Ep15OutIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_out_int_en(&mut self) -> Ep1OutIntEnW<'_, IntroutenSpec> {
        Ep1OutIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_out_int_en(&mut self) -> Ep2OutIntEnW<'_, IntroutenSpec> {
        Ep2OutIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_out_int_en(&mut self) -> Ep3OutIntEnW<'_, IntroutenSpec> {
        Ep3OutIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_out_int_en(&mut self) -> Ep4OutIntEnW<'_, IntroutenSpec> {
        Ep4OutIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_out_int_en(&mut self) -> Ep5OutIntEnW<'_, IntroutenSpec> {
        Ep5OutIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_out_int_en(&mut self) -> Ep6OutIntEnW<'_, IntroutenSpec> {
        Ep6OutIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_out_int_en(&mut self) -> Ep7OutIntEnW<'_, IntroutenSpec> {
        Ep7OutIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_out_int_en(&mut self) -> Ep8OutIntEnW<'_, IntroutenSpec> {
        Ep8OutIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_out_int_en(&mut self) -> Ep9OutIntEnW<'_, IntroutenSpec> {
        Ep9OutIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_out_int_en(&mut self) -> Ep10OutIntEnW<'_, IntroutenSpec> {
        Ep10OutIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_out_int_en(&mut self) -> Ep11OutIntEnW<'_, IntroutenSpec> {
        Ep11OutIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_out_int_en(&mut self) -> Ep12OutIntEnW<'_, IntroutenSpec> {
        Ep12OutIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_out_int_en(&mut self) -> Ep13OutIntEnW<'_, IntroutenSpec> {
        Ep13OutIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_out_int_en(&mut self) -> Ep14OutIntEnW<'_, IntroutenSpec> {
        Ep14OutIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_out_int_en(&mut self) -> Ep15OutIntEnW<'_, IntroutenSpec> {
        Ep15OutIntEnW::new(self, 15)
    }
}
#[doc = "Interrupt enable for OUT EP 1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`introuten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`introuten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntroutenSpec;
impl crate::RegisterSpec for IntroutenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`introuten::R`](R) reader structure"]
impl crate::Readable for IntroutenSpec {}
#[doc = "`write(|w| ..)` method takes [`introuten::W`](W) writer structure"]
impl crate::Writable for IntroutenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTROUTEN to value 0"]
impl crate::Resettable for IntroutenSpec {}
