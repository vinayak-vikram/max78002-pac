#[doc = "Register `VFIFO_AHBM_TRANS_CNT` reader"]
pub type R = crate::R<VfifoAhbmTransCntSpec>;
#[doc = "Register `VFIFO_AHBM_TRANS_CNT` writer"]
pub type W = crate::W<VfifoAhbmTransCntSpec>;
#[doc = "Field `AHBM_TRANS_CNT` reader - AHB master number of words been transferred."]
pub type AhbmTransCntR = crate::FieldReader<u32>;
#[doc = "Field `AHBM_TRANS_CNT` writer - AHB master number of words been transferred."]
pub type AhbmTransCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AHB master number of words been transferred."]
    #[inline(always)]
    pub fn ahbm_trans_cnt(&self) -> AhbmTransCntR {
        AhbmTransCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AHB master number of words been transferred."]
    #[inline(always)]
    pub fn ahbm_trans_cnt(&mut self) -> AhbmTransCntW<'_, VfifoAhbmTransCntSpec> {
        AhbmTransCntW::new(self, 0)
    }
}
#[doc = "Video FIFO AHB Master Transfer Count Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_trans_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_trans_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoAhbmTransCntSpec;
impl crate::RegisterSpec for VfifoAhbmTransCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ahbm_trans_cnt::R`](R) reader structure"]
impl crate::Readable for VfifoAhbmTransCntSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ahbm_trans_cnt::W`](W) writer structure"]
impl crate::Writable for VfifoAhbmTransCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_AHBM_TRANS_CNT to value 0"]
impl crate::Resettable for VfifoAhbmTransCntSpec {}
