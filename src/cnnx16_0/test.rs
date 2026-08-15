#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `bist_err` reader - Memory BIST error. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
pub type BistErrR = crate::FieldReader;
#[doc = "Field `bist_done` reader - Memory BIST complete. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
pub type BistDoneR = crate::FieldReader;
#[doc = "Field `clear_done` reader - Register clear complete."]
pub type ClearDoneR = crate::BitReader;
impl R {
    #[doc = "Bits 14:17 - Memory BIST error. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
    #[inline(always)]
    pub fn bist_err(&self) -> BistErrR {
        BistErrR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Memory BIST complete. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
    #[inline(always)]
    pub fn bist_done(&self) -> BistDoneR {
        BistDoneR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Register clear complete."]
    #[inline(always)]
    pub fn clear_done(&self) -> ClearDoneR {
        ClearDoneR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {}
#[doc = "Register clear and memory BIST control. Written as a composite value; only the completion flag below is named.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
