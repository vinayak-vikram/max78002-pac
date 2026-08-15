#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `READY` reader - Indication that the ADC is in ON power state"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `EMPTY` reader - FIFO Empty"]
pub type EmptyR = crate::BitReader;
#[doc = "Field `FULL` reader - FIFO full"]
pub type FullR = crate::BitReader;
#[doc = "Field `FIFO_LEVEL` reader - Number of entries in FIFO available to read"]
pub type FifoLevelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indication that the ADC is in ON power state"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Empty"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO full"]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Number of entries in FIFO available to read"]
    #[inline(always)]
    pub fn fifo_level(&self) -> FifoLevelR {
        FifoLevelR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
