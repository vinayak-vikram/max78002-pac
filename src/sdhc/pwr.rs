#[doc = "Register `PWR` reader"]
pub type R = crate::R<PwrSpec>;
#[doc = "Register `PWR` writer"]
pub type W = crate::W<PwrSpec>;
#[doc = "Field `BUS_POWER` reader - SD Bus Power."]
pub type BusPowerR = crate::BitReader;
#[doc = "Field `BUS_POWER` writer - SD Bus Power."]
pub type BusPowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_VOLT_SEL` reader - SD Bus Voltage Select."]
pub type BusVoltSelR = crate::FieldReader;
#[doc = "Field `BUS_VOLT_SEL` writer - SD Bus Voltage Select."]
pub type BusVoltSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - SD Bus Power."]
    #[inline(always)]
    pub fn bus_power(&self) -> BusPowerR {
        BusPowerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select."]
    #[inline(always)]
    pub fn bus_volt_sel(&self) -> BusVoltSelR {
        BusVoltSelR::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - SD Bus Power."]
    #[inline(always)]
    pub fn bus_power(&mut self) -> BusPowerW<'_, PwrSpec> {
        BusPowerW::new(self, 0)
    }
    #[doc = "Bits 1:3 - SD Bus Voltage Select."]
    #[inline(always)]
    pub fn bus_volt_sel(&mut self) -> BusVoltSelW<'_, PwrSpec> {
        BusVoltSelW::new(self, 1)
    }
}
#[doc = "Power Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrSpec;
impl crate::RegisterSpec for PwrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwr::R`](R) reader structure"]
impl crate::Readable for PwrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr::W`](W) writer structure"]
impl crate::Writable for PwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR to value 0"]
impl crate::Resettable for PwrSpec {}
