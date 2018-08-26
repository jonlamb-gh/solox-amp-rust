# solox-amp-rust

AMP experiments in feL4 (seL4/Rust) on SoloX ARM SoC (A9 + M4)

The [Nitrogen6 SoloX][solox] SoC has both an A9 core and an M4 core.

The seL4 root task runs on the A9 core.

It first initializes the system, then runs a separate Rust application on the M4 core.

[solox]: https://boundarydevices.com/product/nit6_solox-imx6/
