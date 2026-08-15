#[doc = "Register `CNTRLD` reader"]
pub type R = crate::R<CntrldSpec>;
#[doc = "Register `CNTRLD` writer"]
pub type W = crate::W<CntrldSpec>;
#[doc = "Field `CNT` reader - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Dis,
            true => En::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
impl R {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, CntrldSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 31 - Reload Enable. This bit should be set after the address reload registers have been programmed. This bit is automatically cleared to 0 when reload occurs."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CntrldSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "DMA Channel Count Reload Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cntrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrldSpec;
impl crate::RegisterSpec for CntrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntrld::R`](R) reader structure"]
impl crate::Readable for CntrldSpec {}
#[doc = "`write(|w| ..)` method takes [`cntrld::W`](W) writer structure"]
impl crate::Writable for CntrldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTRLD to value 0"]
impl crate::Resettable for CntrldSpec {}
