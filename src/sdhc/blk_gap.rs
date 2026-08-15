#[doc = "Register `BLK_GAP` reader"]
pub type R = crate::R<BlkGapSpec>;
#[doc = "Register `BLK_GAP` writer"]
pub type W = crate::W<BlkGapSpec>;
#[doc = "Field `STOP` reader - Stop At Block Gap Request."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Stop At Block Gap Request."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - Continue Request."]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - Continue Request."]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_WAIT` reader - Read Wait Control."]
pub type ReadWaitR = crate::BitReader;
#[doc = "Field `READ_WAIT` writer - Read Wait Control."]
pub type ReadWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR` reader - Interrupt At Block Gap."]
pub type IntrR = crate::BitReader;
#[doc = "Field `INTR` writer - Interrupt At Block Gap."]
pub type IntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop At Block Gap Request."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request."]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Wait Control."]
    #[inline(always)]
    pub fn read_wait(&self) -> ReadWaitR {
        ReadWaitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap."]
    #[inline(always)]
    pub fn intr(&self) -> IntrR {
        IntrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop At Block Gap Request."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, BlkGapSpec> {
        StopW::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request."]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, BlkGapSpec> {
        ContW::new(self, 1)
    }
    #[doc = "Bit 2 - Read Wait Control."]
    #[inline(always)]
    pub fn read_wait(&mut self) -> ReadWaitW<'_, BlkGapSpec> {
        ReadWaitW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap."]
    #[inline(always)]
    pub fn intr(&mut self) -> IntrW<'_, BlkGapSpec> {
        IntrW::new(self, 3)
    }
}
#[doc = "Block Gap Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_gap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_gap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkGapSpec;
impl crate::RegisterSpec for BlkGapSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`blk_gap::R`](R) reader structure"]
impl crate::Readable for BlkGapSpec {}
#[doc = "`write(|w| ..)` method takes [`blk_gap::W`](W) writer structure"]
impl crate::Writable for BlkGapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK_GAP to value 0"]
impl crate::Resettable for BlkGapSpec {}
