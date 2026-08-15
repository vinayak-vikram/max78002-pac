#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    faddr: Faddr,
    power: Power,
    intrin: Intrin,
    introut: Introut,
    intrinen: Intrinen,
    introuten: Introuten,
    intrusb: Intrusb,
    intrusben: Intrusben,
    frame: Frame,
    index: Index,
    testmode: Testmode,
    inmaxp: Inmaxp,
    _reserved_12_csr0: [u8; 0x01],
    incsru: Incsru,
    outmaxp: Outmaxp,
    outcsrl: Outcsrl,
    outcsru: Outcsru,
    _reserved_17_count0: [u8; 0x02],
    _reserved18: [u8; 0x06],
    fifo0: Fifo0,
    fifo1: Fifo1,
    fifo2: Fifo2,
    fifo3: Fifo3,
    fifo4: Fifo4,
    fifo5: Fifo5,
    fifo6: Fifo6,
    fifo7: Fifo7,
    fifo8: Fifo8,
    fifo9: Fifo9,
    fifo10: Fifo10,
    fifo11: Fifo11,
    fifo12: Fifo12,
    fifo13: Fifo13,
    fifo14: Fifo14,
    fifo15: Fifo15,
    _reserved34: [u8; 0x0c],
    hwvers: Hwvers,
    _reserved35: [u8; 0x0a],
    epinfo: Epinfo,
    raminfo: Raminfo,
    softreset: Softreset,
    _reserved38: [u8; 0x05],
    ctuch: Ctuch,
    cthsrtn: Cthsrtn,
    _reserved40: [u8; 0x037c],
    mxm_usb_reg_00: MxmUsbReg00,
    m31_phy_utmi_reset: M31PhyUtmiReset,
    m31_phy_utmi_vcontrol: M31PhyUtmiVcontrol,
    m31_phy_clk_en: M31PhyClkEn,
    m31_phy_ponrst: M31PhyPonrst,
    m31_phy_noncry_rstb: M31PhyNoncryRstb,
    m31_phy_noncry_en: M31PhyNoncryEn,
    _reserved47: [u8; 0x04],
    m31_phy_u2_compliance_en: M31PhyU2ComplianceEn,
    m31_phy_u2_compliance_dac_adj: M31PhyU2ComplianceDacAdj,
    m31_phy_u2_compliance_dac_adj_en: M31PhyU2ComplianceDacAdjEn,
    m31_phy_clk_rdy: M31PhyClkRdy,
    m31_phy_pll_en: M31PhyPllEn,
    m31_phy_bist_ok: M31PhyBistOk,
    m31_phy_data_oe: M31PhyDataOe,
    m31_phy_oscouten: M31PhyOscouten,
    m31_phy_lpm_alive: M31PhyLpmAlive,
    m31_phy_hs_bist_mode: M31PhyHsBistMode,
    m31_phy_coreclkin: M31PhyCoreclkin,
    m31_phy_xtlsel: M31PhyXtlsel,
    m31_phy_ls_en: M31PhyLsEn,
    m31_phy_debug_sel: M31PhyDebugSel,
    m31_phy_debug_out: M31PhyDebugOut,
    m31_phy_outclksel: M31PhyOutclksel,
    m31_phy_xcfgi_31_0: M31PhyXcfgi31_0,
    m31_phy_xcfgi_63_32: M31PhyXcfgi63_32,
    m31_phy_xcfgi_95_64: M31PhyXcfgi95_64,
    m31_phy_xcfgi_127_96: M31PhyXcfgi127_96,
    m31_phy_xcfgi_137_128: M31PhyXcfgi137_128,
    m31_phy_xcfg_hs_coarse_tune_num: M31PhyXcfgHsCoarseTuneNum,
    m31_phy_xcfg_hs_fine_tune_num: M31PhyXcfgHsFineTuneNum,
    m31_phy_xcfg_fs_coarse_tune_num: M31PhyXcfgFsCoarseTuneNum,
    m31_phy_xcfg_fs_fine_tune_num: M31PhyXcfgFsFineTuneNum,
    m31_phy_xcfg_lock_range_max: M31PhyXcfgLockRangeMax,
    m31_phy_xcfgi_lock_range_min: M31PhyXcfgiLockRangeMin,
    m31_phy_xcfg_ob_rsel: M31PhyXcfgObRsel,
    m31_phy_xcfg_oc_rsel: M31PhyXcfgOcRsel,
    m31_phy_xcfgo: M31PhyXcfgo,
    mxm_int: MxmInt,
    mxm_int_en: MxmIntEn,
    mxm_suspend: MxmSuspend,
    mxm_reg_a4: MxmRegA4,
}
impl RegisterBlock {
    #[doc = "0x00 - Function address register."]
    #[inline(always)]
    pub const fn faddr(&self) -> &Faddr {
        &self.faddr
    }
    #[doc = "0x01 - Power management register."]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
    #[doc = "0x02 - Interrupt register for EP0 and IN EP1-15."]
    #[inline(always)]
    pub const fn intrin(&self) -> &Intrin {
        &self.intrin
    }
    #[doc = "0x04 - Interrupt register for OUT EP 1-15."]
    #[inline(always)]
    pub const fn introut(&self) -> &Introut {
        &self.introut
    }
    #[doc = "0x06 - Interrupt enable for EP 0 and IN EP 1-15."]
    #[inline(always)]
    pub const fn intrinen(&self) -> &Intrinen {
        &self.intrinen
    }
    #[doc = "0x08 - Interrupt enable for OUT EP 1-15."]
    #[inline(always)]
    pub const fn introuten(&self) -> &Introuten {
        &self.introuten
    }
    #[doc = "0x0a - Interrupt register for common USB interrupts."]
    #[inline(always)]
    pub const fn intrusb(&self) -> &Intrusb {
        &self.intrusb
    }
    #[doc = "0x0b - Interrupt enable for common USB interrupts."]
    #[inline(always)]
    pub const fn intrusben(&self) -> &Intrusben {
        &self.intrusben
    }
    #[doc = "0x0c - Frame number."]
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    #[doc = "0x0e - Index for banked registers."]
    #[inline(always)]
    pub const fn index(&self) -> &Index {
        &self.index
    }
    #[doc = "0x0f - USB 2.0 test mode enable register."]
    #[inline(always)]
    pub const fn testmode(&self) -> &Testmode {
        &self.testmode
    }
    #[doc = "0x10 - Maximum packet size for INx endpoint (x == INDEX)."]
    #[inline(always)]
    pub const fn inmaxp(&self) -> &Inmaxp {
        &self.inmaxp
    }
    #[doc = "0x12 - Control status lower register for INx endpoint (x == INDEX)."]
    #[inline(always)]
    pub const fn incsrl(&self) -> &Incsrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x12 - Control status register for EP 0 (when INDEX == 0)."]
    #[inline(always)]
    pub const fn csr0(&self) -> &Csr0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }
    #[doc = "0x13 - Control status upper register for INx endpoint (x == INDEX)."]
    #[inline(always)]
    pub const fn incsru(&self) -> &Incsru {
        &self.incsru
    }
    #[doc = "0x14 - Maximum packet size for OUTx endpoint (x == INDEX)."]
    #[inline(always)]
    pub const fn outmaxp(&self) -> &Outmaxp {
        &self.outmaxp
    }
    #[doc = "0x16 - Control status lower register for OUTx endpoint (x == INDEX)."]
    #[inline(always)]
    pub const fn outcsrl(&self) -> &Outcsrl {
        &self.outcsrl
    }
    #[doc = "0x17 - Control status upper register for OUTx endpoint (x == INDEX)."]
    #[inline(always)]
    pub const fn outcsru(&self) -> &Outcsru {
        &self.outcsru
    }
    #[doc = "0x18 - Number of received bytes in OUT EPx FIFO (x == INDEX)."]
    #[inline(always)]
    pub const fn outcount(&self) -> &Outcount {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18 - Number of received bytes in EP 0 FIFO (INDEX == 0)."]
    #[inline(always)]
    pub const fn count0(&self) -> &Count0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo0(&self) -> &Fifo0 {
        &self.fifo0
    }
    #[doc = "0x24 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo1(&self) -> &Fifo1 {
        &self.fifo1
    }
    #[doc = "0x28 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo2(&self) -> &Fifo2 {
        &self.fifo2
    }
    #[doc = "0x2c - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo3(&self) -> &Fifo3 {
        &self.fifo3
    }
    #[doc = "0x30 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo4(&self) -> &Fifo4 {
        &self.fifo4
    }
    #[doc = "0x34 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo5(&self) -> &Fifo5 {
        &self.fifo5
    }
    #[doc = "0x38 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo6(&self) -> &Fifo6 {
        &self.fifo6
    }
    #[doc = "0x3c - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo7(&self) -> &Fifo7 {
        &self.fifo7
    }
    #[doc = "0x40 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo8(&self) -> &Fifo8 {
        &self.fifo8
    }
    #[doc = "0x44 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo9(&self) -> &Fifo9 {
        &self.fifo9
    }
    #[doc = "0x48 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo10(&self) -> &Fifo10 {
        &self.fifo10
    }
    #[doc = "0x4c - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo11(&self) -> &Fifo11 {
        &self.fifo11
    }
    #[doc = "0x50 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo12(&self) -> &Fifo12 {
        &self.fifo12
    }
    #[doc = "0x54 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo13(&self) -> &Fifo13 {
        &self.fifo13
    }
    #[doc = "0x58 - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo14(&self) -> &Fifo14 {
        &self.fifo14
    }
    #[doc = "0x5c - Read for OUT data FIFO, write for IN data FIFO."]
    #[inline(always)]
    pub const fn fifo15(&self) -> &Fifo15 {
        &self.fifo15
    }
    #[doc = "0x6c - HWVERS"]
    #[inline(always)]
    pub const fn hwvers(&self) -> &Hwvers {
        &self.hwvers
    }
    #[doc = "0x78 - Endpoint hardware information."]
    #[inline(always)]
    pub const fn epinfo(&self) -> &Epinfo {
        &self.epinfo
    }
    #[doc = "0x79 - RAM width information."]
    #[inline(always)]
    pub const fn raminfo(&self) -> &Raminfo {
        &self.raminfo
    }
    #[doc = "0x7a - Software reset register."]
    #[inline(always)]
    pub const fn softreset(&self) -> &Softreset {
        &self.softreset
    }
    #[doc = "0x80 - Chirp timeout timer setting."]
    #[inline(always)]
    pub const fn ctuch(&self) -> &Ctuch {
        &self.ctuch
    }
    #[doc = "0x82 - Sets delay between HS resume to UTM normal operating mode."]
    #[inline(always)]
    pub const fn cthsrtn(&self) -> &Cthsrtn {
        &self.cthsrtn
    }
    #[doc = "0x400 - MXM_USB_REG_00"]
    #[inline(always)]
    pub const fn mxm_usb_reg_00(&self) -> &MxmUsbReg00 {
        &self.mxm_usb_reg_00
    }
    #[doc = "0x404 - M31_PHY_UTMI_RESET"]
    #[inline(always)]
    pub const fn m31_phy_utmi_reset(&self) -> &M31PhyUtmiReset {
        &self.m31_phy_utmi_reset
    }
    #[doc = "0x408 - M31_PHY_UTMI_VCONTROL"]
    #[inline(always)]
    pub const fn m31_phy_utmi_vcontrol(&self) -> &M31PhyUtmiVcontrol {
        &self.m31_phy_utmi_vcontrol
    }
    #[doc = "0x40c - M31_PHY_CLK_EN"]
    #[inline(always)]
    pub const fn m31_phy_clk_en(&self) -> &M31PhyClkEn {
        &self.m31_phy_clk_en
    }
    #[doc = "0x410 - M31_PHY_PONRST"]
    #[inline(always)]
    pub const fn m31_phy_ponrst(&self) -> &M31PhyPonrst {
        &self.m31_phy_ponrst
    }
    #[doc = "0x414 - M31_PHY_NONCRY_RSTB"]
    #[inline(always)]
    pub const fn m31_phy_noncry_rstb(&self) -> &M31PhyNoncryRstb {
        &self.m31_phy_noncry_rstb
    }
    #[doc = "0x418 - M31_PHY_NONCRY_EN"]
    #[inline(always)]
    pub const fn m31_phy_noncry_en(&self) -> &M31PhyNoncryEn {
        &self.m31_phy_noncry_en
    }
    #[doc = "0x420 - M31_PHY_U2_COMPLIANCE_EN"]
    #[inline(always)]
    pub const fn m31_phy_u2_compliance_en(&self) -> &M31PhyU2ComplianceEn {
        &self.m31_phy_u2_compliance_en
    }
    #[doc = "0x424 - M31_PHY_U2_COMPLIANCE_DAC_ADJ"]
    #[inline(always)]
    pub const fn m31_phy_u2_compliance_dac_adj(&self) -> &M31PhyU2ComplianceDacAdj {
        &self.m31_phy_u2_compliance_dac_adj
    }
    #[doc = "0x428 - M31_PHY_U2_COMPLIANCE_DAC_ADJ_EN"]
    #[inline(always)]
    pub const fn m31_phy_u2_compliance_dac_adj_en(&self) -> &M31PhyU2ComplianceDacAdjEn {
        &self.m31_phy_u2_compliance_dac_adj_en
    }
    #[doc = "0x42c - M31_PHY_CLK_RDY"]
    #[inline(always)]
    pub const fn m31_phy_clk_rdy(&self) -> &M31PhyClkRdy {
        &self.m31_phy_clk_rdy
    }
    #[doc = "0x430 - M31_PHY_PLL_EN"]
    #[inline(always)]
    pub const fn m31_phy_pll_en(&self) -> &M31PhyPllEn {
        &self.m31_phy_pll_en
    }
    #[doc = "0x434 - M31_PHY_BIST_OK"]
    #[inline(always)]
    pub const fn m31_phy_bist_ok(&self) -> &M31PhyBistOk {
        &self.m31_phy_bist_ok
    }
    #[doc = "0x438 - M31_PHY_DATA_OE"]
    #[inline(always)]
    pub const fn m31_phy_data_oe(&self) -> &M31PhyDataOe {
        &self.m31_phy_data_oe
    }
    #[doc = "0x43c - M31_PHY_OSCOUTEN"]
    #[inline(always)]
    pub const fn m31_phy_oscouten(&self) -> &M31PhyOscouten {
        &self.m31_phy_oscouten
    }
    #[doc = "0x440 - M31_PHY_LPM_ALIVE"]
    #[inline(always)]
    pub const fn m31_phy_lpm_alive(&self) -> &M31PhyLpmAlive {
        &self.m31_phy_lpm_alive
    }
    #[doc = "0x444 - M31_PHY_HS_BIST_MODE"]
    #[inline(always)]
    pub const fn m31_phy_hs_bist_mode(&self) -> &M31PhyHsBistMode {
        &self.m31_phy_hs_bist_mode
    }
    #[doc = "0x448 - M31_PHY_CORECLKIN"]
    #[inline(always)]
    pub const fn m31_phy_coreclkin(&self) -> &M31PhyCoreclkin {
        &self.m31_phy_coreclkin
    }
    #[doc = "0x44c - M31_PHY_XTLSEL"]
    #[inline(always)]
    pub const fn m31_phy_xtlsel(&self) -> &M31PhyXtlsel {
        &self.m31_phy_xtlsel
    }
    #[doc = "0x450 - M31_PHY_LS_EN"]
    #[inline(always)]
    pub const fn m31_phy_ls_en(&self) -> &M31PhyLsEn {
        &self.m31_phy_ls_en
    }
    #[doc = "0x454 - M31_PHY_DEBUG_SEL"]
    #[inline(always)]
    pub const fn m31_phy_debug_sel(&self) -> &M31PhyDebugSel {
        &self.m31_phy_debug_sel
    }
    #[doc = "0x458 - M31_PHY_DEBUG_OUT"]
    #[inline(always)]
    pub const fn m31_phy_debug_out(&self) -> &M31PhyDebugOut {
        &self.m31_phy_debug_out
    }
    #[doc = "0x45c - M31_PHY_OUTCLKSEL"]
    #[inline(always)]
    pub const fn m31_phy_outclksel(&self) -> &M31PhyOutclksel {
        &self.m31_phy_outclksel
    }
    #[doc = "0x460 - M31_PHY_XCFGI_31_0"]
    #[inline(always)]
    pub const fn m31_phy_xcfgi_31_0(&self) -> &M31PhyXcfgi31_0 {
        &self.m31_phy_xcfgi_31_0
    }
    #[doc = "0x464 - M31_PHY_XCFGI_63_32"]
    #[inline(always)]
    pub const fn m31_phy_xcfgi_63_32(&self) -> &M31PhyXcfgi63_32 {
        &self.m31_phy_xcfgi_63_32
    }
    #[doc = "0x468 - M31_PHY_XCFGI_95_64"]
    #[inline(always)]
    pub const fn m31_phy_xcfgi_95_64(&self) -> &M31PhyXcfgi95_64 {
        &self.m31_phy_xcfgi_95_64
    }
    #[doc = "0x46c - M31_PHY_XCFGI_127_96"]
    #[inline(always)]
    pub const fn m31_phy_xcfgi_127_96(&self) -> &M31PhyXcfgi127_96 {
        &self.m31_phy_xcfgi_127_96
    }
    #[doc = "0x470 - M31_PHY_XCFGI_137_128"]
    #[inline(always)]
    pub const fn m31_phy_xcfgi_137_128(&self) -> &M31PhyXcfgi137_128 {
        &self.m31_phy_xcfgi_137_128
    }
    #[doc = "0x474 - M31_PHY_XCFG_HS_COARSE_TUNE_NUM"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_hs_coarse_tune_num(&self) -> &M31PhyXcfgHsCoarseTuneNum {
        &self.m31_phy_xcfg_hs_coarse_tune_num
    }
    #[doc = "0x478 - M31_PHY_XCFG_HS_FINE_TUNE_NUM"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_hs_fine_tune_num(&self) -> &M31PhyXcfgHsFineTuneNum {
        &self.m31_phy_xcfg_hs_fine_tune_num
    }
    #[doc = "0x47c - M31_PHY_XCFG_FS_COARSE_TUNE_NUM"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_fs_coarse_tune_num(&self) -> &M31PhyXcfgFsCoarseTuneNum {
        &self.m31_phy_xcfg_fs_coarse_tune_num
    }
    #[doc = "0x480 - M31_PHY_XCFG_FS_FINE_TUNE_NUM"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_fs_fine_tune_num(&self) -> &M31PhyXcfgFsFineTuneNum {
        &self.m31_phy_xcfg_fs_fine_tune_num
    }
    #[doc = "0x484 - M31_PHY_XCFG_LOCK_RANGE_MAX"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_lock_range_max(&self) -> &M31PhyXcfgLockRangeMax {
        &self.m31_phy_xcfg_lock_range_max
    }
    #[doc = "0x488 - M31_PHY_XCFGI_LOCK_RANGE_MIN"]
    #[inline(always)]
    pub const fn m31_phy_xcfgi_lock_range_min(&self) -> &M31PhyXcfgiLockRangeMin {
        &self.m31_phy_xcfgi_lock_range_min
    }
    #[doc = "0x48c - M31_PHY_XCFG_OB_RSEL"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_ob_rsel(&self) -> &M31PhyXcfgObRsel {
        &self.m31_phy_xcfg_ob_rsel
    }
    #[doc = "0x490 - M31_PHY_XCFG_OC_RSEL"]
    #[inline(always)]
    pub const fn m31_phy_xcfg_oc_rsel(&self) -> &M31PhyXcfgOcRsel {
        &self.m31_phy_xcfg_oc_rsel
    }
    #[doc = "0x494 - M31_PHY_XCFGO"]
    #[inline(always)]
    pub const fn m31_phy_xcfgo(&self) -> &M31PhyXcfgo {
        &self.m31_phy_xcfgo
    }
    #[doc = "0x498 - USB Added Maxim Interrupt Flag Register."]
    #[inline(always)]
    pub const fn mxm_int(&self) -> &MxmInt {
        &self.mxm_int
    }
    #[doc = "0x49c - USB Added Maxim Interrupt Enable Register."]
    #[inline(always)]
    pub const fn mxm_int_en(&self) -> &MxmIntEn {
        &self.mxm_int_en
    }
    #[doc = "0x4a0 - USB Added Maxim Suspend Register."]
    #[inline(always)]
    pub const fn mxm_suspend(&self) -> &MxmSuspend {
        &self.mxm_suspend
    }
    #[doc = "0x4a4 - USB Added Maxim Power Status Register"]
    #[inline(always)]
    pub const fn mxm_reg_a4(&self) -> &MxmRegA4 {
        &self.mxm_reg_a4
    }
}
#[doc = "FADDR (rw) register accessor: Function address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`] module"]
#[doc(alias = "FADDR")]
pub type Faddr = crate::Reg<faddr::FaddrSpec>;
#[doc = "Function address register."]
pub mod faddr;
#[doc = "POWER (rw) register accessor: Power management register.\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`] module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Power management register."]
pub mod power;
#[doc = "INTRIN (rw) register accessor: Interrupt register for EP0 and IN EP1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrin`] module"]
#[doc(alias = "INTRIN")]
pub type Intrin = crate::Reg<intrin::IntrinSpec>;
#[doc = "Interrupt register for EP0 and IN EP1-15."]
pub mod intrin;
#[doc = "INTROUT (rw) register accessor: Interrupt register for OUT EP 1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`introut::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`introut::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@introut`] module"]
#[doc(alias = "INTROUT")]
pub type Introut = crate::Reg<introut::IntroutSpec>;
#[doc = "Interrupt register for OUT EP 1-15."]
pub mod introut;
#[doc = "INTRINEN (rw) register accessor: Interrupt enable for EP 0 and IN EP 1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrinen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrinen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrinen`] module"]
#[doc(alias = "INTRINEN")]
pub type Intrinen = crate::Reg<intrinen::IntrinenSpec>;
#[doc = "Interrupt enable for EP 0 and IN EP 1-15."]
pub mod intrinen;
#[doc = "INTROUTEN (rw) register accessor: Interrupt enable for OUT EP 1-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`introuten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`introuten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@introuten`] module"]
#[doc(alias = "INTROUTEN")]
pub type Introuten = crate::Reg<introuten::IntroutenSpec>;
#[doc = "Interrupt enable for OUT EP 1-15."]
pub mod introuten;
#[doc = "INTRUSB (rw) register accessor: Interrupt register for common USB interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrusb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrusb`] module"]
#[doc(alias = "INTRUSB")]
pub type Intrusb = crate::Reg<intrusb::IntrusbSpec>;
#[doc = "Interrupt register for common USB interrupts."]
pub mod intrusb;
#[doc = "INTRUSBEN (rw) register accessor: Interrupt enable for common USB interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrusben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrusben`] module"]
#[doc(alias = "INTRUSBEN")]
pub type Intrusben = crate::Reg<intrusben::IntrusbenSpec>;
#[doc = "Interrupt enable for common USB interrupts."]
pub mod intrusben;
#[doc = "FRAME (rw) register accessor: Frame number.\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`] module"]
#[doc(alias = "FRAME")]
pub type Frame = crate::Reg<frame::FrameSpec>;
#[doc = "Frame number."]
pub mod frame;
#[doc = "INDEX (rw) register accessor: Index for banked registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`index::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`index::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@index`] module"]
#[doc(alias = "INDEX")]
pub type Index = crate::Reg<index::IndexSpec>;
#[doc = "Index for banked registers."]
pub mod index;
#[doc = "TESTMODE (rw) register accessor: USB 2.0 test mode enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`testmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testmode`] module"]
#[doc(alias = "TESTMODE")]
pub type Testmode = crate::Reg<testmode::TestmodeSpec>;
#[doc = "USB 2.0 test mode enable register."]
pub mod testmode;
#[doc = "INMAXP (rw) register accessor: Maximum packet size for INx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`inmaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inmaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inmaxp`] module"]
#[doc(alias = "INMAXP")]
pub type Inmaxp = crate::Reg<inmaxp::InmaxpSpec>;
#[doc = "Maximum packet size for INx endpoint (x == INDEX)."]
pub mod inmaxp;
#[doc = "CSR0 (rw) register accessor: Control status register for EP 0 (when INDEX == 0).\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`] module"]
#[doc(alias = "CSR0")]
pub type Csr0 = crate::Reg<csr0::Csr0Spec>;
#[doc = "Control status register for EP 0 (when INDEX == 0)."]
pub mod csr0;
#[doc = "INCSRL (rw) register accessor: Control status lower register for INx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`incsrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`incsrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@incsrl`] module"]
#[doc(alias = "INCSRL")]
pub type Incsrl = crate::Reg<incsrl::IncsrlSpec>;
#[doc = "Control status lower register for INx endpoint (x == INDEX)."]
pub mod incsrl;
#[doc = "INCSRU (rw) register accessor: Control status upper register for INx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`incsru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`incsru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@incsru`] module"]
#[doc(alias = "INCSRU")]
pub type Incsru = crate::Reg<incsru::IncsruSpec>;
#[doc = "Control status upper register for INx endpoint (x == INDEX)."]
pub mod incsru;
#[doc = "OUTMAXP (rw) register accessor: Maximum packet size for OUTx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outmaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outmaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outmaxp`] module"]
#[doc(alias = "OUTMAXP")]
pub type Outmaxp = crate::Reg<outmaxp::OutmaxpSpec>;
#[doc = "Maximum packet size for OUTx endpoint (x == INDEX)."]
pub mod outmaxp;
#[doc = "OUTCSRL (rw) register accessor: Control status lower register for OUTx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outcsrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcsrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcsrl`] module"]
#[doc(alias = "OUTCSRL")]
pub type Outcsrl = crate::Reg<outcsrl::OutcsrlSpec>;
#[doc = "Control status lower register for OUTx endpoint (x == INDEX)."]
pub mod outcsrl;
#[doc = "OUTCSRU (rw) register accessor: Control status upper register for OUTx endpoint (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outcsru::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcsru::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcsru`] module"]
#[doc(alias = "OUTCSRU")]
pub type Outcsru = crate::Reg<outcsru::OutcsruSpec>;
#[doc = "Control status upper register for OUTx endpoint (x == INDEX)."]
pub mod outcsru;
#[doc = "COUNT0 (rw) register accessor: Number of received bytes in EP 0 FIFO (INDEX == 0).\n\nYou can [`read`](crate::Reg::read) this register and get [`count0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count0`] module"]
#[doc(alias = "COUNT0")]
pub type Count0 = crate::Reg<count0::Count0Spec>;
#[doc = "Number of received bytes in EP 0 FIFO (INDEX == 0)."]
pub mod count0;
#[doc = "OUTCOUNT (rw) register accessor: Number of received bytes in OUT EPx FIFO (x == INDEX).\n\nYou can [`read`](crate::Reg::read) this register and get [`outcount::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outcount::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outcount`] module"]
#[doc(alias = "OUTCOUNT")]
pub type Outcount = crate::Reg<outcount::OutcountSpec>;
#[doc = "Number of received bytes in OUT EPx FIFO (x == INDEX)."]
pub mod outcount;
#[doc = "FIFO0 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo0`] module"]
#[doc(alias = "FIFO0")]
pub type Fifo0 = crate::Reg<fifo0::Fifo0Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo0;
#[doc = "FIFO1 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo1`] module"]
#[doc(alias = "FIFO1")]
pub type Fifo1 = crate::Reg<fifo1::Fifo1Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo1;
#[doc = "FIFO2 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo2`] module"]
#[doc(alias = "FIFO2")]
pub type Fifo2 = crate::Reg<fifo2::Fifo2Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo2;
#[doc = "FIFO3 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo3`] module"]
#[doc(alias = "FIFO3")]
pub type Fifo3 = crate::Reg<fifo3::Fifo3Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo3;
#[doc = "FIFO4 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo4`] module"]
#[doc(alias = "FIFO4")]
pub type Fifo4 = crate::Reg<fifo4::Fifo4Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo4;
#[doc = "FIFO5 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo5`] module"]
#[doc(alias = "FIFO5")]
pub type Fifo5 = crate::Reg<fifo5::Fifo5Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo5;
#[doc = "FIFO6 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo6`] module"]
#[doc(alias = "FIFO6")]
pub type Fifo6 = crate::Reg<fifo6::Fifo6Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo6;
#[doc = "FIFO7 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo7`] module"]
#[doc(alias = "FIFO7")]
pub type Fifo7 = crate::Reg<fifo7::Fifo7Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo7;
#[doc = "FIFO8 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo8`] module"]
#[doc(alias = "FIFO8")]
pub type Fifo8 = crate::Reg<fifo8::Fifo8Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo8;
#[doc = "FIFO9 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo9`] module"]
#[doc(alias = "FIFO9")]
pub type Fifo9 = crate::Reg<fifo9::Fifo9Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo9;
#[doc = "FIFO10 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo10`] module"]
#[doc(alias = "FIFO10")]
pub type Fifo10 = crate::Reg<fifo10::Fifo10Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo10;
#[doc = "FIFO11 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo11`] module"]
#[doc(alias = "FIFO11")]
pub type Fifo11 = crate::Reg<fifo11::Fifo11Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo11;
#[doc = "FIFO12 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo12`] module"]
#[doc(alias = "FIFO12")]
pub type Fifo12 = crate::Reg<fifo12::Fifo12Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo12;
#[doc = "FIFO13 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo13`] module"]
#[doc(alias = "FIFO13")]
pub type Fifo13 = crate::Reg<fifo13::Fifo13Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo13;
#[doc = "FIFO14 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo14`] module"]
#[doc(alias = "FIFO14")]
pub type Fifo14 = crate::Reg<fifo14::Fifo14Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo14;
#[doc = "FIFO15 (rw) register accessor: Read for OUT data FIFO, write for IN data FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo15`] module"]
#[doc(alias = "FIFO15")]
pub type Fifo15 = crate::Reg<fifo15::Fifo15Spec>;
#[doc = "Read for OUT data FIFO, write for IN data FIFO."]
pub mod fifo15;
#[doc = "HWVERS (rw) register accessor: HWVERS\n\nYou can [`read`](crate::Reg::read) this register and get [`hwvers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwvers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwvers`] module"]
#[doc(alias = "HWVERS")]
pub type Hwvers = crate::Reg<hwvers::HwversSpec>;
#[doc = "HWVERS"]
pub mod hwvers;
#[doc = "EPINFO (rw) register accessor: Endpoint hardware information.\n\nYou can [`read`](crate::Reg::read) this register and get [`epinfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epinfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epinfo`] module"]
#[doc(alias = "EPINFO")]
pub type Epinfo = crate::Reg<epinfo::EpinfoSpec>;
#[doc = "Endpoint hardware information."]
pub mod epinfo;
#[doc = "RAMINFO (rw) register accessor: RAM width information.\n\nYou can [`read`](crate::Reg::read) this register and get [`raminfo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raminfo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raminfo`] module"]
#[doc(alias = "RAMINFO")]
pub type Raminfo = crate::Reg<raminfo::RaminfoSpec>;
#[doc = "RAM width information."]
pub mod raminfo;
#[doc = "SOFTRESET (rw) register accessor: Software reset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`softreset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softreset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softreset`] module"]
#[doc(alias = "SOFTRESET")]
pub type Softreset = crate::Reg<softreset::SoftresetSpec>;
#[doc = "Software reset register."]
pub mod softreset;
#[doc = "CTUCH (rw) register accessor: Chirp timeout timer setting.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctuch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctuch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctuch`] module"]
#[doc(alias = "CTUCH")]
pub type Ctuch = crate::Reg<ctuch::CtuchSpec>;
#[doc = "Chirp timeout timer setting."]
pub mod ctuch;
#[doc = "CTHSRTN (rw) register accessor: Sets delay between HS resume to UTM normal operating mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`cthsrtn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cthsrtn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cthsrtn`] module"]
#[doc(alias = "CTHSRTN")]
pub type Cthsrtn = crate::Reg<cthsrtn::CthsrtnSpec>;
#[doc = "Sets delay between HS resume to UTM normal operating mode."]
pub mod cthsrtn;
#[doc = "MXM_USB_REG_00 (rw) register accessor: MXM_USB_REG_00\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_usb_reg_00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_usb_reg_00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mxm_usb_reg_00`] module"]
#[doc(alias = "MXM_USB_REG_00")]
pub type MxmUsbReg00 = crate::Reg<mxm_usb_reg_00::MxmUsbReg00Spec>;
#[doc = "MXM_USB_REG_00"]
pub mod mxm_usb_reg_00;
#[doc = "M31_PHY_UTMI_RESET (rw) register accessor: M31_PHY_UTMI_RESET\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_utmi_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_utmi_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_utmi_reset`] module"]
#[doc(alias = "M31_PHY_UTMI_RESET")]
pub type M31PhyUtmiReset = crate::Reg<m31_phy_utmi_reset::M31PhyUtmiResetSpec>;
#[doc = "M31_PHY_UTMI_RESET"]
pub mod m31_phy_utmi_reset;
#[doc = "M31_PHY_UTMI_VCONTROL (rw) register accessor: M31_PHY_UTMI_VCONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_utmi_vcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_utmi_vcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_utmi_vcontrol`] module"]
#[doc(alias = "M31_PHY_UTMI_VCONTROL")]
pub type M31PhyUtmiVcontrol = crate::Reg<m31_phy_utmi_vcontrol::M31PhyUtmiVcontrolSpec>;
#[doc = "M31_PHY_UTMI_VCONTROL"]
pub mod m31_phy_utmi_vcontrol;
#[doc = "M31_PHY_CLK_EN (rw) register accessor: M31_PHY_CLK_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_clk_en`] module"]
#[doc(alias = "M31_PHY_CLK_EN")]
pub type M31PhyClkEn = crate::Reg<m31_phy_clk_en::M31PhyClkEnSpec>;
#[doc = "M31_PHY_CLK_EN"]
pub mod m31_phy_clk_en;
#[doc = "M31_PHY_PONRST (rw) register accessor: M31_PHY_PONRST\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_ponrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_ponrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_ponrst`] module"]
#[doc(alias = "M31_PHY_PONRST")]
pub type M31PhyPonrst = crate::Reg<m31_phy_ponrst::M31PhyPonrstSpec>;
#[doc = "M31_PHY_PONRST"]
pub mod m31_phy_ponrst;
#[doc = "M31_PHY_NONCRY_RSTB (rw) register accessor: M31_PHY_NONCRY_RSTB\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_noncry_rstb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_noncry_rstb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_noncry_rstb`] module"]
#[doc(alias = "M31_PHY_NONCRY_RSTB")]
pub type M31PhyNoncryRstb = crate::Reg<m31_phy_noncry_rstb::M31PhyNoncryRstbSpec>;
#[doc = "M31_PHY_NONCRY_RSTB"]
pub mod m31_phy_noncry_rstb;
#[doc = "M31_PHY_NONCRY_EN (rw) register accessor: M31_PHY_NONCRY_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_noncry_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_noncry_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_noncry_en`] module"]
#[doc(alias = "M31_PHY_NONCRY_EN")]
pub type M31PhyNoncryEn = crate::Reg<m31_phy_noncry_en::M31PhyNoncryEnSpec>;
#[doc = "M31_PHY_NONCRY_EN"]
pub mod m31_phy_noncry_en;
#[doc = "M31_PHY_U2_COMPLIANCE_EN (rw) register accessor: M31_PHY_U2_COMPLIANCE_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_u2_compliance_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_u2_compliance_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_u2_compliance_en`] module"]
#[doc(alias = "M31_PHY_U2_COMPLIANCE_EN")]
pub type M31PhyU2ComplianceEn = crate::Reg<m31_phy_u2_compliance_en::M31PhyU2ComplianceEnSpec>;
#[doc = "M31_PHY_U2_COMPLIANCE_EN"]
pub mod m31_phy_u2_compliance_en;
#[doc = "M31_PHY_U2_COMPLIANCE_DAC_ADJ (rw) register accessor: M31_PHY_U2_COMPLIANCE_DAC_ADJ\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_u2_compliance_dac_adj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_u2_compliance_dac_adj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_u2_compliance_dac_adj`] module"]
#[doc(alias = "M31_PHY_U2_COMPLIANCE_DAC_ADJ")]
pub type M31PhyU2ComplianceDacAdj =
    crate::Reg<m31_phy_u2_compliance_dac_adj::M31PhyU2ComplianceDacAdjSpec>;
#[doc = "M31_PHY_U2_COMPLIANCE_DAC_ADJ"]
pub mod m31_phy_u2_compliance_dac_adj;
#[doc = "M31_PHY_U2_COMPLIANCE_DAC_ADJ_EN (rw) register accessor: M31_PHY_U2_COMPLIANCE_DAC_ADJ_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_u2_compliance_dac_adj_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_u2_compliance_dac_adj_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_u2_compliance_dac_adj_en`] module"]
#[doc(alias = "M31_PHY_U2_COMPLIANCE_DAC_ADJ_EN")]
pub type M31PhyU2ComplianceDacAdjEn =
    crate::Reg<m31_phy_u2_compliance_dac_adj_en::M31PhyU2ComplianceDacAdjEnSpec>;
#[doc = "M31_PHY_U2_COMPLIANCE_DAC_ADJ_EN"]
pub mod m31_phy_u2_compliance_dac_adj_en;
#[doc = "M31_PHY_CLK_RDY (rw) register accessor: M31_PHY_CLK_RDY\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_clk_rdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_clk_rdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_clk_rdy`] module"]
#[doc(alias = "M31_PHY_CLK_RDY")]
pub type M31PhyClkRdy = crate::Reg<m31_phy_clk_rdy::M31PhyClkRdySpec>;
#[doc = "M31_PHY_CLK_RDY"]
pub mod m31_phy_clk_rdy;
#[doc = "M31_PHY_PLL_EN (rw) register accessor: M31_PHY_PLL_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_pll_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_pll_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_pll_en`] module"]
#[doc(alias = "M31_PHY_PLL_EN")]
pub type M31PhyPllEn = crate::Reg<m31_phy_pll_en::M31PhyPllEnSpec>;
#[doc = "M31_PHY_PLL_EN"]
pub mod m31_phy_pll_en;
#[doc = "M31_PHY_BIST_OK (rw) register accessor: M31_PHY_BIST_OK\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_bist_ok::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_bist_ok::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_bist_ok`] module"]
#[doc(alias = "M31_PHY_BIST_OK")]
pub type M31PhyBistOk = crate::Reg<m31_phy_bist_ok::M31PhyBistOkSpec>;
#[doc = "M31_PHY_BIST_OK"]
pub mod m31_phy_bist_ok;
#[doc = "M31_PHY_DATA_OE (rw) register accessor: M31_PHY_DATA_OE\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_data_oe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_data_oe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_data_oe`] module"]
#[doc(alias = "M31_PHY_DATA_OE")]
pub type M31PhyDataOe = crate::Reg<m31_phy_data_oe::M31PhyDataOeSpec>;
#[doc = "M31_PHY_DATA_OE"]
pub mod m31_phy_data_oe;
#[doc = "M31_PHY_OSCOUTEN (rw) register accessor: M31_PHY_OSCOUTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_oscouten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_oscouten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_oscouten`] module"]
#[doc(alias = "M31_PHY_OSCOUTEN")]
pub type M31PhyOscouten = crate::Reg<m31_phy_oscouten::M31PhyOscoutenSpec>;
#[doc = "M31_PHY_OSCOUTEN"]
pub mod m31_phy_oscouten;
#[doc = "M31_PHY_LPM_ALIVE (rw) register accessor: M31_PHY_LPM_ALIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_lpm_alive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_lpm_alive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_lpm_alive`] module"]
#[doc(alias = "M31_PHY_LPM_ALIVE")]
pub type M31PhyLpmAlive = crate::Reg<m31_phy_lpm_alive::M31PhyLpmAliveSpec>;
#[doc = "M31_PHY_LPM_ALIVE"]
pub mod m31_phy_lpm_alive;
#[doc = "M31_PHY_HS_BIST_MODE (rw) register accessor: M31_PHY_HS_BIST_MODE\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_hs_bist_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_hs_bist_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_hs_bist_mode`] module"]
#[doc(alias = "M31_PHY_HS_BIST_MODE")]
pub type M31PhyHsBistMode = crate::Reg<m31_phy_hs_bist_mode::M31PhyHsBistModeSpec>;
#[doc = "M31_PHY_HS_BIST_MODE"]
pub mod m31_phy_hs_bist_mode;
#[doc = "M31_PHY_CORECLKIN (rw) register accessor: M31_PHY_CORECLKIN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_coreclkin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_coreclkin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_coreclkin`] module"]
#[doc(alias = "M31_PHY_CORECLKIN")]
pub type M31PhyCoreclkin = crate::Reg<m31_phy_coreclkin::M31PhyCoreclkinSpec>;
#[doc = "M31_PHY_CORECLKIN"]
pub mod m31_phy_coreclkin;
#[doc = "M31_PHY_XTLSEL (rw) register accessor: M31_PHY_XTLSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xtlsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xtlsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xtlsel`] module"]
#[doc(alias = "M31_PHY_XTLSEL")]
pub type M31PhyXtlsel = crate::Reg<m31_phy_xtlsel::M31PhyXtlselSpec>;
#[doc = "M31_PHY_XTLSEL"]
pub mod m31_phy_xtlsel;
#[doc = "M31_PHY_LS_EN (rw) register accessor: M31_PHY_LS_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_ls_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_ls_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_ls_en`] module"]
#[doc(alias = "M31_PHY_LS_EN")]
pub type M31PhyLsEn = crate::Reg<m31_phy_ls_en::M31PhyLsEnSpec>;
#[doc = "M31_PHY_LS_EN"]
pub mod m31_phy_ls_en;
#[doc = "M31_PHY_DEBUG_SEL (rw) register accessor: M31_PHY_DEBUG_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_debug_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_debug_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_debug_sel`] module"]
#[doc(alias = "M31_PHY_DEBUG_SEL")]
pub type M31PhyDebugSel = crate::Reg<m31_phy_debug_sel::M31PhyDebugSelSpec>;
#[doc = "M31_PHY_DEBUG_SEL"]
pub mod m31_phy_debug_sel;
#[doc = "M31_PHY_DEBUG_OUT (rw) register accessor: M31_PHY_DEBUG_OUT\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_debug_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_debug_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_debug_out`] module"]
#[doc(alias = "M31_PHY_DEBUG_OUT")]
pub type M31PhyDebugOut = crate::Reg<m31_phy_debug_out::M31PhyDebugOutSpec>;
#[doc = "M31_PHY_DEBUG_OUT"]
pub mod m31_phy_debug_out;
#[doc = "M31_PHY_OUTCLKSEL (rw) register accessor: M31_PHY_OUTCLKSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_outclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_outclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_outclksel`] module"]
#[doc(alias = "M31_PHY_OUTCLKSEL")]
pub type M31PhyOutclksel = crate::Reg<m31_phy_outclksel::M31PhyOutclkselSpec>;
#[doc = "M31_PHY_OUTCLKSEL"]
pub mod m31_phy_outclksel;
#[doc = "M31_PHY_XCFGI_31_0 (rw) register accessor: M31_PHY_XCFGI_31_0\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgi_31_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgi_31_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgi_31_0`] module"]
#[doc(alias = "M31_PHY_XCFGI_31_0")]
pub type M31PhyXcfgi31_0 = crate::Reg<m31_phy_xcfgi_31_0::M31PhyXcfgi31_0Spec>;
#[doc = "M31_PHY_XCFGI_31_0"]
pub mod m31_phy_xcfgi_31_0;
#[doc = "M31_PHY_XCFGI_63_32 (rw) register accessor: M31_PHY_XCFGI_63_32\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgi_63_32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgi_63_32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgi_63_32`] module"]
#[doc(alias = "M31_PHY_XCFGI_63_32")]
pub type M31PhyXcfgi63_32 = crate::Reg<m31_phy_xcfgi_63_32::M31PhyXcfgi63_32Spec>;
#[doc = "M31_PHY_XCFGI_63_32"]
pub mod m31_phy_xcfgi_63_32;
#[doc = "M31_PHY_XCFGI_95_64 (rw) register accessor: M31_PHY_XCFGI_95_64\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgi_95_64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgi_95_64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgi_95_64`] module"]
#[doc(alias = "M31_PHY_XCFGI_95_64")]
pub type M31PhyXcfgi95_64 = crate::Reg<m31_phy_xcfgi_95_64::M31PhyXcfgi95_64Spec>;
#[doc = "M31_PHY_XCFGI_95_64"]
pub mod m31_phy_xcfgi_95_64;
#[doc = "M31_PHY_XCFGI_127_96 (rw) register accessor: M31_PHY_XCFGI_127_96\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgi_127_96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgi_127_96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgi_127_96`] module"]
#[doc(alias = "M31_PHY_XCFGI_127_96")]
pub type M31PhyXcfgi127_96 = crate::Reg<m31_phy_xcfgi_127_96::M31PhyXcfgi127_96Spec>;
#[doc = "M31_PHY_XCFGI_127_96"]
pub mod m31_phy_xcfgi_127_96;
#[doc = "M31_PHY_XCFGI_137_128 (rw) register accessor: M31_PHY_XCFGI_137_128\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgi_137_128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgi_137_128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgi_137_128`] module"]
#[doc(alias = "M31_PHY_XCFGI_137_128")]
pub type M31PhyXcfgi137_128 = crate::Reg<m31_phy_xcfgi_137_128::M31PhyXcfgi137_128Spec>;
#[doc = "M31_PHY_XCFGI_137_128"]
pub mod m31_phy_xcfgi_137_128;
#[doc = "M31_PHY_XCFG_HS_COARSE_TUNE_NUM (rw) register accessor: M31_PHY_XCFG_HS_COARSE_TUNE_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_hs_coarse_tune_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_hs_coarse_tune_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_hs_coarse_tune_num`] module"]
#[doc(alias = "M31_PHY_XCFG_HS_COARSE_TUNE_NUM")]
pub type M31PhyXcfgHsCoarseTuneNum =
    crate::Reg<m31_phy_xcfg_hs_coarse_tune_num::M31PhyXcfgHsCoarseTuneNumSpec>;
#[doc = "M31_PHY_XCFG_HS_COARSE_TUNE_NUM"]
pub mod m31_phy_xcfg_hs_coarse_tune_num;
#[doc = "M31_PHY_XCFG_HS_FINE_TUNE_NUM (rw) register accessor: M31_PHY_XCFG_HS_FINE_TUNE_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_hs_fine_tune_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_hs_fine_tune_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_hs_fine_tune_num`] module"]
#[doc(alias = "M31_PHY_XCFG_HS_FINE_TUNE_NUM")]
pub type M31PhyXcfgHsFineTuneNum =
    crate::Reg<m31_phy_xcfg_hs_fine_tune_num::M31PhyXcfgHsFineTuneNumSpec>;
#[doc = "M31_PHY_XCFG_HS_FINE_TUNE_NUM"]
pub mod m31_phy_xcfg_hs_fine_tune_num;
#[doc = "M31_PHY_XCFG_FS_COARSE_TUNE_NUM (rw) register accessor: M31_PHY_XCFG_FS_COARSE_TUNE_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_fs_coarse_tune_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_fs_coarse_tune_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_fs_coarse_tune_num`] module"]
#[doc(alias = "M31_PHY_XCFG_FS_COARSE_TUNE_NUM")]
pub type M31PhyXcfgFsCoarseTuneNum =
    crate::Reg<m31_phy_xcfg_fs_coarse_tune_num::M31PhyXcfgFsCoarseTuneNumSpec>;
#[doc = "M31_PHY_XCFG_FS_COARSE_TUNE_NUM"]
pub mod m31_phy_xcfg_fs_coarse_tune_num;
#[doc = "M31_PHY_XCFG_FS_FINE_TUNE_NUM (rw) register accessor: M31_PHY_XCFG_FS_FINE_TUNE_NUM\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_fs_fine_tune_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_fs_fine_tune_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_fs_fine_tune_num`] module"]
#[doc(alias = "M31_PHY_XCFG_FS_FINE_TUNE_NUM")]
pub type M31PhyXcfgFsFineTuneNum =
    crate::Reg<m31_phy_xcfg_fs_fine_tune_num::M31PhyXcfgFsFineTuneNumSpec>;
#[doc = "M31_PHY_XCFG_FS_FINE_TUNE_NUM"]
pub mod m31_phy_xcfg_fs_fine_tune_num;
#[doc = "M31_PHY_XCFG_LOCK_RANGE_MAX (rw) register accessor: M31_PHY_XCFG_LOCK_RANGE_MAX\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_lock_range_max::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_lock_range_max::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_lock_range_max`] module"]
#[doc(alias = "M31_PHY_XCFG_LOCK_RANGE_MAX")]
pub type M31PhyXcfgLockRangeMax =
    crate::Reg<m31_phy_xcfg_lock_range_max::M31PhyXcfgLockRangeMaxSpec>;
#[doc = "M31_PHY_XCFG_LOCK_RANGE_MAX"]
pub mod m31_phy_xcfg_lock_range_max;
#[doc = "M31_PHY_XCFGI_LOCK_RANGE_MIN (rw) register accessor: M31_PHY_XCFGI_LOCK_RANGE_MIN\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgi_lock_range_min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgi_lock_range_min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgi_lock_range_min`] module"]
#[doc(alias = "M31_PHY_XCFGI_LOCK_RANGE_MIN")]
pub type M31PhyXcfgiLockRangeMin =
    crate::Reg<m31_phy_xcfgi_lock_range_min::M31PhyXcfgiLockRangeMinSpec>;
#[doc = "M31_PHY_XCFGI_LOCK_RANGE_MIN"]
pub mod m31_phy_xcfgi_lock_range_min;
#[doc = "M31_PHY_XCFG_OB_RSEL (rw) register accessor: M31_PHY_XCFG_OB_RSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_ob_rsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_ob_rsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_ob_rsel`] module"]
#[doc(alias = "M31_PHY_XCFG_OB_RSEL")]
pub type M31PhyXcfgObRsel = crate::Reg<m31_phy_xcfg_ob_rsel::M31PhyXcfgObRselSpec>;
#[doc = "M31_PHY_XCFG_OB_RSEL"]
pub mod m31_phy_xcfg_ob_rsel;
#[doc = "M31_PHY_XCFG_OC_RSEL (rw) register accessor: M31_PHY_XCFG_OC_RSEL\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfg_oc_rsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfg_oc_rsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfg_oc_rsel`] module"]
#[doc(alias = "M31_PHY_XCFG_OC_RSEL")]
pub type M31PhyXcfgOcRsel = crate::Reg<m31_phy_xcfg_oc_rsel::M31PhyXcfgOcRselSpec>;
#[doc = "M31_PHY_XCFG_OC_RSEL"]
pub mod m31_phy_xcfg_oc_rsel;
#[doc = "M31_PHY_XCFGO (rw) register accessor: M31_PHY_XCFGO\n\nYou can [`read`](crate::Reg::read) this register and get [`m31_phy_xcfgo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m31_phy_xcfgo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m31_phy_xcfgo`] module"]
#[doc(alias = "M31_PHY_XCFGO")]
pub type M31PhyXcfgo = crate::Reg<m31_phy_xcfgo::M31PhyXcfgoSpec>;
#[doc = "M31_PHY_XCFGO"]
pub mod m31_phy_xcfgo;
#[doc = "MXM_INT (rw) register accessor: USB Added Maxim Interrupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mxm_int`] module"]
#[doc(alias = "MXM_INT")]
pub type MxmInt = crate::Reg<mxm_int::MxmIntSpec>;
#[doc = "USB Added Maxim Interrupt Flag Register."]
pub mod mxm_int;
#[doc = "MXM_INT_EN (rw) register accessor: USB Added Maxim Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mxm_int_en`] module"]
#[doc(alias = "MXM_INT_EN")]
pub type MxmIntEn = crate::Reg<mxm_int_en::MxmIntEnSpec>;
#[doc = "USB Added Maxim Interrupt Enable Register."]
pub mod mxm_int_en;
#[doc = "MXM_SUSPEND (rw) register accessor: USB Added Maxim Suspend Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_suspend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_suspend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mxm_suspend`] module"]
#[doc(alias = "MXM_SUSPEND")]
pub type MxmSuspend = crate::Reg<mxm_suspend::MxmSuspendSpec>;
#[doc = "USB Added Maxim Suspend Register."]
pub mod mxm_suspend;
#[doc = "MXM_REG_A4 (rw) register accessor: USB Added Maxim Power Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mxm_reg_a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxm_reg_a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mxm_reg_a4`] module"]
#[doc(alias = "MXM_REG_A4")]
pub type MxmRegA4 = crate::Reg<mxm_reg_a4::MxmRegA4Spec>;
#[doc = "USB Added Maxim Power Status Register"]
pub mod mxm_reg_a4;
