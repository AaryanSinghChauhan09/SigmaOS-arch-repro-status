use sigmaos_arch_repro_status::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_arch_repro_status::ArchReproStatus::new("sigmaos-arch-repro-status").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-arch-repro-status"));
}
