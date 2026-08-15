#[doc = "Register `INTRIN` reader"]
pub type R = crate::R<IntrinSpec>;
#[doc = "Register `INTRIN` writer"]
pub type W = crate::W<IntrinSpec>;
#[doc = "Field `EP0_IN_INT` reader - Endpoint 0 interrupt."]
pub type Ep0InIntR = crate::BitReader;
#[doc = "Field `EP1_IN_INT` reader - Endpoint 1 interrupt."]
pub type Ep1InIntR = crate::BitReader;
#[doc = "Field `EP2_IN_INT` reader - Endpoint 2 interrupt."]
pub type Ep2InIntR = crate::BitReader;
#[doc = "Field `EP3_IN_INT` reader - Endpoint 3 interrupt."]
pub type Ep3InIntR = crate::BitReader;
#[doc = "Field `EP4_IN_INT` reader - Endpoint 4 interrupt."]
pub type Ep4InIntR = crate::BitReader;
#[doc = "Field `EP5_IN_INT` reader - Endpoint 5 interrupt."]
pub type Ep5InIntR = crate::BitReader;
#[doc = "Field `EP6_IN_INT` reader - Endpoint 6 interrupt."]
pub type Ep6InIntR = crate::BitReader;
#[doc = "Field `EP7_IN_INT` reader - Endpoint 7 interrupt."]
pub type Ep7InIntR = crate::BitReader;
#[doc = "Field `EP8_IN_INT` reader - Endpoint 8 interrupt."]
pub type Ep8InIntR = crate::BitReader;
#[doc = "Field `EP9_IN_INT` reader - Endpoint 9 interrupt."]
pub type Ep9InIntR = crate::BitReader;
#[doc = "Field `EP10_IN_INT` reader - Endpoint 10 interrupt."]
pub type Ep10InIntR = crate::BitReader;
#[doc = "Field `EP11_IN_INT` reader - Endpoint 11 interrupt."]
pub type Ep11InIntR = crate::BitReader;
#[doc = "Field `EP12_IN_INT` reader - Endpoint 12 interrupt."]
pub type Ep12InIntR = crate::BitReader;
#[doc = "Field `EP13_IN_INT` reader - Endpoint 13 interrupt."]
pub type Ep13InIntR = crate::BitReader;
#[doc = "Field `EP14_IN_INT` reader - Endpoint 14 interrupt."]
pub type Ep14InIntR = crate::BitReader;
#[doc = "Field `EP15_IN_INT` reader - Endpoint 15 interrupt."]
pub type Ep15InIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Endpoint 0 interrupt."]
    #[inline(always)]
    pub fn ep0_in_int(&self) -> Ep0InIntR {
        Ep0InIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 interrupt."]
    #[inline(always)]
    pub fn ep1_in_int(&self) -> Ep1InIntR {
        Ep1InIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 interrupt."]
    #[inline(always)]
    pub fn ep2_in_int(&self) -> Ep2InIntR {
        Ep2InIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 interrupt."]
    #[inline(always)]
    pub fn ep3_in_int(&self) -> Ep3InIntR {
        Ep3InIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 interrupt."]
    #[inline(always)]
    pub fn ep4_in_int(&self) -> Ep4InIntR {
        Ep4InIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 interrupt."]
    #[inline(always)]
    pub fn ep5_in_int(&self) -> Ep5InIntR {
        Ep5InIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 interrupt."]
    #[inline(always)]
    pub fn ep6_in_int(&self) -> Ep6InIntR {
        Ep6InIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 interrupt."]
    #[inline(always)]
    pub fn ep7_in_int(&self) -> Ep7InIntR {
        Ep7InIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 interrupt."]
    #[inline(always)]
    pub fn ep8_in_int(&self) -> Ep8InIntR {
        Ep8InIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 interrupt."]
    #[inline(always)]
    pub fn ep9_in_int(&self) -> Ep9InIntR {
        Ep9InIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint 10 interrupt."]
    #[inline(always)]
    pub fn ep10_in_int(&self) -> Ep10InIntR {
        Ep10InIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint 11 interrupt."]
    #[inline(always)]
    pub fn ep11_in_int(&self) -> Ep11InIntR {
        Ep11InIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 12 interrupt."]
    #[inline(always)]
    pub fn ep12_in_int(&self) -> Ep12InIntR {
        Ep12InIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 13 interrupt."]
    #[inline(always)]
    pub fn ep13_in_int(&self) -> Ep13InIntR {
        Ep13InIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 14 interrupt."]
    #[inline(always)]
    pub fn ep14_in_int(&self) -> Ep14InIntR {
        Ep14InIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 15 interrupt."]
    #[inline(always)]
    pub fn ep15_in_int(&self) -> Ep15InIntR {
        Ep15InIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt register for EP0 and IN EP1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrinSpec;
impl crate::RegisterSpec for IntrinSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intrin::R`](R) reader structure"]
impl crate::Readable for IntrinSpec {}
#[doc = "`write(|w| ..)` method takes [`intrin::W`](W) writer structure"]
impl crate::Writable for IntrinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRIN to value 0"]
impl crate::Resettable for IntrinSpec {}
