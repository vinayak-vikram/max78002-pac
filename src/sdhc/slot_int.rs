#[doc = "Register `SLOT_INT` reader"]
pub type R = crate::R<SlotIntSpec>;
#[doc = "Field `INT_SIGNALS` reader - Interrupt Signal For Each Slot."]
pub type IntSignalsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Signal For Each Slot."]
    #[inline(always)]
    pub fn int_signals(&self) -> IntSignalsR {
        IntSignalsR::new((self.bits & 1) != 0)
    }
}
#[doc = "Slot Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`slot_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlotIntSpec;
impl crate::RegisterSpec for SlotIntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`slot_int::R`](R) reader structure"]
impl crate::Readable for SlotIntSpec {}
#[doc = "`reset()` method sets SLOT_INT to value 0"]
impl crate::Resettable for SlotIntSpec {}
