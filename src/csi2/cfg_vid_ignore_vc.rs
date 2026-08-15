#[doc = "Register `CFG_VID_IGNORE_VC` reader"]
pub type R = crate::R<CfgVidIgnoreVcSpec>;
#[doc = "Register `CFG_VID_IGNORE_VC` writer"]
pub type W = crate::W<CfgVidIgnoreVcSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_VID_IGNORE_VC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_ignore_vc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_ignore_vc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgVidIgnoreVcSpec;
impl crate::RegisterSpec for CfgVidIgnoreVcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_vid_ignore_vc::R`](R) reader structure"]
impl crate::Readable for CfgVidIgnoreVcSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_vid_ignore_vc::W`](W) writer structure"]
impl crate::Writable for CfgVidIgnoreVcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG_VID_IGNORE_VC to value 0"]
impl crate::Resettable for CfgVidIgnoreVcSpec {}
