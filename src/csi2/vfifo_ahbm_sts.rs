#[doc = "Register `VFIFO_AHBM_STS` reader"]
pub type R = crate::R<VfifoAhbmStsSpec>;
#[doc = "Register `VFIFO_AHBM_STS` writer"]
pub type W = crate::W<VfifoAhbmStsSpec>;
#[doc = "Field `HRDY_TO` reader - AHB master HREADY time-out."]
pub type HrdyToR = crate::BitReader;
#[doc = "Field `HRDY_TO` writer - AHB master HREADY time-out."]
pub type HrdyToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_TO` reader - AHB master Idle time-out."]
pub type IdleToR = crate::BitReader;
#[doc = "Field `IDLE_TO` writer - AHB master Idle time-out."]
pub type IdleToW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_MAX` reader - AHB master maximal transfer count occurrence."]
pub type TransMaxR = crate::BitReader;
#[doc = "Field `TRANS_MAX` writer - AHB master maximal transfer count occurrence."]
pub type TransMaxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB master HREADY time-out."]
    #[inline(always)]
    pub fn hrdy_to(&self) -> HrdyToR {
        HrdyToR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB master Idle time-out."]
    #[inline(always)]
    pub fn idle_to(&self) -> IdleToR {
        IdleToR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB master maximal transfer count occurrence."]
    #[inline(always)]
    pub fn trans_max(&self) -> TransMaxR {
        TransMaxR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB master HREADY time-out."]
    #[inline(always)]
    pub fn hrdy_to(&mut self) -> HrdyToW<'_, VfifoAhbmStsSpec> {
        HrdyToW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB master Idle time-out."]
    #[inline(always)]
    pub fn idle_to(&mut self) -> IdleToW<'_, VfifoAhbmStsSpec> {
        IdleToW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB master maximal transfer count occurrence."]
    #[inline(always)]
    pub fn trans_max(&mut self) -> TransMaxW<'_, VfifoAhbmStsSpec> {
        TransMaxW::new(self, 2)
    }
}
#[doc = "Video FIFO AHB Master Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoAhbmStsSpec;
impl crate::RegisterSpec for VfifoAhbmStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ahbm_sts::R`](R) reader structure"]
impl crate::Readable for VfifoAhbmStsSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ahbm_sts::W`](W) writer structure"]
impl crate::Writable for VfifoAhbmStsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_AHBM_STS to value 0"]
impl crate::Resettable for VfifoAhbmStsSpec {}
