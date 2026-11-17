// Integration test for SigmaOS-tools-gliff
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_tools_gliff as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
