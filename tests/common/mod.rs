use std::{env::temp_dir, fs, path::Path};

use hazard_generator::{HazardsProducer, TemplateType};

const ONTOLOGY_PATH: &str = "./ontology.jsonld";
const OUTPUT_FILE: &str = "ontology";
const OUTPUT_WITH_RISK_FILE: &str = "ontology_with_risk";

pub(crate) fn compare(
    snapshot_name: &str,
    snapshot_path: &Path,
    template_type: TemplateType,
    with_risk: bool,
) {
    let output_dir = temp_dir();

    HazardsProducer::new()
        .with_risk(with_risk)
        .run(Path::new(ONTOLOGY_PATH), &output_dir, template_type)
        .unwrap();

    let output_file = match template_type {
        TemplateType::Rust => {
            output_dir.join(Path::new(output_file(with_risk)).with_extension("rs"))
        }
    };

    let ontology_api = fs::read_to_string(&output_file).unwrap();

    insta::with_settings!({
        snapshot_path => snapshot_path,
        prepend_module_to_snapshot => false,
    },{
        insta::assert_snapshot!(snapshot_name, ontology_api);
    });
}

fn output_file(with_risk: bool) -> &'static Path {
    match with_risk {
        true => Path::new(OUTPUT_WITH_RISK_FILE),
        false => Path::new(OUTPUT_FILE),
    }
}
