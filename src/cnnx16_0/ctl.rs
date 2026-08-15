#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `en` reader - Enable. Set to run the quadrant, cleared to halt it."]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - Enable. Set to run the quadrant, cleared to halt it."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_en` reader - Clock enable. Must be set before any other quadrant access."]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `clk_en` writer - Clock enable. Must be set before any other quadrant access."]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `irq` reader - Interrupt flag. Write zero to acknowledge."]
pub type IrqR = crate::BitReader;
#[doc = "Field `irq` writer - Interrupt flag. Write zero to acknowledge."]
pub type IrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable. Set to run the quadrant, cleared to halt it."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Clock enable. Must be set before any other quadrant access."]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt flag. Write zero to acknowledge."]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. Set to run the quadrant, cleared to halt it."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 3 - Clock enable. Must be set before any other quadrant access."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, CtlSpec> {
        ClkEnW::new(self, 3)
    }
    #[doc = "Bit 12 - Interrupt flag. Write zero to acknowledge."]
    #[inline(always)]
    pub fn irq(&mut self) -> IrqW<'_, CtlSpec> {
        IrqW::new(self, 12)
    }
}
#[doc = "Quadrant control. Bits other than those named below are written only as part of documented composite values.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
