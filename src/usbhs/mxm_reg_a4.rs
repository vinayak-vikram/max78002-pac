#[doc = "Register `MXM_REG_A4` reader"]
pub type R = crate::R<MxmRegA4Spec>;
#[doc = "Register `MXM_REG_A4` writer"]
pub type W = crate::W<MxmRegA4Spec>;
#[doc = "Field `VRST_VDDB_N_A` reader - VRST_VDDB_N_A"]
pub type VrstVddbNAR = crate::BitReader;
#[doc = "Field `VRST_VDDB_N_A` writer - VRST_VDDB_N_A"]
pub type VrstVddbNAW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VRST_VDDB_N_A"]
    #[inline(always)]
    pub fn vrst_vddb_n_a(&self) -> VrstVddbNAR {
        VrstVddbNAR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VRST_VDDB_N_A"]
    #[inline(always)]
    pub fn vrst_vddb_n_a(&mut self) -> VrstVddbNAW<'_, MxmRegA4Spec> {
        VrstVddbNAW::new(self, 0)
    }
}
#[doc = "USB Added Maxim Power Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_reg_a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_reg_a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MxmRegA4Spec;
impl crate::RegisterSpec for MxmRegA4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mxm_reg_a4::R`](R) reader structure"]
impl crate::Readable for MxmRegA4Spec {}
#[doc = "`write(|w| ..)` method takes [`mxm_reg_a4::W`](W) writer structure"]
impl crate::Writable for MxmRegA4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MXM_REG_A4 to value 0"]
impl crate::Resettable for MxmRegA4Spec {}
