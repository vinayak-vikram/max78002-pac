#[doc = "Register `REG2` reader"]
pub type R = crate::R<Reg2Spec>;
#[doc = "Register `REG2` writer"]
pub type W = crate::W<Reg2Spec>;
#[doc = "Field `cnnx16_0_iso` reader - CNNx16_0 Power Domain Isolation"]
pub type Cnnx16_0IsoR = crate::BitReader;
#[doc = "Field `cnnx16_0_iso` writer - CNNx16_0 Power Domain Isolation"]
pub type Cnnx16_0IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_iso` reader - CNNx16_1 Power Domain Isolation"]
pub type Cnnx16_1IsoR = crate::BitReader;
#[doc = "Field `cnnx16_1_iso` writer - CNNx16_1 Power Domain Isolation"]
pub type Cnnx16_1IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_iso` reader - CNNx16_2 Power Domain Isolation"]
pub type Cnnx16_2IsoR = crate::BitReader;
#[doc = "Field `cnnx16_2_iso` writer - CNNx16_2 Power Domain Isolation"]
pub type Cnnx16_2IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_iso` reader - CNNx16_3 Power Domain Isolation"]
pub type Cnnx16_3IsoR = crate::BitReader;
#[doc = "Field `cnnx16_3_iso` writer - CNNx16_3 Power Domain Isolation"]
pub type Cnnx16_3IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_0_data_ret_en` reader - CNNx16_0 Pad Retention Control"]
pub type Cnnx16_0DataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_0_data_ret_en` writer - CNNx16_0 Pad Retention Control"]
pub type Cnnx16_0DataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_data_ret_en` reader - CNNx16_1 Pad Retention Control"]
pub type Cnnx16_1DataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_1_data_ret_en` writer - CNNx16_1 Pad Retention Control"]
pub type Cnnx16_1DataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_data_ret_en` reader - CNNx16_2 Pad Retention Control"]
pub type Cnnx16_2DataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_2_data_ret_en` writer - CNNx16_2 Pad Retention Control"]
pub type Cnnx16_2DataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_data_ret_en` reader - CNNx16_3 Pad Retention Control"]
pub type Cnnx16_3DataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_3_data_ret_en` writer - CNNx16_3 Pad Retention Control"]
pub type Cnnx16_3DataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_0_ram_data_ret_en` reader - CNNx16_0 RAM Pad Retention Control"]
pub type Cnnx16_0RamDataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_0_ram_data_ret_en` writer - CNNx16_0 RAM Pad Retention Control"]
pub type Cnnx16_0RamDataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_ram_data_ret_en` reader - CNNx16_1 RAM Pad Retention Control"]
pub type Cnnx16_1RamDataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_1_ram_data_ret_en` writer - CNNx16_1 RAM Pad Retention Control"]
pub type Cnnx16_1RamDataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_ram_data_ret_en` reader - CNNx16_2 RAM Pad Retention Control"]
pub type Cnnx16_2RamDataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_2_ram_data_ret_en` writer - CNNx16_2 RAM Pad Retention Control"]
pub type Cnnx16_2RamDataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_ram_data_ret_en` reader - CNNx16_3 RAM Pad Retention Control"]
pub type Cnnx16_3RamDataRetEnR = crate::BitReader;
#[doc = "Field `cnnx16_3_ram_data_ret_en` writer - CNNx16_3 RAM Pad Retention Control"]
pub type Cnnx16_3RamDataRetEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_0_iso(&self) -> Cnnx16_0IsoR {
        Cnnx16_0IsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_1_iso(&self) -> Cnnx16_1IsoR {
        Cnnx16_1IsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_2_iso(&self) -> Cnnx16_2IsoR {
        Cnnx16_2IsoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_3_iso(&self) -> Cnnx16_3IsoR {
        Cnnx16_3IsoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - CNNx16_0 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_0_data_ret_en(&self) -> Cnnx16_0DataRetEnR {
        Cnnx16_0DataRetEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CNNx16_1 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_1_data_ret_en(&self) -> Cnnx16_1DataRetEnR {
        Cnnx16_1DataRetEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CNNx16_2 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_2_data_ret_en(&self) -> Cnnx16_2DataRetEnR {
        Cnnx16_2DataRetEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CNNx16_3 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_3_data_ret_en(&self) -> Cnnx16_3DataRetEnR {
        Cnnx16_3DataRetEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CNNx16_0 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_0_ram_data_ret_en(&self) -> Cnnx16_0RamDataRetEnR {
        Cnnx16_0RamDataRetEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CNNx16_1 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_1_ram_data_ret_en(&self) -> Cnnx16_1RamDataRetEnR {
        Cnnx16_1RamDataRetEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CNNx16_2 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_2_ram_data_ret_en(&self) -> Cnnx16_2RamDataRetEnR {
        Cnnx16_2RamDataRetEnR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CNNx16_3 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_3_ram_data_ret_en(&self) -> Cnnx16_3RamDataRetEnR {
        Cnnx16_3RamDataRetEnR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_0_iso(&mut self) -> Cnnx16_0IsoW<'_, Reg2Spec> {
        Cnnx16_0IsoW::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_1_iso(&mut self) -> Cnnx16_1IsoW<'_, Reg2Spec> {
        Cnnx16_1IsoW::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_2_iso(&mut self) -> Cnnx16_2IsoW<'_, Reg2Spec> {
        Cnnx16_2IsoW::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_3_iso(&mut self) -> Cnnx16_3IsoW<'_, Reg2Spec> {
        Cnnx16_3IsoW::new(self, 3)
    }
    #[doc = "Bit 16 - CNNx16_0 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_0_data_ret_en(&mut self) -> Cnnx16_0DataRetEnW<'_, Reg2Spec> {
        Cnnx16_0DataRetEnW::new(self, 16)
    }
    #[doc = "Bit 17 - CNNx16_1 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_1_data_ret_en(&mut self) -> Cnnx16_1DataRetEnW<'_, Reg2Spec> {
        Cnnx16_1DataRetEnW::new(self, 17)
    }
    #[doc = "Bit 18 - CNNx16_2 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_2_data_ret_en(&mut self) -> Cnnx16_2DataRetEnW<'_, Reg2Spec> {
        Cnnx16_2DataRetEnW::new(self, 18)
    }
    #[doc = "Bit 19 - CNNx16_3 Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_3_data_ret_en(&mut self) -> Cnnx16_3DataRetEnW<'_, Reg2Spec> {
        Cnnx16_3DataRetEnW::new(self, 19)
    }
    #[doc = "Bit 20 - CNNx16_0 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_0_ram_data_ret_en(&mut self) -> Cnnx16_0RamDataRetEnW<'_, Reg2Spec> {
        Cnnx16_0RamDataRetEnW::new(self, 20)
    }
    #[doc = "Bit 21 - CNNx16_1 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_1_ram_data_ret_en(&mut self) -> Cnnx16_1RamDataRetEnW<'_, Reg2Spec> {
        Cnnx16_1RamDataRetEnW::new(self, 21)
    }
    #[doc = "Bit 22 - CNNx16_2 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_2_ram_data_ret_en(&mut self) -> Cnnx16_2RamDataRetEnW<'_, Reg2Spec> {
        Cnnx16_2RamDataRetEnW::new(self, 22)
    }
    #[doc = "Bit 23 - CNNx16_3 RAM Pad Retention Control"]
    #[inline(always)]
    pub fn cnnx16_3_ram_data_ret_en(&mut self) -> Cnnx16_3RamDataRetEnW<'_, Reg2Spec> {
        Cnnx16_3RamDataRetEnW::new(self, 23)
    }
}
#[doc = "Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg2Spec;
impl crate::RegisterSpec for Reg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg2::R`](R) reader structure"]
impl crate::Readable for Reg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg2::W`](W) writer structure"]
impl crate::Writable for Reg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG2 to value 0"]
impl crate::Resettable for Reg2Spec {}
