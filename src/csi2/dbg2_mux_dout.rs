#[doc = "Register `DBG2_MUX_DOUT` reader"]
pub type R = crate::R<Dbg2MuxDoutSpec>;
#[doc = "Register `DBG2_MUX_DOUT` writer"]
pub type W = crate::W<Dbg2MuxDoutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DBG2_MUX_DOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg2_mux_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg2_mux_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbg2MuxDoutSpec;
impl crate::RegisterSpec for Dbg2MuxDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg2_mux_dout::R`](R) reader structure"]
impl crate::Readable for Dbg2MuxDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`dbg2_mux_dout::W`](W) writer structure"]
impl crate::Writable for Dbg2MuxDoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG2_MUX_DOUT to value 0"]
impl crate::Resettable for Dbg2MuxDoutSpec {}
