use core::mem;

use hvcore::platform_ops::PlatformOps;
use uefi::boot;
use uefi::proto::pi::mp::MpServices;

pub struct UefiOps {}

extern "efiapi" fn procedure_helper(arg: *mut core::ffi::c_void) {
    let original_fn: fn() = unsafe { mem::transmute(arg) };
    original_fn()
}
impl UefiOps {
    pub const fn new() -> Self {
        Self {}
    }
}
impl PlatformOps for UefiOps {
    // UEFI has identity mapping virt -> phys
    fn virt_to_physical(&self, virt_addr: usize) -> usize {
        virt_addr
    }

    fn run_on_all_processors(&self, function: fn()) {
        // MpServices = Multiprocess Services
        let mp_handle = boot::get_handle_for_protocol::<MpServices>()
            .expect("failed to get multi-processor services handle");
        let mp_support = &boot::open_protocol_exclusive::<MpServices>(mp_handle)
            .expect("failed to open multi-processor services protocol");

        // Call function on current core (BSP)
        function();

        let _ = mp_support.startup_all_aps(
            false,
            procedure_helper,
            function as *mut core::ffi::c_void,
            None,
            None,
        );
    }
}
