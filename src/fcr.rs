#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fctrl0: Fctrl0,
    autocal0: Autocal0,
    autocal1: Autocal1,
    autocal2: Autocal2,
    urvbootaddr: Urvbootaddr,
    urvctrl: Urvctrl,
    xo32mks: Xo32mks,
    _reserved7: [u8; 0x04],
    ts0: Ts0,
    ts1: Ts1,
    adcreftrim0: Adcreftrim0,
    adcreftrim1: Adcreftrim1,
    adcreftrim2: Adcreftrim2,
}
impl RegisterBlock {
    #[doc = "0x00 - Function Control 0."]
    #[inline(always)]
    pub const fn fctrl0(&self) -> &Fctrl0 {
        &self.fctrl0
    }
    #[doc = "0x04 - Automatic Calibration 0."]
    #[inline(always)]
    pub const fn autocal0(&self) -> &Autocal0 {
        &self.autocal0
    }
    #[doc = "0x08 - Automatic Calibration 1."]
    #[inline(always)]
    pub const fn autocal1(&self) -> &Autocal1 {
        &self.autocal1
    }
    #[doc = "0x0c - Automatic Calibration 2"]
    #[inline(always)]
    pub const fn autocal2(&self) -> &Autocal2 {
        &self.autocal2
    }
    #[doc = "0x10 - RISC-V Boot Address."]
    #[inline(always)]
    pub const fn urvbootaddr(&self) -> &Urvbootaddr {
        &self.urvbootaddr
    }
    #[doc = "0x14 - RISC-V Control Register."]
    #[inline(always)]
    pub const fn urvctrl(&self) -> &Urvctrl {
        &self.urvctrl
    }
    #[doc = "0x18 - RISC-V Control Register."]
    #[inline(always)]
    pub const fn xo32mks(&self) -> &Xo32mks {
        &self.xo32mks
    }
    #[doc = "0x20 - Temp Sensor trim0"]
    #[inline(always)]
    pub const fn ts0(&self) -> &Ts0 {
        &self.ts0
    }
    #[doc = "0x24 - Temp Sensor trim1"]
    #[inline(always)]
    pub const fn ts1(&self) -> &Ts1 {
        &self.ts1
    }
    #[doc = "0x28 - Temp Sensor trim1"]
    #[inline(always)]
    pub const fn adcreftrim0(&self) -> &Adcreftrim0 {
        &self.adcreftrim0
    }
    #[doc = "0x2c - Temp Sensor trim1"]
    #[inline(always)]
    pub const fn adcreftrim1(&self) -> &Adcreftrim1 {
        &self.adcreftrim1
    }
    #[doc = "0x30 - Temp Sensor trim1"]
    #[inline(always)]
    pub const fn adcreftrim2(&self) -> &Adcreftrim2 {
        &self.adcreftrim2
    }
}
#[doc = "FCTRL0 (rw) register accessor: Function Control 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl0`] module"]
#[doc(alias = "FCTRL0")]
pub type Fctrl0 = crate::Reg<fctrl0::Fctrl0Spec>;
#[doc = "Function Control 0."]
pub mod fctrl0;
#[doc = "AUTOCAL0 (rw) register accessor: Automatic Calibration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal0`] module"]
#[doc(alias = "AUTOCAL0")]
pub type Autocal0 = crate::Reg<autocal0::Autocal0Spec>;
#[doc = "Automatic Calibration 0."]
pub mod autocal0;
#[doc = "AUTOCAL1 (rw) register accessor: Automatic Calibration 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal1`] module"]
#[doc(alias = "AUTOCAL1")]
pub type Autocal1 = crate::Reg<autocal1::Autocal1Spec>;
#[doc = "Automatic Calibration 1."]
pub mod autocal1;
#[doc = "AUTOCAL2 (rw) register accessor: Automatic Calibration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal2`] module"]
#[doc(alias = "AUTOCAL2")]
pub type Autocal2 = crate::Reg<autocal2::Autocal2Spec>;
#[doc = "Automatic Calibration 2"]
pub mod autocal2;
#[doc = "URVBOOTADDR (rw) register accessor: RISC-V Boot Address.\n\nYou can [`read`](crate::Reg::read) this register and get [`urvbootaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urvbootaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urvbootaddr`] module"]
#[doc(alias = "URVBOOTADDR")]
pub type Urvbootaddr = crate::Reg<urvbootaddr::UrvbootaddrSpec>;
#[doc = "RISC-V Boot Address."]
pub mod urvbootaddr;
#[doc = "URVCTRL (rw) register accessor: RISC-V Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`urvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urvctrl`] module"]
#[doc(alias = "URVCTRL")]
pub type Urvctrl = crate::Reg<urvctrl::UrvctrlSpec>;
#[doc = "RISC-V Control Register."]
pub mod urvctrl;
#[doc = "XO32MKS (rw) register accessor: RISC-V Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`xo32mks::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xo32mks::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xo32mks`] module"]
#[doc(alias = "XO32MKS")]
pub type Xo32mks = crate::Reg<xo32mks::Xo32mksSpec>;
#[doc = "RISC-V Control Register."]
pub mod xo32mks;
#[doc = "TS0 (rw) register accessor: Temp Sensor trim0\n\nYou can [`read`](crate::Reg::read) this register and get [`ts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts0`] module"]
#[doc(alias = "TS0")]
pub type Ts0 = crate::Reg<ts0::Ts0Spec>;
#[doc = "Temp Sensor trim0"]
pub mod ts0;
#[doc = "TS1 (rw) register accessor: Temp Sensor trim1\n\nYou can [`read`](crate::Reg::read) this register and get [`ts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts1`] module"]
#[doc(alias = "TS1")]
pub type Ts1 = crate::Reg<ts1::Ts1Spec>;
#[doc = "Temp Sensor trim1"]
pub mod ts1;
#[doc = "ADCREFTRIM0 (rw) register accessor: Temp Sensor trim1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcreftrim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcreftrim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcreftrim0`] module"]
#[doc(alias = "ADCREFTRIM0")]
pub type Adcreftrim0 = crate::Reg<adcreftrim0::Adcreftrim0Spec>;
#[doc = "Temp Sensor trim1"]
pub mod adcreftrim0;
#[doc = "ADCREFTRIM1 (rw) register accessor: Temp Sensor trim1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcreftrim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcreftrim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcreftrim1`] module"]
#[doc(alias = "ADCREFTRIM1")]
pub type Adcreftrim1 = crate::Reg<adcreftrim1::Adcreftrim1Spec>;
#[doc = "Temp Sensor trim1"]
pub mod adcreftrim1;
#[doc = "ADCREFTRIM2 (rw) register accessor: Temp Sensor trim1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcreftrim2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcreftrim2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcreftrim2`] module"]
#[doc(alias = "ADCREFTRIM2")]
pub type Adcreftrim2 = crate::Reg<adcreftrim2::Adcreftrim2Spec>;
#[doc = "Temp Sensor trim1"]
pub mod adcreftrim2;
