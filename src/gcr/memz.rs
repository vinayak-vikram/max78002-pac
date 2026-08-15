#[doc = "Register `MEMZ` reader"]
pub type R = crate::R<MemzSpec>;
#[doc = "Register `MEMZ` writer"]
pub type W = crate::W<MemzSpec>;
#[doc = "System RAM Block 0 Zeroization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram0 {
    #[doc = "0: No operation/complete."]
    Nop = 0,
    #[doc = "1: Start operation."]
    Start = 1,
}
impl From<Ram0> for bool {
    #[inline(always)]
    fn from(variant: Ram0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM0` reader - System RAM Block 0 Zeroization."]
pub type Ram0R = crate::BitReader<Ram0>;
impl Ram0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram0 {
        match self.bits {
            false => Ram0::Nop,
            true => Ram0::Start,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == Ram0::Nop
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Ram0::Start
    }
}
#[doc = "Field `RAM0` writer - System RAM Block 0 Zeroization."]
pub type Ram0W<'a, REG> = crate::BitWriter<'a, REG, Ram0>;
impl<'a, REG> Ram0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0::Nop)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0::Start)
    }
}
#[doc = "System RAM Block 1 Zeroization."]
pub use Ram0 as Ram1;
#[doc = "System RAM Block 2 Zeroization."]
pub use Ram0 as Ram2;
#[doc = "System RAM Block 3 Zeroization."]
pub use Ram0 as Ram3;
#[doc = "System RAM Block 4 Zeroization."]
pub use Ram0 as Ram4;
#[doc = "System RAM Block 5 Zeroization."]
pub use Ram0 as Ram5;
#[doc = "System RAM Block 6 Zeroization."]
pub use Ram0 as Ram6;
#[doc = "System RAM Block 7 Zeroization."]
pub use Ram0 as Ram7;
#[doc = "System RAM Block 0 ECC Zeroization."]
pub use Ram0 as Ram0ecc;
#[doc = "Instruction Cache 0 Zeroization."]
pub use Ram0 as Icc0;
#[doc = "Instruction Cache 1 Zeroization."]
pub use Ram0 as Icc1;
#[doc = "USB FIFO Zeroization."]
pub use Ram0 as Usbfifo;
#[doc = "Field `RAM1` reader - System RAM Block 1 Zeroization."]
pub use Ram0R as Ram1R;
#[doc = "Field `RAM2` reader - System RAM Block 2 Zeroization."]
pub use Ram0R as Ram2R;
#[doc = "Field `RAM3` reader - System RAM Block 3 Zeroization."]
pub use Ram0R as Ram3R;
#[doc = "Field `RAM4` reader - System RAM Block 4 Zeroization."]
pub use Ram0R as Ram4R;
#[doc = "Field `RAM5` reader - System RAM Block 5 Zeroization."]
pub use Ram0R as Ram5R;
#[doc = "Field `RAM6` reader - System RAM Block 6 Zeroization."]
pub use Ram0R as Ram6R;
#[doc = "Field `RAM7` reader - System RAM Block 7 Zeroization."]
pub use Ram0R as Ram7R;
#[doc = "Field `RAM0ECC` reader - System RAM Block 0 ECC Zeroization."]
pub use Ram0R as Ram0eccR;
#[doc = "Field `ICC0` reader - Instruction Cache 0 Zeroization."]
pub use Ram0R as Icc0R;
#[doc = "Field `ICC1` reader - Instruction Cache 1 Zeroization."]
pub use Ram0R as Icc1R;
#[doc = "Field `USBFIFO` reader - USB FIFO Zeroization."]
pub use Ram0R as UsbfifoR;
#[doc = "Field `RAM1` writer - System RAM Block 1 Zeroization."]
pub use Ram0W as Ram1W;
#[doc = "Field `RAM2` writer - System RAM Block 2 Zeroization."]
pub use Ram0W as Ram2W;
#[doc = "Field `RAM3` writer - System RAM Block 3 Zeroization."]
pub use Ram0W as Ram3W;
#[doc = "Field `RAM4` writer - System RAM Block 4 Zeroization."]
pub use Ram0W as Ram4W;
#[doc = "Field `RAM5` writer - System RAM Block 5 Zeroization."]
pub use Ram0W as Ram5W;
#[doc = "Field `RAM6` writer - System RAM Block 6 Zeroization."]
pub use Ram0W as Ram6W;
#[doc = "Field `RAM7` writer - System RAM Block 7 Zeroization."]
pub use Ram0W as Ram7W;
#[doc = "Field `RAM0ECC` writer - System RAM Block 0 ECC Zeroization."]
pub use Ram0W as Ram0eccW;
#[doc = "Field `ICC0` writer - Instruction Cache 0 Zeroization."]
pub use Ram0W as Icc0W;
#[doc = "Field `ICC1` writer - Instruction Cache 1 Zeroization."]
pub use Ram0W as Icc1W;
#[doc = "Field `USBFIFO` writer - USB FIFO Zeroization."]
pub use Ram0W as UsbfifoW;
impl R {
    #[doc = "Bit 0 - System RAM Block 0 Zeroization."]
    #[inline(always)]
    pub fn ram0(&self) -> Ram0R {
        Ram0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM Block 1 Zeroization."]
    #[inline(always)]
    pub fn ram1(&self) -> Ram1R {
        Ram1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM Block 2 Zeroization."]
    #[inline(always)]
    pub fn ram2(&self) -> Ram2R {
        Ram2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM Block 3 Zeroization."]
    #[inline(always)]
    pub fn ram3(&self) -> Ram3R {
        Ram3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System RAM Block 4 Zeroization."]
    #[inline(always)]
    pub fn ram4(&self) -> Ram4R {
        Ram4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System RAM Block 5 Zeroization."]
    #[inline(always)]
    pub fn ram5(&self) -> Ram5R {
        Ram5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System RAM Block 6 Zeroization."]
    #[inline(always)]
    pub fn ram6(&self) -> Ram6R {
        Ram6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - System RAM Block 7 Zeroization."]
    #[inline(always)]
    pub fn ram7(&self) -> Ram7R {
        Ram7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - System RAM Block 0 ECC Zeroization."]
    #[inline(always)]
    pub fn ram0ecc(&self) -> Ram0eccR {
        Ram0eccR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Instruction Cache 0 Zeroization."]
    #[inline(always)]
    pub fn icc0(&self) -> Icc0R {
        Icc0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Instruction Cache 1 Zeroization."]
    #[inline(always)]
    pub fn icc1(&self) -> Icc1R {
        Icc1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB FIFO Zeroization."]
    #[inline(always)]
    pub fn usbfifo(&self) -> UsbfifoR {
        UsbfifoR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM Block 0 Zeroization."]
    #[inline(always)]
    pub fn ram0(&mut self) -> Ram0W<'_, MemzSpec> {
        Ram0W::new(self, 0)
    }
    #[doc = "Bit 1 - System RAM Block 1 Zeroization."]
    #[inline(always)]
    pub fn ram1(&mut self) -> Ram1W<'_, MemzSpec> {
        Ram1W::new(self, 1)
    }
    #[doc = "Bit 2 - System RAM Block 2 Zeroization."]
    #[inline(always)]
    pub fn ram2(&mut self) -> Ram2W<'_, MemzSpec> {
        Ram2W::new(self, 2)
    }
    #[doc = "Bit 3 - System RAM Block 3 Zeroization."]
    #[inline(always)]
    pub fn ram3(&mut self) -> Ram3W<'_, MemzSpec> {
        Ram3W::new(self, 3)
    }
    #[doc = "Bit 4 - System RAM Block 4 Zeroization."]
    #[inline(always)]
    pub fn ram4(&mut self) -> Ram4W<'_, MemzSpec> {
        Ram4W::new(self, 4)
    }
    #[doc = "Bit 5 - System RAM Block 5 Zeroization."]
    #[inline(always)]
    pub fn ram5(&mut self) -> Ram5W<'_, MemzSpec> {
        Ram5W::new(self, 5)
    }
    #[doc = "Bit 6 - System RAM Block 6 Zeroization."]
    #[inline(always)]
    pub fn ram6(&mut self) -> Ram6W<'_, MemzSpec> {
        Ram6W::new(self, 6)
    }
    #[doc = "Bit 7 - System RAM Block 7 Zeroization."]
    #[inline(always)]
    pub fn ram7(&mut self) -> Ram7W<'_, MemzSpec> {
        Ram7W::new(self, 7)
    }
    #[doc = "Bit 8 - System RAM Block 0 ECC Zeroization."]
    #[inline(always)]
    pub fn ram0ecc(&mut self) -> Ram0eccW<'_, MemzSpec> {
        Ram0eccW::new(self, 8)
    }
    #[doc = "Bit 9 - Instruction Cache 0 Zeroization."]
    #[inline(always)]
    pub fn icc0(&mut self) -> Icc0W<'_, MemzSpec> {
        Icc0W::new(self, 9)
    }
    #[doc = "Bit 10 - Instruction Cache 1 Zeroization."]
    #[inline(always)]
    pub fn icc1(&mut self) -> Icc1W<'_, MemzSpec> {
        Icc1W::new(self, 10)
    }
    #[doc = "Bit 11 - USB FIFO Zeroization."]
    #[inline(always)]
    pub fn usbfifo(&mut self) -> UsbfifoW<'_, MemzSpec> {
        UsbfifoW::new(self, 11)
    }
}
#[doc = "Memory Zeroize Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`memz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemzSpec;
impl crate::RegisterSpec for MemzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memz::R`](R) reader structure"]
impl crate::Readable for MemzSpec {}
#[doc = "`write(|w| ..)` method takes [`memz::W`](W) writer structure"]
impl crate::Writable for MemzSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMZ to value 0"]
impl crate::Resettable for MemzSpec {}
