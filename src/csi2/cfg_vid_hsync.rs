#[doc = "Register `CFG_VID_HSYNC` reader"]
pub type R = crate::R<CfgVidHsyncSpec>;
#[doc = "Register `CFG_VID_HSYNC` writer"]
pub type W = crate::W<CfgVidHsyncSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_VID_HSYNC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_hsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_hsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgVidHsyncSpec;
impl crate::RegisterSpec for CfgVidHsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_vid_hsync::R`](R) reader structure"]
impl crate::Readable for CfgVidHsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_vid_hsync::W`](W) writer structure"]
impl crate::Writable for CfgVidHsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_VID_HSYNC to value 0"]
impl crate::Resettable for CfgVidHsyncSpec {}
