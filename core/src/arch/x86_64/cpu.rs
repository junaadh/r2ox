use x86::msr::{rdmsr, IA32_GS_BASE};
use x86_64::structures::{gdt::GlobalDescriptorTable, tss::TaskStateSegment};

pub struct CpuData {
    pub sp: usize,
    pub gdt: GlobalDescriptorTable,
}

#[repr(C, packed)]
pub struct Kpcr {
    pub tss: TaskStateSegment,
    pub cpu: &'static mut CpuData,
    pub user_sp: usize,
}

impl Kpcr {
    /// get kcpr structure from cpu
    /// # Safety
    /// unsafe because it dereferencfes a mutable pointer
    pub unsafe fn get_kpcr() -> &'static mut Self {
        &mut *(rdmsr(IA32_GS_BASE) as *mut _)
    }

    /// get tss from cpu
    /// # Safety
    /// unsafe because it dereferencfes a mutable pointer
    pub unsafe fn get_tss() -> &'static mut TaskStateSegment {
        &mut *(rdmsr(IA32_GS_BASE) as *mut _)
    }
}
