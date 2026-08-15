#[doc = "Register `RAMINFO` reader"]
pub type R = crate::R<RaminfoSpec>;
#[doc = "Register `RAMINFO` writer"]
pub type W = crate::W<RaminfoSpec>;
#[doc = "Field `RAMBITS` reader - "]
pub type RambitsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rambits(&self) -> RambitsR {
        RambitsR::new(self.bits & 0x0f)
    }
}
impl W {}
#[doc = "RAM width information.\n\nYou can [`read`](crate::Reg::read) this register and get [`raminfo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raminfo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RaminfoSpec;
impl crate::RegisterSpec for RaminfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`raminfo::R`](R) reader structure"]
impl crate::Readable for RaminfoSpec {}
#[doc = "`write(|w| ..)` method takes [`raminfo::W`](W) writer structure"]
impl crate::Writable for RaminfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAMINFO to value 0"]
impl crate::Resettable for RaminfoSpec {}
