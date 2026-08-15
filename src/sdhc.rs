#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sdma: Sdma,
    blk_size: BlkSize,
    blk_cnt: BlkCnt,
    arg_1: Arg1,
    trans: Trans,
    cmd: Cmd,
    resp: [Resp; 4],
    buffer: Buffer,
    present: Present,
    host_cn_1: HostCn1,
    pwr: Pwr,
    blk_gap: BlkGap,
    wakeup: Wakeup,
    clk_cn: ClkCn,
    to: To,
    sw_reset: SwReset,
    int_stat: IntStat,
    er_int_stat: ErIntStat,
    int_en: IntEn,
    er_int_en: ErIntEn,
    int_signal: IntSignal,
    er_int_signal: ErIntSignal,
    auto_cmd_er: AutoCmdEr,
    host_cn_2: HostCn2,
    cfg_0: Cfg0,
    cfg_1: Cfg1,
    max_curr_cfg: MaxCurrCfg,
    _reserved27: [u8; 0x04],
    force_cmd: ForceCmd,
    force_event_int_stat: ForceEventIntStat,
    adma_er: AdmaEr,
    _reserved30: [u8; 0x03],
    adma_addr_0: AdmaAddr0,
    adma_addr_1: AdmaAddr1,
    preset_0: Preset0,
    preset_1: Preset1,
    preset_2: Preset2,
    preset_3: Preset3,
    preset_4: Preset4,
    preset_5: Preset5,
    preset_6: Preset6,
    preset_7: Preset7,
    _reserved40: [u8; 0x70],
    shared_bus: SharedBus,
    _reserved41: [u8; 0x18],
    slot_int: SlotInt,
    host_cn_ver: HostCnVer,
}
impl RegisterBlock {
    #[doc = "0x00 - SDMA System Address / Argument 2."]
    #[inline(always)]
    pub const fn sdma(&self) -> &Sdma {
        &self.sdma
    }
    #[doc = "0x04 - Block Size."]
    #[inline(always)]
    pub const fn blk_size(&self) -> &BlkSize {
        &self.blk_size
    }
    #[doc = "0x06 - Block Count."]
    #[inline(always)]
    pub const fn blk_cnt(&self) -> &BlkCnt {
        &self.blk_cnt
    }
    #[doc = "0x08 - Argument 1."]
    #[inline(always)]
    pub const fn arg_1(&self) -> &Arg1 {
        &self.arg_1
    }
    #[doc = "0x0c - Transfer Mode."]
    #[inline(always)]
    pub const fn trans(&self) -> &Trans {
        &self.trans
    }
    #[doc = "0x0e - Command."]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10..0x20 - Response 0 Register 0-15."]
    #[inline(always)]
    pub const fn resp(&self, n: usize) -> &Resp {
        &self.resp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Response 0 Register 0-15."]
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = &Resp> {
        self.resp.iter()
    }
    #[doc = "0x20 - Buffer Data Port."]
    #[inline(always)]
    pub const fn buffer(&self) -> &Buffer {
        &self.buffer
    }
    #[doc = "0x24 - Present State."]
    #[inline(always)]
    pub const fn present(&self) -> &Present {
        &self.present
    }
    #[doc = "0x28 - Host Control 1."]
    #[inline(always)]
    pub const fn host_cn_1(&self) -> &HostCn1 {
        &self.host_cn_1
    }
    #[doc = "0x29 - Power Control."]
    #[inline(always)]
    pub const fn pwr(&self) -> &Pwr {
        &self.pwr
    }
    #[doc = "0x2a - Block Gap Control."]
    #[inline(always)]
    pub const fn blk_gap(&self) -> &BlkGap {
        &self.blk_gap
    }
    #[doc = "0x2b - Wakeup Control."]
    #[inline(always)]
    pub const fn wakeup(&self) -> &Wakeup {
        &self.wakeup
    }
    #[doc = "0x2c - Clock Control."]
    #[inline(always)]
    pub const fn clk_cn(&self) -> &ClkCn {
        &self.clk_cn
    }
    #[doc = "0x2e - Timeout Control."]
    #[inline(always)]
    pub const fn to(&self) -> &To {
        &self.to
    }
    #[doc = "0x2f - Software Reset."]
    #[inline(always)]
    pub const fn sw_reset(&self) -> &SwReset {
        &self.sw_reset
    }
    #[doc = "0x30 - Normal Interrupt Status."]
    #[inline(always)]
    pub const fn int_stat(&self) -> &IntStat {
        &self.int_stat
    }
    #[doc = "0x32 - Error Interrupt Status."]
    #[inline(always)]
    pub const fn er_int_stat(&self) -> &ErIntStat {
        &self.er_int_stat
    }
    #[doc = "0x34 - Normal Interrupt Status Enable."]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x36 - Error Interrupt Status Enable."]
    #[inline(always)]
    pub const fn er_int_en(&self) -> &ErIntEn {
        &self.er_int_en
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn int_signal(&self) -> &IntSignal {
        &self.int_signal
    }
    #[doc = "0x3a - Error Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn er_int_signal(&self) -> &ErIntSignal {
        &self.er_int_signal
    }
    #[doc = "0x3c - Auto CMD Error Status."]
    #[inline(always)]
    pub const fn auto_cmd_er(&self) -> &AutoCmdEr {
        &self.auto_cmd_er
    }
    #[doc = "0x3e - Host Control 2."]
    #[inline(always)]
    pub const fn host_cn_2(&self) -> &HostCn2 {
        &self.host_cn_2
    }
    #[doc = "0x40 - Capabilities 0-31."]
    #[inline(always)]
    pub const fn cfg_0(&self) -> &Cfg0 {
        &self.cfg_0
    }
    #[doc = "0x44 - Capabilities 32-63."]
    #[inline(always)]
    pub const fn cfg_1(&self) -> &Cfg1 {
        &self.cfg_1
    }
    #[doc = "0x48 - Maximum Current Capabilities."]
    #[inline(always)]
    pub const fn max_curr_cfg(&self) -> &MaxCurrCfg {
        &self.max_curr_cfg
    }
    #[doc = "0x50 - Force Event for Auto CMD Error Status."]
    #[inline(always)]
    pub const fn force_cmd(&self) -> &ForceCmd {
        &self.force_cmd
    }
    #[doc = "0x52 - Force Event for Error Interrupt Status."]
    #[inline(always)]
    pub const fn force_event_int_stat(&self) -> &ForceEventIntStat {
        &self.force_event_int_stat
    }
    #[doc = "0x54 - ADMA Error Status."]
    #[inline(always)]
    pub const fn adma_er(&self) -> &AdmaEr {
        &self.adma_er
    }
    #[doc = "0x58 - ADMA System Address 0-31."]
    #[inline(always)]
    pub const fn adma_addr_0(&self) -> &AdmaAddr0 {
        &self.adma_addr_0
    }
    #[doc = "0x5c - ADMA System Address 32-63."]
    #[inline(always)]
    pub const fn adma_addr_1(&self) -> &AdmaAddr1 {
        &self.adma_addr_1
    }
    #[doc = "0x60 - Preset Value for Initialization."]
    #[inline(always)]
    pub const fn preset_0(&self) -> &Preset0 {
        &self.preset_0
    }
    #[doc = "0x62 - Preset Value for Default Speed."]
    #[inline(always)]
    pub const fn preset_1(&self) -> &Preset1 {
        &self.preset_1
    }
    #[doc = "0x64 - Preset Value for High Speed."]
    #[inline(always)]
    pub const fn preset_2(&self) -> &Preset2 {
        &self.preset_2
    }
    #[doc = "0x66 - Preset Value for SDR12."]
    #[inline(always)]
    pub const fn preset_3(&self) -> &Preset3 {
        &self.preset_3
    }
    #[doc = "0x68 - Preset Value for SDR25."]
    #[inline(always)]
    pub const fn preset_4(&self) -> &Preset4 {
        &self.preset_4
    }
    #[doc = "0x6a - Preset Value for SDR50."]
    #[inline(always)]
    pub const fn preset_5(&self) -> &Preset5 {
        &self.preset_5
    }
    #[doc = "0x6c - Preset Value for SDR104."]
    #[inline(always)]
    pub const fn preset_6(&self) -> &Preset6 {
        &self.preset_6
    }
    #[doc = "0x6e - Preset Value for DDR50."]
    #[inline(always)]
    pub const fn preset_7(&self) -> &Preset7 {
        &self.preset_7
    }
    #[doc = "0xe0 - SHARED_BUS."]
    #[inline(always)]
    pub const fn shared_bus(&self) -> &SharedBus {
        &self.shared_bus
    }
    #[doc = "0xfc - Slot Interrupt Status."]
    #[inline(always)]
    pub const fn slot_int(&self) -> &SlotInt {
        &self.slot_int
    }
    #[doc = "0xfe - Host Controller Version."]
    #[inline(always)]
    pub const fn host_cn_ver(&self) -> &HostCnVer {
        &self.host_cn_ver
    }
}
#[doc = "SDMA (rw) register accessor: SDMA System Address / Argument 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdma`] module"]
#[doc(alias = "SDMA")]
pub type Sdma = crate::Reg<sdma::SdmaSpec>;
#[doc = "SDMA System Address / Argument 2."]
pub mod sdma;
#[doc = "BLK_SIZE (rw) register accessor: Block Size.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_size`] module"]
#[doc(alias = "BLK_SIZE")]
pub type BlkSize = crate::Reg<blk_size::BlkSizeSpec>;
#[doc = "Block Size."]
pub mod blk_size;
#[doc = "BLK_CNT (rw) register accessor: Block Count.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_cnt`] module"]
#[doc(alias = "BLK_CNT")]
pub type BlkCnt = crate::Reg<blk_cnt::BlkCntSpec>;
#[doc = "Block Count."]
pub mod blk_cnt;
#[doc = "ARG_1 (rw) register accessor: Argument 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`arg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arg_1`] module"]
#[doc(alias = "ARG_1")]
pub type Arg1 = crate::Reg<arg_1::Arg1Spec>;
#[doc = "Argument 1."]
pub mod arg_1;
#[doc = "TRANS (rw) register accessor: Transfer Mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`trans::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trans::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trans`] module"]
#[doc(alias = "TRANS")]
pub type Trans = crate::Reg<trans::TransSpec>;
#[doc = "Transfer Mode."]
pub mod trans;
#[doc = "CMD (rw) register accessor: Command.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command."]
pub mod cmd;
#[doc = "RESP (rw) register accessor: Response 0 Register 0-15.\n\nYou can [`read`](crate::Reg::read) this register and get [`resp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp`] module"]
#[doc(alias = "RESP")]
pub type Resp = crate::Reg<resp::RespSpec>;
#[doc = "Response 0 Register 0-15."]
pub mod resp;
#[doc = "BUFFER (rw) register accessor: Buffer Data Port.\n\nYou can [`read`](crate::Reg::read) this register and get [`buffer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buffer`] module"]
#[doc(alias = "BUFFER")]
pub type Buffer = crate::Reg<buffer::BufferSpec>;
#[doc = "Buffer Data Port."]
pub mod buffer;
#[doc = "PRESENT (r) register accessor: Present State.\n\nYou can [`read`](crate::Reg::read) this register and get [`present::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@present`] module"]
#[doc(alias = "PRESENT")]
pub type Present = crate::Reg<present::PresentSpec>;
#[doc = "Present State."]
pub mod present;
#[doc = "HOST_CN_1 (rw) register accessor: Host Control 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cn_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cn_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cn_1`] module"]
#[doc(alias = "HOST_CN_1")]
pub type HostCn1 = crate::Reg<host_cn_1::HostCn1Spec>;
#[doc = "Host Control 1."]
pub mod host_cn_1;
#[doc = "PWR (rw) register accessor: Power Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr`] module"]
#[doc(alias = "PWR")]
pub type Pwr = crate::Reg<pwr::PwrSpec>;
#[doc = "Power Control."]
pub mod pwr;
#[doc = "BLK_GAP (rw) register accessor: Block Gap Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk_gap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk_gap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_gap`] module"]
#[doc(alias = "BLK_GAP")]
pub type BlkGap = crate::Reg<blk_gap::BlkGapSpec>;
#[doc = "Block Gap Control."]
pub mod blk_gap;
#[doc = "WAKEUP (rw) register accessor: Wakeup Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup`] module"]
#[doc(alias = "WAKEUP")]
pub type Wakeup = crate::Reg<wakeup::WakeupSpec>;
#[doc = "Wakeup Control."]
pub mod wakeup;
#[doc = "CLK_CN (rw) register accessor: Clock Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cn`] module"]
#[doc(alias = "CLK_CN")]
pub type ClkCn = crate::Reg<clk_cn::ClkCnSpec>;
#[doc = "Clock Control."]
pub mod clk_cn;
#[doc = "TO (rw) register accessor: Timeout Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`to::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@to`] module"]
#[doc(alias = "TO")]
pub type To = crate::Reg<to::ToSpec>;
#[doc = "Timeout Control."]
pub mod to;
#[doc = "SW_RESET (rw) register accessor: Software Reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_reset`] module"]
#[doc(alias = "SW_RESET")]
pub type SwReset = crate::Reg<sw_reset::SwResetSpec>;
#[doc = "Software Reset."]
pub mod sw_reset;
#[doc = "INT_STAT (rw) register accessor: Normal Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_stat`] module"]
#[doc(alias = "INT_STAT")]
pub type IntStat = crate::Reg<int_stat::IntStatSpec>;
#[doc = "Normal Interrupt Status."]
pub mod int_stat;
#[doc = "ER_INT_STAT (rw) register accessor: Error Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_int_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_int_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er_int_stat`] module"]
#[doc(alias = "ER_INT_STAT")]
pub type ErIntStat = crate::Reg<er_int_stat::ErIntStatSpec>;
#[doc = "Error Interrupt Status."]
pub mod er_int_stat;
#[doc = "INT_EN (rw) register accessor: Normal Interrupt Status Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`] module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "Normal Interrupt Status Enable."]
pub mod int_en;
#[doc = "ER_INT_EN (rw) register accessor: Error Interrupt Status Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er_int_en`] module"]
#[doc(alias = "ER_INT_EN")]
pub type ErIntEn = crate::Reg<er_int_en::ErIntEnSpec>;
#[doc = "Error Interrupt Status Enable."]
pub mod er_int_en;
#[doc = "INT_SIGNAL (rw) register accessor: Normal Interrupt Signal Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_signal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_signal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_signal`] module"]
#[doc(alias = "INT_SIGNAL")]
pub type IntSignal = crate::Reg<int_signal::IntSignalSpec>;
#[doc = "Normal Interrupt Signal Enable."]
pub mod int_signal;
#[doc = "ER_INT_SIGNAL (rw) register accessor: Error Interrupt Signal Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`er_int_signal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`er_int_signal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@er_int_signal`] module"]
#[doc(alias = "ER_INT_SIGNAL")]
pub type ErIntSignal = crate::Reg<er_int_signal::ErIntSignalSpec>;
#[doc = "Error Interrupt Signal Enable."]
pub mod er_int_signal;
#[doc = "AUTO_CMD_ER (rw) register accessor: Auto CMD Error Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`auto_cmd_er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`auto_cmd_er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_cmd_er`] module"]
#[doc(alias = "AUTO_CMD_ER")]
pub type AutoCmdEr = crate::Reg<auto_cmd_er::AutoCmdErSpec>;
#[doc = "Auto CMD Error Status."]
pub mod auto_cmd_er;
#[doc = "HOST_CN_2 (rw) register accessor: Host Control 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cn_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cn_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cn_2`] module"]
#[doc(alias = "HOST_CN_2")]
pub type HostCn2 = crate::Reg<host_cn_2::HostCn2Spec>;
#[doc = "Host Control 2."]
pub mod host_cn_2;
#[doc = "CFG_0 (r) register accessor: Capabilities 0-31.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_0`] module"]
#[doc(alias = "CFG_0")]
pub type Cfg0 = crate::Reg<cfg_0::Cfg0Spec>;
#[doc = "Capabilities 0-31."]
pub mod cfg_0;
#[doc = "CFG_1 (r) register accessor: Capabilities 32-63.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_1`] module"]
#[doc(alias = "CFG_1")]
pub type Cfg1 = crate::Reg<cfg_1::Cfg1Spec>;
#[doc = "Capabilities 32-63."]
pub mod cfg_1;
#[doc = "MAX_CURR_CFG (r) register accessor: Maximum Current Capabilities.\n\nYou can [`read`](crate::Reg::read) this register and get [`max_curr_cfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_curr_cfg`] module"]
#[doc(alias = "MAX_CURR_CFG")]
pub type MaxCurrCfg = crate::Reg<max_curr_cfg::MaxCurrCfgSpec>;
#[doc = "Maximum Current Capabilities."]
pub mod max_curr_cfg;
#[doc = "FORCE_CMD (w) register accessor: Force Event for Auto CMD Error Status.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_cmd`] module"]
#[doc(alias = "FORCE_CMD")]
pub type ForceCmd = crate::Reg<force_cmd::ForceCmdSpec>;
#[doc = "Force Event for Auto CMD Error Status."]
pub mod force_cmd;
#[doc = "FORCE_EVENT_INT_STAT (rw) register accessor: Force Event for Error Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`force_event_int_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_event_int_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_event_int_stat`] module"]
#[doc(alias = "FORCE_EVENT_INT_STAT")]
pub type ForceEventIntStat = crate::Reg<force_event_int_stat::ForceEventIntStatSpec>;
#[doc = "Force Event for Error Interrupt Status."]
pub mod force_event_int_stat;
#[doc = "ADMA_ER (rw) register accessor: ADMA Error Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_er::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_er::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_er`] module"]
#[doc(alias = "ADMA_ER")]
pub type AdmaEr = crate::Reg<adma_er::AdmaErSpec>;
#[doc = "ADMA Error Status."]
pub mod adma_er;
#[doc = "ADMA_ADDR_0 (rw) register accessor: ADMA System Address 0-31.\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_addr_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_addr_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_addr_0`] module"]
#[doc(alias = "ADMA_ADDR_0")]
pub type AdmaAddr0 = crate::Reg<adma_addr_0::AdmaAddr0Spec>;
#[doc = "ADMA System Address 0-31."]
pub mod adma_addr_0;
#[doc = "ADMA_ADDR_1 (rw) register accessor: ADMA System Address 32-63.\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_addr_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_addr_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_addr_1`] module"]
#[doc(alias = "ADMA_ADDR_1")]
pub type AdmaAddr1 = crate::Reg<adma_addr_1::AdmaAddr1Spec>;
#[doc = "ADMA System Address 32-63."]
pub mod adma_addr_1;
#[doc = "PRESET_0 (r) register accessor: Preset Value for Initialization.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_0`] module"]
#[doc(alias = "PRESET_0")]
pub type Preset0 = crate::Reg<preset_0::Preset0Spec>;
#[doc = "Preset Value for Initialization."]
pub mod preset_0;
#[doc = "PRESET_1 (r) register accessor: Preset Value for Default Speed.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_1`] module"]
#[doc(alias = "PRESET_1")]
pub type Preset1 = crate::Reg<preset_1::Preset1Spec>;
#[doc = "Preset Value for Default Speed."]
pub mod preset_1;
#[doc = "PRESET_2 (r) register accessor: Preset Value for High Speed.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_2`] module"]
#[doc(alias = "PRESET_2")]
pub type Preset2 = crate::Reg<preset_2::Preset2Spec>;
#[doc = "Preset Value for High Speed."]
pub mod preset_2;
#[doc = "PRESET_3 (r) register accessor: Preset Value for SDR12.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_3`] module"]
#[doc(alias = "PRESET_3")]
pub type Preset3 = crate::Reg<preset_3::Preset3Spec>;
#[doc = "Preset Value for SDR12."]
pub mod preset_3;
#[doc = "PRESET_4 (r) register accessor: Preset Value for SDR25.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_4`] module"]
#[doc(alias = "PRESET_4")]
pub type Preset4 = crate::Reg<preset_4::Preset4Spec>;
#[doc = "Preset Value for SDR25."]
pub mod preset_4;
#[doc = "PRESET_5 (r) register accessor: Preset Value for SDR50.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_5`] module"]
#[doc(alias = "PRESET_5")]
pub type Preset5 = crate::Reg<preset_5::Preset5Spec>;
#[doc = "Preset Value for SDR50."]
pub mod preset_5;
#[doc = "PRESET_6 (r) register accessor: Preset Value for SDR104.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_6`] module"]
#[doc(alias = "PRESET_6")]
pub type Preset6 = crate::Reg<preset_6::Preset6Spec>;
#[doc = "Preset Value for SDR104."]
pub mod preset_6;
#[doc = "PRESET_7 (r) register accessor: Preset Value for DDR50.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset_7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset_7`] module"]
#[doc(alias = "PRESET_7")]
pub type Preset7 = crate::Reg<preset_7::Preset7Spec>;
#[doc = "Preset Value for DDR50."]
pub mod preset_7;
#[doc = "SHARED_BUS (rw) register accessor: SHARED_BUS.\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_bus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_bus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shared_bus`] module"]
#[doc(alias = "SHARED_BUS")]
pub type SharedBus = crate::Reg<shared_bus::SharedBusSpec>;
#[doc = "SHARED_BUS."]
pub mod shared_bus;
#[doc = "SLOT_INT (r) register accessor: Slot Interrupt Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`slot_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slot_int`] module"]
#[doc(alias = "SLOT_INT")]
pub type SlotInt = crate::Reg<slot_int::SlotIntSpec>;
#[doc = "Slot Interrupt Status."]
pub mod slot_int;
#[doc = "HOST_CN_VER (rw) register accessor: Host Controller Version.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cn_ver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_cn_ver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cn_ver`] module"]
#[doc(alias = "HOST_CN_VER")]
pub type HostCnVer = crate::Reg<host_cn_ver::HostCnVerSpec>;
#[doc = "Host Controller Version."]
pub mod host_cn_ver;
