#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FstatSpec>;
#[doc = "FPU Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpu {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Fpu> for bool {
    #[inline(always)]
    fn from(variant: Fpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPU` reader - FPU Function."]
pub type FpuR = crate::BitReader<Fpu>;
impl FpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpu {
        match self.bits {
            false => Fpu::No,
            true => Fpu::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Fpu::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Fpu::Yes
    }
}
#[doc = "USB Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Usb> for bool {
    #[inline(always)]
    fn from(variant: Usb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` reader - USB Function."]
pub type UsbR = crate::BitReader<Usb>;
impl UsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb {
        match self.bits {
            false => Usb::No,
            true => Usb::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Usb::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Usb::Yes
    }
}
#[doc = "ADC Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Adc> for bool {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - ADC Function."]
pub type AdcR = crate::BitReader<Adc>;
impl AdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            false => Adc::No,
            true => Adc::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Adc::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Adc::Yes
    }
}
#[doc = "SDIO Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdio {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Sdio> for bool {
    #[inline(always)]
    fn from(variant: Sdio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO` reader - SDIO Function."]
pub type SdioR = crate::BitReader<Sdio>;
impl SdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio {
        match self.bits {
            false => Sdio::No,
            true => Sdio::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Sdio::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Sdio::Yes
    }
}
#[doc = "SMPHR function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smphr {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Smphr> for bool {
    #[inline(always)]
    fn from(variant: Smphr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMPHR` reader - SMPHR function."]
pub type SmphrR = crate::BitReader<Smphr>;
impl SmphrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smphr {
        match self.bits {
            false => Smphr::No,
            true => Smphr::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Smphr::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Smphr::Yes
    }
}
impl R {
    #[doc = "Bit 0 - FPU Function."]
    #[inline(always)]
    pub fn fpu(&self) -> FpuR {
        FpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Function."]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Function."]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - SDIO Function."]
    #[inline(always)]
    pub fn sdio(&self) -> SdioR {
        SdioR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SMPHR function."]
    #[inline(always)]
    pub fn smphr(&self) -> SmphrR {
        SmphrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "funcstat register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstatSpec;
impl crate::RegisterSpec for FstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FstatSpec {}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FstatSpec {}
