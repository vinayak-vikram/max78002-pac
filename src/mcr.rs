#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eccen: Eccen,
    ipo_mtrim: IpoMtrim,
    outen: Outen,
    cmp_ctrl: CmpCtrl,
    ctrl: Ctrl,
    _reserved5: [u8; 0x0c],
    gpio3_ctrl: Gpio3Ctrl,
    _reserved6: [u8; 0x1c],
    cwd0: Cwd0,
    cwd1: Cwd1,
    _reserved8: [u8; 0x08],
    adccfg0: Adccfg0,
    adccfg1: Adccfg1,
    adccfg2: Adccfg2,
    _reserved11: [u8; 0x04],
    ldoctrl: Ldoctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - ECC Enable Register"]
    #[inline(always)]
    pub const fn eccen(&self) -> &Eccen {
        &self.eccen
    }
    #[doc = "0x04 - IPO Manual Register"]
    #[inline(always)]
    pub const fn ipo_mtrim(&self) -> &IpoMtrim {
        &self.ipo_mtrim
    }
    #[doc = "0x08 - Output Enable Register"]
    #[inline(always)]
    pub const fn outen(&self) -> &Outen {
        &self.outen
    }
    #[doc = "0x0c - Comparator Control Register."]
    #[inline(always)]
    pub const fn cmp_ctrl(&self) -> &CmpCtrl {
        &self.cmp_ctrl
    }
    #[doc = "0x10 - Miscellaneous Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x20 - GPIO3 Pin Control Register."]
    #[inline(always)]
    pub const fn gpio3_ctrl(&self) -> &Gpio3Ctrl {
        &self.gpio3_ctrl
    }
    #[doc = "0x40 - Code Word Data0"]
    #[inline(always)]
    pub const fn cwd0(&self) -> &Cwd0 {
        &self.cwd0
    }
    #[doc = "0x44 - Code Word Data1"]
    #[inline(always)]
    pub const fn cwd1(&self) -> &Cwd1 {
        &self.cwd1
    }
    #[doc = "0x50 - ADC Config 0"]
    #[inline(always)]
    pub const fn adccfg0(&self) -> &Adccfg0 {
        &self.adccfg0
    }
    #[doc = "0x54 - ADC Config 1"]
    #[inline(always)]
    pub const fn adccfg1(&self) -> &Adccfg1 {
        &self.adccfg1
    }
    #[doc = "0x58 - ADC Config 2"]
    #[inline(always)]
    pub const fn adccfg2(&self) -> &Adccfg2 {
        &self.adccfg2
    }
    #[doc = "0x60 - LDO Control"]
    #[inline(always)]
    pub const fn ldoctrl(&self) -> &Ldoctrl {
        &self.ldoctrl
    }
}
#[doc = "ECCEN (rw) register accessor: ECC Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccen`] module"]
#[doc(alias = "ECCEN")]
pub type Eccen = crate::Reg<eccen::EccenSpec>;
#[doc = "ECC Enable Register"]
pub mod eccen;
#[doc = "IPO_MTRIM (rw) register accessor: IPO Manual Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipo_mtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipo_mtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipo_mtrim`] module"]
#[doc(alias = "IPO_MTRIM")]
pub type IpoMtrim = crate::Reg<ipo_mtrim::IpoMtrimSpec>;
#[doc = "IPO Manual Register"]
pub mod ipo_mtrim;
#[doc = "OUTEN (rw) register accessor: Output Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outen`] module"]
#[doc(alias = "OUTEN")]
pub type Outen = crate::Reg<outen::OutenSpec>;
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "CMP_CTRL (rw) register accessor: Comparator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ctrl`] module"]
#[doc(alias = "CMP_CTRL")]
pub type CmpCtrl = crate::Reg<cmp_ctrl::CmpCtrlSpec>;
#[doc = "Comparator Control Register."]
pub mod cmp_ctrl;
#[doc = "CTRL (rw) register accessor: Miscellaneous Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Miscellaneous Control Register."]
pub mod ctrl;
#[doc = "GPIO3_CTRL (rw) register accessor: GPIO3 Pin Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3_ctrl`] module"]
#[doc(alias = "GPIO3_CTRL")]
pub type Gpio3Ctrl = crate::Reg<gpio3_ctrl::Gpio3CtrlSpec>;
#[doc = "GPIO3 Pin Control Register."]
pub mod gpio3_ctrl;
#[doc = "CWD0 (rw) register accessor: Code Word Data0\n\nYou can [`read`](crate::Reg::read) this register and get [`cwd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwd0`] module"]
#[doc(alias = "CWD0")]
pub type Cwd0 = crate::Reg<cwd0::Cwd0Spec>;
#[doc = "Code Word Data0"]
pub mod cwd0;
#[doc = "CWD1 (rw) register accessor: Code Word Data1\n\nYou can [`read`](crate::Reg::read) this register and get [`cwd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwd1`] module"]
#[doc(alias = "CWD1")]
pub type Cwd1 = crate::Reg<cwd1::Cwd1Spec>;
#[doc = "Code Word Data1"]
pub mod cwd1;
#[doc = "ADCCFG0 (rw) register accessor: ADC Config 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccfg0`] module"]
#[doc(alias = "ADCCFG0")]
pub type Adccfg0 = crate::Reg<adccfg0::Adccfg0Spec>;
#[doc = "ADC Config 0"]
pub mod adccfg0;
#[doc = "ADCCFG1 (rw) register accessor: ADC Config 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccfg1`] module"]
#[doc(alias = "ADCCFG1")]
pub type Adccfg1 = crate::Reg<adccfg1::Adccfg1Spec>;
#[doc = "ADC Config 1"]
pub mod adccfg1;
#[doc = "ADCCFG2 (rw) register accessor: ADC Config 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccfg2`] module"]
#[doc(alias = "ADCCFG2")]
pub type Adccfg2 = crate::Reg<adccfg2::Adccfg2Spec>;
#[doc = "ADC Config 2"]
pub mod adccfg2;
#[doc = "LDOCTRL (rw) register accessor: LDO Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldoctrl`] module"]
#[doc(alias = "LDOCTRL")]
pub type Ldoctrl = crate::Reg<ldoctrl::LdoctrlSpec>;
#[doc = "LDO Control"]
pub mod ldoctrl;
