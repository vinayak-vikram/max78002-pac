#[doc = "Register `ULPS_MARK_STATUS` reader"]
pub type R = crate::R<UlpsMarkStatusSpec>;
#[doc = "Register `ULPS_MARK_STATUS` writer"]
pub type W = crate::W<UlpsMarkStatusSpec>;
#[doc = "Field `DATA_LANE0` reader - Data Lane 0."]
pub type DataLane0R = crate::BitReader;
#[doc = "Field `DATA_LANE0` writer - Data Lane 0."]
pub type DataLane0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_LANE1` reader - Data Lane 1."]
pub type DataLane1R = crate::BitReader;
#[doc = "Field `DATA_LANE1` writer - Data Lane 1."]
pub type DataLane1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Lane 0."]
    #[inline(always)]
    pub fn data_lane0(&self) -> DataLane0R {
        DataLane0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Lane 1."]
    #[inline(always)]
    pub fn data_lane1(&self) -> DataLane1R {
        DataLane1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Lane 0."]
    #[inline(always)]
    pub fn data_lane0(&mut self) -> DataLane0W<'_, UlpsMarkStatusSpec> {
        DataLane0W::new(self, 0)
    }
    #[doc = "Bit 1 - Data Lane 1."]
    #[inline(always)]
    pub fn data_lane1(&mut self) -> DataLane1W<'_, UlpsMarkStatusSpec> {
        DataLane1W::new(self, 1)
    }
}
#[doc = "ULPS_MARK_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_mark_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_mark_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UlpsMarkStatusSpec;
impl crate::RegisterSpec for UlpsMarkStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulps_mark_status::R`](R) reader structure"]
impl crate::Readable for UlpsMarkStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ulps_mark_status::W`](W) writer structure"]
impl crate::Writable for UlpsMarkStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ULPS_MARK_STATUS to value 0"]
impl crate::Resettable for UlpsMarkStatusSpec {}
