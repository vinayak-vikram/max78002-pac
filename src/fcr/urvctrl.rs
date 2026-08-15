#[doc = "Register `URVCTRL` reader"]
pub type R = crate::R<UrvctrlSpec>;
#[doc = "Register `URVCTRL` writer"]
pub type W = crate::W<UrvctrlSpec>;
#[doc = "Field `MEMSEL` reader - RAM2, RAM3 exclusive ownership."]
pub type MemselR = crate::BitReader;
#[doc = "Field `MEMSEL` writer - RAM2, RAM3 exclusive ownership."]
pub type MemselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFLUSHEN` reader - URV instruction flush enable."]
pub type IflushenR = crate::BitReader;
#[doc = "Field `IFLUSHEN` writer - URV instruction flush enable."]
pub type IflushenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAM2, RAM3 exclusive ownership."]
    #[inline(always)]
    pub fn memsel(&self) -> MemselR {
        MemselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - URV instruction flush enable."]
    #[inline(always)]
    pub fn iflushen(&self) -> IflushenR {
        IflushenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM2, RAM3 exclusive ownership."]
    #[inline(always)]
    pub fn memsel(&mut self) -> MemselW<'_, UrvctrlSpec> {
        MemselW::new(self, 0)
    }
    #[doc = "Bit 1 - URV instruction flush enable."]
    #[inline(always)]
    pub fn iflushen(&mut self) -> IflushenW<'_, UrvctrlSpec> {
        IflushenW::new(self, 1)
    }
}
#[doc = "RISC-V Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`urvctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urvctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UrvctrlSpec;
impl crate::RegisterSpec for UrvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`urvctrl::R`](R) reader structure"]
impl crate::Readable for UrvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`urvctrl::W`](W) writer structure"]
impl crate::Writable for UrvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets URVCTRL to value 0"]
impl crate::Resettable for UrvctrlSpec {}
