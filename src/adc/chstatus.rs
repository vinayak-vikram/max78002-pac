#[doc = "Register `CHSTATUS` reader"]
pub type R = crate::R<ChstatusSpec>;
#[doc = "Register `CHSTATUS` writer"]
pub type W = crate::W<ChstatusSpec>;
#[doc = "Field `CLIPPED` reader - "]
pub type ClippedR = crate::FieldReader<u32>;
#[doc = "Field `CLIPPED` writer - "]
pub type ClippedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clipped(&self) -> ClippedR {
        ClippedR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clipped(&mut self) -> ClippedW<'_, ChstatusSpec> {
        ClippedW::new(self, 0)
    }
}
#[doc = "Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChstatusSpec;
impl crate::RegisterSpec for ChstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chstatus::R`](R) reader structure"]
impl crate::Readable for ChstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`chstatus::W`](W) writer structure"]
impl crate::Writable for ChstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHSTATUS to value 0"]
impl crate::Resettable for ChstatusSpec {}
