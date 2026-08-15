#[doc = "Register `CFG_PACKET_INTERFACE_EN` reader"]
pub type R = crate::R<CfgPacketInterfaceEnSpec>;
#[doc = "Register `CFG_PACKET_INTERFACE_EN` writer"]
pub type W = crate::W<CfgPacketInterfaceEnSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_PACKET_INTERFACE_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_packet_interface_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_packet_interface_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPacketInterfaceEnSpec;
impl crate::RegisterSpec for CfgPacketInterfaceEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_packet_interface_en::R`](R) reader structure"]
impl crate::Readable for CfgPacketInterfaceEnSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_packet_interface_en::W`](W) writer structure"]
impl crate::Writable for CfgPacketInterfaceEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_PACKET_INTERFACE_EN to value 0"]
impl crate::Resettable for CfgPacketInterfaceEnSpec {}
