#[doc = "Register `MXM_SUSPEND` reader"]
pub type R = crate::R<MxmSuspendSpec>;
#[doc = "Register `MXM_SUSPEND` writer"]
pub type W = crate::W<MxmSuspendSpec>;
#[doc = "Field `SEL` reader - Suspend register"]
pub type SelR = crate::BitReader;
#[doc = "Field `SEL` writer - Suspend register"]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Suspend register"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend register"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, MxmSuspendSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "USB Added Maxim Suspend Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_suspend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_suspend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MxmSuspendSpec;
impl crate::RegisterSpec for MxmSuspendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mxm_suspend::R`](R) reader structure"]
impl crate::Readable for MxmSuspendSpec {}
#[doc = "`write(|w| ..)` method takes [`mxm_suspend::W`](W) writer structure"]
impl crate::Writable for MxmSuspendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MXM_SUSPEND to value 0"]
impl crate::Resettable for MxmSuspendSpec {}
