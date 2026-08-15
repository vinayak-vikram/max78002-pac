#[doc = "Register `INTROUT` reader"]
pub type R = crate::R<IntroutSpec>;
#[doc = "Register `INTROUT` writer"]
pub type W = crate::W<IntroutSpec>;
#[doc = "Field `EP1_OUT_INT` reader - Endpoint 1 interrupt."]
pub type Ep1OutIntR = crate::BitReader;
#[doc = "Field `EP2_OUT_INT` reader - Endpoint 2 interrupt."]
pub type Ep2OutIntR = crate::BitReader;
#[doc = "Field `EP3_OUT_INT` reader - Endpoint 3 interrupt."]
pub type Ep3OutIntR = crate::BitReader;
#[doc = "Field `EP4_OUT_INT` reader - Endpoint 4 interrupt."]
pub type Ep4OutIntR = crate::BitReader;
#[doc = "Field `EP5_OUT_INT` reader - Endpoint 5 interrupt."]
pub type Ep5OutIntR = crate::BitReader;
#[doc = "Field `EP6_OUT_INT` reader - Endpoint 6 interrupt."]
pub type Ep6OutIntR = crate::BitReader;
#[doc = "Field `EP7_OUT_INT` reader - Endpoint 7 interrupt."]
pub type Ep7OutIntR = crate::BitReader;
#[doc = "Field `EP8_OUT_INT` reader - Endpoint 8 interrupt."]
pub type Ep8OutIntR = crate::BitReader;
#[doc = "Field `EP9_OUT_INT` reader - Endpoint 9 interrupt."]
pub type Ep9OutIntR = crate::BitReader;
#[doc = "Field `EP10_OUT_INT` reader - Endpoint 10 interrupt."]
pub type Ep10OutIntR = crate::BitReader;
#[doc = "Field `EP11_OUT_INT` reader - Endpoint 11 interrupt."]
pub type Ep11OutIntR = crate::BitReader;
#[doc = "Field `EP12_OUT_INT` reader - Endpoint 12 interrupt."]
pub type Ep12OutIntR = crate::BitReader;
#[doc = "Field `EP13_OUT_INT` reader - Endpoint 13 interrupt."]
pub type Ep13OutIntR = crate::BitReader;
#[doc = "Field `EP14_OUT_INT` reader - Endpoint 14 interrupt."]
pub type Ep14OutIntR = crate::BitReader;
#[doc = "Field `EP15_OUT_INT` reader - Endpoint 15 interrupt."]
pub type Ep15OutIntR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_out_int(&self) -> Ep1OutIntR {
        Ep1OutIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_out_int(&self) -> Ep2OutIntR {
        Ep2OutIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_out_int(&self) -> Ep3OutIntR {
        Ep3OutIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_out_int(&self) -> Ep4OutIntR {
        Ep4OutIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_out_int(&self) -> Ep5OutIntR {
        Ep5OutIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_out_int(&self) -> Ep6OutIntR {
        Ep6OutIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_out_int(&self) -> Ep7OutIntR {
        Ep7OutIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_out_int(&self) -> Ep8OutIntR {
        Ep8OutIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_out_int(&self) -> Ep9OutIntR {
        Ep9OutIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_out_int(&self) -> Ep10OutIntR {
        Ep10OutIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_out_int(&self) -> Ep11OutIntR {
        Ep11OutIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_out_int(&self) -> Ep12OutIntR {
        Ep12OutIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_out_int(&self) -> Ep13OutIntR {
        Ep13OutIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_out_int(&self) -> Ep14OutIntR {
        Ep14OutIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_out_int(&self) -> Ep15OutIntR {
        Ep15OutIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt register for OUT EP 1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`introut::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`introut::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntroutSpec;
impl crate::RegisterSpec for IntroutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`introut::R`](R) reader structure"]
impl crate::Readable for IntroutSpec {}
#[doc = "`write(|w| ..)` method takes [`introut::W`](W) writer structure"]
impl crate::Writable for IntroutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTROUT to value 0"]
impl crate::Resettable for IntroutSpec {}
