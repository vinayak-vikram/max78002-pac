#[doc = "Register `SRAM` reader"]
pub type R = crate::R<SramSpec>;
#[doc = "Register `SRAM` writer"]
pub type W = crate::W<SramSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SRAM control. Written as a composite value; no documented fields.\n\nYou can [`read`](crate::Reg::read) this register and get [`sram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramSpec;
impl crate::RegisterSpec for SramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram::R`](R) reader structure"]
impl crate::Readable for SramSpec {}
#[doc = "`write(|w| ..)` method takes [`sram::W`](W) writer structure"]
impl crate::Writable for SramSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM to value 0"]
impl crate::Resettable for SramSpec {}
