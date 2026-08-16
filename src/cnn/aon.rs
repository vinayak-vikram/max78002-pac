#[doc = "Register `AON` reader"]
pub type R = crate::R<AonSpec>;
#[doc = "Register `AON` writer"]
pub type W = crate::W<AonSpec>;
#[doc = "Field `rdy_sel` reader - APB ready wait select for the always-on domain."]
pub type RdySelR = crate::FieldReader;
#[doc = "Field `rdy_sel` writer - APB ready wait select for the always-on domain."]
pub type RdySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `quad_pd` reader - Quadrant power down. Bit n powers down quadrant n and is set for each quadrant the network does not use."]
pub type QuadPdR = crate::FieldReader;
#[doc = "Field `quad_pd` writer - Quadrant power down. Bit n powers down quadrant n and is set for each quadrant the network does not use."]
pub type QuadPdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - APB ready wait select for the always-on domain."]
    #[inline(always)]
    pub fn rdy_sel(&self) -> RdySelR {
        RdySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 12:15 - Quadrant power down. Bit n powers down quadrant n and is set for each quadrant the network does not use."]
    #[inline(always)]
    pub fn quad_pd(&self) -> QuadPdR {
        QuadPdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB ready wait select for the always-on domain."]
    #[inline(always)]
    pub fn rdy_sel(&mut self) -> RdySelW<'_, AonSpec> {
        RdySelW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Quadrant power down. Bit n powers down quadrant n and is set for each quadrant the network does not use."]
    #[inline(always)]
    pub fn quad_pd(&mut self) -> QuadPdW<'_, AonSpec> {
        QuadPdW::new(self, 12)
    }
}
#[doc = "Always-on domain control. Reset to zero during initialization.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSpec;
impl crate::RegisterSpec for AonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon::R`](R) reader structure"]
impl crate::Readable for AonSpec {}
#[doc = "`write(|w| ..)` method takes [`aon::W`](W) writer structure"]
impl crate::Writable for AonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AON to value 0"]
impl crate::Resettable for AonSpec {}
