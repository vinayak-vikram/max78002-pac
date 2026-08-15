#[doc = "Register `DBG2_MUX_SEL` reader"]
pub type R = crate::R<Dbg2MuxSelSpec>;
#[doc = "Register `DBG2_MUX_SEL` writer"]
pub type W = crate::W<Dbg2MuxSelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DBG2_MUX_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg2_mux_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg2_mux_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg2MuxSelSpec;
impl crate::RegisterSpec for Dbg2MuxSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg2_mux_sel::R`](R) reader structure"]
impl crate::Readable for Dbg2MuxSelSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg2_mux_sel::W`](W) writer structure"]
impl crate::Writable for Dbg2MuxSelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG2_MUX_SEL to value 0"]
impl crate::Resettable for Dbg2MuxSelSpec {}
