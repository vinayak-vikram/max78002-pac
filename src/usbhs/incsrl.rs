#[doc = "Register `INCSRL` reader"]
pub type R = crate::R<IncsrlSpec>;
#[doc = "Register `INCSRL` writer"]
pub type W = crate::W<IncsrlSpec>;
#[doc = "Field `INPKTRDY` reader - IN Packet Ready. Write a 1 to clear"]
pub type InpktrdyR = crate::BitReader;
#[doc = "Field `FIFONOTEMPTY` reader - Read FIFO Not Empty Status. Automatically set when there is at least one packet in the IN FIFO. Write a 0 to clear."]
pub type FifonotemptyR = crate::BitReader;
#[doc = "Field `FIFONOTEMPTY` writer - Read FIFO Not Empty Status. Automatically set when there is at least one packet in the IN FIFO. Write a 0 to clear."]
pub type FifonotemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERRUN` reader - Read IN FIFO Underrun Error Status Isochronous Mode: Automatically set if the IN FIFO is empty. Write 0 to clear"]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - Read IN FIFO Underrun Error Status Isochronous Mode: Automatically set if the IN FIFO is empty. Write 0 to clear"]
pub type UnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSHFIFO` reader - Flush Next Packet from IN FIFO. Write 1 to clear"]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - Flush Next Packet from IN FIFO. Write 1 to clear"]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Send STALL Handshake.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sendstall {
    #[doc = "0: Terminate STALL handhsake"]
    Terminate = 0,
    #[doc = "1: Respond to an IN token with a STALL handshake"]
    Respond = 1,
}
impl From<Sendstall> for bool {
    #[inline(always)]
    fn from(variant: Sendstall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SENDSTALL` reader - Send STALL Handshake."]
pub type SendstallR = crate::BitReader<Sendstall>;
impl SendstallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sendstall {
        match self.bits {
            false => Sendstall::Terminate,
            true => Sendstall::Respond,
        }
    }
    #[doc = "Terminate STALL handhsake"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == Sendstall::Terminate
    }
    #[doc = "Respond to an IN token with a STALL handshake"]
    #[inline(always)]
    pub fn is_respond(&self) -> bool {
        *self == Sendstall::Respond
    }
}
#[doc = "Field `SENTSTALL` reader - Read STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted, at which time the IN FIFO is flushed, and inpktrdy is cleared. Write 0 to clear."]
pub type SentstallR = crate::BitReader;
#[doc = "Field `SENTSTALL` writer - Read STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted, at which time the IN FIFO is flushed, and inpktrdy is cleared. Write 0 to clear."]
pub type SentstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRDATATOG` reader - Write 1 to clear IN endpoint data-toggle to 0."]
pub type ClrdatatogR = crate::BitReader;
#[doc = "Field `CLRDATATOG` writer - Write 1 to clear IN endpoint data-toggle to 0."]
pub type ClrdatatogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPTX` reader - Read Incomplete Split Transfer Error Status High-bandwidth isochronous transfers: Automatically set when a payload is split into multiple packets but insufficient IN tokens were received to send all packets. The current packets is flushed from the IN FIFO. Write 0 to clear."]
pub type IncomptxR = crate::BitReader;
#[doc = "Field `INCOMPTX` writer - Read Incomplete Split Transfer Error Status High-bandwidth isochronous transfers: Automatically set when a payload is split into multiple packets but insufficient IN tokens were received to send all packets. The current packets is flushed from the IN FIFO. Write 0 to clear."]
pub type IncomptxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IN Packet Ready. Write a 1 to clear"]
    #[inline(always)]
    pub fn inpktrdy(&self) -> InpktrdyR {
        InpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read FIFO Not Empty Status. Automatically set when there is at least one packet in the IN FIFO. Write a 0 to clear."]
    #[inline(always)]
    pub fn fifonotempty(&self) -> FifonotemptyR {
        FifonotemptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read IN FIFO Underrun Error Status Isochronous Mode: Automatically set if the IN FIFO is empty. Write 0 to clear"]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flush Next Packet from IN FIFO. Write 1 to clear"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Send STALL Handshake."]
    #[inline(always)]
    pub fn sendstall(&self) -> SendstallR {
        SendstallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted, at which time the IN FIFO is flushed, and inpktrdy is cleared. Write 0 to clear."]
    #[inline(always)]
    pub fn sentstall(&self) -> SentstallR {
        SentstallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to clear IN endpoint data-toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&self) -> ClrdatatogR {
        ClrdatatogR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Incomplete Split Transfer Error Status High-bandwidth isochronous transfers: Automatically set when a payload is split into multiple packets but insufficient IN tokens were received to send all packets. The current packets is flushed from the IN FIFO. Write 0 to clear."]
    #[inline(always)]
    pub fn incomptx(&self) -> IncomptxR {
        IncomptxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Read FIFO Not Empty Status. Automatically set when there is at least one packet in the IN FIFO. Write a 0 to clear."]
    #[inline(always)]
    pub fn fifonotempty(&mut self) -> FifonotemptyW<'_, IncsrlSpec> {
        FifonotemptyW::new(self, 1)
    }
    #[doc = "Bit 2 - Read IN FIFO Underrun Error Status Isochronous Mode: Automatically set if the IN FIFO is empty. Write 0 to clear"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UnderrunW<'_, IncsrlSpec> {
        UnderrunW::new(self, 2)
    }
    #[doc = "Bit 3 - Flush Next Packet from IN FIFO. Write 1 to clear"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, IncsrlSpec> {
        FlushfifoW::new(self, 3)
    }
    #[doc = "Bit 5 - Read STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted, at which time the IN FIFO is flushed, and inpktrdy is cleared. Write 0 to clear."]
    #[inline(always)]
    pub fn sentstall(&mut self) -> SentstallW<'_, IncsrlSpec> {
        SentstallW::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear IN endpoint data-toggle to 0."]
    #[inline(always)]
    pub fn clrdatatog(&mut self) -> ClrdatatogW<'_, IncsrlSpec> {
        ClrdatatogW::new(self, 6)
    }
    #[doc = "Bit 7 - Read Incomplete Split Transfer Error Status High-bandwidth isochronous transfers: Automatically set when a payload is split into multiple packets but insufficient IN tokens were received to send all packets. The current packets is flushed from the IN FIFO. Write 0 to clear."]
    #[inline(always)]
    pub fn incomptx(&mut self) -> IncomptxW<'_, IncsrlSpec> {
        IncomptxW::new(self, 7)
    }
}
#[doc = "Control status lower register for INx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`incsrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`incsrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IncsrlSpec;
impl crate::RegisterSpec for IncsrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`incsrl::R`](R) reader structure"]
impl crate::Readable for IncsrlSpec {}
#[doc = "`write(|w| ..)` method takes [`incsrl::W`](W) writer structure"]
impl crate::Writable for IncsrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INCSRL to value 0"]
impl crate::Resettable for IncsrlSpec {}
