use super::instructions::*;

pub fn verify_vmx_support() -> bool {
    // Set leaf to 01H (returns feature information in ecx and edx)
    const FEATURE_INFO_LEAF: u32 = 0x01;

    // Mask on the 5th bit
    const VMX_BIT_MASK: u32 = 0x1 << 5;

    let (_, _, ecx, _) = cpuid(FEATURE_INFO_LEAF);

    (ecx & VMX_BIT_MASK) != 0
}

pub fn vmx_enable() -> Result<(), &'static str> {
    const IA32_FEATURE_CONTROL_INDEX: u32 = 0x3A;
    const LOCK_BITMASK: u64 = 0x1;
    const VMX_ENABLE_BITMASK: u64 = 0x1 << 2;

    let value = rdmsr(IA32_FEATURE_CONTROL_INDEX);

    if (value & LOCK_BITMASK) == 0 {
        let enabled_value = value | LOCK_BITMASK | VMX_ENABLE_BITMASK;
        wrmsr(IA32_FEATURE_CONTROL_INDEX, enabled_value);
        Ok(())
    } else if (value & VMX_ENABLE_BITMASK) == 0 {
        Err("VMX operation cannot be enabled. Lock bit has been set with VMX disabled.")
    } else {
        Ok(())
    }
}

pub fn set_vmxe_bit() -> Result<(), &'static str> {
    // CR4.VMXE = bit 13
    const VMXE_BITMASK: u64 = 0x1 << 13;

    let value = read_cr4();

    if value & VMXE_BITMASK == 0 {
        let enabled_value = value | VMXE_BITMASK;
        write_cr4(enabled_value);
    }
    Ok(())
}
