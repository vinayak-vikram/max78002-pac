#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    clkctrl: Clkctrl,
    sampclkctrl: Sampclkctrl,
    chsel0: Chsel0,
    chsel1: Chsel1,
    chsel2: Chsel2,
    chsel3: Chsel3,
    chsel4: Chsel4,
    chsel5: Chsel5,
    chsel6: Chsel6,
    chsel7: Chsel7,
    restart: Restart,
    _reserved13: [u8; 0x08],
    datafmt: Datafmt,
    fifodmactrl: Fifodmactrl,
    data: Data,
    status: Status,
    chstatus: Chstatus,
    inten: Inten,
    intfl: Intfl,
    _reserved20: [u8; 0x08],
    sfraddroffset: Sfraddroffset,
    sfraddr: Sfraddr,
    sfrwrdata: Sfrwrdata,
    sfrrddata: Sfrrddata,
    sfrstatus: Sfrstatus,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 0."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x04 - Control Register 1."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - Clock Control Register."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x0c - Sample Clock Control Register."]
    #[inline(always)]
    pub const fn sampclkctrl(&self) -> &Sampclkctrl {
        &self.sampclkctrl
    }
    #[doc = "0x10 - Channel Select Register 0."]
    #[inline(always)]
    pub const fn chsel0(&self) -> &Chsel0 {
        &self.chsel0
    }
    #[doc = "0x14 - Channel Select Register 1."]
    #[inline(always)]
    pub const fn chsel1(&self) -> &Chsel1 {
        &self.chsel1
    }
    #[doc = "0x18 - Channel Select Register 2."]
    #[inline(always)]
    pub const fn chsel2(&self) -> &Chsel2 {
        &self.chsel2
    }
    #[doc = "0x1c - Channel Select Register 3."]
    #[inline(always)]
    pub const fn chsel3(&self) -> &Chsel3 {
        &self.chsel3
    }
    #[doc = "0x20 - Channel Select Register 4."]
    #[inline(always)]
    pub const fn chsel4(&self) -> &Chsel4 {
        &self.chsel4
    }
    #[doc = "0x24 - Channel Select Register 5."]
    #[inline(always)]
    pub const fn chsel5(&self) -> &Chsel5 {
        &self.chsel5
    }
    #[doc = "0x28 - Channel Select Register 6."]
    #[inline(always)]
    pub const fn chsel6(&self) -> &Chsel6 {
        &self.chsel6
    }
    #[doc = "0x2c - Channel Select Register 7."]
    #[inline(always)]
    pub const fn chsel7(&self) -> &Chsel7 {
        &self.chsel7
    }
    #[doc = "0x30 - Restart Count Control Register"]
    #[inline(always)]
    pub const fn restart(&self) -> &Restart {
        &self.restart
    }
    #[doc = "0x3c - Channel Data Format Register"]
    #[inline(always)]
    pub const fn datafmt(&self) -> &Datafmt {
        &self.datafmt
    }
    #[doc = "0x40 - FIFO and DMA control"]
    #[inline(always)]
    pub const fn fifodmactrl(&self) -> &Fifodmactrl {
        &self.fifodmactrl
    }
    #[doc = "0x44 - Data Register (FIFO)."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x48 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x4c - Channel Status"]
    #[inline(always)]
    pub const fn chstatus(&self) -> &Chstatus {
        &self.chstatus
    }
    #[doc = "0x50 - Interrupt Enable Register."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x54 - Interrupt Flags Register."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x60 - SFR Address Offset Register"]
    #[inline(always)]
    pub const fn sfraddroffset(&self) -> &Sfraddroffset {
        &self.sfraddroffset
    }
    #[doc = "0x64 - SFR Address Register"]
    #[inline(always)]
    pub const fn sfraddr(&self) -> &Sfraddr {
        &self.sfraddr
    }
    #[doc = "0x68 - SFR Write Data Register"]
    #[inline(always)]
    pub const fn sfrwrdata(&self) -> &Sfrwrdata {
        &self.sfrwrdata
    }
    #[doc = "0x6c - SFR Read Data Register"]
    #[inline(always)]
    pub const fn sfrrddata(&self) -> &Sfrrddata {
        &self.sfrrddata
    }
    #[doc = "0x70 - SFR Status Register"]
    #[inline(always)]
    pub const fn sfrstatus(&self) -> &Sfrstatus {
        &self.sfrstatus
    }
}
#[doc = "CTRL0 (rw) register accessor: Control Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`] module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "Control Register 0."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Control Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control Register 1."]
pub mod ctrl1;
#[doc = "CLKCTRL (rw) register accessor: Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`] module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock Control Register."]
pub mod clkctrl;
#[doc = "SAMPCLKCTRL (rw) register accessor: Sample Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sampclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampclkctrl`] module"]
#[doc(alias = "SAMPCLKCTRL")]
pub type Sampclkctrl = crate::Reg<sampclkctrl::SampclkctrlSpec>;
#[doc = "Sample Clock Control Register."]
pub mod sampclkctrl;
#[doc = "CHSEL0 (rw) register accessor: Channel Select Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel0`] module"]
#[doc(alias = "CHSEL0")]
pub type Chsel0 = crate::Reg<chsel0::Chsel0Spec>;
#[doc = "Channel Select Register 0."]
pub mod chsel0;
#[doc = "CHSEL1 (rw) register accessor: Channel Select Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel1`] module"]
#[doc(alias = "CHSEL1")]
pub type Chsel1 = crate::Reg<chsel1::Chsel1Spec>;
#[doc = "Channel Select Register 1."]
pub mod chsel1;
#[doc = "CHSEL2 (rw) register accessor: Channel Select Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel2`] module"]
#[doc(alias = "CHSEL2")]
pub type Chsel2 = crate::Reg<chsel2::Chsel2Spec>;
#[doc = "Channel Select Register 2."]
pub mod chsel2;
#[doc = "CHSEL3 (rw) register accessor: Channel Select Register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel3`] module"]
#[doc(alias = "CHSEL3")]
pub type Chsel3 = crate::Reg<chsel3::Chsel3Spec>;
#[doc = "Channel Select Register 3."]
pub mod chsel3;
#[doc = "CHSEL4 (rw) register accessor: Channel Select Register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel4`] module"]
#[doc(alias = "CHSEL4")]
pub type Chsel4 = crate::Reg<chsel4::Chsel4Spec>;
#[doc = "Channel Select Register 4."]
pub mod chsel4;
#[doc = "CHSEL5 (rw) register accessor: Channel Select Register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel5`] module"]
#[doc(alias = "CHSEL5")]
pub type Chsel5 = crate::Reg<chsel5::Chsel5Spec>;
#[doc = "Channel Select Register 5."]
pub mod chsel5;
#[doc = "CHSEL6 (rw) register accessor: Channel Select Register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel6`] module"]
#[doc(alias = "CHSEL6")]
pub type Chsel6 = crate::Reg<chsel6::Chsel6Spec>;
#[doc = "Channel Select Register 6."]
pub mod chsel6;
#[doc = "CHSEL7 (rw) register accessor: Channel Select Register 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`chsel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chsel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsel7`] module"]
#[doc(alias = "CHSEL7")]
pub type Chsel7 = crate::Reg<chsel7::Chsel7Spec>;
#[doc = "Channel Select Register 7."]
pub mod chsel7;
#[doc = "RESTART (rw) register accessor: Restart Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`restart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restart`] module"]
#[doc(alias = "RESTART")]
pub type Restart = crate::Reg<restart::RestartSpec>;
#[doc = "Restart Count Control Register"]
pub mod restart;
#[doc = "DATAFMT (rw) register accessor: Channel Data Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datafmt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datafmt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datafmt`] module"]
#[doc(alias = "DATAFMT")]
pub type Datafmt = crate::Reg<datafmt::DatafmtSpec>;
#[doc = "Channel Data Format Register"]
pub mod datafmt;
#[doc = "FIFODMACTRL (rw) register accessor: FIFO and DMA control\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifodmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodmactrl`] module"]
#[doc(alias = "FIFODMACTRL")]
pub type Fifodmactrl = crate::Reg<fifodmactrl::FifodmactrlSpec>;
#[doc = "FIFO and DMA control"]
pub mod fifodmactrl;
#[doc = "DATA (rw) register accessor: Data Register (FIFO).\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data Register (FIFO)."]
pub mod data;
#[doc = "STATUS (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "CHSTATUS (rw) register accessor: Channel Status\n\nYou can [`read`](crate::Reg::read) this register and get [`chstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chstatus`] module"]
#[doc(alias = "CHSTATUS")]
pub type Chstatus = crate::Reg<chstatus::ChstatusSpec>;
#[doc = "Channel Status"]
pub mod chstatus;
#[doc = "INTEN (rw) register accessor: Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable Register."]
pub mod inten;
#[doc = "INTFL (rw) register accessor: Interrupt Flags Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`] module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "Interrupt Flags Register."]
pub mod intfl;
#[doc = "SFRADDROFFSET (rw) register accessor: SFR Address Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfraddroffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfraddroffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfraddroffset`] module"]
#[doc(alias = "SFRADDROFFSET")]
pub type Sfraddroffset = crate::Reg<sfraddroffset::SfraddroffsetSpec>;
#[doc = "SFR Address Offset Register"]
pub mod sfraddroffset;
#[doc = "SFRADDR (rw) register accessor: SFR Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfraddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfraddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfraddr`] module"]
#[doc(alias = "SFRADDR")]
pub type Sfraddr = crate::Reg<sfraddr::SfraddrSpec>;
#[doc = "SFR Address Register"]
pub mod sfraddr;
#[doc = "SFRWRDATA (rw) register accessor: SFR Write Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrwrdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrwrdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrwrdata`] module"]
#[doc(alias = "SFRWRDATA")]
pub type Sfrwrdata = crate::Reg<sfrwrdata::SfrwrdataSpec>;
#[doc = "SFR Write Data Register"]
pub mod sfrwrdata;
#[doc = "SFRRDDATA (rw) register accessor: SFR Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrrddata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrrddata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrrddata`] module"]
#[doc(alias = "SFRRDDATA")]
pub type Sfrrddata = crate::Reg<sfrrddata::SfrrddataSpec>;
#[doc = "SFR Read Data Register"]
pub mod sfrrddata;
#[doc = "SFRSTATUS (rw) register accessor: SFR Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfrstatus`] module"]
#[doc(alias = "SFRSTATUS")]
pub type Sfrstatus = crate::Reg<sfrstatus::SfrstatusSpec>;
#[doc = "SFR Status Register"]
pub mod sfrstatus;
