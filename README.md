# max78002-pac

A [Peripheral Access Crate](https://docs.rust-embedded.org/book/start/registers.html) for Analog Devices' [MAX78002](https://www.analog.com/products/max78002.html) microcontroller.

This crate is generated from the SVD file in [Analog Devices' MSDK](https://github.com/analogdevicesinc/msdk) using [svd2rust](https://github.com/rust-embedded/svd2rust). The vendor SVD has several mistakes, which are patched with [svdtools](https://github.com/rust-embedded/svdtools) before generation. It follows the layout and conventions of [sigpwny/max78000-pac](https://github.com/sigpwny/max78000-pac) so that both crates can back the same HAL.

## What is patched

`svd/max78002.yaml` fixes:

- Peripheral names that omit the instance number (`UART` -> `UART0`, `TMR` -> `TMR0`, `WDT` -> `WDT0`, `PT` -> `PT0`).
- Interrupt names that disagree with the `IRQn_Type` table in `max78002.h` (`WWDT` -> `WDT0`, `Flash_Controller` -> `FLC0`, `CameraIF` -> `PCIF`, `OneWire` -> `OWM`).
- Interrupts missing from the SVD (`UART0`-`UART3`, `AES`, `CRC`, `CSI2`, `SIMO`, `USBDMA`, `PF`, `GPIOWAKE`).
- `SDHC.CFG_1.SDR104`, which is declared with a bit width of zero.
- `WUT.CTRL.PRES` / `WUT.CTRL.PRES3`, which both enumerate the combined 4-bit prescaler encoding and therefore assign the same raw value to several names.

The CNN accelerator is not described by the vendor SVD and is therefore absent from this crate.

## Provenance

`svd/max78002.svd` is copied verbatim from a local MSDK install:

```
Libraries/CMSIS/Device/Maxim/MAX78002/Include/max78002.svd
sha256: 706261a1533735ab0ffec1ca10a77853169279eebe3796e25680ff3d29cb9bf6
```

## Building

> [!NOTE]
> This section is only relevant if you need to rebuild the crate. Generated code is already included in the repository.

```bash
cargo install svd2rust svdtools form
make
cargo build --target thumbv7em-none-eabihf
```

## License

This crate is licensed under the [Apache-2.0](./LICENSE) license.

Copyright 2025 Analog Devices, Inc. (SVD file)
