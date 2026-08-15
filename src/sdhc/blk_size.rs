#[doc = "Register `BLK_SIZE` reader"]
pub type R = crate::R<BlkSizeSpec>;
#[doc = "Register `BLK_SIZE` writer"]
pub type W = crate::W<BlkSizeSpec>;
#[doc = "Field `TRANS` reader - Transfer Block Size."]
pub type TransR = crate::FieldReader<u16>;
#[doc = "Field `TRANS` writer - Transfer Block Size."]
pub type TransW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HOST_BUFF` reader - Host SDMA Buffer Boundary."]
pub type HostBuffR = crate::FieldReader;
#[doc = "Field `HOST_BUFF` writer - Host SDMA Buffer Boundary."]
pub type HostBuffW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size."]
    #[inline(always)]
    pub fn trans(&self) -> TransR {
        TransR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Boundary."]
    #[inline(always)]
    pub fn host_buff(&self) -> HostBuffR {
        HostBuffR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size."]
    #[inline(always)]
    pub fn trans(&mut self) -> TransW<'_, BlkSizeSpec> {
        TransW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Host SDMA Buffer Boundary."]
    #[inline(always)]
    pub fn host_buff(&mut self) -> HostBuffW<'_, BlkSizeSpec> {
        HostBuffW::new(self, 12)
    }
}
#[doc = "Block Size.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkSizeSpec;
impl crate::RegisterSpec for BlkSizeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blk_size::R`](R) reader structure"]
impl crate::Readable for BlkSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_size::W`](W) writer structure"]
impl crate::Writable for BlkSizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK_SIZE to value 0"]
impl crate::Resettable for BlkSizeSpec {}
