#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `clear_done` reader - Register clear complete."]
pub type ClearDoneR = crate::BitReader;
impl R {
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
