/*!
Tests for parameter configuration and scenario handling.
*/

use crate::prelude::*;

#[test]
fn test_params_builder() {
    let params = Params::for_scenario(Scenario::FastDemo)
        .expect("Failed to create params");

    assert!(params.supports_boolean());
    assert_eq!(params.scenario(), Scenario::FastDemo);
}

#[test]
fn test_scenario_based_params() {
    // Test all scenarios can be created
    let scenarios = [
        Scenario::SafeAndBalanced,
        Scenario::DeepCircuitUltraLowError,
        Scenario::FastDemo,
        Scenario::MaxSecurityMinimalOps,
    ];

    for scenario in scenarios {
        let params = Params::for_scenario(scenario)
            .expect("Failed to create params for scenario");
        
        assert!(params.supports_boolean());
        assert_eq!(params.scenario(), scenario);
    }
}

#[test]
fn test_scenario_to_tfhe_params_mapping() {
    use crate::params::{scenario_to_tfhe_params, Scenario};

    // Test that each scenario maps to the expected TFHE parameters
    // We can't compare pointers since they may differ, so we compare parameter properties
    let safe_params = scenario_to_tfhe_params(Scenario::SafeAndBalanced);
    let deep_params = scenario_to_tfhe_params(Scenario::DeepCircuitUltraLowError);
    let fast_params = scenario_to_tfhe_params(Scenario::FastDemo);
    let max_security_params = scenario_to_tfhe_params(Scenario::MaxSecurityMinimalOps);

    // Verify we get different parameter sets for different scenarios
    // These should have different LWE dimensions or polynomial sizes
    assert_ne!(
        (safe_params.lwe_dimension, safe_params.glwe_dimension),
        (deep_params.lwe_dimension, deep_params.glwe_dimension)
    );
    
    // Just verify we get valid parameter objects
    assert!(safe_params.lwe_dimension.0 > 0);
    assert!(deep_params.lwe_dimension.0 > 0);
    assert!(fast_params.lwe_dimension.0 > 0);
    assert!(max_security_params.lwe_dimension.0 > 0);
}

#[test]
fn test_params_direct_creation() {
    let params = Params::for_scenario(Scenario::FastDemo)
        .expect("Failed to create params");

    assert!(params.supports_boolean());
    assert_eq!(params.scenario(), Scenario::FastDemo);
}

#[test]
fn test_params_tfhe_boolean_params_with_scenario() {
    let params = Params::for_scenario(Scenario::DeepCircuitUltraLowError)
        .expect("Failed to create params");
    
    let tfhe_params = params.tfhe_boolean_params();
    
    // Verify it returns a valid parameter set
    assert!(tfhe_params.lwe_dimension.0 > 0);
    assert!(tfhe_params.glwe_dimension.0 > 0);
    
    // Verify it's different from the default parameters
    use tfhe::boolean::prelude::*;
    let default_params = &DEFAULT_PARAMETERS;
    
    // The deep circuit parameters should have different characteristics
    // (either different dimensions or different noise parameters)
    let is_different = tfhe_params.lwe_dimension != default_params.lwe_dimension
        || tfhe_params.glwe_dimension != default_params.glwe_dimension
        || tfhe_params.pbs_base_log != default_params.pbs_base_log;
    
    assert!(is_different, "Deep circuit parameters should differ from default");
}
