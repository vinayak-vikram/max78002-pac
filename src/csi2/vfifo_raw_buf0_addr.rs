#[doc = "Register `VFIFO_RAW_BUF0_ADDR` reader"]
pub type R = crate::R<VfifoRawBuf0AddrSpec>;
#[doc = "Register `VFIFO_RAW_BUF0_ADDR` writer"]
pub type W = crate::W<VfifoRawBuf0AddrSpec>;
#[doc = "Field `ADDR` reader - RAM address for RAW conversion buffer 0, word-aligned."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - RAM address for RAW conversion buffer 0, word-aligned."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - RAM address for RAW conversion buffer 0, word-aligned."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - RAM address for RAW conversion buffer 0, word-aligned."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, VfifoRawBuf0AddrSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Video FIFO RAW-to-RGB Line Buffer0 Address.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_raw_buf0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_raw_buf0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoRawBuf0AddrSpec;
impl crate::RegisterSpec for VfifoRawBuf0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_raw_buf0_addr::R`](R) reader structure"]
impl crate::Readable for VfifoRawBuf0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_raw_buf0_addr::W`](W) writer structure"]
impl crate::Writable for VfifoRawBuf0AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_RAW_BUF0_ADDR to value 0"]
impl crate::Resettable for VfifoRawBuf0AddrSpec {}
