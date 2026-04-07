use std::arch::asm;

fn main() {
    let supported: bool = verify_vmx_support();
    println!("{}", supported);
}

fn cpuid(leaf: u32) -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov {0:e}, ebx",
            "pop rbx",
            out(reg) ebx,
            inout("eax") leaf=>eax,
            out("ecx") ecx,
            out("edx") edx,
        );
    }
    (eax, ebx, ecx, edx)
}

fn verify_vmx_support() -> bool {
    // Set leaf to 01H (returns feature information in ecx and edx)
    const FEATURE_INFO_LEAF: u32 = 0x01;

    // Mask on the 5th bit
    const VMX_BIT_MASK: u32 = 0x1 << 5;

    let (_, _, ecx, _) = cpuid(FEATURE_INFO_LEAF);

    (ecx & VMX_BIT_MASK) != 0
}
