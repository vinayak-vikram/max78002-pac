#[doc = "Register `AUTO_CMD_ER` reader"]
pub type R = crate::R<AutoCmdErSpec>;
#[doc = "Register `AUTO_CMD_ER` writer"]
pub type W = crate::W<AutoCmdErSpec>;
#[doc = "Field `NOT_EXCUTED` reader - Auto CMD12 Not Executed."]
pub type NotExcutedR = crate::BitReader;
#[doc = "Field `NOT_EXCUTED` writer - Auto CMD12 Not Executed."]
pub type NotExcutedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO` reader - Auto CMD Timeout Error."]
pub type ToR = crate::BitReader;
#[doc = "Field `TO` writer - Auto CMD Timeout Error."]
pub type ToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - Auto CMD CRC Error."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - Auto CMD CRC Error."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_BIT` reader - Auto CMD End Bit Error."]
pub type EndBitR = crate::BitReader;
#[doc = "Field `END_BIT` writer - Auto CMD End Bit Error."]
pub type EndBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INDEX` reader - Auto CMD Index Error."]
pub type IndexR = crate::BitReader;
#[doc = "Field `INDEX` writer - Auto CMD Index Error."]
pub type IndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_ISSUED` reader - Command Not Issued By Auto CMD12 Error."]
pub type NotIssuedR = crate::BitReader;
#[doc = "Field `NOT_ISSUED` writer - Command Not Issued By Auto CMD12 Error."]
pub type NotIssuedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed."]
    #[inline(always)]
    pub fn not_excuted(&self) -> NotExcutedR {
        NotExcutedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error."]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error."]
    #[inline(always)]
    pub fn end_bit(&self) -> EndBitR {
        EndBitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error."]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub fn not_issued(&self) -> NotIssuedR {
        NotIssuedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto CMD12 Not Executed."]
    #[inline(always)]
    pub fn not_excuted(&mut self) -> NotExcutedW<'_, AutoCmdErSpec> {
        NotExcutedW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto CMD Timeout Error."]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<'_, AutoCmdErSpec> {
        ToW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error."]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, AutoCmdErSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error."]
    #[inline(always)]
    pub fn end_bit(&mut self) -> EndBitW<'_, AutoCmdErSpec> {
        EndBitW::new(self, 3)
    }
    #[doc = "Bit 4 - Auto CMD Index Error."]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<'_, AutoCmdErSpec> {
        IndexW::new(self, 4)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error."]
    #[inline(always)]
    pub fn not_issued(&mut self) -> NotIssuedW<'_, AutoCmdErSpec> {
        NotIssuedW::new(self, 7)
    }
}
#[doc = "Auto CMD Error Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`auto_cmd_er::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auto_cmd_er::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutoCmdErSpec;
impl crate::RegisterSpec for AutoCmdErSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`auto_cmd_er::R`](R) reader structure"]
impl crate::Readable for AutoCmdErSpec {}
#[doc = "`write(|w| ..)` method takes [`auto_cmd_er::W`](W) writer structure"]
impl crate::Writable for AutoCmdErSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTO_CMD_ER to value 0"]
impl crate::Resettable for AutoCmdErSpec {}
