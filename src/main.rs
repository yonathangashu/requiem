mod instructions;
mod vmx;
fn main() {
    let supported: bool = vmx::verify_vmx_support();
    println!("{}", supported);
}
