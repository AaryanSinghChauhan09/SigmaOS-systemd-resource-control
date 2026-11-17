// Integration test for SigmaOS-systemd-resource-control
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use systemd_resource_control as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
