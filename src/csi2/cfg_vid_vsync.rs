#[doc = "Register `CFG_VID_VSYNC` reader"]
pub type R = crate::R<CfgVidVsyncSpec>;
#[doc = "Register `CFG_VID_VSYNC` writer"]
pub type W = crate::W<CfgVidVsyncSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_VID_VSYNC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_vsync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_vsync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgVidVsyncSpec;
impl crate::RegisterSpec for CfgVidVsyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_vid_vsync::R`](R) reader structure"]
impl crate::Readable for CfgVidVsyncSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_vid_vsync::W`](W) writer structure"]
impl crate::Writable for CfgVidVsyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_VID_VSYNC to value 0"]
impl crate::Resettable for CfgVidVsyncSpec {}
