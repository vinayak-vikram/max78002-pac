#[doc = "Register `ADCCFG2` reader"]
pub type R = crate::R<Adccfg2Spec>;
#[doc = "Register `ADCCFG2` writer"]
pub type W = crate::W<Adccfg2Spec>;
#[doc = "Divider option for ADC input channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ch0 {
    #[doc = "0: div1"]
    Div1 = 0,
    #[doc = "1: 5k ohom"]
    Div2_5k = 1,
    #[doc = "2: 50k ohom"]
    Div2_50k = 2,
}
impl From<Ch0> for u8 {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ch0 {
    type Ux = u8;
}
impl crate::IsEnum for Ch0 {}
#[doc = "Field `CH0` reader - Divider option for ADC input channel 0"]
pub type Ch0R = crate::FieldReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ch0> {
        match self.bits {
            0 => Some(Ch0::Div1),
            1 => Some(Ch0::Div2_5k),
            2 => Some(Ch0::Div2_50k),
            _ => None,
        }
    }
    #[doc = "div1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ch0::Div1
    }
    #[doc = "5k ohom"]
    #[inline(always)]
    pub fn is_div2_5k(&self) -> bool {
        *self == Ch0::Div2_5k
    }
    #[doc = "50k ohom"]
    #[inline(always)]
    pub fn is_div2_50k(&self) -> bool {
        *self == Ch0::Div2_50k
    }
}
#[doc = "Field `CH0` writer - Divider option for ADC input channel 0"]
pub type Ch0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "div1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Div1)
    }
    #[doc = "5k ohom"]
    #[inline(always)]
    pub fn div2_5k(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Div2_5k)
    }
    #[doc = "50k ohom"]
    #[inline(always)]
    pub fn div2_50k(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Div2_50k)
    }
}
#[doc = "Divider option for ADC input channel 1"]
pub use Ch0 as Ch1;
#[doc = "Divider option for ADC input channel 2"]
pub use Ch0 as Ch2;
#[doc = "Divider option for ADC input channel 3"]
pub use Ch0 as Ch3;
#[doc = "Divider option for ADC input channel 4"]
pub use Ch0 as Ch4;
#[doc = "Divider option for ADC input channel 5"]
pub use Ch0 as Ch5;
#[doc = "Divider option for ADC input channel 6"]
pub use Ch0 as Ch6;
#[doc = "Divider option for ADC input channel 7"]
pub use Ch0 as Ch7;
#[doc = "Field `CH1` reader - Divider option for ADC input channel 1"]
pub use Ch0R as Ch1R;
#[doc = "Field `CH2` reader - Divider option for ADC input channel 2"]
pub use Ch0R as Ch2R;
#[doc = "Field `CH3` reader - Divider option for ADC input channel 3"]
pub use Ch0R as Ch3R;
#[doc = "Field `CH4` reader - Divider option for ADC input channel 4"]
pub use Ch0R as Ch4R;
#[doc = "Field `CH5` reader - Divider option for ADC input channel 5"]
pub use Ch0R as Ch5R;
#[doc = "Field `CH6` reader - Divider option for ADC input channel 6"]
pub use Ch0R as Ch6R;
#[doc = "Field `CH7` reader - Divider option for ADC input channel 7"]
pub use Ch0R as Ch7R;
#[doc = "Field `CH1` writer - Divider option for ADC input channel 1"]
pub use Ch0W as Ch1W;
#[doc = "Field `CH2` writer - Divider option for ADC input channel 2"]
pub use Ch0W as Ch2W;
#[doc = "Field `CH3` writer - Divider option for ADC input channel 3"]
pub use Ch0W as Ch3W;
#[doc = "Field `CH4` writer - Divider option for ADC input channel 4"]
pub use Ch0W as Ch4W;
#[doc = "Field `CH5` writer - Divider option for ADC input channel 5"]
pub use Ch0W as Ch5W;
#[doc = "Field `CH6` writer - Divider option for ADC input channel 6"]
pub use Ch0W as Ch6W;
#[doc = "Field `CH7` writer - Divider option for ADC input channel 7"]
pub use Ch0W as Ch7W;
impl R {
    #[doc = "Bits 0:1 - Divider option for ADC input channel 0"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Divider option for ADC input channel 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Divider option for ADC input channel 2"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Divider option for ADC input channel 3"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Divider option for ADC input channel 4"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Divider option for ADC input channel 5"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Divider option for ADC input channel 6"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Divider option for ADC input channel 7"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Divider option for ADC input channel 0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<'_, Adccfg2Spec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Divider option for ADC input channel 1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<'_, Adccfg2Spec> {
        Ch1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Divider option for ADC input channel 2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<'_, Adccfg2Spec> {
        Ch2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Divider option for ADC input channel 3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<'_, Adccfg2Spec> {
        Ch3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Divider option for ADC input channel 4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> Ch4W<'_, Adccfg2Spec> {
        Ch4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Divider option for ADC input channel 5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> Ch5W<'_, Adccfg2Spec> {
        Ch5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Divider option for ADC input channel 6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> Ch6W<'_, Adccfg2Spec> {
        Ch6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Divider option for ADC input channel 7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> Ch7W<'_, Adccfg2Spec> {
        Ch7W::new(self, 14)
    }
}
#[doc = "ADC Config 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccfg2Spec;
impl crate::RegisterSpec for Adccfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccfg2::R`](R) reader structure"]
impl crate::Readable for Adccfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`adccfg2::W`](W) writer structure"]
impl crate::Writable for Adccfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCFG2 to value 0"]
impl crate::Resettable for Adccfg2Spec {}
