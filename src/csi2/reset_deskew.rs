#[doc = "Register `RESET_DESKEW` reader"]
pub type R = crate::R<ResetDeskewSpec>;
#[doc = "Register `RESET_DESKEW` writer"]
pub type W = crate::W<ResetDeskewSpec>;
#[doc = "Field `DATA_LANE0` reader - DATA_LANE0."]
pub type DataLane0R = crate::BitReader;
#[doc = "Field `DATA_LANE0` writer - DATA_LANE0."]
pub type DataLane0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_LANE1` reader - DATA_LANE1."]
pub type DataLane1R = crate::BitReader;
#[doc = "Field `DATA_LANE1` writer - DATA_LANE1."]
pub type DataLane1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_LANE2` reader - DATA_LANE2."]
pub type DataLane2R = crate::BitReader;
#[doc = "Field `DATA_LANE2` writer - DATA_LANE2."]
pub type DataLane2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_LANE3` reader - DATA_LANE3."]
pub type DataLane3R = crate::BitReader;
#[doc = "Field `DATA_LANE3` writer - DATA_LANE3."]
pub type DataLane3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DATA_LANE0."]
    #[inline(always)]
    pub fn data_lane0(&self) -> DataLane0R {
        DataLane0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DATA_LANE1."]
    #[inline(always)]
    pub fn data_lane1(&self) -> DataLane1R {
        DataLane1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DATA_LANE2."]
    #[inline(always)]
    pub fn data_lane2(&self) -> DataLane2R {
        DataLane2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DATA_LANE3."]
    #[inline(always)]
    pub fn data_lane3(&self) -> DataLane3R {
        DataLane3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DATA_LANE0."]
    #[inline(always)]
    pub fn data_lane0(&mut self) -> DataLane0W<'_, ResetDeskewSpec> {
        DataLane0W::new(self, 0)
    }
    #[doc = "Bit 1 - DATA_LANE1."]
    #[inline(always)]
    pub fn data_lane1(&mut self) -> DataLane1W<'_, ResetDeskewSpec> {
        DataLane1W::new(self, 1)
    }
    #[doc = "Bit 2 - DATA_LANE2."]
    #[inline(always)]
    pub fn data_lane2(&mut self) -> DataLane2W<'_, ResetDeskewSpec> {
        DataLane2W::new(self, 2)
    }
    #[doc = "Bit 3 - DATA_LANE3."]
    #[inline(always)]
    pub fn data_lane3(&mut self) -> DataLane3W<'_, ResetDeskewSpec> {
        DataLane3W::new(self, 3)
    }
}
#[doc = "RESET_DESKEW.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_deskew::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_deskew::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetDeskewSpec;
impl crate::RegisterSpec for ResetDeskewSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_deskew::R`](R) reader structure"]
impl crate::Readable for ResetDeskewSpec {}
#[doc = "`write(|w| ..)` method takes [`reset_deskew::W`](W) writer structure"]
impl crate::Writable for ResetDeskewSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_DESKEW to value 0"]
impl crate::Resettable for ResetDeskewSpec {}
