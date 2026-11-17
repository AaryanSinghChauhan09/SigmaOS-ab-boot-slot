//! Integration tests for SigmaOS-ab-boot-slot.

use sigmaos_ab_boot_slot_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
