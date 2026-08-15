#[doc = "Register `RESP[%s]` reader"]
pub type R = crate::R<RespSpec>;
#[doc = "Register `RESP[%s]` writer"]
pub type W = crate::W<RespSpec>;
#[doc = "Field `CMD_RESP` reader - Command Response."]
pub type CmdRespR = crate::FieldReader<u32>;
#[doc = "Field `CMD_RESP` writer - Command Response."]
pub type CmdRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response."]
    #[inline(always)]
    pub fn cmd_resp(&self) -> CmdRespR {
        CmdRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Response."]
    #[inline(always)]
    pub fn cmd_resp(&mut self) -> CmdRespW<'_, RespSpec> {
        CmdRespW::new(self, 0)
    }
}
#[doc = "Response 0 Register 0-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`resp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespSpec;
impl crate::RegisterSpec for RespSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp::R`](R) reader structure"]
impl crate::Readable for RespSpec {}
#[doc = "`write(|w| ..)` method takes [`resp::W`](W) writer structure"]
impl crate::Writable for RespSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESP[%s] to value 0"]
impl crate::Resettable for RespSpec {}
