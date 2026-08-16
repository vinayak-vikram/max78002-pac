#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `rdy_sel` reader - APB ready wait select."]
pub type RdySelR = crate::FieldReader;
#[doc = "Field `rdy_sel` writer - APB ready wait select."]
pub type RdySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `thresh_a` reader - FIFO threshold A."]
pub type ThreshAR = crate::FieldReader;
#[doc = "Field `thresh_a` writer - FIFO threshold A."]
pub type ThreshAW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `thresh_b` reader - FIFO threshold B."]
pub type ThreshBR = crate::FieldReader;
#[doc = "Field `thresh_b` writer - FIFO threshold B."]
pub type ThreshBW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cpl` reader - FIFO completion. Not used on this device."]
pub type CplR = crate::BitReader;
#[doc = "Field `cpl` writer - FIFO completion. Not used on this device."]
pub type CplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `quad_en` reader - FIFO enable per quadrant. Bit n enables the FIFO for quadrant n."]
pub type QuadEnR = crate::FieldReader;
#[doc = "Field `quad_en` writer - FIFO enable per quadrant. Bit n enables the FIFO for quadrant n."]
pub type QuadEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - APB ready wait select."]
    #[inline(always)]
    pub fn rdy_sel(&self) -> RdySelR {
        RdySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - FIFO threshold A."]
    #[inline(always)]
    pub fn thresh_a(&self) -> ThreshAR {
        ThreshAR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:10 - FIFO threshold B."]
    #[inline(always)]
    pub fn thresh_b(&self) -> ThreshBR {
        ThreshBR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - FIFO completion. Not used on this device."]
    #[inline(always)]
    pub fn cpl(&self) -> CplR {
        CplR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - FIFO enable per quadrant. Bit n enables the FIFO for quadrant n."]
    #[inline(always)]
    pub fn quad_en(&self) -> QuadEnR {
        QuadEnR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB ready wait select."]
    #[inline(always)]
    pub fn rdy_sel(&mut self) -> RdySelW<'_, CtlSpec> {
        RdySelW::new(self, 0)
    }
    #[doc = "Bits 2:6 - FIFO threshold A."]
    #[inline(always)]
    pub fn thresh_a(&mut self) -> ThreshAW<'_, CtlSpec> {
        ThreshAW::new(self, 2)
    }
    #[doc = "Bits 7:10 - FIFO threshold B."]
    #[inline(always)]
    pub fn thresh_b(&mut self) -> ThreshBW<'_, CtlSpec> {
        ThreshBW::new(self, 7)
    }
    #[doc = "Bit 11 - FIFO completion. Not used on this device."]
    #[inline(always)]
    pub fn cpl(&mut self) -> CplW<'_, CtlSpec> {
        CplW::new(self, 11)
    }
    #[doc = "Bits 12:15 - FIFO enable per quadrant. Bit n enables the FIFO for quadrant n."]
    #[inline(always)]
    pub fn quad_en(&mut self) -> QuadEnW<'_, CtlSpec> {
        QuadEnW::new(self, 12)
    }
}
#[doc = "FIFO control.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
