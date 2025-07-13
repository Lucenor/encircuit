/*!
Parameter configuration for FHE operations.

This module provides opinionated presets that wrap TFHE configuration to make
it easier to configure FHE parameters for common use cases.

## Scenario-Based Parameter Selection

The module supports scenario-based parameter selection through the [`Scenario`] enum,
which maps common use cases to appropriate TFHE parameter sets:

- **SafeAndBalanced**: General-purpose circuits with good security/performance balance
- **DeepCircuitUltraLowError**: Deep circuits requiring ultra-low error probability  
- **FastDemo**: Quick prototyping with minimal security requirements
- **MaxSecurityMinimalOps**: Maximum security for minimal operations

## Example

```rust,no_run
use encircuit::prelude::*;
# fn main() -> anyhow::Result<()> {

// Using scenarios for easy parameter selection
let params = Params::for_scenario(Scenario::SafeAndBalanced)?;
let keyset = Keyset::generate(&params)?;

// Different scenarios for different use cases
let demo_params = Params::for_scenario(Scenario::FastDemo)?;
let secure_params = Params::for_scenario(Scenario::MaxSecurityMinimalOps)?;
# Ok(())
# }
```
*/

use anyhow::Result;
use tfhe::boolean::parameters::BooleanParameters;

/// Describes typical TFHE usage scenarios with different security/performance trade-offs.
///
/// Each scenario maps to a specific set of TFHE Boolean parameters that have been
/// carefully selected for that use case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scenario {
    /// Safe and balanced use (default for general-purpose circuits).
    ///
    /// Provides good security with reasonable performance. Suitable for most
    /// production applications that need a balance between security and speed.
    SafeAndBalanced,
    
    /// Deep circuits with ultra-low error probability.
    ///
    /// Optimized for circuits with many sequential operations where error
    /// accumulation is a concern. Provides 2^-165 error probability.
    DeepCircuitUltraLowError,
    
    /// Fast, low-sensitivity demo or prototype.
    ///
    /// Prioritizes performance over security. Suitable for demonstrations,
    /// prototypes, or applications where speed is more important than security.
    FastDemo,
    
    /// Maximum security with minimal operations.
    ///
    /// Provides the highest security level with ultra-low error probability.
    /// Best for high-security applications with relatively simple circuits.
    MaxSecurityMinimalOps,
}

/// Returns the recommended TFHE BooleanParameters for the given scenario.
///
/// This function maps each scenario to specific TFHE parameter constants that
/// have been validated for that use case.
///
/// # Arguments
///
/// * `scenario` - The usage scenario to get parameters for
///
/// # Returns
///
/// A reference to the appropriate TFHE `BooleanParameters` constant.
pub fn scenario_to_tfhe_params(scenario: Scenario) -> &'static BooleanParameters {
    use tfhe::boolean::prelude::*;
    
    match scenario {
        Scenario::SafeAndBalanced => &DEFAULT_PARAMETERS,
        Scenario::DeepCircuitUltraLowError => &PARAMETERS_ERROR_PROB_2_POW_MINUS_165, 
        Scenario::FastDemo => &DEFAULT_PARAMETERS_KS_PBS,
        Scenario::MaxSecurityMinimalOps => &PARAMETERS_ERROR_PROB_2_POW_MINUS_165_KS_PBS,
    }
}

/// FHE parameter configuration.
///
/// Provides opinionated presets for common FHE configurations using scenario-based
/// parameter selection.
#[derive(Debug, Clone)]
pub struct Params {
    scenario: Scenario,
}

impl Params {
    /// Create parameters directly from a scenario.
    ///
    /// This creates parameters optimized for the specified scenario with Boolean operations.
    pub fn for_scenario(scenario: Scenario) -> Result<Self> {
        Ok(Self {
            scenario,
        })
    }

    /// Get the scenario.
    pub fn scenario(&self) -> Scenario {
        self.scenario
    }

    /// Check if Boolean operations are supported.
    /// 
    /// Always returns true as this implementation only supports Boolean operations.
    pub fn supports_boolean(&self) -> bool {
        true
    }

    /// Get the TFHE Boolean parameters for key generation.
    ///
    /// Uses the scenario to select appropriate TFHE parameters.
    pub fn tfhe_boolean_params(&self) -> &'static BooleanParameters {
        scenario_to_tfhe_params(self.scenario)
    }
}
