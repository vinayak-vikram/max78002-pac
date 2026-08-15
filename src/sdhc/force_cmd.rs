#[doc = "Register `FORCE_CMD` writer"]
pub type W = crate::W<ForceCmdSpec>;
#[doc = "Field `NOT_EXCU` writer - Force Event for Auto CMD12 Not Executed."]
pub type NotExcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO` writer - Force Event for Auto CMD Timeout Error."]
pub type ToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` writer - Force Event for Auto CMD CRC Error."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BIT` writer - Force Event for Auto CMD End Bit Error."]
pub type EndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDEX` writer - Force Event for Auto CMD Index Error."]
pub type IndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_ISSUED` writer - Force Event for Command Not Issued By Auto CMD12 Error."]
pub type NotIssuedW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed."]
    #[inline(always)]
    pub fn not_excu(&mut self) -> NotExcuW<'_, ForceCmdSpec> {
        NotExcuW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error."]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<'_, ForceCmdSpec> {
        ToW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error."]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, ForceCmdSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error."]
    #[inline(always)]
    pub fn end_bit(&mut self) -> EndBitW<'_, ForceCmdSpec> {
        EndBitW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error."]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, ForceCmdSpec> {
        IndexW::new(self, 4)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub fn not_issued(&mut self) -> NotIssuedW<'_, ForceCmdSpec> {
        NotIssuedW::new(self, 7)
    }
}
#[doc = "Force Event for Auto CMD Error Status.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceCmdSpec;
impl crate::RegisterSpec for ForceCmdSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`force_cmd::W`](W) writer structure"]
impl crate::Writable for ForceCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FORCE_CMD to value 0"]
impl crate::Resettable for ForceCmdSpec {}
