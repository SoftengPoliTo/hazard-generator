mod common;

use std::path::Path;

use common::compare;
use hazard_generator::TemplateType;

const RUST_SNAPSHOTS_PATH: &str = "../snapshots/rust/";

#[test]
fn ontology() {
    compare(
        "ontology.rs",
        Path::new(RUST_SNAPSHOTS_PATH),
        TemplateType::Rust,
        false,
    );
}

#[test]
fn ontology_with_risk() {
    compare(
        "ontology_with_risk.rs",
        Path::new(RUST_SNAPSHOTS_PATH),
        TemplateType::Rust,
        true,
    );
}
