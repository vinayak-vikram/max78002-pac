#[doc = "Register `INTRINEN` reader"]
pub type R = crate::R<IntrinenSpec>;
#[doc = "Register `INTRINEN` writer"]
pub type W = crate::W<IntrinenSpec>;
#[doc = "Field `EP0_INT_EN` reader - Endpoint 0 interrupt enable."]
pub type Ep0IntEnR = crate::BitReader;
#[doc = "Field `EP0_INT_EN` writer - Endpoint 0 interrupt enable."]
pub type Ep0IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1_IN_INT_EN` reader - Endpoint 1 interrupt enable."]
pub type Ep1InIntEnR = crate::BitReader;
#[doc = "Field `EP1_IN_INT_EN` writer - Endpoint 1 interrupt enable."]
pub type Ep1InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_IN_INT_EN` reader - Endpoint 2 interrupt enable."]
pub type Ep2InIntEnR = crate::BitReader;
#[doc = "Field `EP2_IN_INT_EN` writer - Endpoint 2 interrupt enable."]
pub type Ep2InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_IN_INT_EN` reader - Endpoint 3 interrupt enable."]
pub type Ep3InIntEnR = crate::BitReader;
#[doc = "Field `EP3_IN_INT_EN` writer - Endpoint 3 interrupt enable."]
pub type Ep3InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_IN_INT_EN` reader - Endpoint 4 interrupt enable."]
pub type Ep4InIntEnR = crate::BitReader;
#[doc = "Field `EP4_IN_INT_EN` writer - Endpoint 4 interrupt enable."]
pub type Ep4InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_IN_INT_EN` reader - Endpoint 5 interrupt enable."]
pub type Ep5InIntEnR = crate::BitReader;
#[doc = "Field `EP5_IN_INT_EN` writer - Endpoint 5 interrupt enable."]
pub type Ep5InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_IN_INT_EN` reader - Endpoint 6 interrupt enable."]
pub type Ep6InIntEnR = crate::BitReader;
#[doc = "Field `EP6_IN_INT_EN` writer - Endpoint 6 interrupt enable."]
pub type Ep6InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_IN_INT_EN` reader - Endpoint 7 interrupt enable."]
pub type Ep7InIntEnR = crate::BitReader;
#[doc = "Field `EP7_IN_INT_EN` writer - Endpoint 7 interrupt enable."]
pub type Ep7InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_IN_INT_EN` reader - Endpoint 8 interrupt enable."]
pub type Ep8InIntEnR = crate::BitReader;
#[doc = "Field `EP8_IN_INT_EN` writer - Endpoint 8 interrupt enable."]
pub type Ep8InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP9_IN_INT_EN` reader - Endpoint 9 interrupt enable."]
pub type Ep9InIntEnR = crate::BitReader;
#[doc = "Field `EP9_IN_INT_EN` writer - Endpoint 9 interrupt enable."]
pub type Ep9InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP10_IN_INT_EN` reader - Endpoint 10 interrupt enable."]
pub type Ep10InIntEnR = crate::BitReader;
#[doc = "Field `EP10_IN_INT_EN` writer - Endpoint 10 interrupt enable."]
pub type Ep10InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP11_IN_INT_EN` reader - Endpoint 11 interrupt enable."]
pub type Ep11InIntEnR = crate::BitReader;
#[doc = "Field `EP11_IN_INT_EN` writer - Endpoint 11 interrupt enable."]
pub type Ep11InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP12_IN_INT_EN` reader - Endpoint 12 interrupt enable."]
pub type Ep12InIntEnR = crate::BitReader;
#[doc = "Field `EP12_IN_INT_EN` writer - Endpoint 12 interrupt enable."]
pub type Ep12InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP13_IN_INT_EN` reader - Endpoint 13 interrupt enable."]
pub type Ep13InIntEnR = crate::BitReader;
#[doc = "Field `EP13_IN_INT_EN` writer - Endpoint 13 interrupt enable."]
pub type Ep13InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP14_IN_INT_EN` reader - Endpoint 14 interrupt enable."]
pub type Ep14InIntEnR = crate::BitReader;
#[doc = "Field `EP14_IN_INT_EN` writer - Endpoint 14 interrupt enable."]
pub type Ep14InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP15_IN_INT_EN` reader - Endpoint 15 interrupt enable."]
pub type Ep15InIntEnR = crate::BitReader;
#[doc = "Field `EP15_IN_INT_EN` writer - Endpoint 15 interrupt enable."]
pub type Ep15InIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 interrupt enable."]
    #[inline(always)]
    pub fn ep0_int_en(&self) -> Ep0IntEnR {
        Ep0IntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt enable."]
    #[inline(always)]
    pub fn ep1_in_int_en(&self) -> Ep1InIntEnR {
        Ep1InIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt enable."]
    #[inline(always)]
    pub fn ep2_in_int_en(&self) -> Ep2InIntEnR {
        Ep2InIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt enable."]
    #[inline(always)]
    pub fn ep3_in_int_en(&self) -> Ep3InIntEnR {
        Ep3InIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt enable."]
    #[inline(always)]
    pub fn ep4_in_int_en(&self) -> Ep4InIntEnR {
        Ep4InIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt enable."]
    #[inline(always)]
    pub fn ep5_in_int_en(&self) -> Ep5InIntEnR {
        Ep5InIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt enable."]
    #[inline(always)]
    pub fn ep6_in_int_en(&self) -> Ep6InIntEnR {
        Ep6InIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt enable."]
    #[inline(always)]
    pub fn ep7_in_int_en(&self) -> Ep7InIntEnR {
        Ep7InIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt enable."]
    #[inline(always)]
    pub fn ep8_in_int_en(&self) -> Ep8InIntEnR {
        Ep8InIntEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt enable."]
    #[inline(always)]
    pub fn ep9_in_int_en(&self) -> Ep9InIntEnR {
        Ep9InIntEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt enable."]
    #[inline(always)]
    pub fn ep10_in_int_en(&self) -> Ep10InIntEnR {
        Ep10InIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt enable."]
    #[inline(always)]
    pub fn ep11_in_int_en(&self) -> Ep11InIntEnR {
        Ep11InIntEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt enable."]
    #[inline(always)]
    pub fn ep12_in_int_en(&self) -> Ep12InIntEnR {
        Ep12InIntEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt enable."]
    #[inline(always)]
    pub fn ep13_in_int_en(&self) -> Ep13InIntEnR {
        Ep13InIntEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt enable."]
    #[inline(always)]
    pub fn ep14_in_int_en(&self) -> Ep14InIntEnR {
        Ep14InIntEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 15 interrupt enable."]
    #[inline(always)]
    pub fn ep15_in_int_en(&self) -> Ep15InIntEnR {
        Ep15InIntEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 interrupt enable."]
    #[inline(always)]
    pub fn ep0_int_en(&mut self) -> Ep0IntEnW<'_, IntrinenSpec> {
        Ep0IntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt enable."]
    #[inline(always)]
    pub fn ep1_in_int_en(&mut self) -> Ep1InIntEnW<'_, IntrinenSpec> {
        Ep1InIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt enable."]
    #[inline(always)]
    pub fn ep2_in_int_en(&mut self) -> Ep2InIntEnW<'_, IntrinenSpec> {
        Ep2InIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt enable."]
    #[inline(always)]
    pub fn ep3_in_int_en(&mut self) -> Ep3InIntEnW<'_, IntrinenSpec> {
        Ep3InIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt enable."]
    #[inline(always)]
    pub fn ep4_in_int_en(&mut self) -> Ep4InIntEnW<'_, IntrinenSpec> {
        Ep4InIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt enable."]
    #[inline(always)]
    pub fn ep5_in_int_en(&mut self) -> Ep5InIntEnW<'_, IntrinenSpec> {
        Ep5InIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt enable."]
    #[inline(always)]
    pub fn ep6_in_int_en(&mut self) -> Ep6InIntEnW<'_, IntrinenSpec> {
        Ep6InIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt enable."]
    #[inline(always)]
    pub fn ep7_in_int_en(&mut self) -> Ep7InIntEnW<'_, IntrinenSpec> {
        Ep7InIntEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt enable."]
    #[inline(always)]
    pub fn ep8_in_int_en(&mut self) -> Ep8InIntEnW<'_, IntrinenSpec> {
        Ep8InIntEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt enable."]
    #[inline(always)]
    pub fn ep9_in_int_en(&mut self) -> Ep9InIntEnW<'_, IntrinenSpec> {
        Ep9InIntEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt enable."]
    #[inline(always)]
    pub fn ep10_in_int_en(&mut self) -> Ep10InIntEnW<'_, IntrinenSpec> {
        Ep10InIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt enable."]
    #[inline(always)]
    pub fn ep11_in_int_en(&mut self) -> Ep11InIntEnW<'_, IntrinenSpec> {
        Ep11InIntEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt enable."]
    #[inline(always)]
    pub fn ep12_in_int_en(&mut self) -> Ep12InIntEnW<'_, IntrinenSpec> {
        Ep12InIntEnW::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt enable."]
    #[inline(always)]
    pub fn ep13_in_int_en(&mut self) -> Ep13InIntEnW<'_, IntrinenSpec> {
        Ep13InIntEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt enable."]
    #[inline(always)]
    pub fn ep14_in_int_en(&mut self) -> Ep14InIntEnW<'_, IntrinenSpec> {
        Ep14InIntEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Endpoint 15 interrupt enable."]
    #[inline(always)]
    pub fn ep15_in_int_en(&mut self) -> Ep15InIntEnW<'_, IntrinenSpec> {
        Ep15InIntEnW::new(self, 15)
    }
}
#[doc = "Interrupt enable for EP 0 and IN EP 1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrinen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrinen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrinenSpec;
impl crate::RegisterSpec for IntrinenSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intrinen::R`](R) reader structure"]
impl crate::Readable for IntrinenSpec {}
#[doc = "`write(|w| ..)` method takes [`intrinen::W`](W) writer structure"]
impl crate::Writable for IntrinenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRINEN to value 0"]
impl crate::Resettable for IntrinenSpec {}
