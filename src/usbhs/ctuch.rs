#[doc = "Register `CTUCH` reader"]
pub type R = crate::R<CtuchSpec>;
#[doc = "Register `CTUCH` writer"]
pub type W = crate::W<CtuchSpec>;
#[doc = "Field `C_T_UCH` reader - HS Chirp Timeout Clock Cycles. This configures the chirp timeout used by this device to negotiate a HS connection with a FS Host."]
pub type CTUchR = crate::FieldReader<u16>;
#[doc = "Field `C_T_UCH` writer - HS Chirp Timeout Clock Cycles. This configures the chirp timeout used by this device to negotiate a HS connection with a FS Host."]
pub type CTUchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HS Chirp Timeout Clock Cycles. This configures the chirp timeout used by this device to negotiate a HS connection with a FS Host."]
    #[inline(always)]
    pub fn c_t_uch(&self) -> CTUchR {
        CTUchR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - HS Chirp Timeout Clock Cycles. This configures the chirp timeout used by this device to negotiate a HS connection with a FS Host."]
    #[inline(always)]
    pub fn c_t_uch(&mut self) -> CTUchW<'_, CtuchSpec> {
        CTUchW::new(self, 0)
    }
}
#[doc = "Chirp timeout timer setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctuch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctuch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtuchSpec;
impl crate::RegisterSpec for CtuchSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctuch::R`](R) reader structure"]
impl crate::Readable for CtuchSpec {}
#[doc = "`write(|w| ..)` method takes [`ctuch::W`](W) writer structure"]
impl crate::Writable for CtuchSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTUCH to value 0"]
impl crate::Resettable for CtuchSpec {}
