#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `IRQ_CLR` reader - Timer Interrupt."]
pub type IrqClrR = crate::BitReader;
#[doc = "Field `IRQ_CLR` writer - Timer Interrupt."]
pub type IrqClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer Interrupt."]
    #[inline(always)]
    pub fn irq_clr(&self) -> IrqClrR {
        IrqClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt."]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IrqClrW<'_, IntflSpec> {
        IrqClrW::new(self, 0)
    }
}
#[doc = "Wakeup Timer Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for IntflSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {}
