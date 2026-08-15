#[doc = "Register `INMAXP` reader"]
pub type R = crate::R<InmaxpSpec>;
#[doc = "Register `INMAXP` writer"]
pub type W = crate::W<InmaxpSpec>;
#[doc = "Field `MAXPACKETSIZE` reader - Maximum Packet Size in a Single Transaction. That is the maximum packet size in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations of the endpoint type set in USB 2.0 Specification, Chapter 9"]
pub type MaxpacketsizeR = crate::FieldReader<u16>;
#[doc = "Field `MAXPACKETSIZE` writer - Maximum Packet Size in a Single Transaction. That is the maximum packet size in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations of the endpoint type set in USB 2.0 Specification, Chapter 9"]
pub type MaxpacketsizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NUMPACKMINUS1` reader - Number of Split Packets - 1. Defines the maximum number of packets minus 1 that a USB payload can be split into. THis must be an exact multiple of maxpacketsize. Only applicable for HS High-Bandwidth isochronous endpoints and Bulk endpoints. Ignored in all other cases."]
pub type Numpackminus1R = crate::FieldReader;
#[doc = "Field `NUMPACKMINUS1` writer - Number of Split Packets - 1. Defines the maximum number of packets minus 1 that a USB payload can be split into. THis must be an exact multiple of maxpacketsize. Only applicable for HS High-Bandwidth isochronous endpoints and Bulk endpoints. Ignored in all other cases."]
pub type Numpackminus1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size in a Single Transaction. That is the maximum packet size in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations of the endpoint type set in USB 2.0 Specification, Chapter 9"]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MaxpacketsizeR {
        MaxpacketsizeR::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 11:15 - Number of Split Packets - 1. Defines the maximum number of packets minus 1 that a USB payload can be split into. THis must be an exact multiple of maxpacketsize. Only applicable for HS High-Bandwidth isochronous endpoints and Bulk endpoints. Ignored in all other cases."]
    #[inline(always)]
    pub fn numpackminus1(&self) -> Numpackminus1R {
        Numpackminus1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size in a Single Transaction. That is the maximum packet size in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations of the endpoint type set in USB 2.0 Specification, Chapter 9"]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MaxpacketsizeW<'_, InmaxpSpec> {
        MaxpacketsizeW::new(self, 0)
    }
    #[doc = "Bits 11:15 - Number of Split Packets - 1. Defines the maximum number of packets minus 1 that a USB payload can be split into. THis must be an exact multiple of maxpacketsize. Only applicable for HS High-Bandwidth isochronous endpoints and Bulk endpoints. Ignored in all other cases."]
    #[inline(always)]
    pub fn numpackminus1(&mut self) -> Numpackminus1W<'_, InmaxpSpec> {
        Numpackminus1W::new(self, 11)
    }
}
#[doc = "Maximum packet size for INx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`inmaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inmaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InmaxpSpec;
impl crate::RegisterSpec for InmaxpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`inmaxp::R`](R) reader structure"]
impl crate::Readable for InmaxpSpec {}
#[doc = "`write(|w| ..)` method takes [`inmaxp::W`](W) writer structure"]
impl crate::Writable for InmaxpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INMAXP to value 0"]
impl crate::Resettable for InmaxpSpec {}
