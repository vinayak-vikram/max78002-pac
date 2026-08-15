#[doc = "Register `DPHY_RST_N` reader"]
pub type R = crate::R<DphyRstNSpec>;
#[doc = "Register `DPHY_RST_N` writer"]
pub type W = crate::W<DphyRstNSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DPHY_RST_N.\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_rst_n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_rst_n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DphyRstNSpec;
impl crate::RegisterSpec for DphyRstNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_rst_n::R`](R) reader structure"]
impl crate::Readable for DphyRstNSpec {}
#[doc = "`write(|w| ..)` method takes [`dphy_rst_n::W`](W) writer structure"]
impl crate::Writable for DphyRstNSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_RST_N to value 0"]
impl crate::Resettable for DphyRstNSpec {}
