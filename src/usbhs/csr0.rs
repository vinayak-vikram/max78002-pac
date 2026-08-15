#[doc = "Register `CSR0` reader"]
pub type R = crate::R<Csr0Spec>;
#[doc = "Register `CSR0` writer"]
pub type W = crate::W<Csr0Spec>;
#[doc = "Field `OUTPKTRDY` reader - EP0 OUT Packet Ready Status Automatically set when a data packet is received in the OUT FIFO. An interrupt is generated when this bit is set. Write a 1 to the servicedoutpktrdy bit (above) to clear after the packet is unloaded from the OUT FIFO."]
pub type OutpktrdyR = crate::BitReader;
#[doc = "Field `INPKTRDY` reader - EP0 IN Packet Ready 1: Write a 1 after writing a data packet to the IN FIFO. Automatically cleared when the data packet is transmitted. An interrupt is generated when this bit is cleared."]
pub type InpktrdyR = crate::BitReader;
#[doc = "Field `INPKTRDY` writer - EP0 IN Packet Ready 1: Write a 1 after writing a data packet to the IN FIFO. Automatically cleared when the data packet is transmitted. An interrupt is generated when this bit is cleared."]
pub type InpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENT_STALL` reader - Read EP0 STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted. Write a 0 to clear."]
pub type SentStallR = crate::BitReader;
#[doc = "Field `SENT_STALL` writer - Read EP0 STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted. Write a 0 to clear."]
pub type SentStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END` reader - Control Transaction Data End. Write a 1 to this bit after firmware completes any of the following three transactions: 1. set inpktrdy = 1 for the last data packet. 2. Set inpktrdy =1 for a zero-length data packet. 3. Clear outpktrdy = 0 after unloading the last data packet."]
pub type DataEndR = crate::BitReader;
#[doc = "Field `DATA_END` writer - Control Transaction Data End. Write a 1 to this bit after firmware completes any of the following three transactions: 1. set inpktrdy = 1 for the last data packet. 2. Set inpktrdy =1 for a zero-length data packet. 3. Clear outpktrdy = 0 after unloading the last data packet."]
pub type DataEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP_END` reader - Read Setup End Status. Automatically set when a contorl transaction ends before the dataend bit has been set by fimrware. An interrupt is generated when this bit is set. Write 1 to servicedsetupend to clear."]
pub type SetupEndR = crate::BitReader;
#[doc = "Field `SEND_STALL` reader - Send EP0 STALL Handshake. Write a 1 to this bit to terminate the current control transaction by sneding a STALL handshake. Automatically cleared after being set."]
pub type SendStallR = crate::BitReader;
#[doc = "Field `SEND_STALL` writer - Send EP0 STALL Handshake. Write a 1 to this bit to terminate the current control transaction by sneding a STALL handshake. Automatically cleared after being set."]
pub type SendStallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERV_OUTPKTRDY` reader - Clear EP0 Out Packet Ready Bit. Write a 1 to clear the outpktrdy bit. Automatically cleared after being set."]
pub type ServOutpktrdyR = crate::BitReader;
#[doc = "Field `SERV_OUTPKTRDY` writer - Clear EP0 Out Packet Ready Bit. Write a 1 to clear the outpktrdy bit. Automatically cleared after being set."]
pub type ServOutpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERV_SETUP_END` reader - Clear EP0 Setup End Bit. Write a 1 to clear the setupend bit. Automatically cleared after being set"]
pub type ServSetupEndR = crate::BitReader;
#[doc = "Field `SERV_SETUP_END` writer - Clear EP0 Setup End Bit. Write a 1 to clear the setupend bit. Automatically cleared after being set"]
pub type ServSetupEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EP0 OUT Packet Ready Status Automatically set when a data packet is received in the OUT FIFO. An interrupt is generated when this bit is set. Write a 1 to the servicedoutpktrdy bit (above) to clear after the packet is unloaded from the OUT FIFO."]
    #[inline(always)]
    pub fn outpktrdy(&self) -> OutpktrdyR {
        OutpktrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP0 IN Packet Ready 1: Write a 1 after writing a data packet to the IN FIFO. Automatically cleared when the data packet is transmitted. An interrupt is generated when this bit is cleared."]
    #[inline(always)]
    pub fn inpktrdy(&self) -> InpktrdyR {
        InpktrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read EP0 STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted. Write a 0 to clear."]
    #[inline(always)]
    pub fn sent_stall(&self) -> SentStallR {
        SentStallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control Transaction Data End. Write a 1 to this bit after firmware completes any of the following three transactions: 1. set inpktrdy = 1 for the last data packet. 2. Set inpktrdy =1 for a zero-length data packet. 3. Clear outpktrdy = 0 after unloading the last data packet."]
    #[inline(always)]
    pub fn data_end(&self) -> DataEndR {
        DataEndR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Setup End Status. Automatically set when a contorl transaction ends before the dataend bit has been set by fimrware. An interrupt is generated when this bit is set. Write 1 to servicedsetupend to clear."]
    #[inline(always)]
    pub fn setup_end(&self) -> SetupEndR {
        SetupEndR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Send EP0 STALL Handshake. Write a 1 to this bit to terminate the current control transaction by sneding a STALL handshake. Automatically cleared after being set."]
    #[inline(always)]
    pub fn send_stall(&self) -> SendStallR {
        SendStallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear EP0 Out Packet Ready Bit. Write a 1 to clear the outpktrdy bit. Automatically cleared after being set."]
    #[inline(always)]
    pub fn serv_outpktrdy(&self) -> ServOutpktrdyR {
        ServOutpktrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear EP0 Setup End Bit. Write a 1 to clear the setupend bit. Automatically cleared after being set"]
    #[inline(always)]
    pub fn serv_setup_end(&self) -> ServSetupEndR {
        ServSetupEndR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EP0 IN Packet Ready 1: Write a 1 after writing a data packet to the IN FIFO. Automatically cleared when the data packet is transmitted. An interrupt is generated when this bit is cleared."]
    #[inline(always)]
    pub fn inpktrdy(&mut self) -> InpktrdyW<'_, Csr0Spec> {
        InpktrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Read EP0 STALL Handshake Sent Status Automatically set when a STALL handshake is transmitted. Write a 0 to clear."]
    #[inline(always)]
    pub fn sent_stall(&mut self) -> SentStallW<'_, Csr0Spec> {
        SentStallW::new(self, 2)
    }
    #[doc = "Bit 3 - Control Transaction Data End. Write a 1 to this bit after firmware completes any of the following three transactions: 1. set inpktrdy = 1 for the last data packet. 2. Set inpktrdy =1 for a zero-length data packet. 3. Clear outpktrdy = 0 after unloading the last data packet."]
    #[inline(always)]
    pub fn data_end(&mut self) -> DataEndW<'_, Csr0Spec> {
        DataEndW::new(self, 3)
    }
    #[doc = "Bit 5 - Send EP0 STALL Handshake. Write a 1 to this bit to terminate the current control transaction by sneding a STALL handshake. Automatically cleared after being set."]
    #[inline(always)]
    pub fn send_stall(&mut self) -> SendStallW<'_, Csr0Spec> {
        SendStallW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear EP0 Out Packet Ready Bit. Write a 1 to clear the outpktrdy bit. Automatically cleared after being set."]
    #[inline(always)]
    pub fn serv_outpktrdy(&mut self) -> ServOutpktrdyW<'_, Csr0Spec> {
        ServOutpktrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear EP0 Setup End Bit. Write a 1 to clear the setupend bit. Automatically cleared after being set"]
    #[inline(always)]
    pub fn serv_setup_end(&mut self) -> ServSetupEndW<'_, Csr0Spec> {
        ServSetupEndW::new(self, 7)
    }
}
#[doc = "Control status register for EP 0 (when INDEX == 0).\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0Spec;
impl crate::RegisterSpec for Csr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr0::R`](R) reader structure"]
impl crate::Readable for Csr0Spec {}
#[doc = "`write(|w| ..)` method takes [`csr0::W`](W) writer structure"]
impl crate::Writable for Csr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0 to value 0"]
impl crate::Resettable for Csr0Spec {}
