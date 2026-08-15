#[doc = "Register `BLK_CNT` reader"]
pub type R = crate::R<BlkCntSpec>;
#[doc = "Register `BLK_CNT` writer"]
pub type W = crate::W<BlkCntSpec>;
#[doc = "Field `COUNT` reader - Blocks Count For Current Transfer."]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - Blocks Count For Current Transfer."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Blocks Count For Current Transfer."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blocks Count For Current Transfer."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<'_, BlkCntSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "Block Count.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkCntSpec;
impl crate::RegisterSpec for BlkCntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blk_cnt::R`](R) reader structure"]
impl crate::Readable for BlkCntSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_cnt::W`](W) writer structure"]
impl crate::Writable for BlkCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK_CNT to value 0"]
impl crate::Resettable for BlkCntSpec {}
