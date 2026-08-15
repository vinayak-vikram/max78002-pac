#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA` reader - Conversion data."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `CHAN` reader - Channel for the data."]
pub type ChanR = crate::FieldReader;
#[doc = "Field `INVALID` reader - Invalid status for the data."]
pub type InvalidR = crate::BitReader;
#[doc = "Field `CLIPPED` reader - Clipped status for the data."]
pub type ClippedR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Conversion data."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:20 - Channel for the data."]
    #[inline(always)]
    pub fn chan(&self) -> ChanR {
        ChanR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Invalid status for the data."]
    #[inline(always)]
    pub fn invalid(&self) -> InvalidR {
        InvalidR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - Clipped status for the data."]
    #[inline(always)]
    pub fn clipped(&self) -> ClippedR {
        ClippedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Data Register (FIFO).\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
