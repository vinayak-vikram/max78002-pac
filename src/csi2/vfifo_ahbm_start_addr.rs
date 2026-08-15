#[doc = "Register `VFIFO_AHBM_START_ADDR` reader"]
pub type R = crate::R<VfifoAhbmStartAddrSpec>;
#[doc = "Register `VFIFO_AHBM_START_ADDR` writer"]
pub type W = crate::W<VfifoAhbmStartAddrSpec>;
#[doc = "Field `AHBM_START_ADDR` reader - AHB master transfer starting address, word-aligned."]
pub type AhbmStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `AHBM_START_ADDR` writer - AHB master transfer starting address, word-aligned."]
pub type AhbmStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - AHB master transfer starting address, word-aligned."]
    #[inline(always)]
    pub fn ahbm_start_addr(&self) -> AhbmStartAddrR {
        AhbmStartAddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - AHB master transfer starting address, word-aligned."]
    #[inline(always)]
    pub fn ahbm_start_addr(&mut self) -> AhbmStartAddrW<'_, VfifoAhbmStartAddrSpec> {
        AhbmStartAddrW::new(self, 2)
    }
}
#[doc = "Video FIFO AHB Master Start Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoAhbmStartAddrSpec;
impl crate::RegisterSpec for VfifoAhbmStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ahbm_start_addr::R`](R) reader structure"]
impl crate::Readable for VfifoAhbmStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ahbm_start_addr::W`](W) writer structure"]
impl crate::Writable for VfifoAhbmStartAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_AHBM_START_ADDR to value 0"]
impl crate::Resettable for VfifoAhbmStartAddrSpec {}
