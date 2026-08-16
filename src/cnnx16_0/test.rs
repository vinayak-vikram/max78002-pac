#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `data_run` reader - Run the data SRAM BIST."]
pub type DataRunR = crate::BitReader;
#[doc = "Field `data_run` writer - Run the data SRAM BIST."]
pub type DataRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mask_run` reader - Run the mask RAM BIST."]
pub type MaskRunR = crate::BitReader;
#[doc = "Field `mask_run` writer - Run the mask RAM BIST."]
pub type MaskRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tram_run` reader - Run the TRAM BIST."]
pub type TramRunR = crate::BitReader;
#[doc = "Field `tram_run` writer - Run the TRAM BIST."]
pub type TramRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bias_run` reader - Run the bias register file BIST."]
pub type BiasRunR = crate::BitReader;
#[doc = "Field `bias_run` writer - Run the bias register file BIST."]
pub type BiasRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `zero_run` reader - Run zeroization. Mandatory during initialization on this device."]
pub type ZeroRunR = crate::BitReader;
#[doc = "Field `zero_run` writer - Run zeroization. Mandatory during initialization on this device."]
pub type ZeroRunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bias_sel` reader - Include the bias register file in the operation. Set for zeroization only when the network uses bias."]
pub type BiasSelR = crate::BitReader;
#[doc = "Field `bias_sel` writer - Include the bias register file in the operation. Set for zeroization only when the network uses bias."]
pub type BiasSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bist_err` reader - Memory BIST error. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
pub type BistErrR = crate::FieldReader;
#[doc = "Field `bist_done` reader - Memory BIST complete. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
pub type BistDoneR = crate::FieldReader;
#[doc = "Field `zero_done` reader - Zeroization complete."]
pub type ZeroDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Run the data SRAM BIST."]
    #[inline(always)]
    pub fn data_run(&self) -> DataRunR {
        DataRunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Run the mask RAM BIST."]
    #[inline(always)]
    pub fn mask_run(&self) -> MaskRunR {
        MaskRunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Run the TRAM BIST."]
    #[inline(always)]
    pub fn tram_run(&self) -> TramRunR {
        TramRunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Run the bias register file BIST."]
    #[inline(always)]
    pub fn bias_run(&self) -> BiasRunR {
        BiasRunR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run zeroization. Mandatory during initialization on this device."]
    #[inline(always)]
    pub fn zero_run(&self) -> ZeroRunR {
        ZeroRunR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Include the bias register file in the operation. Set for zeroization only when the network uses bias."]
    #[inline(always)]
    pub fn bias_sel(&self) -> BiasSelR {
        BiasSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:17 - Memory BIST error. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
    #[inline(always)]
    pub fn bist_err(&self) -> BistErrR {
        BistErrR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Memory BIST complete. Bit 0 data SRAM, bit 1 mask SRAM, bit 2 tornado SRAM, bit 3 bias register file."]
    #[inline(always)]
    pub fn bist_done(&self) -> BistDoneR {
        BistDoneR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Zeroization complete."]
    #[inline(always)]
    pub fn zero_done(&self) -> ZeroDoneR {
        ZeroDoneR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run the data SRAM BIST."]
    #[inline(always)]
    pub fn data_run(&mut self) -> DataRunW<'_, TestSpec> {
        DataRunW::new(self, 0)
    }
    #[doc = "Bit 2 - Run the mask RAM BIST."]
    #[inline(always)]
    pub fn mask_run(&mut self) -> MaskRunW<'_, TestSpec> {
        MaskRunW::new(self, 2)
    }
    #[doc = "Bit 4 - Run the TRAM BIST."]
    #[inline(always)]
    pub fn tram_run(&mut self) -> TramRunW<'_, TestSpec> {
        TramRunW::new(self, 4)
    }
    #[doc = "Bit 6 - Run the bias register file BIST."]
    #[inline(always)]
    pub fn bias_run(&mut self) -> BiasRunW<'_, TestSpec> {
        BiasRunW::new(self, 6)
    }
    #[doc = "Bit 7 - Run zeroization. Mandatory during initialization on this device."]
    #[inline(always)]
    pub fn zero_run(&mut self) -> ZeroRunW<'_, TestSpec> {
        ZeroRunW::new(self, 7)
    }
    #[doc = "Bit 10 - Include the bias register file in the operation. Set for zeroization only when the network uses bias."]
    #[inline(always)]
    pub fn bias_sel(&mut self) -> BiasSelW<'_, TestSpec> {
        BiasSelW::new(self, 10)
    }
}
#[doc = "Register clear and memory BIST control. Run bits are write-1, then poll the matching done bit, then write zero to reset. Bits 13 to 11 are engine-coupled qualifiers that cannot be varied independently; use the documented composite command values.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {}
