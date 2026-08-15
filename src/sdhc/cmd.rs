#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `RESP_TYPE` reader - Response Type Select."]
pub type RespTypeR = crate::FieldReader;
#[doc = "Field `RESP_TYPE` writer - Response Type Select."]
pub type RespTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRC_CHK_EN` reader - Command CRC Check Enable."]
pub type CrcChkEnR = crate::BitReader;
#[doc = "Field `CRC_CHK_EN` writer - Command CRC Check Enable."]
pub type CrcChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDX_CHK_EN` reader - Command Index Check Enable."]
pub type IdxChkEnR = crate::BitReader;
#[doc = "Field `IDX_CHK_EN` writer - Command Index Check Enable."]
pub type IdxChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_PRES_SEL` reader - Data Present Select."]
pub type DataPresSelR = crate::BitReader;
#[doc = "Field `DATA_PRES_SEL` writer - Data Present Select."]
pub type DataPresSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE` reader - Command Type."]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - Command Type."]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX` reader - Command Index."]
pub type IdxR = crate::FieldReader;
#[doc = "Field `IDX` writer - Command Index."]
pub type IdxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Response Type Select."]
    #[inline(always)]
    pub fn resp_type(&self) -> RespTypeR {
        RespTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Command CRC Check Enable."]
    #[inline(always)]
    pub fn crc_chk_en(&self) -> CrcChkEnR {
        CrcChkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable."]
    #[inline(always)]
    pub fn idx_chk_en(&self) -> IdxChkEnR {
        IdxChkEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select."]
    #[inline(always)]
    pub fn data_pres_sel(&self) -> DataPresSelR {
        DataPresSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index."]
    #[inline(always)]
    pub fn idx(&self) -> IdxR {
        IdxR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select."]
    #[inline(always)]
    pub fn resp_type(&mut self) -> RespTypeW<'_, CmdSpec> {
        RespTypeW::new(self, 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable."]
    #[inline(always)]
    pub fn crc_chk_en(&mut self) -> CrcChkEnW<'_, CmdSpec> {
        CrcChkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Index Check Enable."]
    #[inline(always)]
    pub fn idx_chk_en(&mut self) -> IdxChkEnW<'_, CmdSpec> {
        IdxChkEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Present Select."]
    #[inline(always)]
    pub fn data_pres_sel(&mut self) -> DataPresSelW<'_, CmdSpec> {
        DataPresSelW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Command Type."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<'_, CmdSpec> {
        TypeW::new(self, 6)
    }
    #[doc = "Bits 8:13 - Command Index."]
    #[inline(always)]
    pub fn idx(&mut self) -> IdxW<'_, CmdSpec> {
        IdxW::new(self, 8)
    }
}
#[doc = "Command.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
