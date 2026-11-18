// Integration test for SigmaOS-arch-ai-kernel
// Verifies the crate builds and links as a dependency.

use sigmaos_arch_ai_kernel::{AiKernel, ModelConfig, ModelType};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut kernel = AiKernel::new();
    let config = ModelConfig::new("test-model", ModelType::NeuralNet);
    kernel.register_model(config).unwrap();

    // Act
    let res = kernel.load_model("model-1");

    // Assert
    assert!(res.is_ok());
    assert_eq!(kernel.list_models().len(), 1);
}
