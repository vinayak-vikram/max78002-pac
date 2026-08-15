#[doc = "Register `DATAFMT` reader"]
pub type R = crate::R<DatafmtSpec>;
#[doc = "Register `DATAFMT` writer"]
pub type W = crate::W<DatafmtSpec>;
#[doc = "Field `MODE` reader - Data format control"]
pub type ModeR = crate::FieldReader<u32>;
#[doc = "Field `MODE` writer - Data format control"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data format control"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data format control"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DatafmtSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Channel Data Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datafmt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datafmt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatafmtSpec;
impl crate::RegisterSpec for DatafmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datafmt::R`](R) reader structure"]
impl crate::Readable for DatafmtSpec {}
#[doc = "`write(|w| ..)` method takes [`datafmt::W`](W) writer structure"]
impl crate::Writable for DatafmtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAFMT to value 0"]
impl crate::Resettable for DatafmtSpec {}
