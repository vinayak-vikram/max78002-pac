#[doc = "Register `IPLL_CTRL` reader"]
pub type R = crate::R<IpllCtrlSpec>;
#[doc = "Register `IPLL_CTRL` writer"]
pub type W = crate::W<IpllCtrlSpec>;
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDY` reader - "]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - "]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, IpllCtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<'_, IpllCtrlSpec> {
        RdyW::new(self, 1)
    }
}
#[doc = "IPLL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ipll_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipll_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpllCtrlSpec;
impl crate::RegisterSpec for IpllCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipll_ctrl::R`](R) reader structure"]
impl crate::Readable for IpllCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ipll_ctrl::W`](W) writer structure"]
impl crate::Writable for IpllCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPLL_CTRL to value 0"]
impl crate::Resettable for IpllCtrlSpec {}
