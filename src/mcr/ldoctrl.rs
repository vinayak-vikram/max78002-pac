#[doc = "Register `LDOCTRL` reader"]
pub type R = crate::R<LdoctrlSpec>;
#[doc = "Register `LDOCTRL` writer"]
pub type W = crate::W<LdoctrlSpec>;
#[doc = "Field `0P9EN` reader - LDO 0.9V Enable"]
pub type _0p9enR = crate::BitReader;
#[doc = "Field `0P9EN` writer - LDO 0.9V Enable"]
pub type _0p9enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `2P5EN` reader - LDO 2.5V Enable"]
pub type _2p5enR = crate::BitReader;
#[doc = "Field `2P5EN` writer - LDO 2.5V Enable"]
pub type _2p5enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LDO 0.9V Enable"]
    #[inline(always)]
    pub fn _0p9en(&self) -> _0p9enR {
        _0p9enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LDO 2.5V Enable"]
    #[inline(always)]
    pub fn _2p5en(&self) -> _2p5enR {
        _2p5enR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDO 0.9V Enable"]
    #[inline(always)]
    pub fn _0p9en(&mut self) -> _0p9enW<'_, LdoctrlSpec> {
        _0p9enW::new(self, 0)
    }
    #[doc = "Bit 1 - LDO 2.5V Enable"]
    #[inline(always)]
    pub fn _2p5en(&mut self) -> _2p5enW<'_, LdoctrlSpec> {
        _2p5enW::new(self, 1)
    }
}
#[doc = "LDO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LdoctrlSpec;
impl crate::RegisterSpec for LdoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoctrl::R`](R) reader structure"]
impl crate::Readable for LdoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ldoctrl::W`](W) writer structure"]
impl crate::Writable for LdoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDOCTRL to value 0"]
impl crate::Resettable for LdoctrlSpec {}
