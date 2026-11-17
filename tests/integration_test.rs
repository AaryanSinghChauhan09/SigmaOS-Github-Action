// Integration test for SigmaOS-Github-Action
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_github_action as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
