use std::path::Path;

use crate::{BuildTemplate, OutputFile};

use super::builtin_templates;

const OUTPUT_FILE: &str = "ontology.rs";
const OUTPUT_WITH_RISK_FILE: &str = "ontology_with_risk.rs";

static RUST_TEMPLATES: &[(&str, &str)] = &builtin_templates!["rust" =>
    ("rs.ontology", "ontology.rs"),
    ("rs.risk", "risk.rs")
];

pub(crate) struct Rust;

impl Rust {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl BuildTemplate for Rust {
    #[inline(always)]
    fn get_templates() -> &'static [(&'static str, &'static str)] {
        RUST_TEMPLATES
    }

    #[inline(always)]
    fn output_file(output_path: &Path, with_risk: bool) -> OutputFile {
        match with_risk {
            true => OutputFile::new(output_path.join(OUTPUT_WITH_RISK_FILE), "rs.risk"),
            false => OutputFile::new(output_path.join(OUTPUT_FILE), "rs.ontology"),
        }
    }
}
