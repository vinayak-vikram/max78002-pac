#[doc = "Register `OUTMAXP` reader"]
pub type R = crate::R<OutmaxpSpec>;
#[doc = "Register `OUTMAXP` writer"]
pub type W = crate::W<OutmaxpSpec>;
#[doc = "Field `MAXPACKETSIZE` reader - Maximum Packet in a Single Transaction. This is the maximum packet size, in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations for the endpoint type set in USB2.0 spesification, chapter 9."]
pub type MaxpacketsizeR = crate::FieldReader<u16>;
#[doc = "Field `MAXPACKETSIZE` writer - Maximum Packet in a Single Transaction. This is the maximum packet size, in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations for the endpoint type set in USB2.0 spesification, chapter 9."]
pub type MaxpacketsizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NUMPACKMINUS1` reader - Number of Split Packets -1. Defines the maximum number of packets - 1 that a usb payload is combined into. The value must be exact multiple of maxpacketsize."]
pub type Numpackminus1R = crate::FieldReader;
#[doc = "Field `NUMPACKMINUS1` writer - Number of Split Packets -1. Defines the maximum number of packets - 1 that a usb payload is combined into. The value must be exact multiple of maxpacketsize."]
pub type Numpackminus1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet in a Single Transaction. This is the maximum packet size, in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations for the endpoint type set in USB2.0 spesification, chapter 9."]
    #[inline(always)]
    pub fn maxpacketsize(&self) -> MaxpacketsizeR {
        MaxpacketsizeR::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 11:15 - Number of Split Packets -1. Defines the maximum number of packets - 1 that a usb payload is combined into. The value must be exact multiple of maxpacketsize."]
    #[inline(always)]
    pub fn numpackminus1(&self) -> Numpackminus1R {
        Numpackminus1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet in a Single Transaction. This is the maximum packet size, in bytes, that is transmitted for each microframe. The maximum value is 1024, subject to the limitations for the endpoint type set in USB2.0 spesification, chapter 9."]
    #[inline(always)]
    pub fn maxpacketsize(&mut self) -> MaxpacketsizeW<'_, OutmaxpSpec> {
        MaxpacketsizeW::new(self, 0)
    }
    #[doc = "Bits 11:15 - Number of Split Packets -1. Defines the maximum number of packets - 1 that a usb payload is combined into. The value must be exact multiple of maxpacketsize."]
    #[inline(always)]
    pub fn numpackminus1(&mut self) -> Numpackminus1W<'_, OutmaxpSpec> {
        Numpackminus1W::new(self, 11)
    }
}
#[doc = "Maximum packet size for OUTx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outmaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outmaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutmaxpSpec;
impl crate::RegisterSpec for OutmaxpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`outmaxp::R`](R) reader structure"]
impl crate::Readable for OutmaxpSpec {}
#[doc = "`write(|w| ..)` method takes [`outmaxp::W`](W) writer structure"]
impl crate::Writable for OutmaxpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTMAXP to value 0"]
impl crate::Resettable for OutmaxpSpec {}
