#![allow(unused)]

pub const EXTENSION_BASE: usize = 0x10;
pub const EXTENSION_TIMER: usize = 0x54494D45;
pub const EXTENSION_IPI: usize = 0x735049;
pub const EXTENSION_RFENCE: usize = 0x52464E43;
pub const EXTENSION_HSM: usize = 0x48534D;
pub const EXTENSION_SRST: usize = 0x53525354;

const FUNCTION_BASE_GET_SPEC_VERSION: usize = 0x0;
const FUNCTION_BASE_GET_SBI_IMPL_ID: usize = 0x1;
const FUNCTION_BASE_GET_SBI_IMPL_VERSION: usize = 0x2;
const FUNCTION_BASE_PROBE_EXTENSION: usize = 0x3;
const FUNCTION_BASE_GET_MVENDORID: usize = 0x4;
const FUNCTION_BASE_GET_MARCHID: usize = 0x5;
const FUNCTION_BASE_GET_MIMPID: usize = 0x6;

#[derive(Debug)]
#[repr(C)]
pub struct SbiRet {
    /// Error number
    pub error: usize,
    /// Result value
    pub value: usize,
}

#[inline]
pub fn get_spec_version() -> usize {
    sbi_call_0(EXTENSION_BASE, FUNCTION_BASE_GET_SPEC_VERSION).value
}

#[inline]
pub fn get_sbi_impl_id() -> usize {
    sbi_call_0(EXTENSION_BASE, FUNCTION_BASE_GET_SBI_IMPL_ID).value
}

#[inline]
pub fn get_sbi_impl_version() -> usize {
    sbi_call_0(EXTENSION_BASE, FUNCTION_BASE_GET_SBI_IMPL_VERSION).value
}

#[inline]
pub fn probe_extension(extension_id: usize) -> usize {
    sbi_call_1(EXTENSION_BASE, FUNCTION_BASE_PROBE_EXTENSION, extension_id).value
}

#[inline]
pub fn get_mvendorid() -> usize {
    sbi_call_0(EXTENSION_BASE, FUNCTION_BASE_GET_MVENDORID).value
}

#[inline]
pub fn get_marchid() -> usize {
    sbi_call_0(EXTENSION_BASE, FUNCTION_BASE_GET_MARCHID).value
}

#[inline]
pub fn get_mimpid() -> usize {
    sbi_call_0(EXTENSION_BASE, FUNCTION_BASE_GET_MIMPID).value
}

#[inline(always)]
fn sbi_call_legacy(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0, in("a1") arg1, in("a2") arg2,
                in("a7") which,
                lateout("a0") ret,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((which, arg0, arg1, arg2));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    ret
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

pub fn console_putchar(c: usize) {
    sbi_call_legacy(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call_legacy(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn shutdown() -> ! {
    sbi_call_legacy(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}

pub fn set_timer(time: usize) {
    sbi_call_legacy(SBI_SET_TIMER, time, 0, 0);
}

const FUNCTION_IPI_SEND_IPI: usize = 0x0;

pub fn send_ipi(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call_2(EXTENSION_IPI, FUNCTION_IPI_SEND_IPI, hart_mask, hart_mask_base)
}

const FUNCTION_HSM_HART_START: usize = 0x0;
const FUNCTION_HSM_HART_STOP: usize = 0x1;
const FUNCTION_HSM_HART_GET_STATUS: usize = 0x2;
const FUNCTION_HSM_HART_SUSPEND: usize = 0x3;

pub fn hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
    sbi_call_3(EXTENSION_HSM, FUNCTION_HSM_HART_START, hartid, start_addr, opaque)
}

pub fn hart_stop(hartid: usize) -> SbiRet {
    sbi_call_1(EXTENSION_HSM, FUNCTION_HSM_HART_STOP, hartid)
}

pub fn hart_get_status(hartid: usize) -> SbiRet {
    sbi_call_1(EXTENSION_HSM, FUNCTION_HSM_HART_GET_STATUS, hartid)
}

pub fn hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet {
    sbi_call_3(EXTENSION_HSM, FUNCTION_HSM_HART_SUSPEND, suspend_type as usize, resume_addr, opaque)
}

#[inline(always)]
fn sbi_call_0(extension: usize, function: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

#[inline(always)]
fn sbi_call_1(extension: usize, function: usize, arg0: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0,
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

#[inline(always)]
fn sbi_call_2(extension: usize, function: usize, arg0: usize, arg1: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0, in("a1") arg1,
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}

#[inline(always)]
fn sbi_call_3(extension: usize, function: usize, arg0: usize, arg1: usize, arg2: usize) -> SbiRet {
    let (error, value);
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => unsafe {
            asm!(
                "ecall",
                in("a0") arg0, in("a1") arg1, in("a2") arg2,
                in("a6") function, in("a7") extension,
                lateout("a0") error, lateout("a1") value,
            )
        },
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => {
            drop((extension, function, arg0, arg1, arg2));
            unimplemented!("not RISC-V instruction set architecture")
        }
    };
    SbiRet { error, value }
}
