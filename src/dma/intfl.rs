#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Inactive,
            true => Ch0::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ch0::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Ch0::Pending
    }
}
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch1;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch2;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch3;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch4;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch5;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch6;
#[doc = "Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0 as Ch7;
#[doc = "Field `CH1` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch1R;
#[doc = "Field `CH2` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch2R;
#[doc = "Field `CH3` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch3R;
#[doc = "Field `CH4` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch4R;
#[doc = "Field `CH5` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch5R;
#[doc = "Field `CH6` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch6R;
#[doc = "Field `CH7` reader - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
pub use Ch0R as Ch7R;
impl R {
    #[doc = "Bit 0 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Interrupt. To clear an interrupt, all active interrupt bits of the DMA_ST must be cleared. The interrupt bits are set only if their corresponding interrupt enable bits are set in DMA_CN."]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Interrupt Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {}
