#[doc = "Register `OUTCSRL` reader"]
pub type R = crate::R<OutcsrlSpec>;
#[doc = "Register `OUTCSRL` writer"]
pub type W = crate::W<OutcsrlSpec>;
#[doc = "Field `OUTPKTRDY` reader - "]
pub type OutpktrdyR = crate::BitReader;
#[doc = "Field `OUTPKTRDY` writer - "]
pub type OutpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - "]
pub type FifofullR = crate::BitReader;
#[doc = "Field `OVERRUN` reader - "]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - "]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAERROR` reader - "]
pub type DataerrorR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` reader - "]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - "]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDSTALL` reader - "]
pub type SendstallR = crate::BitReader;
#[doc = "Field `SENDSTALL` writer - "]
pub type SendstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENTSTALL` reader - "]
pub type SentstallR = crate::BitReader;
#[doc = "Field `SENTSTALL` writer - "]
pub type SentstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRDATATOG` reader - "]
pub type ClrdatatogR = crate::BitReader;
#[doc = "Field `CLRDATATOG` writer - "]
pub type ClrdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OutpktrdyR {
        OutpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifofull(&self) -> FifofullR {
        FifofullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dataerror(&self) -> DataerrorR {
        DataerrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sendstall(&self) -> SendstallR {
        SendstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sentstall(&self) -> SentstallR {
        SentstallR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clrdatatog(&self) -> ClrdatatogR {
        ClrdatatogR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn outpktrdy(&mut self) -> OutpktrdyW<'_, OutcsrlSpec> {
        OutpktrdyW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OverrunW<'_, OutcsrlSpec> {
        OverrunW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, OutcsrlSpec> {
        FlushfifoW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sendstall(&mut self) -> SendstallW<'_, OutcsrlSpec> {
        SendstallW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SentstallW<'_, OutcsrlSpec> {
        SentstallW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> ClrdatatogW<'_, OutcsrlSpec> {
        ClrdatatogW::new(self, 7)
    }
}
#[doc = "Control status lower register for OUTx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outcsrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcsrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutcsrlSpec;
impl crate::RegisterSpec for OutcsrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`outcsrl::R`](R) reader structure"]
impl crate::Readable for OutcsrlSpec {}
#[doc = "`write(|w| ..)` method takes [`outcsrl::W`](W) writer structure"]
impl crate::Writable for OutcsrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTCSRL to value 0"]
impl crate::Resettable for OutcsrlSpec {}
