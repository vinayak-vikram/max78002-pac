#[doc = "Register `EPINFO` reader"]
pub type R = crate::R<EpinfoSpec>;
#[doc = "Register `EPINFO` writer"]
pub type W = crate::W<EpinfoSpec>;
#[doc = "Field `INTENDPOINTS` reader - "]
pub type IntendpointsR = crate::FieldReader;
#[doc = "Field `OUTENDPOINTS` reader - "]
pub type OutendpointsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn intendpoints(&self) -> IntendpointsR {
        IntendpointsR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn outendpoints(&self) -> OutendpointsR {
        OutendpointsR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {}
#[doc = "Endpoint hardware information.\n\nYou can [`read`](crate::Reg::read) this register and get [`epinfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpinfoSpec;
impl crate::RegisterSpec for EpinfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`epinfo::R`](R) reader structure"]
impl crate::Readable for EpinfoSpec {}
#[doc = "`write(|w| ..)` method takes [`epinfo::W`](W) writer structure"]
impl crate::Writable for EpinfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPINFO to value 0"]
impl crate::Resettable for EpinfoSpec {}
