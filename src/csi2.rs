#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_num_lanes: CfgNumLanes,
    cfg_clk_lane_en: CfgClkLaneEn,
    cfg_data_lane_en: CfgDataLaneEn,
    cfg_flush_count: CfgFlushCount,
    cfg_bit_err: CfgBitErr,
    irq_status: IrqStatus,
    irq_enable: IrqEnable,
    irq_clr: IrqClr,
    ulps_clk_status: UlpsClkStatus,
    ulps_status: UlpsStatus,
    ulps_clk_mark_status: UlpsClkMarkStatus,
    ulps_mark_status: UlpsMarkStatus,
    ppi_errsot_hs: PpiErrsotHs,
    ppi_errsotsync_hs: PpiErrsotsyncHs,
    ppi_erresc: PpiErresc,
    ppi_errsyncesc: PpiErrsyncesc,
    ppi_errcontrol: PpiErrcontrol,
    cfg_cphy_en: CfgCphyEn,
    cfg_ppi_16_en: CfgPpi16En,
    cfg_packet_interface_en: CfgPacketInterfaceEn,
    cfg_vcx_en: CfgVcxEn,
    cfg_byte_data_format: CfgByteDataFormat,
    cfg_disable_payload_0: CfgDisablePayload0,
    cfg_disable_payload_1: CfgDisablePayload1,
    _reserved24: [u8; 0x20],
    cfg_vid_ignore_vc: CfgVidIgnoreVc,
    cfg_vid_vc: CfgVidVc,
    cfg_p_fifo_send_level: CfgPFifoSendLevel,
    cfg_vid_vsync: CfgVidVsync,
    cfg_vid_hsync_fp: CfgVidHsyncFp,
    cfg_vid_hsync: CfgVidHsync,
    cfg_vid_hsync_bp: CfgVidHsyncBp,
    _reserved31: [u8; 0x0364],
    cfg_databus16_sel: CfgDatabus16Sel,
    cfg_d0_swap_sel: CfgD0SwapSel,
    cfg_d1_swap_sel: CfgD1SwapSel,
    cfg_d2_swap_sel: CfgD2SwapSel,
    cfg_d3_swap_sel: CfgD3SwapSel,
    cfg_c0_swap_sel: CfgC0SwapSel,
    cfg_dpdn_swap: CfgDpdnSwap,
    rg_cfgclk_1us_cnt: RgCfgclk1usCnt,
    rg_hsrx_clk_pre_time_grp0: RgHsrxClkPreTimeGrp0,
    rg_hsrx_data_pre_time_grp0: RgHsrxDataPreTimeGrp0,
    reset_deskew: ResetDeskew,
    pma_rdy: PmaRdy,
    xcfgi_dw00: XcfgiDw00,
    xcfgi_dw01: XcfgiDw01,
    xcfgi_dw02: XcfgiDw02,
    xcfgi_dw03: XcfgiDw03,
    xcfgi_dw04: XcfgiDw04,
    xcfgi_dw05: XcfgiDw05,
    xcfgi_dw06: XcfgiDw06,
    xcfgi_dw07: XcfgiDw07,
    xcfgi_dw08: XcfgiDw08,
    xcfgi_dw09: XcfgiDw09,
    xcfgi_dw0a: XcfgiDw0a,
    xcfgi_dw0b: XcfgiDw0b,
    xcfgi_dw0c: XcfgiDw0c,
    xcfgi_dw0d: XcfgiDw0d,
    gpio_mode: GpioMode,
    gpio_dp_ie: GpioDpIe,
    gpio_dn_ie: GpioDnIe,
    gpio_dp_c: GpioDpC,
    gpio_dn_c: GpioDnC,
    vcontrol: Vcontrol,
    mpsov1: Mpsov1,
    mpsov2: Mpsov2,
    mpsov3: Mpsov3,
    _reserved66: [u8; 0x04],
    rg_cdrx_dsirx_en: RgCdrxDsirxEn,
    rg_cdrx_l012_sublvds_en: RgCdrxL012SublvdsEn,
    rg_cdrx_l012_hsrt_ctrl: RgCdrxL012HsrtCtrl,
    rg_cdrx_bisths_pll_en: RgCdrxBisthsPllEn,
    rg_cdrx_bisths_pll_pre_div2: RgCdrxBisthsPllPreDiv2,
    rg_cdrx_bisths_pll_fbk_int: RgCdrxBisthsPllFbkInt,
    dbg1_mux_sel: Dbg1MuxSel,
    dbg2_mux_sel: Dbg2MuxSel,
    dbg1_mux_dout: Dbg1MuxDout,
    dbg2_mux_dout: Dbg2MuxDout,
    aon_power_ready_n: AonPowerReadyN,
    dphy_rst_n: DphyRstN,
    rxbyteclkhs_inv: RxbyteclkhsInv,
    _reserved79: [u8; 0x3c],
    vfifo_cfg0: VfifoCfg0,
    vfifo_cfg1: VfifoCfg1,
    vfifo_ctrl: VfifoCtrl,
    vfifo_sts: VfifoSts,
    vfifo_line_num: VfifoLineNum,
    vfifo_pixel_num: VfifoPixelNum,
    vfifo_line_cnt: VfifoLineCnt,
    vfifo_pixel_cnt: VfifoPixelCnt,
    vfifo_frame_sts: VfifoFrameSts,
    vfifo_raw_ctrl: VfifoRawCtrl,
    vfifo_raw_buf0_addr: VfifoRawBuf0Addr,
    vfifo_raw_buf1_addr: VfifoRawBuf1Addr,
    vfifo_ahbm_ctrl: VfifoAhbmCtrl,
    vfifo_ahbm_sts: VfifoAhbmSts,
    vfifo_ahbm_start_addr: VfifoAhbmStartAddr,
    vfifo_ahbm_addr_range: VfifoAhbmAddrRange,
    vfifo_ahbm_max_trans: VfifoAhbmMaxTrans,
    vfifo_ahbm_trans_cnt: VfifoAhbmTransCnt,
    _reserved97: [u8; 0xb8],
    rx_eint_vff_ie: RxEintVffIe,
    rx_eint_vff_if: RxEintVffIf,
    rx_eint_ppi_ie: RxEintPpiIe,
    rx_eint_ppi_if: RxEintPpiIf,
    rx_eint_ctrl_ie: RxEintCtrlIe,
    rx_eint_ctrl_if: RxEintCtrlIf,
    _reserved103: [u8; 0xe8],
    ppi_stopstate: PpiStopstate,
    ppi_turnaround_cfg: PpiTurnaroundCfg,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG_NUM_LANES."]
    #[inline(always)]
    pub const fn cfg_num_lanes(&self) -> &CfgNumLanes {
        &self.cfg_num_lanes
    }
    #[doc = "0x04 - CFG_CLK_LANE_EN."]
    #[inline(always)]
    pub const fn cfg_clk_lane_en(&self) -> &CfgClkLaneEn {
        &self.cfg_clk_lane_en
    }
    #[doc = "0x08 - CFG_DATA_LANE_EN."]
    #[inline(always)]
    pub const fn cfg_data_lane_en(&self) -> &CfgDataLaneEn {
        &self.cfg_data_lane_en
    }
    #[doc = "0x0c - CFG_FLUSH_COUNT."]
    #[inline(always)]
    pub const fn cfg_flush_count(&self) -> &CfgFlushCount {
        &self.cfg_flush_count
    }
    #[doc = "0x10 - CFG_BIT_ERR."]
    #[inline(always)]
    pub const fn cfg_bit_err(&self) -> &CfgBitErr {
        &self.cfg_bit_err
    }
    #[doc = "0x14 - IRQ_STATUS."]
    #[inline(always)]
    pub const fn irq_status(&self) -> &IrqStatus {
        &self.irq_status
    }
    #[doc = "0x18 - IRQ_ENABLE."]
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IrqEnable {
        &self.irq_enable
    }
    #[doc = "0x1c - IRQ_CLR."]
    #[inline(always)]
    pub const fn irq_clr(&self) -> &IrqClr {
        &self.irq_clr
    }
    #[doc = "0x20 - ULPS_CLK_STATUS."]
    #[inline(always)]
    pub const fn ulps_clk_status(&self) -> &UlpsClkStatus {
        &self.ulps_clk_status
    }
    #[doc = "0x24 - ULPS_STATUS."]
    #[inline(always)]
    pub const fn ulps_status(&self) -> &UlpsStatus {
        &self.ulps_status
    }
    #[doc = "0x28 - ULPS_CLK_MARK_STATUS."]
    #[inline(always)]
    pub const fn ulps_clk_mark_status(&self) -> &UlpsClkMarkStatus {
        &self.ulps_clk_mark_status
    }
    #[doc = "0x2c - ULPS_MARK_STATUS."]
    #[inline(always)]
    pub const fn ulps_mark_status(&self) -> &UlpsMarkStatus {
        &self.ulps_mark_status
    }
    #[doc = "0x30 - PPI_ERRSOT_HS."]
    #[inline(always)]
    pub const fn ppi_errsot_hs(&self) -> &PpiErrsotHs {
        &self.ppi_errsot_hs
    }
    #[doc = "0x34 - PPI_ERRSOTSYNC_HS."]
    #[inline(always)]
    pub const fn ppi_errsotsync_hs(&self) -> &PpiErrsotsyncHs {
        &self.ppi_errsotsync_hs
    }
    #[doc = "0x38 - PPI_ERRESC."]
    #[inline(always)]
    pub const fn ppi_erresc(&self) -> &PpiErresc {
        &self.ppi_erresc
    }
    #[doc = "0x3c - PPI_ERRSYNCESC."]
    #[inline(always)]
    pub const fn ppi_errsyncesc(&self) -> &PpiErrsyncesc {
        &self.ppi_errsyncesc
    }
    #[doc = "0x40 - PPI_ERRCONTROL."]
    #[inline(always)]
    pub const fn ppi_errcontrol(&self) -> &PpiErrcontrol {
        &self.ppi_errcontrol
    }
    #[doc = "0x44 - CFG_CPHY_EN."]
    #[inline(always)]
    pub const fn cfg_cphy_en(&self) -> &CfgCphyEn {
        &self.cfg_cphy_en
    }
    #[doc = "0x48 - CFG_PPI_16_EN."]
    #[inline(always)]
    pub const fn cfg_ppi_16_en(&self) -> &CfgPpi16En {
        &self.cfg_ppi_16_en
    }
    #[doc = "0x4c - CFG_PACKET_INTERFACE_EN."]
    #[inline(always)]
    pub const fn cfg_packet_interface_en(&self) -> &CfgPacketInterfaceEn {
        &self.cfg_packet_interface_en
    }
    #[doc = "0x50 - CFG_VCX_EN."]
    #[inline(always)]
    pub const fn cfg_vcx_en(&self) -> &CfgVcxEn {
        &self.cfg_vcx_en
    }
    #[doc = "0x54 - CFG_BYTE_DATA_FORMAT."]
    #[inline(always)]
    pub const fn cfg_byte_data_format(&self) -> &CfgByteDataFormat {
        &self.cfg_byte_data_format
    }
    #[doc = "0x58 - CFG_DISABLE_PAYLOAD_0."]
    #[inline(always)]
    pub const fn cfg_disable_payload_0(&self) -> &CfgDisablePayload0 {
        &self.cfg_disable_payload_0
    }
    #[doc = "0x5c - CFG_DISABLE_PAYLOAD_1."]
    #[inline(always)]
    pub const fn cfg_disable_payload_1(&self) -> &CfgDisablePayload1 {
        &self.cfg_disable_payload_1
    }
    #[doc = "0x80 - CFG_VID_IGNORE_VC."]
    #[inline(always)]
    pub const fn cfg_vid_ignore_vc(&self) -> &CfgVidIgnoreVc {
        &self.cfg_vid_ignore_vc
    }
    #[doc = "0x84 - CFG_VID_VC."]
    #[inline(always)]
    pub const fn cfg_vid_vc(&self) -> &CfgVidVc {
        &self.cfg_vid_vc
    }
    #[doc = "0x88 - CFG_P_FIFO_SEND_LEVEL."]
    #[inline(always)]
    pub const fn cfg_p_fifo_send_level(&self) -> &CfgPFifoSendLevel {
        &self.cfg_p_fifo_send_level
    }
    #[doc = "0x8c - CFG_VID_VSYNC."]
    #[inline(always)]
    pub const fn cfg_vid_vsync(&self) -> &CfgVidVsync {
        &self.cfg_vid_vsync
    }
    #[doc = "0x90 - CFG_VID_HSYNC_FP."]
    #[inline(always)]
    pub const fn cfg_vid_hsync_fp(&self) -> &CfgVidHsyncFp {
        &self.cfg_vid_hsync_fp
    }
    #[doc = "0x94 - CFG_VID_HSYNC."]
    #[inline(always)]
    pub const fn cfg_vid_hsync(&self) -> &CfgVidHsync {
        &self.cfg_vid_hsync
    }
    #[doc = "0x98 - CFG_VID_HSYNC_BP."]
    #[inline(always)]
    pub const fn cfg_vid_hsync_bp(&self) -> &CfgVidHsyncBp {
        &self.cfg_vid_hsync_bp
    }
    #[doc = "0x400 - CFG_DATABUS16_SEL."]
    #[inline(always)]
    pub const fn cfg_databus16_sel(&self) -> &CfgDatabus16Sel {
        &self.cfg_databus16_sel
    }
    #[doc = "0x404 - CFG_D0_SWAP_SEL."]
    #[inline(always)]
    pub const fn cfg_d0_swap_sel(&self) -> &CfgD0SwapSel {
        &self.cfg_d0_swap_sel
    }
    #[doc = "0x408 - CFG_D1_SWAP_SEL."]
    #[inline(always)]
    pub const fn cfg_d1_swap_sel(&self) -> &CfgD1SwapSel {
        &self.cfg_d1_swap_sel
    }
    #[doc = "0x40c - CFG_D2_SWAP_SEL."]
    #[inline(always)]
    pub const fn cfg_d2_swap_sel(&self) -> &CfgD2SwapSel {
        &self.cfg_d2_swap_sel
    }
    #[doc = "0x410 - CFG_D3_SWAP_SEL."]
    #[inline(always)]
    pub const fn cfg_d3_swap_sel(&self) -> &CfgD3SwapSel {
        &self.cfg_d3_swap_sel
    }
    #[doc = "0x414 - CFG_C0_SWAP_SEL."]
    #[inline(always)]
    pub const fn cfg_c0_swap_sel(&self) -> &CfgC0SwapSel {
        &self.cfg_c0_swap_sel
    }
    #[doc = "0x418 - CFG_DPDN_SWAP."]
    #[inline(always)]
    pub const fn cfg_dpdn_swap(&self) -> &CfgDpdnSwap {
        &self.cfg_dpdn_swap
    }
    #[doc = "0x41c - RG_CFGCLK_1US_CNT."]
    #[inline(always)]
    pub const fn rg_cfgclk_1us_cnt(&self) -> &RgCfgclk1usCnt {
        &self.rg_cfgclk_1us_cnt
    }
    #[doc = "0x420 - RG_HSRX_CLK_PRE_TIME_GRP0."]
    #[inline(always)]
    pub const fn rg_hsrx_clk_pre_time_grp0(&self) -> &RgHsrxClkPreTimeGrp0 {
        &self.rg_hsrx_clk_pre_time_grp0
    }
    #[doc = "0x424 - RG_HSRX_DATA_PRE_TIME_GRP0."]
    #[inline(always)]
    pub const fn rg_hsrx_data_pre_time_grp0(&self) -> &RgHsrxDataPreTimeGrp0 {
        &self.rg_hsrx_data_pre_time_grp0
    }
    #[doc = "0x428 - RESET_DESKEW."]
    #[inline(always)]
    pub const fn reset_deskew(&self) -> &ResetDeskew {
        &self.reset_deskew
    }
    #[doc = "0x42c - PMA_RDY."]
    #[inline(always)]
    pub const fn pma_rdy(&self) -> &PmaRdy {
        &self.pma_rdy
    }
    #[doc = "0x430 - XCFGI_DW00."]
    #[inline(always)]
    pub const fn xcfgi_dw00(&self) -> &XcfgiDw00 {
        &self.xcfgi_dw00
    }
    #[doc = "0x434 - XCFGI_DW01."]
    #[inline(always)]
    pub const fn xcfgi_dw01(&self) -> &XcfgiDw01 {
        &self.xcfgi_dw01
    }
    #[doc = "0x438 - XCFGI_DW02."]
    #[inline(always)]
    pub const fn xcfgi_dw02(&self) -> &XcfgiDw02 {
        &self.xcfgi_dw02
    }
    #[doc = "0x43c - XCFGI_DW03."]
    #[inline(always)]
    pub const fn xcfgi_dw03(&self) -> &XcfgiDw03 {
        &self.xcfgi_dw03
    }
    #[doc = "0x440 - XCFGI_DW04."]
    #[inline(always)]
    pub const fn xcfgi_dw04(&self) -> &XcfgiDw04 {
        &self.xcfgi_dw04
    }
    #[doc = "0x444 - XCFGI_DW05."]
    #[inline(always)]
    pub const fn xcfgi_dw05(&self) -> &XcfgiDw05 {
        &self.xcfgi_dw05
    }
    #[doc = "0x448 - XCFGI_DW06."]
    #[inline(always)]
    pub const fn xcfgi_dw06(&self) -> &XcfgiDw06 {
        &self.xcfgi_dw06
    }
    #[doc = "0x44c - XCFGI_DW07."]
    #[inline(always)]
    pub const fn xcfgi_dw07(&self) -> &XcfgiDw07 {
        &self.xcfgi_dw07
    }
    #[doc = "0x450 - XCFGI_DW08."]
    #[inline(always)]
    pub const fn xcfgi_dw08(&self) -> &XcfgiDw08 {
        &self.xcfgi_dw08
    }
    #[doc = "0x454 - XCFGI_DW09."]
    #[inline(always)]
    pub const fn xcfgi_dw09(&self) -> &XcfgiDw09 {
        &self.xcfgi_dw09
    }
    #[doc = "0x458 - XCFGI_DW0A."]
    #[inline(always)]
    pub const fn xcfgi_dw0a(&self) -> &XcfgiDw0a {
        &self.xcfgi_dw0a
    }
    #[doc = "0x45c - XCFGI_DW0B."]
    #[inline(always)]
    pub const fn xcfgi_dw0b(&self) -> &XcfgiDw0b {
        &self.xcfgi_dw0b
    }
    #[doc = "0x460 - XCFGI_DW0C."]
    #[inline(always)]
    pub const fn xcfgi_dw0c(&self) -> &XcfgiDw0c {
        &self.xcfgi_dw0c
    }
    #[doc = "0x464 - XCFGI_DW0D."]
    #[inline(always)]
    pub const fn xcfgi_dw0d(&self) -> &XcfgiDw0d {
        &self.xcfgi_dw0d
    }
    #[doc = "0x468 - GPIO_MODE."]
    #[inline(always)]
    pub const fn gpio_mode(&self) -> &GpioMode {
        &self.gpio_mode
    }
    #[doc = "0x46c - GPIO_DP_IE."]
    #[inline(always)]
    pub const fn gpio_dp_ie(&self) -> &GpioDpIe {
        &self.gpio_dp_ie
    }
    #[doc = "0x470 - GPIO_DN_IE."]
    #[inline(always)]
    pub const fn gpio_dn_ie(&self) -> &GpioDnIe {
        &self.gpio_dn_ie
    }
    #[doc = "0x474 - GPIO_DP_C."]
    #[inline(always)]
    pub const fn gpio_dp_c(&self) -> &GpioDpC {
        &self.gpio_dp_c
    }
    #[doc = "0x478 - GPIO_DN_C."]
    #[inline(always)]
    pub const fn gpio_dn_c(&self) -> &GpioDnC {
        &self.gpio_dn_c
    }
    #[doc = "0x47c - PMA_RDY."]
    #[inline(always)]
    pub const fn vcontrol(&self) -> &Vcontrol {
        &self.vcontrol
    }
    #[doc = "0x480 - MPSOV1."]
    #[inline(always)]
    pub const fn mpsov1(&self) -> &Mpsov1 {
        &self.mpsov1
    }
    #[doc = "0x484 - MPSOV2."]
    #[inline(always)]
    pub const fn mpsov2(&self) -> &Mpsov2 {
        &self.mpsov2
    }
    #[doc = "0x488 - MPSOV3."]
    #[inline(always)]
    pub const fn mpsov3(&self) -> &Mpsov3 {
        &self.mpsov3
    }
    #[doc = "0x490 - RG_CDRX_DSIRX_EN."]
    #[inline(always)]
    pub const fn rg_cdrx_dsirx_en(&self) -> &RgCdrxDsirxEn {
        &self.rg_cdrx_dsirx_en
    }
    #[doc = "0x494 - RG_CDRX_L012_SUBLVDS_EN."]
    #[inline(always)]
    pub const fn rg_cdrx_l012_sublvds_en(&self) -> &RgCdrxL012SublvdsEn {
        &self.rg_cdrx_l012_sublvds_en
    }
    #[doc = "0x498 - RG_CDRX_L012_HSRT_CTRL."]
    #[inline(always)]
    pub const fn rg_cdrx_l012_hsrt_ctrl(&self) -> &RgCdrxL012HsrtCtrl {
        &self.rg_cdrx_l012_hsrt_ctrl
    }
    #[doc = "0x49c - RG_CDRX_BISTHS_PLL_EN."]
    #[inline(always)]
    pub const fn rg_cdrx_bisths_pll_en(&self) -> &RgCdrxBisthsPllEn {
        &self.rg_cdrx_bisths_pll_en
    }
    #[doc = "0x4a0 - RG_CDRX_BISTHS_PLL_PRE_DIV2."]
    #[inline(always)]
    pub const fn rg_cdrx_bisths_pll_pre_div2(&self) -> &RgCdrxBisthsPllPreDiv2 {
        &self.rg_cdrx_bisths_pll_pre_div2
    }
    #[doc = "0x4a4 - RG_CDRX_BISTHS_PLL_FBK_INT."]
    #[inline(always)]
    pub const fn rg_cdrx_bisths_pll_fbk_int(&self) -> &RgCdrxBisthsPllFbkInt {
        &self.rg_cdrx_bisths_pll_fbk_int
    }
    #[doc = "0x4a8 - DBG1_MUX_SEL."]
    #[inline(always)]
    pub const fn dbg1_mux_sel(&self) -> &Dbg1MuxSel {
        &self.dbg1_mux_sel
    }
    #[doc = "0x4ac - DBG2_MUX_SEL."]
    #[inline(always)]
    pub const fn dbg2_mux_sel(&self) -> &Dbg2MuxSel {
        &self.dbg2_mux_sel
    }
    #[doc = "0x4b0 - DBG1_MUX_DOUT."]
    #[inline(always)]
    pub const fn dbg1_mux_dout(&self) -> &Dbg1MuxDout {
        &self.dbg1_mux_dout
    }
    #[doc = "0x4b4 - DBG2_MUX_DOUT."]
    #[inline(always)]
    pub const fn dbg2_mux_dout(&self) -> &Dbg2MuxDout {
        &self.dbg2_mux_dout
    }
    #[doc = "0x4b8 - AON_POWER_READY_N."]
    #[inline(always)]
    pub const fn aon_power_ready_n(&self) -> &AonPowerReadyN {
        &self.aon_power_ready_n
    }
    #[doc = "0x4bc - DPHY_RST_N."]
    #[inline(always)]
    pub const fn dphy_rst_n(&self) -> &DphyRstN {
        &self.dphy_rst_n
    }
    #[doc = "0x4c0 - RXBYTECLKHS_INV."]
    #[inline(always)]
    pub const fn rxbyteclkhs_inv(&self) -> &RxbyteclkhsInv {
        &self.rxbyteclkhs_inv
    }
    #[doc = "0x500 - Video FIFO Configuration Register 0."]
    #[inline(always)]
    pub const fn vfifo_cfg0(&self) -> &VfifoCfg0 {
        &self.vfifo_cfg0
    }
    #[doc = "0x504 - Video FIFO Configuration Register 1."]
    #[inline(always)]
    pub const fn vfifo_cfg1(&self) -> &VfifoCfg1 {
        &self.vfifo_cfg1
    }
    #[doc = "0x508 - Video FIFO Control Register."]
    #[inline(always)]
    pub const fn vfifo_ctrl(&self) -> &VfifoCtrl {
        &self.vfifo_ctrl
    }
    #[doc = "0x50c - Video FIFO Status Register."]
    #[inline(always)]
    pub const fn vfifo_sts(&self) -> &VfifoSts {
        &self.vfifo_sts
    }
    #[doc = "0x510 - Video FIFO CSI Line Number Per Frame."]
    #[inline(always)]
    pub const fn vfifo_line_num(&self) -> &VfifoLineNum {
        &self.vfifo_line_num
    }
    #[doc = "0x514 - Video FIFO CSI Pixel Number Per Line."]
    #[inline(always)]
    pub const fn vfifo_pixel_num(&self) -> &VfifoPixelNum {
        &self.vfifo_pixel_num
    }
    #[doc = "0x518 - Video FIFO CSI Line Count."]
    #[inline(always)]
    pub const fn vfifo_line_cnt(&self) -> &VfifoLineCnt {
        &self.vfifo_line_cnt
    }
    #[doc = "0x51c - Video FIFO CSI Pixel Count."]
    #[inline(always)]
    pub const fn vfifo_pixel_cnt(&self) -> &VfifoPixelCnt {
        &self.vfifo_pixel_cnt
    }
    #[doc = "0x520 - Video FIFO Frame Status Register."]
    #[inline(always)]
    pub const fn vfifo_frame_sts(&self) -> &VfifoFrameSts {
        &self.vfifo_frame_sts
    }
    #[doc = "0x524 - Video FIFO RAW-to-RGB Control Register."]
    #[inline(always)]
    pub const fn vfifo_raw_ctrl(&self) -> &VfifoRawCtrl {
        &self.vfifo_raw_ctrl
    }
    #[doc = "0x528 - Video FIFO RAW-to-RGB Line Buffer0 Address."]
    #[inline(always)]
    pub const fn vfifo_raw_buf0_addr(&self) -> &VfifoRawBuf0Addr {
        &self.vfifo_raw_buf0_addr
    }
    #[doc = "0x52c - Video FIFO RAW-to-RGB Line Buffer1 Address."]
    #[inline(always)]
    pub const fn vfifo_raw_buf1_addr(&self) -> &VfifoRawBuf1Addr {
        &self.vfifo_raw_buf1_addr
    }
    #[doc = "0x530 - Video FIFO AHB Master Control Register."]
    #[inline(always)]
    pub const fn vfifo_ahbm_ctrl(&self) -> &VfifoAhbmCtrl {
        &self.vfifo_ahbm_ctrl
    }
    #[doc = "0x534 - Video FIFO AHB Master Status Register."]
    #[inline(always)]
    pub const fn vfifo_ahbm_sts(&self) -> &VfifoAhbmSts {
        &self.vfifo_ahbm_sts
    }
    #[doc = "0x538 - Video FIFO AHB Master Start Address Register."]
    #[inline(always)]
    pub const fn vfifo_ahbm_start_addr(&self) -> &VfifoAhbmStartAddr {
        &self.vfifo_ahbm_start_addr
    }
    #[doc = "0x53c - Video FIFO AHB Master Address Range Register."]
    #[inline(always)]
    pub const fn vfifo_ahbm_addr_range(&self) -> &VfifoAhbmAddrRange {
        &self.vfifo_ahbm_addr_range
    }
    #[doc = "0x540 - Video FIFO AHB Master Maximal Transfer Number Register."]
    #[inline(always)]
    pub const fn vfifo_ahbm_max_trans(&self) -> &VfifoAhbmMaxTrans {
        &self.vfifo_ahbm_max_trans
    }
    #[doc = "0x544 - Video FIFO AHB Master Transfer Count Register."]
    #[inline(always)]
    pub const fn vfifo_ahbm_trans_cnt(&self) -> &VfifoAhbmTransCnt {
        &self.vfifo_ahbm_trans_cnt
    }
    #[doc = "0x600 - RX Video FIFO Interrupt Enable Register."]
    #[inline(always)]
    pub const fn rx_eint_vff_ie(&self) -> &RxEintVffIe {
        &self.rx_eint_vff_ie
    }
    #[doc = "0x604 - RX Video FIFO Interrupt Flag Register."]
    #[inline(always)]
    pub const fn rx_eint_vff_if(&self) -> &RxEintVffIf {
        &self.rx_eint_vff_if
    }
    #[doc = "0x608 - RX D-PHY Interrupt Enable Register."]
    #[inline(always)]
    pub const fn rx_eint_ppi_ie(&self) -> &RxEintPpiIe {
        &self.rx_eint_ppi_ie
    }
    #[doc = "0x60c - RX D-PHY Interrupt Flag Register."]
    #[inline(always)]
    pub const fn rx_eint_ppi_if(&self) -> &RxEintPpiIf {
        &self.rx_eint_ppi_if
    }
    #[doc = "0x610 - RX Controller Interrupt Enable Register."]
    #[inline(always)]
    pub const fn rx_eint_ctrl_ie(&self) -> &RxEintCtrlIe {
        &self.rx_eint_ctrl_ie
    }
    #[doc = "0x614 - RX Controller Interrupt Flag Register."]
    #[inline(always)]
    pub const fn rx_eint_ctrl_if(&self) -> &RxEintCtrlIf {
        &self.rx_eint_ctrl_if
    }
    #[doc = "0x700 - DPHY PPI Stop State Register."]
    #[inline(always)]
    pub const fn ppi_stopstate(&self) -> &PpiStopstate {
        &self.ppi_stopstate
    }
    #[doc = "0x704 - DPHY PPI Turn-Around Configuration Register."]
    #[inline(always)]
    pub const fn ppi_turnaround_cfg(&self) -> &PpiTurnaroundCfg {
        &self.ppi_turnaround_cfg
    }
}
#[doc = "CFG_NUM_LANES (rw) register accessor: CFG_NUM_LANES.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_num_lanes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_num_lanes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_num_lanes`] module"]
#[doc(alias = "CFG_NUM_LANES")]
pub type CfgNumLanes = crate::Reg<cfg_num_lanes::CfgNumLanesSpec>;
#[doc = "CFG_NUM_LANES."]
pub mod cfg_num_lanes;
#[doc = "CFG_CLK_LANE_EN (rw) register accessor: CFG_CLK_LANE_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_clk_lane_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_clk_lane_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_clk_lane_en`] module"]
#[doc(alias = "CFG_CLK_LANE_EN")]
pub type CfgClkLaneEn = crate::Reg<cfg_clk_lane_en::CfgClkLaneEnSpec>;
#[doc = "CFG_CLK_LANE_EN."]
pub mod cfg_clk_lane_en;
#[doc = "CFG_DATA_LANE_EN (rw) register accessor: CFG_DATA_LANE_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_lane_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_lane_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_data_lane_en`] module"]
#[doc(alias = "CFG_DATA_LANE_EN")]
pub type CfgDataLaneEn = crate::Reg<cfg_data_lane_en::CfgDataLaneEnSpec>;
#[doc = "CFG_DATA_LANE_EN."]
pub mod cfg_data_lane_en;
#[doc = "CFG_FLUSH_COUNT (rw) register accessor: CFG_FLUSH_COUNT.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_flush_count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_flush_count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_flush_count`] module"]
#[doc(alias = "CFG_FLUSH_COUNT")]
pub type CfgFlushCount = crate::Reg<cfg_flush_count::CfgFlushCountSpec>;
#[doc = "CFG_FLUSH_COUNT."]
pub mod cfg_flush_count;
#[doc = "CFG_BIT_ERR (rw) register accessor: CFG_BIT_ERR.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_bit_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_bit_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_bit_err`] module"]
#[doc(alias = "CFG_BIT_ERR")]
pub type CfgBitErr = crate::Reg<cfg_bit_err::CfgBitErrSpec>;
#[doc = "CFG_BIT_ERR."]
pub mod cfg_bit_err;
#[doc = "IRQ_STATUS (rw) register accessor: IRQ_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_status`] module"]
#[doc(alias = "IRQ_STATUS")]
pub type IrqStatus = crate::Reg<irq_status::IrqStatusSpec>;
#[doc = "IRQ_STATUS."]
pub mod irq_status;
#[doc = "IRQ_ENABLE (rw) register accessor: IRQ_ENABLE.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`] module"]
#[doc(alias = "IRQ_ENABLE")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
#[doc = "IRQ_ENABLE."]
pub mod irq_enable;
#[doc = "IRQ_CLR (rw) register accessor: IRQ_CLR.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_clr`] module"]
#[doc(alias = "IRQ_CLR")]
pub type IrqClr = crate::Reg<irq_clr::IrqClrSpec>;
#[doc = "IRQ_CLR."]
pub mod irq_clr;
#[doc = "ULPS_CLK_STATUS (rw) register accessor: ULPS_CLK_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_clk_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_clk_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulps_clk_status`] module"]
#[doc(alias = "ULPS_CLK_STATUS")]
pub type UlpsClkStatus = crate::Reg<ulps_clk_status::UlpsClkStatusSpec>;
#[doc = "ULPS_CLK_STATUS."]
pub mod ulps_clk_status;
#[doc = "ULPS_STATUS (rw) register accessor: ULPS_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulps_status`] module"]
#[doc(alias = "ULPS_STATUS")]
pub type UlpsStatus = crate::Reg<ulps_status::UlpsStatusSpec>;
#[doc = "ULPS_STATUS."]
pub mod ulps_status;
#[doc = "ULPS_CLK_MARK_STATUS (rw) register accessor: ULPS_CLK_MARK_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_clk_mark_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_clk_mark_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulps_clk_mark_status`] module"]
#[doc(alias = "ULPS_CLK_MARK_STATUS")]
pub type UlpsClkMarkStatus = crate::Reg<ulps_clk_mark_status::UlpsClkMarkStatusSpec>;
#[doc = "ULPS_CLK_MARK_STATUS."]
pub mod ulps_clk_mark_status;
#[doc = "ULPS_MARK_STATUS (rw) register accessor: ULPS_MARK_STATUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ulps_mark_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulps_mark_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ulps_mark_status`] module"]
#[doc(alias = "ULPS_MARK_STATUS")]
pub type UlpsMarkStatus = crate::Reg<ulps_mark_status::UlpsMarkStatusSpec>;
#[doc = "ULPS_MARK_STATUS."]
pub mod ulps_mark_status;
#[doc = "PPI_ERRSOT_HS (rw) register accessor: PPI_ERRSOT_HS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errsot_hs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errsot_hs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_errsot_hs`] module"]
#[doc(alias = "PPI_ERRSOT_HS")]
pub type PpiErrsotHs = crate::Reg<ppi_errsot_hs::PpiErrsotHsSpec>;
#[doc = "PPI_ERRSOT_HS."]
pub mod ppi_errsot_hs;
#[doc = "PPI_ERRSOTSYNC_HS (rw) register accessor: PPI_ERRSOTSYNC_HS.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errsotsync_hs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errsotsync_hs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_errsotsync_hs`] module"]
#[doc(alias = "PPI_ERRSOTSYNC_HS")]
pub type PpiErrsotsyncHs = crate::Reg<ppi_errsotsync_hs::PpiErrsotsyncHsSpec>;
#[doc = "PPI_ERRSOTSYNC_HS."]
pub mod ppi_errsotsync_hs;
#[doc = "PPI_ERRESC (rw) register accessor: PPI_ERRESC.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_erresc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_erresc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_erresc`] module"]
#[doc(alias = "PPI_ERRESC")]
pub type PpiErresc = crate::Reg<ppi_erresc::PpiErrescSpec>;
#[doc = "PPI_ERRESC."]
pub mod ppi_erresc;
#[doc = "PPI_ERRSYNCESC (rw) register accessor: PPI_ERRSYNCESC.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errsyncesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errsyncesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_errsyncesc`] module"]
#[doc(alias = "PPI_ERRSYNCESC")]
pub type PpiErrsyncesc = crate::Reg<ppi_errsyncesc::PpiErrsyncescSpec>;
#[doc = "PPI_ERRSYNCESC."]
pub mod ppi_errsyncesc;
#[doc = "PPI_ERRCONTROL (rw) register accessor: PPI_ERRCONTROL.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_errcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_errcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_errcontrol`] module"]
#[doc(alias = "PPI_ERRCONTROL")]
pub type PpiErrcontrol = crate::Reg<ppi_errcontrol::PpiErrcontrolSpec>;
#[doc = "PPI_ERRCONTROL."]
pub mod ppi_errcontrol;
#[doc = "CFG_CPHY_EN (rw) register accessor: CFG_CPHY_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_cphy_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_cphy_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cphy_en`] module"]
#[doc(alias = "CFG_CPHY_EN")]
pub type CfgCphyEn = crate::Reg<cfg_cphy_en::CfgCphyEnSpec>;
#[doc = "CFG_CPHY_EN."]
pub mod cfg_cphy_en;
#[doc = "CFG_PPI_16_EN (rw) register accessor: CFG_PPI_16_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_ppi_16_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_ppi_16_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ppi_16_en`] module"]
#[doc(alias = "CFG_PPI_16_EN")]
pub type CfgPpi16En = crate::Reg<cfg_ppi_16_en::CfgPpi16EnSpec>;
#[doc = "CFG_PPI_16_EN."]
pub mod cfg_ppi_16_en;
#[doc = "CFG_PACKET_INTERFACE_EN (rw) register accessor: CFG_PACKET_INTERFACE_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_packet_interface_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_packet_interface_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_packet_interface_en`] module"]
#[doc(alias = "CFG_PACKET_INTERFACE_EN")]
pub type CfgPacketInterfaceEn = crate::Reg<cfg_packet_interface_en::CfgPacketInterfaceEnSpec>;
#[doc = "CFG_PACKET_INTERFACE_EN."]
pub mod cfg_packet_interface_en;
#[doc = "CFG_VCX_EN (rw) register accessor: CFG_VCX_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vcx_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vcx_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vcx_en`] module"]
#[doc(alias = "CFG_VCX_EN")]
pub type CfgVcxEn = crate::Reg<cfg_vcx_en::CfgVcxEnSpec>;
#[doc = "CFG_VCX_EN."]
pub mod cfg_vcx_en;
#[doc = "CFG_BYTE_DATA_FORMAT (rw) register accessor: CFG_BYTE_DATA_FORMAT.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_byte_data_format::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_byte_data_format::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_byte_data_format`] module"]
#[doc(alias = "CFG_BYTE_DATA_FORMAT")]
pub type CfgByteDataFormat = crate::Reg<cfg_byte_data_format::CfgByteDataFormatSpec>;
#[doc = "CFG_BYTE_DATA_FORMAT."]
pub mod cfg_byte_data_format;
#[doc = "CFG_DISABLE_PAYLOAD_0 (rw) register accessor: CFG_DISABLE_PAYLOAD_0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_disable_payload_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_disable_payload_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_disable_payload_0`] module"]
#[doc(alias = "CFG_DISABLE_PAYLOAD_0")]
pub type CfgDisablePayload0 = crate::Reg<cfg_disable_payload_0::CfgDisablePayload0Spec>;
#[doc = "CFG_DISABLE_PAYLOAD_0."]
pub mod cfg_disable_payload_0;
#[doc = "CFG_DISABLE_PAYLOAD_1 (rw) register accessor: CFG_DISABLE_PAYLOAD_1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_disable_payload_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_disable_payload_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_disable_payload_1`] module"]
#[doc(alias = "CFG_DISABLE_PAYLOAD_1")]
pub type CfgDisablePayload1 = crate::Reg<cfg_disable_payload_1::CfgDisablePayload1Spec>;
#[doc = "CFG_DISABLE_PAYLOAD_1."]
pub mod cfg_disable_payload_1;
#[doc = "CFG_VID_IGNORE_VC (rw) register accessor: CFG_VID_IGNORE_VC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_ignore_vc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_ignore_vc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vid_ignore_vc`] module"]
#[doc(alias = "CFG_VID_IGNORE_VC")]
pub type CfgVidIgnoreVc = crate::Reg<cfg_vid_ignore_vc::CfgVidIgnoreVcSpec>;
#[doc = "CFG_VID_IGNORE_VC."]
pub mod cfg_vid_ignore_vc;
#[doc = "CFG_VID_VC (rw) register accessor: CFG_VID_VC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_vc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_vc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vid_vc`] module"]
#[doc(alias = "CFG_VID_VC")]
pub type CfgVidVc = crate::Reg<cfg_vid_vc::CfgVidVcSpec>;
#[doc = "CFG_VID_VC."]
pub mod cfg_vid_vc;
#[doc = "CFG_P_FIFO_SEND_LEVEL (rw) register accessor: CFG_P_FIFO_SEND_LEVEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_p_fifo_send_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_p_fifo_send_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_p_fifo_send_level`] module"]
#[doc(alias = "CFG_P_FIFO_SEND_LEVEL")]
pub type CfgPFifoSendLevel = crate::Reg<cfg_p_fifo_send_level::CfgPFifoSendLevelSpec>;
#[doc = "CFG_P_FIFO_SEND_LEVEL."]
pub mod cfg_p_fifo_send_level;
#[doc = "CFG_VID_VSYNC (rw) register accessor: CFG_VID_VSYNC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_vsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_vsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vid_vsync`] module"]
#[doc(alias = "CFG_VID_VSYNC")]
pub type CfgVidVsync = crate::Reg<cfg_vid_vsync::CfgVidVsyncSpec>;
#[doc = "CFG_VID_VSYNC."]
pub mod cfg_vid_vsync;
#[doc = "CFG_VID_HSYNC_FP (rw) register accessor: CFG_VID_HSYNC_FP.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_hsync_fp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_hsync_fp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vid_hsync_fp`] module"]
#[doc(alias = "CFG_VID_HSYNC_FP")]
pub type CfgVidHsyncFp = crate::Reg<cfg_vid_hsync_fp::CfgVidHsyncFpSpec>;
#[doc = "CFG_VID_HSYNC_FP."]
pub mod cfg_vid_hsync_fp;
#[doc = "CFG_VID_HSYNC (rw) register accessor: CFG_VID_HSYNC.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_hsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_hsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vid_hsync`] module"]
#[doc(alias = "CFG_VID_HSYNC")]
pub type CfgVidHsync = crate::Reg<cfg_vid_hsync::CfgVidHsyncSpec>;
#[doc = "CFG_VID_HSYNC."]
pub mod cfg_vid_hsync;
#[doc = "CFG_VID_HSYNC_BP (rw) register accessor: CFG_VID_HSYNC_BP.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_vid_hsync_bp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_vid_hsync_bp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_vid_hsync_bp`] module"]
#[doc(alias = "CFG_VID_HSYNC_BP")]
pub type CfgVidHsyncBp = crate::Reg<cfg_vid_hsync_bp::CfgVidHsyncBpSpec>;
#[doc = "CFG_VID_HSYNC_BP."]
pub mod cfg_vid_hsync_bp;
#[doc = "CFG_DATABUS16_SEL (rw) register accessor: CFG_DATABUS16_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_databus16_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_databus16_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_databus16_sel`] module"]
#[doc(alias = "CFG_DATABUS16_SEL")]
pub type CfgDatabus16Sel = crate::Reg<cfg_databus16_sel::CfgDatabus16SelSpec>;
#[doc = "CFG_DATABUS16_SEL."]
pub mod cfg_databus16_sel;
#[doc = "CFG_D0_SWAP_SEL (rw) register accessor: CFG_D0_SWAP_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_d0_swap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_d0_swap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_d0_swap_sel`] module"]
#[doc(alias = "CFG_D0_SWAP_SEL")]
pub type CfgD0SwapSel = crate::Reg<cfg_d0_swap_sel::CfgD0SwapSelSpec>;
#[doc = "CFG_D0_SWAP_SEL."]
pub mod cfg_d0_swap_sel;
#[doc = "CFG_D1_SWAP_SEL (rw) register accessor: CFG_D1_SWAP_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_d1_swap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_d1_swap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_d1_swap_sel`] module"]
#[doc(alias = "CFG_D1_SWAP_SEL")]
pub type CfgD1SwapSel = crate::Reg<cfg_d1_swap_sel::CfgD1SwapSelSpec>;
#[doc = "CFG_D1_SWAP_SEL."]
pub mod cfg_d1_swap_sel;
#[doc = "CFG_D2_SWAP_SEL (rw) register accessor: CFG_D2_SWAP_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_d2_swap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_d2_swap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_d2_swap_sel`] module"]
#[doc(alias = "CFG_D2_SWAP_SEL")]
pub type CfgD2SwapSel = crate::Reg<cfg_d2_swap_sel::CfgD2SwapSelSpec>;
#[doc = "CFG_D2_SWAP_SEL."]
pub mod cfg_d2_swap_sel;
#[doc = "CFG_D3_SWAP_SEL (rw) register accessor: CFG_D3_SWAP_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_d3_swap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_d3_swap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_d3_swap_sel`] module"]
#[doc(alias = "CFG_D3_SWAP_SEL")]
pub type CfgD3SwapSel = crate::Reg<cfg_d3_swap_sel::CfgD3SwapSelSpec>;
#[doc = "CFG_D3_SWAP_SEL."]
pub mod cfg_d3_swap_sel;
#[doc = "CFG_C0_SWAP_SEL (rw) register accessor: CFG_C0_SWAP_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_c0_swap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_c0_swap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_c0_swap_sel`] module"]
#[doc(alias = "CFG_C0_SWAP_SEL")]
pub type CfgC0SwapSel = crate::Reg<cfg_c0_swap_sel::CfgC0SwapSelSpec>;
#[doc = "CFG_C0_SWAP_SEL."]
pub mod cfg_c0_swap_sel;
#[doc = "CFG_DPDN_SWAP (rw) register accessor: CFG_DPDN_SWAP.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_dpdn_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_dpdn_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dpdn_swap`] module"]
#[doc(alias = "CFG_DPDN_SWAP")]
pub type CfgDpdnSwap = crate::Reg<cfg_dpdn_swap::CfgDpdnSwapSpec>;
#[doc = "CFG_DPDN_SWAP."]
pub mod cfg_dpdn_swap;
#[doc = "RG_CFGCLK_1US_CNT (rw) register accessor: RG_CFGCLK_1US_CNT.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cfgclk_1us_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cfgclk_1us_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cfgclk_1us_cnt`] module"]
#[doc(alias = "RG_CFGCLK_1US_CNT")]
pub type RgCfgclk1usCnt = crate::Reg<rg_cfgclk_1us_cnt::RgCfgclk1usCntSpec>;
#[doc = "RG_CFGCLK_1US_CNT."]
pub mod rg_cfgclk_1us_cnt;
#[doc = "RG_HSRX_CLK_PRE_TIME_GRP0 (rw) register accessor: RG_HSRX_CLK_PRE_TIME_GRP0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_hsrx_clk_pre_time_grp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_hsrx_clk_pre_time_grp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_hsrx_clk_pre_time_grp0`] module"]
#[doc(alias = "RG_HSRX_CLK_PRE_TIME_GRP0")]
pub type RgHsrxClkPreTimeGrp0 = crate::Reg<rg_hsrx_clk_pre_time_grp0::RgHsrxClkPreTimeGrp0Spec>;
#[doc = "RG_HSRX_CLK_PRE_TIME_GRP0."]
pub mod rg_hsrx_clk_pre_time_grp0;
#[doc = "RG_HSRX_DATA_PRE_TIME_GRP0 (rw) register accessor: RG_HSRX_DATA_PRE_TIME_GRP0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_hsrx_data_pre_time_grp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_hsrx_data_pre_time_grp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_hsrx_data_pre_time_grp0`] module"]
#[doc(alias = "RG_HSRX_DATA_PRE_TIME_GRP0")]
pub type RgHsrxDataPreTimeGrp0 = crate::Reg<rg_hsrx_data_pre_time_grp0::RgHsrxDataPreTimeGrp0Spec>;
#[doc = "RG_HSRX_DATA_PRE_TIME_GRP0."]
pub mod rg_hsrx_data_pre_time_grp0;
#[doc = "RESET_DESKEW (rw) register accessor: RESET_DESKEW.\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_deskew::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_deskew::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_deskew`] module"]
#[doc(alias = "RESET_DESKEW")]
pub type ResetDeskew = crate::Reg<reset_deskew::ResetDeskewSpec>;
#[doc = "RESET_DESKEW."]
pub mod reset_deskew;
#[doc = "PMA_RDY (rw) register accessor: PMA_RDY.\n\nYou can [`read`](crate::Reg::read) this register and get [`pma_rdy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pma_rdy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pma_rdy`] module"]
#[doc(alias = "PMA_RDY")]
pub type PmaRdy = crate::Reg<pma_rdy::PmaRdySpec>;
#[doc = "PMA_RDY."]
pub mod pma_rdy;
#[doc = "XCFGI_DW00 (rw) register accessor: XCFGI_DW00.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw00`] module"]
#[doc(alias = "XCFGI_DW00")]
pub type XcfgiDw00 = crate::Reg<xcfgi_dw00::XcfgiDw00Spec>;
#[doc = "XCFGI_DW00."]
pub mod xcfgi_dw00;
#[doc = "XCFGI_DW01 (rw) register accessor: XCFGI_DW01.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw01`] module"]
#[doc(alias = "XCFGI_DW01")]
pub type XcfgiDw01 = crate::Reg<xcfgi_dw01::XcfgiDw01Spec>;
#[doc = "XCFGI_DW01."]
pub mod xcfgi_dw01;
#[doc = "XCFGI_DW02 (rw) register accessor: XCFGI_DW02.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw02::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw02::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw02`] module"]
#[doc(alias = "XCFGI_DW02")]
pub type XcfgiDw02 = crate::Reg<xcfgi_dw02::XcfgiDw02Spec>;
#[doc = "XCFGI_DW02."]
pub mod xcfgi_dw02;
#[doc = "XCFGI_DW03 (rw) register accessor: XCFGI_DW03.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw03::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw03::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw03`] module"]
#[doc(alias = "XCFGI_DW03")]
pub type XcfgiDw03 = crate::Reg<xcfgi_dw03::XcfgiDw03Spec>;
#[doc = "XCFGI_DW03."]
pub mod xcfgi_dw03;
#[doc = "XCFGI_DW04 (rw) register accessor: XCFGI_DW04.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw04`] module"]
#[doc(alias = "XCFGI_DW04")]
pub type XcfgiDw04 = crate::Reg<xcfgi_dw04::XcfgiDw04Spec>;
#[doc = "XCFGI_DW04."]
pub mod xcfgi_dw04;
#[doc = "XCFGI_DW05 (rw) register accessor: XCFGI_DW05.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw05::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw05::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw05`] module"]
#[doc(alias = "XCFGI_DW05")]
pub type XcfgiDw05 = crate::Reg<xcfgi_dw05::XcfgiDw05Spec>;
#[doc = "XCFGI_DW05."]
pub mod xcfgi_dw05;
#[doc = "XCFGI_DW06 (rw) register accessor: XCFGI_DW06.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw06::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw06::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw06`] module"]
#[doc(alias = "XCFGI_DW06")]
pub type XcfgiDw06 = crate::Reg<xcfgi_dw06::XcfgiDw06Spec>;
#[doc = "XCFGI_DW06."]
pub mod xcfgi_dw06;
#[doc = "XCFGI_DW07 (rw) register accessor: XCFGI_DW07.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw07::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw07::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw07`] module"]
#[doc(alias = "XCFGI_DW07")]
pub type XcfgiDw07 = crate::Reg<xcfgi_dw07::XcfgiDw07Spec>;
#[doc = "XCFGI_DW07."]
pub mod xcfgi_dw07;
#[doc = "XCFGI_DW08 (rw) register accessor: XCFGI_DW08.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw08`] module"]
#[doc(alias = "XCFGI_DW08")]
pub type XcfgiDw08 = crate::Reg<xcfgi_dw08::XcfgiDw08Spec>;
#[doc = "XCFGI_DW08."]
pub mod xcfgi_dw08;
#[doc = "XCFGI_DW09 (rw) register accessor: XCFGI_DW09.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw09::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw09::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw09`] module"]
#[doc(alias = "XCFGI_DW09")]
pub type XcfgiDw09 = crate::Reg<xcfgi_dw09::XcfgiDw09Spec>;
#[doc = "XCFGI_DW09."]
pub mod xcfgi_dw09;
#[doc = "XCFGI_DW0A (rw) register accessor: XCFGI_DW0A.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw0a`] module"]
#[doc(alias = "XCFGI_DW0A")]
pub type XcfgiDw0a = crate::Reg<xcfgi_dw0a::XcfgiDw0aSpec>;
#[doc = "XCFGI_DW0A."]
pub mod xcfgi_dw0a;
#[doc = "XCFGI_DW0B (rw) register accessor: XCFGI_DW0B.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw0b`] module"]
#[doc(alias = "XCFGI_DW0B")]
pub type XcfgiDw0b = crate::Reg<xcfgi_dw0b::XcfgiDw0bSpec>;
#[doc = "XCFGI_DW0B."]
pub mod xcfgi_dw0b;
#[doc = "XCFGI_DW0C (rw) register accessor: XCFGI_DW0C.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw0c`] module"]
#[doc(alias = "XCFGI_DW0C")]
pub type XcfgiDw0c = crate::Reg<xcfgi_dw0c::XcfgiDw0cSpec>;
#[doc = "XCFGI_DW0C."]
pub mod xcfgi_dw0c;
#[doc = "XCFGI_DW0D (rw) register accessor: XCFGI_DW0D.\n\nYou can [`read`](crate::Reg::read) this register and get [`xcfgi_dw0d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcfgi_dw0d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xcfgi_dw0d`] module"]
#[doc(alias = "XCFGI_DW0D")]
pub type XcfgiDw0d = crate::Reg<xcfgi_dw0d::XcfgiDw0dSpec>;
#[doc = "XCFGI_DW0D."]
pub mod xcfgi_dw0d;
#[doc = "GPIO_MODE (rw) register accessor: GPIO_MODE.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_mode`] module"]
#[doc(alias = "GPIO_MODE")]
pub type GpioMode = crate::Reg<gpio_mode::GpioModeSpec>;
#[doc = "GPIO_MODE."]
pub mod gpio_mode;
#[doc = "GPIO_DP_IE (rw) register accessor: GPIO_DP_IE.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dp_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dp_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_dp_ie`] module"]
#[doc(alias = "GPIO_DP_IE")]
pub type GpioDpIe = crate::Reg<gpio_dp_ie::GpioDpIeSpec>;
#[doc = "GPIO_DP_IE."]
pub mod gpio_dp_ie;
#[doc = "GPIO_DN_IE (rw) register accessor: GPIO_DN_IE.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dn_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dn_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_dn_ie`] module"]
#[doc(alias = "GPIO_DN_IE")]
pub type GpioDnIe = crate::Reg<gpio_dn_ie::GpioDnIeSpec>;
#[doc = "GPIO_DN_IE."]
pub mod gpio_dn_ie;
#[doc = "GPIO_DP_C (rw) register accessor: GPIO_DP_C.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dp_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dp_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_dp_c`] module"]
#[doc(alias = "GPIO_DP_C")]
pub type GpioDpC = crate::Reg<gpio_dp_c::GpioDpCSpec>;
#[doc = "GPIO_DP_C."]
pub mod gpio_dp_c;
#[doc = "GPIO_DN_C (rw) register accessor: GPIO_DN_C.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_dn_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_dn_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_dn_c`] module"]
#[doc(alias = "GPIO_DN_C")]
pub type GpioDnC = crate::Reg<gpio_dn_c::GpioDnCSpec>;
#[doc = "GPIO_DN_C."]
pub mod gpio_dn_c;
#[doc = "VCONTROL (rw) register accessor: PMA_RDY.\n\nYou can [`read`](crate::Reg::read) this register and get [`vcontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vcontrol`] module"]
#[doc(alias = "VCONTROL")]
pub type Vcontrol = crate::Reg<vcontrol::VcontrolSpec>;
#[doc = "PMA_RDY."]
pub mod vcontrol;
#[doc = "MPSOV1 (rw) register accessor: MPSOV1.\n\nYou can [`read`](crate::Reg::read) this register and get [`mpsov1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpsov1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpsov1`] module"]
#[doc(alias = "MPSOV1")]
pub type Mpsov1 = crate::Reg<mpsov1::Mpsov1Spec>;
#[doc = "MPSOV1."]
pub mod mpsov1;
#[doc = "MPSOV2 (rw) register accessor: MPSOV2.\n\nYou can [`read`](crate::Reg::read) this register and get [`mpsov2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpsov2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpsov2`] module"]
#[doc(alias = "MPSOV2")]
pub type Mpsov2 = crate::Reg<mpsov2::Mpsov2Spec>;
#[doc = "MPSOV2."]
pub mod mpsov2;
#[doc = "MPSOV3 (rw) register accessor: MPSOV3.\n\nYou can [`read`](crate::Reg::read) this register and get [`mpsov3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpsov3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mpsov3`] module"]
#[doc(alias = "MPSOV3")]
pub type Mpsov3 = crate::Reg<mpsov3::Mpsov3Spec>;
#[doc = "MPSOV3."]
pub mod mpsov3;
#[doc = "RG_CDRX_DSIRX_EN (rw) register accessor: RG_CDRX_DSIRX_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_dsirx_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_dsirx_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cdrx_dsirx_en`] module"]
#[doc(alias = "RG_CDRX_DSIRX_EN")]
pub type RgCdrxDsirxEn = crate::Reg<rg_cdrx_dsirx_en::RgCdrxDsirxEnSpec>;
#[doc = "RG_CDRX_DSIRX_EN."]
pub mod rg_cdrx_dsirx_en;
#[doc = "RG_CDRX_L012_SUBLVDS_EN (rw) register accessor: RG_CDRX_L012_SUBLVDS_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_l012_sublvds_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_l012_sublvds_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cdrx_l012_sublvds_en`] module"]
#[doc(alias = "RG_CDRX_L012_SUBLVDS_EN")]
pub type RgCdrxL012SublvdsEn = crate::Reg<rg_cdrx_l012_sublvds_en::RgCdrxL012SublvdsEnSpec>;
#[doc = "RG_CDRX_L012_SUBLVDS_EN."]
pub mod rg_cdrx_l012_sublvds_en;
#[doc = "RG_CDRX_L012_HSRT_CTRL (rw) register accessor: RG_CDRX_L012_HSRT_CTRL.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_l012_hsrt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_l012_hsrt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cdrx_l012_hsrt_ctrl`] module"]
#[doc(alias = "RG_CDRX_L012_HSRT_CTRL")]
pub type RgCdrxL012HsrtCtrl = crate::Reg<rg_cdrx_l012_hsrt_ctrl::RgCdrxL012HsrtCtrlSpec>;
#[doc = "RG_CDRX_L012_HSRT_CTRL."]
pub mod rg_cdrx_l012_hsrt_ctrl;
#[doc = "RG_CDRX_BISTHS_PLL_EN (rw) register accessor: RG_CDRX_BISTHS_PLL_EN.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_bisths_pll_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_bisths_pll_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cdrx_bisths_pll_en`] module"]
#[doc(alias = "RG_CDRX_BISTHS_PLL_EN")]
pub type RgCdrxBisthsPllEn = crate::Reg<rg_cdrx_bisths_pll_en::RgCdrxBisthsPllEnSpec>;
#[doc = "RG_CDRX_BISTHS_PLL_EN."]
pub mod rg_cdrx_bisths_pll_en;
#[doc = "RG_CDRX_BISTHS_PLL_PRE_DIV2 (rw) register accessor: RG_CDRX_BISTHS_PLL_PRE_DIV2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_bisths_pll_pre_div2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_bisths_pll_pre_div2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cdrx_bisths_pll_pre_div2`] module"]
#[doc(alias = "RG_CDRX_BISTHS_PLL_PRE_DIV2")]
pub type RgCdrxBisthsPllPreDiv2 =
    crate::Reg<rg_cdrx_bisths_pll_pre_div2::RgCdrxBisthsPllPreDiv2Spec>;
#[doc = "RG_CDRX_BISTHS_PLL_PRE_DIV2."]
pub mod rg_cdrx_bisths_pll_pre_div2;
#[doc = "RG_CDRX_BISTHS_PLL_FBK_INT (rw) register accessor: RG_CDRX_BISTHS_PLL_FBK_INT.\n\nYou can [`read`](crate::Reg::read) this register and get [`rg_cdrx_bisths_pll_fbk_int::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg_cdrx_bisths_pll_fbk_int::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg_cdrx_bisths_pll_fbk_int`] module"]
#[doc(alias = "RG_CDRX_BISTHS_PLL_FBK_INT")]
pub type RgCdrxBisthsPllFbkInt = crate::Reg<rg_cdrx_bisths_pll_fbk_int::RgCdrxBisthsPllFbkIntSpec>;
#[doc = "RG_CDRX_BISTHS_PLL_FBK_INT."]
pub mod rg_cdrx_bisths_pll_fbk_int;
#[doc = "DBG1_MUX_SEL (rw) register accessor: DBG1_MUX_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg1_mux_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg1_mux_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg1_mux_sel`] module"]
#[doc(alias = "DBG1_MUX_SEL")]
pub type Dbg1MuxSel = crate::Reg<dbg1_mux_sel::Dbg1MuxSelSpec>;
#[doc = "DBG1_MUX_SEL."]
pub mod dbg1_mux_sel;
#[doc = "DBG2_MUX_SEL (rw) register accessor: DBG2_MUX_SEL.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg2_mux_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg2_mux_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg2_mux_sel`] module"]
#[doc(alias = "DBG2_MUX_SEL")]
pub type Dbg2MuxSel = crate::Reg<dbg2_mux_sel::Dbg2MuxSelSpec>;
#[doc = "DBG2_MUX_SEL."]
pub mod dbg2_mux_sel;
#[doc = "DBG1_MUX_DOUT (rw) register accessor: DBG1_MUX_DOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg1_mux_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg1_mux_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg1_mux_dout`] module"]
#[doc(alias = "DBG1_MUX_DOUT")]
pub type Dbg1MuxDout = crate::Reg<dbg1_mux_dout::Dbg1MuxDoutSpec>;
#[doc = "DBG1_MUX_DOUT."]
pub mod dbg1_mux_dout;
#[doc = "DBG2_MUX_DOUT (rw) register accessor: DBG2_MUX_DOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg2_mux_dout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg2_mux_dout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbg2_mux_dout`] module"]
#[doc(alias = "DBG2_MUX_DOUT")]
pub type Dbg2MuxDout = crate::Reg<dbg2_mux_dout::Dbg2MuxDoutSpec>;
#[doc = "DBG2_MUX_DOUT."]
pub mod dbg2_mux_dout;
#[doc = "AON_POWER_READY_N (rw) register accessor: AON_POWER_READY_N.\n\nYou can [`read`](crate::Reg::read) this register and get [`aon_power_ready_n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aon_power_ready_n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_power_ready_n`] module"]
#[doc(alias = "AON_POWER_READY_N")]
pub type AonPowerReadyN = crate::Reg<aon_power_ready_n::AonPowerReadyNSpec>;
#[doc = "AON_POWER_READY_N."]
pub mod aon_power_ready_n;
#[doc = "DPHY_RST_N (rw) register accessor: DPHY_RST_N.\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_rst_n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_rst_n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dphy_rst_n`] module"]
#[doc(alias = "DPHY_RST_N")]
pub type DphyRstN = crate::Reg<dphy_rst_n::DphyRstNSpec>;
#[doc = "DPHY_RST_N."]
pub mod dphy_rst_n;
#[doc = "RXBYTECLKHS_INV (rw) register accessor: RXBYTECLKHS_INV.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbyteclkhs_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbyteclkhs_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbyteclkhs_inv`] module"]
#[doc(alias = "RXBYTECLKHS_INV")]
pub type RxbyteclkhsInv = crate::Reg<rxbyteclkhs_inv::RxbyteclkhsInvSpec>;
#[doc = "RXBYTECLKHS_INV."]
pub mod rxbyteclkhs_inv;
#[doc = "VFIFO_CFG0 (rw) register accessor: Video FIFO Configuration Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_cfg0`] module"]
#[doc(alias = "VFIFO_CFG0")]
pub type VfifoCfg0 = crate::Reg<vfifo_cfg0::VfifoCfg0Spec>;
#[doc = "Video FIFO Configuration Register 0."]
pub mod vfifo_cfg0;
#[doc = "VFIFO_CFG1 (rw) register accessor: Video FIFO Configuration Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_cfg1`] module"]
#[doc(alias = "VFIFO_CFG1")]
pub type VfifoCfg1 = crate::Reg<vfifo_cfg1::VfifoCfg1Spec>;
#[doc = "Video FIFO Configuration Register 1."]
pub mod vfifo_cfg1;
#[doc = "VFIFO_CTRL (rw) register accessor: Video FIFO Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ctrl`] module"]
#[doc(alias = "VFIFO_CTRL")]
pub type VfifoCtrl = crate::Reg<vfifo_ctrl::VfifoCtrlSpec>;
#[doc = "Video FIFO Control Register."]
pub mod vfifo_ctrl;
#[doc = "VFIFO_STS (rw) register accessor: Video FIFO Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_sts`] module"]
#[doc(alias = "VFIFO_STS")]
pub type VfifoSts = crate::Reg<vfifo_sts::VfifoStsSpec>;
#[doc = "Video FIFO Status Register."]
pub mod vfifo_sts;
#[doc = "VFIFO_LINE_NUM (rw) register accessor: Video FIFO CSI Line Number Per Frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_line_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_line_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_line_num`] module"]
#[doc(alias = "VFIFO_LINE_NUM")]
pub type VfifoLineNum = crate::Reg<vfifo_line_num::VfifoLineNumSpec>;
#[doc = "Video FIFO CSI Line Number Per Frame."]
pub mod vfifo_line_num;
#[doc = "VFIFO_PIXEL_NUM (rw) register accessor: Video FIFO CSI Pixel Number Per Line.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_pixel_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_pixel_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_pixel_num`] module"]
#[doc(alias = "VFIFO_PIXEL_NUM")]
pub type VfifoPixelNum = crate::Reg<vfifo_pixel_num::VfifoPixelNumSpec>;
#[doc = "Video FIFO CSI Pixel Number Per Line."]
pub mod vfifo_pixel_num;
#[doc = "VFIFO_LINE_CNT (rw) register accessor: Video FIFO CSI Line Count.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_line_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_line_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_line_cnt`] module"]
#[doc(alias = "VFIFO_LINE_CNT")]
pub type VfifoLineCnt = crate::Reg<vfifo_line_cnt::VfifoLineCntSpec>;
#[doc = "Video FIFO CSI Line Count."]
pub mod vfifo_line_cnt;
#[doc = "VFIFO_PIXEL_CNT (rw) register accessor: Video FIFO CSI Pixel Count.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_pixel_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_pixel_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_pixel_cnt`] module"]
#[doc(alias = "VFIFO_PIXEL_CNT")]
pub type VfifoPixelCnt = crate::Reg<vfifo_pixel_cnt::VfifoPixelCntSpec>;
#[doc = "Video FIFO CSI Pixel Count."]
pub mod vfifo_pixel_cnt;
#[doc = "VFIFO_FRAME_STS (rw) register accessor: Video FIFO Frame Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_frame_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_frame_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_frame_sts`] module"]
#[doc(alias = "VFIFO_FRAME_STS")]
pub type VfifoFrameSts = crate::Reg<vfifo_frame_sts::VfifoFrameStsSpec>;
#[doc = "Video FIFO Frame Status Register."]
pub mod vfifo_frame_sts;
#[doc = "VFIFO_RAW_CTRL (rw) register accessor: Video FIFO RAW-to-RGB Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_raw_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_raw_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_raw_ctrl`] module"]
#[doc(alias = "VFIFO_RAW_CTRL")]
pub type VfifoRawCtrl = crate::Reg<vfifo_raw_ctrl::VfifoRawCtrlSpec>;
#[doc = "Video FIFO RAW-to-RGB Control Register."]
pub mod vfifo_raw_ctrl;
#[doc = "VFIFO_RAW_BUF0_ADDR (rw) register accessor: Video FIFO RAW-to-RGB Line Buffer0 Address.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_raw_buf0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_raw_buf0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_raw_buf0_addr`] module"]
#[doc(alias = "VFIFO_RAW_BUF0_ADDR")]
pub type VfifoRawBuf0Addr = crate::Reg<vfifo_raw_buf0_addr::VfifoRawBuf0AddrSpec>;
#[doc = "Video FIFO RAW-to-RGB Line Buffer0 Address."]
pub mod vfifo_raw_buf0_addr;
#[doc = "VFIFO_RAW_BUF1_ADDR (rw) register accessor: Video FIFO RAW-to-RGB Line Buffer1 Address.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_raw_buf1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_raw_buf1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_raw_buf1_addr`] module"]
#[doc(alias = "VFIFO_RAW_BUF1_ADDR")]
pub type VfifoRawBuf1Addr = crate::Reg<vfifo_raw_buf1_addr::VfifoRawBuf1AddrSpec>;
#[doc = "Video FIFO RAW-to-RGB Line Buffer1 Address."]
pub mod vfifo_raw_buf1_addr;
#[doc = "VFIFO_AHBM_CTRL (rw) register accessor: Video FIFO AHB Master Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ahbm_ctrl`] module"]
#[doc(alias = "VFIFO_AHBM_CTRL")]
pub type VfifoAhbmCtrl = crate::Reg<vfifo_ahbm_ctrl::VfifoAhbmCtrlSpec>;
#[doc = "Video FIFO AHB Master Control Register."]
pub mod vfifo_ahbm_ctrl;
#[doc = "VFIFO_AHBM_STS (rw) register accessor: Video FIFO AHB Master Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ahbm_sts`] module"]
#[doc(alias = "VFIFO_AHBM_STS")]
pub type VfifoAhbmSts = crate::Reg<vfifo_ahbm_sts::VfifoAhbmStsSpec>;
#[doc = "Video FIFO AHB Master Status Register."]
pub mod vfifo_ahbm_sts;
#[doc = "VFIFO_AHBM_START_ADDR (rw) register accessor: Video FIFO AHB Master Start Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ahbm_start_addr`] module"]
#[doc(alias = "VFIFO_AHBM_START_ADDR")]
pub type VfifoAhbmStartAddr = crate::Reg<vfifo_ahbm_start_addr::VfifoAhbmStartAddrSpec>;
#[doc = "Video FIFO AHB Master Start Address Register."]
pub mod vfifo_ahbm_start_addr;
#[doc = "VFIFO_AHBM_ADDR_RANGE (rw) register accessor: Video FIFO AHB Master Address Range Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_addr_range::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_addr_range::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ahbm_addr_range`] module"]
#[doc(alias = "VFIFO_AHBM_ADDR_RANGE")]
pub type VfifoAhbmAddrRange = crate::Reg<vfifo_ahbm_addr_range::VfifoAhbmAddrRangeSpec>;
#[doc = "Video FIFO AHB Master Address Range Register."]
pub mod vfifo_ahbm_addr_range;
#[doc = "VFIFO_AHBM_MAX_TRANS (rw) register accessor: Video FIFO AHB Master Maximal Transfer Number Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_max_trans::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_max_trans::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ahbm_max_trans`] module"]
#[doc(alias = "VFIFO_AHBM_MAX_TRANS")]
pub type VfifoAhbmMaxTrans = crate::Reg<vfifo_ahbm_max_trans::VfifoAhbmMaxTransSpec>;
#[doc = "Video FIFO AHB Master Maximal Transfer Number Register."]
pub mod vfifo_ahbm_max_trans;
#[doc = "VFIFO_AHBM_TRANS_CNT (rw) register accessor: Video FIFO AHB Master Transfer Count Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`vfifo_ahbm_trans_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vfifo_ahbm_trans_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vfifo_ahbm_trans_cnt`] module"]
#[doc(alias = "VFIFO_AHBM_TRANS_CNT")]
pub type VfifoAhbmTransCnt = crate::Reg<vfifo_ahbm_trans_cnt::VfifoAhbmTransCntSpec>;
#[doc = "Video FIFO AHB Master Transfer Count Register."]
pub mod vfifo_ahbm_trans_cnt;
#[doc = "RX_EINT_VFF_IE (rw) register accessor: RX Video FIFO Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_vff_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_vff_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_eint_vff_ie`] module"]
#[doc(alias = "RX_EINT_VFF_IE")]
pub type RxEintVffIe = crate::Reg<rx_eint_vff_ie::RxEintVffIeSpec>;
#[doc = "RX Video FIFO Interrupt Enable Register."]
pub mod rx_eint_vff_ie;
#[doc = "RX_EINT_VFF_IF (rw) register accessor: RX Video FIFO Interrupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_vff_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_vff_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_eint_vff_if`] module"]
#[doc(alias = "RX_EINT_VFF_IF")]
pub type RxEintVffIf = crate::Reg<rx_eint_vff_if::RxEintVffIfSpec>;
#[doc = "RX Video FIFO Interrupt Flag Register."]
pub mod rx_eint_vff_if;
#[doc = "RX_EINT_PPI_IE (rw) register accessor: RX D-PHY Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_ppi_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_ppi_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_eint_ppi_ie`] module"]
#[doc(alias = "RX_EINT_PPI_IE")]
pub type RxEintPpiIe = crate::Reg<rx_eint_ppi_ie::RxEintPpiIeSpec>;
#[doc = "RX D-PHY Interrupt Enable Register."]
pub mod rx_eint_ppi_ie;
#[doc = "RX_EINT_PPI_IF (rw) register accessor: RX D-PHY Interrupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_ppi_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_ppi_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_eint_ppi_if`] module"]
#[doc(alias = "RX_EINT_PPI_IF")]
pub type RxEintPpiIf = crate::Reg<rx_eint_ppi_if::RxEintPpiIfSpec>;
#[doc = "RX D-PHY Interrupt Flag Register."]
pub mod rx_eint_ppi_if;
#[doc = "RX_EINT_CTRL_IE (rw) register accessor: RX Controller Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_ctrl_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_ctrl_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_eint_ctrl_ie`] module"]
#[doc(alias = "RX_EINT_CTRL_IE")]
pub type RxEintCtrlIe = crate::Reg<rx_eint_ctrl_ie::RxEintCtrlIeSpec>;
#[doc = "RX Controller Interrupt Enable Register."]
pub mod rx_eint_ctrl_ie;
#[doc = "RX_EINT_CTRL_IF (rw) register accessor: RX Controller Interrupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_eint_ctrl_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_eint_ctrl_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_eint_ctrl_if`] module"]
#[doc(alias = "RX_EINT_CTRL_IF")]
pub type RxEintCtrlIf = crate::Reg<rx_eint_ctrl_if::RxEintCtrlIfSpec>;
#[doc = "RX Controller Interrupt Flag Register."]
pub mod rx_eint_ctrl_if;
#[doc = "PPI_STOPSTATE (rw) register accessor: DPHY PPI Stop State Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_stopstate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_stopstate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_stopstate`] module"]
#[doc(alias = "PPI_STOPSTATE")]
pub type PpiStopstate = crate::Reg<ppi_stopstate::PpiStopstateSpec>;
#[doc = "DPHY PPI Stop State Register."]
pub mod ppi_stopstate;
#[doc = "PPI_TURNAROUND_CFG (rw) register accessor: DPHY PPI Turn-Around Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ppi_turnaround_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppi_turnaround_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppi_turnaround_cfg`] module"]
#[doc(alias = "PPI_TURNAROUND_CFG")]
pub type PpiTurnaroundCfg = crate::Reg<ppi_turnaround_cfg::PpiTurnaroundCfgSpec>;
#[doc = "DPHY PPI Turn-Around Configuration Register."]
pub mod ppi_turnaround_cfg;
