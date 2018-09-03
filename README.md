# solox-amp-rust

[AMP][open-amp] experiments in feL4 (seL4/Rust) on SoloX ARM SoC (A9 + M4)

The [Nitrogen6 SoloX][solox] SoC has both an A9 core and an M4 core.

Here's a good [article][bd-article] that describes the platform.

See the [TRM][trm] for register details.

## About the project

The seL4 kernel, root-task and threads run on the A9 core.

A bare metal Rust cortex-m project runs on the M4 core.

- U-boot does initial bootstrapping of the A9 core and loads the master ELF binary
- seL4 kernel is started on the A9 core
- seL4 root task initializes some memory, devices and I/O for a thread
- seL4 thread loads the M4 core firmware from the CPIO archive linked into the master ELF binary
- seL4 thread starts up the M4 core and clock

## Building

** REQUIRES a cargo-fel4 that doesn't overwrite the src/bin/root-task.rs file **

Here's a [branch](https://github.com/jonlamb-gh/cargo-fel4/tree/keep-root-rask-for-development) I use.

Note that the L2 cache memory is currently defined as OCRAM for the M4 core.

Modify U-boot environment:

```bash
# default loadaddr
loadaddr=0x82000000

# move it up a bit so we don't overlap with the elf-loader, Rust binaries are big right now
setenv loadaddr 0x83000000

# boot alias command
setenv bootfel4img 'tftp ${loadaddr} ${serverip}:feL4img; dcache flush; dcache off; go ${loadaddr}'
```

Apply local patches to convert the imx6/sabre-lite platform into something the SoloX will run.

```bash
./scripts/apply-patches

# a build.rs script is used to invoke the 'm4-firmware' project build
# required because we can't have a normal dependency on a thing for a different target (A9/M4)
cargo fel4 build
```

## Running

On UART1 (U-boot/A9) console:

```text
U-Boot 2017.07-28767-g87d490f (Jun 20 2018 - 10:29:54 -0700)

CPU:   Freescale i.MX6SX rev1.3 at 792 MHz
Reset cause: POR
Board: nitrogen6sx
I2C:   ready
DRAM:  1 GiB
MMC:   FSL_SDHC: 0, FSL_SDHC: 1
SF: Detected sst25vf016b with page size 256 Bytes, erase size 4 KiB, total 2 MiB
Display: lcd:1280x720M@60 (1280x720)
In:    serial
Out:   serial
Err:   serial
Net:   AR8035 at 4
AR8035 at 5
FEC0 [PRIME], FEC1, usb_ether
Hit any key to stop autoboot:  0
Using FEC0 device
Filename 'feL4img'.
Load address: 0x83000000
Loading: #################################################################
         ############################
         7.7 MiB/s
done
Bytes transferred = 1363524 (14ce44 hex)
## Starting application at 0x83000000 ...

ELF-loader started on CPU: ARM Ltd. Cortex-A9 r2p10
  paddr=[83000000..8315ffff]
ELF-loading image 'kernel'
  paddr=[80000000..80032fff]
  vaddr=[e0000000..e0032fff]
  virt_entry=e0000000
ELF-loading image 'root-task'
  paddr=[80033000..82080fff]
  vaddr=[10000..205dfff]
  virt_entry=10554
Enabling MMU and paging
Jumping to kernel-image entry point...

Bootstrapping kernel
Booting all finished, dropped to user space
feL4 app init

hello from a feL4 thread!

SRC paddr = 0x20D8000 -- vaddr = 0x10000000
CCM paddr = 0x20C4000 -- vaddr = 0x10001000
TCM paddr = 0x7F8000 -- vaddr = 0x10002000

created new CPIO reader
Reader {
    archive_size: 3584,
    base_ptr: 0x00044010
}

parsed CPIO entry 'm4-firmware.bin'

enabling M4 core clock
copying M4 binary to TCM - 3044 bytes
enabling and starting the M4 core
waiting for SRC_SCR reset auto-clear (bit 3) to clear

thread work all done, going to fault now

!!! Fault from badge 0xBEEF
```

On UART2 (M4) console:

```text
M4 core is up and running

Hello world from Rust on Cortex-M4
Hello world from Rust on Cortex-M4
Hello world from Rust on Cortex-M4
Hello world from Rust on Cortex-M4
Hello world from Rust on Cortex-M4
Hello world from Rust on Cortex-M4
...
```

### A9 Test Deployment

```bash
tftp ${a9ocramloadaddr} ${serverip}:a9.bin

go ${a9ocramloadaddr}
```

### M4 Test Deployment

```bash
tftp ${m4loadaddr} ${serverip}:m4.bin

dcache flush

bootaux ${m4loadaddr}
```

## Notes

### L2 Cache

seL4 is attempting to use the TCM region for L2 cache.
Since we're using it for the M4 core, can we instead use the
region at `0x009C_0000`; which is 256 KB, labeled as
`L2 cache memory used as OCRAM aliased`?

Could just be an issue with my config.

L2 cache is configured as OCRAM by default I think.
See Table 8-2 in the TRM, fuse `USE_L2_CACHE_AS_OCRAM BOOT_CFG2`.

### Memory Map

The i.MX6 SoloX has two cores with different address mapping.

The ARM IP Bus (AIPS) memory map shows there is a `0x4000_0000` offset
from the A9's address space to the M4's.

Refer to Table 2-1 (System memory map) for Cortex-A9 core and
to Table 2-2 (CM4 memory map) for Cortex-M4 of the i.MX6 SoloX
reference ranual.

To run Cortex-M4 it is needed to fill `TCM(L)`, that
is addressed as `TCML ALIAS` (from zero).

The same memory is mapped to `0x007f8000` of the
Cortex-A9 (non-reflected in the Table 2-1).

Note, this area is accessible by the Cortex-A9 after M4 clock
is enabled in `CCM_CCGR3`.

[solox]: https://boundarydevices.com/product/nit6_solox-imx6/
[bd-article]: https://boundarydevices.com/using-the-cortex-m4-mcu-on-the-nit6_solox/
[trm]: http://cache.freescale.com/files/32bit/doc/ref_manual/IMX6SXRM.pdf
[open-amp]: https://github.com/OpenAMP/open-amp/wiki
