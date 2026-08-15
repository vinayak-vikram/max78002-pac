#[doc = "Register `M31_PHY_U2_COMPLIANCE_DAC_ADJ` reader"]
pub type R = crate::R<M31PhyU2ComplianceDacAdjSpec>;
#[doc = "Register `M31_PHY_U2_COMPLIANCE_DAC_ADJ` writer"]
pub type W = crate::W<M31PhyU2ComplianceDacAdjSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M31_PHY_U2_COMPLIANCE_DAC_ADJ\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_u2_compliance_dac_adj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_u2_compliance_dac_adj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M31PhyU2ComplianceDacAdjSpec;
impl crate::RegisterSpec for M31PhyU2ComplianceDacAdjSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m31_phy_u2_compliance_dac_adj::R`](R) reader structure"]
impl crate::Readable for M31PhyU2ComplianceDacAdjSpec {}
#[doc = "`write(|w| ..)` method takes [`m31_phy_u2_compliance_dac_adj::W`](W) writer structure"]
impl crate::Writable for M31PhyU2ComplianceDacAdjSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M31_PHY_U2_COMPLIANCE_DAC_ADJ to value 0"]
impl crate::Resettable for M31PhyU2ComplianceDacAdjSpec {}
