#[doc = "Register `VFIFO_AHBM_ADDR_RANGE` reader"]
pub type R = crate::R<VfifoAhbmAddrRangeSpec>;
#[doc = "Register `VFIFO_AHBM_ADDR_RANGE` writer"]
pub type W = crate::W<VfifoAhbmAddrRangeSpec>;
#[doc = "Field `AHBM_ADDR_RANGE` reader - AHB master address range."]
pub type AhbmAddrRangeR = crate::FieldReader<u16>;
#[doc = "Field `AHBM_ADDR_RANGE` writer - AHB master address range."]
pub type AhbmAddrRangeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 2:15 - AHB master address range."]
    #[inline(always)]
    pub fn ahbm_addr_range(&self) -> AhbmAddrRangeR {
        AhbmAddrRangeR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - AHB master address range."]
    #[inline(always)]
    pub fn ahbm_addr_range(&mut self) -> AhbmAddrRangeW<'_, VfifoAhbmAddrRangeSpec> {
        AhbmAddrRangeW::new(self, 2)
    }
}
#[doc = "Video FIFO AHB Master Address Range Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_addr_range::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_addr_range::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoAhbmAddrRangeSpec;
impl crate::RegisterSpec for VfifoAhbmAddrRangeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ahbm_addr_range::R`](R) reader structure"]
impl crate::Readable for VfifoAhbmAddrRangeSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ahbm_addr_range::W`](W) writer structure"]
impl crate::Writable for VfifoAhbmAddrRangeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_AHBM_ADDR_RANGE to value 0"]
impl crate::Resettable for VfifoAhbmAddrRangeSpec {}
