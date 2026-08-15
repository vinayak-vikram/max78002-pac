#[doc = "Register `VFIFO_AHBM_CTRL` reader"]
pub type R = crate::R<VfifoAhbmCtrlSpec>;
#[doc = "Register `VFIFO_AHBM_CTRL` writer"]
pub type W = crate::W<VfifoAhbmCtrlSpec>;
#[doc = "Field `AHBMEN` reader - AHB Master Enable."]
pub type AhbmenR = crate::BitReader;
#[doc = "Field `AHBMEN` writer - AHB Master Enable."]
pub type AhbmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBMCLR` reader - AHB Master Status Clear."]
pub type AhbmclrR = crate::BitReader;
#[doc = "Field `AHBMCLR` writer - AHB Master Status Clear."]
pub type AhbmclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "AHB Burst Length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bstlen {
    #[doc = "0: Video FIFO THD."]
    VfifoThd = 0,
    #[doc = "1: ONE_WORD."]
    OneWord = 1,
    #[doc = "2: FOUR_WORDS."]
    FourWords = 2,
    #[doc = "3: EIGHT_WORDS."]
    EightWords = 3,
}
impl From<Bstlen> for u8 {
    #[inline(always)]
    fn from(variant: Bstlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bstlen {
    type Ux = u8;
}
impl crate::IsEnum for Bstlen {}
#[doc = "Field `BSTLEN` reader - AHB Burst Length."]
pub type BstlenR = crate::FieldReader<Bstlen>;
impl BstlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bstlen {
        match self.bits {
            0 => Bstlen::VfifoThd,
            1 => Bstlen::OneWord,
            2 => Bstlen::FourWords,
            3 => Bstlen::EightWords,
            _ => unreachable!(),
        }
    }
    #[doc = "Video FIFO THD."]
    #[inline(always)]
    pub fn is_vfifo_thd(&self) -> bool {
        *self == Bstlen::VfifoThd
    }
    #[doc = "ONE_WORD."]
    #[inline(always)]
    pub fn is_one_word(&self) -> bool {
        *self == Bstlen::OneWord
    }
    #[doc = "FOUR_WORDS."]
    #[inline(always)]
    pub fn is_four_words(&self) -> bool {
        *self == Bstlen::FourWords
    }
    #[doc = "EIGHT_WORDS."]
    #[inline(always)]
    pub fn is_eight_words(&self) -> bool {
        *self == Bstlen::EightWords
    }
}
#[doc = "Field `BSTLEN` writer - AHB Burst Length."]
pub type BstlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bstlen, crate::Safe>;
impl<'a, REG> BstlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Video FIFO THD."]
    #[inline(always)]
    pub fn vfifo_thd(self) -> &'a mut crate::W<REG> {
        self.variant(Bstlen::VfifoThd)
    }
    #[doc = "ONE_WORD."]
    #[inline(always)]
    pub fn one_word(self) -> &'a mut crate::W<REG> {
        self.variant(Bstlen::OneWord)
    }
    #[doc = "FOUR_WORDS."]
    #[inline(always)]
    pub fn four_words(self) -> &'a mut crate::W<REG> {
        self.variant(Bstlen::FourWords)
    }
    #[doc = "EIGHT_WORDS."]
    #[inline(always)]
    pub fn eight_words(self) -> &'a mut crate::W<REG> {
        self.variant(Bstlen::EightWords)
    }
}
impl R {
    #[doc = "Bit 0 - AHB Master Enable."]
    #[inline(always)]
    pub fn ahbmen(&self) -> AhbmenR {
        AhbmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Master Status Clear."]
    #[inline(always)]
    pub fn ahbmclr(&self) -> AhbmclrR {
        AhbmclrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - AHB Burst Length."]
    #[inline(always)]
    pub fn bstlen(&self) -> BstlenR {
        BstlenR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHB Master Enable."]
    #[inline(always)]
    pub fn ahbmen(&mut self) -> AhbmenW<'_, VfifoAhbmCtrlSpec> {
        AhbmenW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB Master Status Clear."]
    #[inline(always)]
    pub fn ahbmclr(&mut self) -> AhbmclrW<'_, VfifoAhbmCtrlSpec> {
        AhbmclrW::new(self, 1)
    }
    #[doc = "Bits 4:5 - AHB Burst Length."]
    #[inline(always)]
    pub fn bstlen(&mut self) -> BstlenW<'_, VfifoAhbmCtrlSpec> {
        BstlenW::new(self, 4)
    }
}
#[doc = "Video FIFO AHB Master Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VfifoAhbmCtrlSpec;
impl crate::RegisterSpec for VfifoAhbmCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vfifo_ahbm_ctrl::R`](R) reader structure"]
impl crate::Readable for VfifoAhbmCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vfifo_ahbm_ctrl::W`](W) writer structure"]
impl crate::Writable for VfifoAhbmCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VFIFO_AHBM_CTRL to value 0"]
impl crate::Resettable for VfifoAhbmCtrlSpec {}
