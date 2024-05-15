#![deny(missing_docs, unsafe_code)]

//! The `hazard-generator` tool generates APIs
//! starting from a JSON ontology
//! for the following programming languages:
//!
//! - Rust

mod error;
mod filters;
mod toolchain;

use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
    str::FromStr,
};

use error::{Error, ErrorKind, Result};
use filters::camelcase_to_whitespaces;
use futures::executor;
use json_ld::{syntax::Parse, Flatten, Indexed, JsonLdProcessor, Node, RemoteDocument};
use minijinja::{Environment, Value};
use toolchain::{
    context, create_hazard, description, is_category, is_hazard, name, object_type, rust::Rust,
    CategoryData,
};

/// Supported templates.
#[derive(Debug, Clone)]
pub enum TemplateType {
    /// Generate hazards APIs for Rust.
    Rust,
}

impl TemplateType {
    /// All `TemplateType` options.
    pub const fn all() -> &'static [&'static str] {
        &["rust"]
    }
}

impl FromStr for TemplateType {
    type Err = Error;

    fn from_str(template: &str) -> std::result::Result<Self, Self::Err> {
        match template {
            "rust" => Ok(Self::Rust),
            template => Err(Error::new(
                ErrorKind::Template,
                format!("{template:?} is not a supported template."),
            )),
        }
    }
}

struct OutputFile {
    path: PathBuf,
    template_name: &'static str,
}

impl OutputFile {
    pub(crate) const fn new(path: PathBuf, template_name: &'static str) -> Self {
        OutputFile {
            path,
            template_name,
        }
    }
}

struct Output {
    context: HashMap<&'static str, Value>,
    file: OutputFile,
}

impl Output {
    pub(crate) const fn new(context: HashMap<&'static str, Value>, file: OutputFile) -> Self {
        Output { context, file }
    }
}

struct Template {
    context: HashMap<&'static str, Value>,
    output_file: OutputFile,
    env: Environment<'static>,
}

impl Template {
    const fn new(
        context: HashMap<&'static str, Value>,
        output_file: OutputFile,
        env: Environment<'static>,
    ) -> Self {
        Self {
            context,
            output_file,
            env,
        }
    }

    fn render(self) -> Result<()> {
        let Template {
            context,
            output_file,
            mut env,
        } = self;

        env.add_filter("camelcase_to_whitespaces", camelcase_to_whitespaces);

        // Fill in the templates.
        let template = env.get_template(output_file.template_name)?;
        let filled_template = template.render(&context)?;
        write(output_file.path, filled_template)?;

        Ok(())
    }
}

/// Build a template.
trait BuildTemplate {
    fn get_templates() -> &'static [(&'static str, &'static str)];

    fn output_file(output_path: &Path, with_risk: bool) -> OutputFile;

    fn define(&self, ontology: Ontology, output_path: &Path, with_risk: bool) -> Output {
        let mut hazards = Vec::new();
        let mut categories = Vec::new();
        let mut categories_hazards: HashMap<&str, Vec<&str>> = HashMap::new();

        ontology.iter().for_each(|object| {
            if let Some(object_type) = object_type(object) {
                let name = name(object);
                let description = description(object);

                if is_hazard(object_type) {
                    // Create a new hazard.
                    let hazard = create_hazard(object, name, description, with_risk);

                    // Map hazard to its category.
                    categories_hazards
                        .entry(hazard.category)
                        .or_default()
                        .push(name);

                    hazards.push(hazard);
                } else if is_category(object_type) {
                    categories.push(CategoryData::new(name, description, Vec::new()))
                }
            }
        });

        // Set corresponding hazards for each category.
        categories.iter_mut().for_each(|category| {
            category.hazards = categories_hazards
                .get(category.name)
                .map(|hazards| hazards.to_owned())
                .unwrap_or_default();
        });

        // Define context.
        let context = context(hazards, categories);

        // Define output file of the API.
        let output_file = Self::output_file(output_path, with_risk);

        Output::new(context, output_file)
    }

    fn build(&self, ontology: Ontology, output_path: &Path, with_risk: bool) -> Template {
        let output = self.define(ontology, output_path, with_risk);
        let env = build_environment(Self::get_templates(), with_risk);

        Template::new(output.context, output.file, env)
    }
}

fn build_environment(
    templates: &'static [(&'static str, &'static str)],
    with_risk: bool,
) -> Environment<'static> {
    let mut environment = Environment::new();

    // Add base ontology template to the `Environment`.
    add_template(&mut environment, templates[0]);

    // Add risk template to the `Environment`.
    if with_risk {
        add_template(&mut environment, templates[1]);
    }

    environment
}

fn add_template(environment: &mut Environment, template: (&'static str, &'static str)) {
    let (name, src) = template;
    environment
        .add_template(name, src)
        .expect("Internal error, built-in template");
}

pub(crate) type Ontology = Vec<Indexed<Node>>;

/// Produce hazards.
#[derive(Default)]
pub struct HazardsProducer {
    with_risk: bool,
}

impl HazardsProducer {
    /// Creates a new `HazardsProducer` instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to parse the risk score associated to the hazards.
    pub const fn with_risk(mut self, with_risk: bool) -> Self {
        self.with_risk = with_risk;
        self
    }

    /// Runs hazards producer.
    pub fn run<P: AsRef<Path>, O: AsRef<Path>>(
        self,
        ontology_path: P,
        output_path: O,
        template_type: TemplateType,
    ) -> Result<()> {
        // Check output path.
        self.check_output_path(&output_path)?;

        // Obtain the ontology.
        let ontology = self.open_ontology(ontology_path)?;

        // Build the tamplate.
        let template = match template_type {
            TemplateType::Rust => Rust::new().build(ontology, output_path.as_ref(), self.with_risk),
        };

        // Render the template.
        template.render()
    }

    fn open_ontology<P: AsRef<Path>>(&self, ontology_path: P) -> error::Result<Ontology> {
        // Return an error if ontology path is not a file.
        if !ontology_path.as_ref().is_file() {
            return Err(Error::new(
                ErrorKind::PathFormat,
                "Ontology path MUST be a file path",
            ));
        }

        // Create a `RemoteDocument` by parsing the local ontology file.
        let input: json_ld::RemoteDocument = RemoteDocument::new(
            // Use `None` since the ontology file is a local file that does not have an URL.
            None,
            None,
            // Parse the file.
            json_ld::syntax::Value::parse_str(read_to_string(ontology_path)?.as_str())?.0,
        );

        // Use `NoLoader` as we won't need to load any remote document.
        let mut loader = json_ld::NoLoader;

        // Expand the jsonld ontology.
        let expanded = executor::block_on(input.expand(&mut loader))?;

        // Define the generator that is in charge of creating identifiers for nested anonymous nodes.
        let generator = rdf_types::generator::Blank::new();

        // Flatten the jsonld ontology.
        let ontology = expanded.flatten(generator, true)?;

        Ok(ontology)
    }

    #[inline(always)]
    fn check_output_path<P: AsRef<Path>>(&self, output_path: P) -> error::Result<()> {
        // Return an error if output path is not a directory.
        if !output_path.as_ref().is_dir() {
            return Err(Error::new(
                ErrorKind::PathFormat,
                "Output path MUST be a directory path",
            ));
        }

        Ok(())
    }
}
