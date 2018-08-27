# solox-amp-rust

[AMP][open-amp] experiments in feL4 (seL4/Rust) on SoloX ARM SoC (A9 + M4)

The [Nitrogen6 SoloX][solox] SoC has both an A9 core and an M4 core.

The seL4 root task runs on the A9 core.

It first initializes the system, then runs a separate Rust application on the M4 core.

## Building

Note that the L2 cache memory is currently defined as OCRAM for the M4 core.

Modify U-boot environment:

```bash
# default loadaddr
loadaddr=0x82000000

# move it up a bit so we don't overlap with the elf-loader
setenv loadaddr 0x83000000

# boot alias command
setenv bootfel4img 'tftp ${loadaddr} ${serverip}:feL4img; dcache flush; dcache off; go ${loadaddr}'
```

Apply local patches to convert the imx6/sabre-lite platform into something the SoloX will run.

```bash
./scripts/apply-patches

cargo fel4 build
```

[solox]: https://boundarydevices.com/product/nit6_solox-imx6/
[open-amp]: https://github.com/OpenAMP/open-amp/wiki
