pub trait PlatformOps {
    fn virt_to_physical(&self, virt_addr: usize) -> usize;
    fn run_on_all_processors(&self, function: fn());
}
