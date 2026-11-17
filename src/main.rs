//! SigmaOS-arch-repro-status — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_arch_repro_status_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-arch-repro-status: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-arch-repro-status: ok");
}
