#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `fifo_full` reader - FIFO full. Bit n is set when FIFO n is full."]
pub type FifoFullR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - FIFO full. Bit n is set when FIFO n is full."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "FIFO status.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
