#[doc = "Register `VFIFO_AHBM_MAX_TRANS` reader"]
pub type R = crate::R<VfifoAhbmMaxTransSpec>;
#[doc = "Register `VFIFO_AHBM_MAX_TRANS` writer"]
pub type W = crate::W<VfifoAhbmMaxTransSpec>;
#[doc = "Field `AHBM_MAX_TRANS` reader - AHB master maximal number of transfer word count."]
pub type AhbmMaxTransR = crate::FieldReader<u32>;
#[doc = "Field `AHBM_MAX_TRANS` writer - AHB master maximal number of transfer word count."]
pub type AhbmMaxTransW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AHB master maximal number of transfer word count."]
    #[inline(always)]
    pub fn ahbm_max_trans(&self) -> AhbmMaxTransR {
        AhbmMaxTransR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AHB master maximal number of transfer word count."]
    #[inline(always)]
    pub fn ahbm_max_trans(&mut self) -> AhbmMaxTransW<'_, VfifoAhbmMaxTransSpec> {
        AhbmMaxTransW::new(self, 0)
    }
}
#[doc = "Video FIFO AHB Master Maximal Transfer Number Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_max_trans::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_max_trans::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoAhbmMaxTransSpec;
impl crate::RegisterSpec for VfifoAhbmMaxTransSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ahbm_max_trans::R`](R) reader structure"]
impl crate::Readable for VfifoAhbmMaxTransSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ahbm_max_trans::W`](W) writer structure"]
impl crate::Writable for VfifoAhbmMaxTransSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_AHBM_MAX_TRANS to value 0"]
impl crate::Resettable for VfifoAhbmMaxTransSpec {}
