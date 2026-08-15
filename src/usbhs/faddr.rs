#[doc = "Register `FADDR` reader"]
pub type R = crate::R<FaddrSpec>;
#[doc = "Register `FADDR` writer"]
pub type W = crate::W<FaddrSpec>;
#[doc = "Field `ADDR` reader - Function address for this controller."]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Function address for this controller."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `UPDATE` reader - Set when ADDR is written, cleared when new address takes effect."]
pub type UpdateR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Function address for this controller."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Set when ADDR is written, cleared when new address takes effect."]
    #[inline(always)]
    pub fn update(&self) -> UpdateR {
        UpdateR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Function address for this controller."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, FaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Function address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaddrSpec;
impl crate::RegisterSpec for FaddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`faddr::R`](R) reader structure"]
impl crate::Readable for FaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`faddr::W`](W) writer structure"]
impl crate::Writable for FaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FADDR to value 0"]
impl crate::Resettable for FaddrSpec {}
