#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    addr: Addr,
    clkdiv: Clkdiv,
    ctrl: Ctrl,
    _reserved3: [u8; 0x18],
    intr: Intr,
    _reserved4: [u8; 0x08],
    data: [Data; 4],
    actrl: Actrl,
    _reserved6: [u8; 0x16],
    welr2: Welr2,
    _reserved7: [u8; 0x22],
    welr0: Welr0,
    rlr0: Rlr0,
    welr1: Welr1,
    rlr1: Rlr1,
    _reserved11: [u8; 0x04],
    rlr2: Rlr2,
    welr3: Welr3,
    rlr3: Rlr3,
    welr4: Welr4,
    rlr4: Rlr4,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Write Address."]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller."]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x08 - Flash Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x24 - Flash Interrupt Register."]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x30..0x40 - Flash Write Data."]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Flash Write Data."]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x40 - Access Control Register. Writing the ACTRL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-actrl = 0x3a7f5ca3; pflc-actrl = 0xa1e34f20; pflc-actrl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero."]
    #[inline(always)]
    pub const fn actrl(&self) -> &Actrl {
        &self.actrl
    }
    #[doc = "0x5a - WELR2"]
    #[inline(always)]
    pub const fn welr2(&self) -> &Welr2 {
        &self.welr2
    }
    #[doc = "0x80 - WELR0"]
    #[inline(always)]
    pub const fn welr0(&self) -> &Welr0 {
        &self.welr0
    }
    #[doc = "0x84 - RLR0"]
    #[inline(always)]
    pub const fn rlr0(&self) -> &Rlr0 {
        &self.rlr0
    }
    #[doc = "0x88 - WELR1"]
    #[inline(always)]
    pub const fn welr1(&self) -> &Welr1 {
        &self.welr1
    }
    #[doc = "0x8c - RLR1"]
    #[inline(always)]
    pub const fn rlr1(&self) -> &Rlr1 {
        &self.rlr1
    }
    #[doc = "0x94 - RLR2"]
    #[inline(always)]
    pub const fn rlr2(&self) -> &Rlr2 {
        &self.rlr2
    }
    #[doc = "0x98 - WELR3"]
    #[inline(always)]
    pub const fn welr3(&self) -> &Welr3 {
        &self.welr3
    }
    #[doc = "0x9c - RLR3"]
    #[inline(always)]
    pub const fn rlr3(&self) -> &Rlr3 {
        &self.rlr3
    }
    #[doc = "0xa0 - WELR4"]
    #[inline(always)]
    pub const fn welr4(&self) -> &Welr4 {
        &self.welr4
    }
    #[doc = "0xa4 - RLR4"]
    #[inline(always)]
    pub const fn rlr4(&self) -> &Rlr4 {
        &self.rlr4
    }
}
#[doc = "ADDR (rw) register accessor: Flash Write Address.\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Flash Write Address."]
pub mod addr;
#[doc = "CLKDIV (rw) register accessor: Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller."]
pub mod clkdiv;
#[doc = "CTRL (rw) register accessor: Flash Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Flash Control Register."]
pub mod ctrl;
#[doc = "INTR (rw) register accessor: Flash Interrupt Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`] module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Flash Interrupt Register."]
pub mod intr;
#[doc = "DATA (rw) register accessor: Flash Write Data.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Flash Write Data."]
pub mod data;
#[doc = "ACTRL (w) register accessor: Access Control Register. Writing the ACTRL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-actrl = 0x3a7f5ca3; pflc-actrl = 0xa1e34f20; pflc-actrl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@actrl`] module"]
#[doc(alias = "ACTRL")]
pub type Actrl = crate::Reg<actrl::ActrlSpec>;
#[doc = "Access Control Register. Writing the ACTRL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-actrl = 0x3a7f5ca3; pflc-actrl = 0xa1e34f20; pflc-actrl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero."]
pub mod actrl;
#[doc = "WELR0 (rw) register accessor: WELR0\n\nYou can [`read`](crate::Reg::read) this register and get [`welr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@welr0`] module"]
#[doc(alias = "WELR0")]
pub type Welr0 = crate::Reg<welr0::Welr0Spec>;
#[doc = "WELR0"]
pub mod welr0;
#[doc = "RLR0 (rw) register accessor: RLR0\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr0`] module"]
#[doc(alias = "RLR0")]
pub type Rlr0 = crate::Reg<rlr0::Rlr0Spec>;
#[doc = "RLR0"]
pub mod rlr0;
#[doc = "WELR1 (rw) register accessor: WELR1\n\nYou can [`read`](crate::Reg::read) this register and get [`welr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@welr1`] module"]
#[doc(alias = "WELR1")]
pub type Welr1 = crate::Reg<welr1::Welr1Spec>;
#[doc = "WELR1"]
pub mod welr1;
#[doc = "RLR1 (rw) register accessor: RLR1\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr1`] module"]
#[doc(alias = "RLR1")]
pub type Rlr1 = crate::Reg<rlr1::Rlr1Spec>;
#[doc = "RLR1"]
pub mod rlr1;
#[doc = "WELR2 (rw) register accessor: WELR2\n\nYou can [`read`](crate::Reg::read) this register and get [`welr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@welr2`] module"]
#[doc(alias = "WELR2")]
pub type Welr2 = crate::Reg<welr2::Welr2Spec>;
#[doc = "WELR2"]
pub mod welr2;
#[doc = "RLR2 (rw) register accessor: RLR2\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr2`] module"]
#[doc(alias = "RLR2")]
pub type Rlr2 = crate::Reg<rlr2::Rlr2Spec>;
#[doc = "RLR2"]
pub mod rlr2;
#[doc = "WELR3 (rw) register accessor: WELR3\n\nYou can [`read`](crate::Reg::read) this register and get [`welr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@welr3`] module"]
#[doc(alias = "WELR3")]
pub type Welr3 = crate::Reg<welr3::Welr3Spec>;
#[doc = "WELR3"]
pub mod welr3;
#[doc = "RLR3 (rw) register accessor: RLR3\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr3`] module"]
#[doc(alias = "RLR3")]
pub type Rlr3 = crate::Reg<rlr3::Rlr3Spec>;
#[doc = "RLR3"]
pub mod rlr3;
#[doc = "WELR4 (rw) register accessor: WELR4\n\nYou can [`read`](crate::Reg::read) this register and get [`welr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`welr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@welr4`] module"]
#[doc(alias = "WELR4")]
pub type Welr4 = crate::Reg<welr4::Welr4Spec>;
#[doc = "WELR4"]
pub mod welr4;
#[doc = "RLR4 (rw) register accessor: RLR4\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr4`] module"]
#[doc(alias = "RLR4")]
pub type Rlr4 = crate::Reg<rlr4::Rlr4Spec>;
#[doc = "RLR4"]
pub mod rlr4;
