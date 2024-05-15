pub mod rust;

use json_ld::{object::Any, Indexed, IndexedNode, Node};
use minijinja::Value;
use serde::Serialize;
use static_iref::iri;
use std::collections::HashMap;

macro_rules! builtin_templates {
    ($root:expr => $(($name:expr, $template:expr)),+) => {
        [
        $(
            (
                $name,
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/templates/", $root, "/", $template)),
            )
        ),+
        ]
    }
}
pub(crate) use builtin_templates;

#[derive(Serialize)]
pub(crate) struct HazardData<'a> {
    name: &'a str,
    description: &'a str,
    pub(crate) category: &'a str,
    risk_score: Option<u8>,
}

impl<'a> HazardData<'a> {
    pub(crate) const fn new(
        name: &'a str,
        description: &'a str,
        category: &'a str,
        risk_score: Option<u8>,
    ) -> Self {
        Self {
            name,
            description,
            category,
            risk_score,
        }
    }
}

#[derive(Serialize)]
pub(crate) struct CategoryData<'a> {
    pub(crate) name: &'a str,
    description: &'a str,
    pub(crate) hazards: Vec<&'a str>,
}

impl<'a> CategoryData<'a> {
    pub(crate) const fn new(name: &'a str, description: &'a str, hazards: Vec<&'a str>) -> Self {
        Self {
            name,
            description,
            hazards,
        }
    }
}

// Retrieves object type.
#[inline(always)]
pub(crate) fn object_type(object: &Indexed<Node>) -> Option<&str> {
    object
        .get_any(&iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))
        .and_then(|t| t.as_node())
        .and_then(|t| t.id())
        .map(|t| t.as_str())
}

// Extracts the name from an onotology object.
#[inline(always)]
pub(crate) fn name(object_value: &Indexed<Node>) -> &str {
    object_value
        .get_any(&iri!("http://www.w3.org/2000/01/rdf-schema#label"))
        .and_then(|n| n.as_value())
        .and_then(|n| n.as_str())
        .unwrap_or_default()
}

// Extracts the description from an onotology object.
#[inline(always)]
pub(crate) fn description(object_value: &Indexed<Node>) -> &str {
    object_value
        .get_any(&iri!("https://purl.org/sifis/hazards#description"))
        .and_then(|d| d.as_str())
        .unwrap_or_default()
}

// Extracts the category of an onotology hazard object.
#[inline(always)]
pub(crate) fn category(object_value: &Indexed<Node>) -> &str {
    object_value
        .get_any(&iri!("https://purl.org/sifis/hazards#hasCategory"))
        .and_then(|c| c.as_str())
        .unwrap_or_default()
        .split('#')
        .last()
        .unwrap_or_default()
}

// Extracts the risk score of an onotology hazard object.
#[inline(always)]
pub(crate) fn risk_score(object_value: &Indexed<Node>) -> Option<u8> {
    object_value
        .get_any(&iri!("https://purl.org/sifis/hazards#riskScore"))?
        .as_value()?
        .as_number()?
        .as_u32()?
        .try_into()
        .ok()
}

const HAZARD_TYPE: &str = "https://purl.org/sifis/hazards#Hazard";

// Checks whether an ontology object is a hazard.
#[inline(always)]
pub(crate) fn is_hazard(object_type: &str) -> bool {
    object_type == HAZARD_TYPE
}

const CATEGORY_TYPE: &str = "https://purl.org/sifis/hazards#Category";

// Checks whether an ontology object is a category.
#[inline(always)]
pub(crate) fn is_category(object_type: &str) -> bool {
    object_type == CATEGORY_TYPE
}

// Creates a new hazard and updates related data structures.
pub(crate) fn create_hazard<'a>(
    object_value: &'a IndexedNode,
    name: &'a str,
    description: &'a str,
    with_risk: bool,
) -> HazardData<'a> {
    // Retrieve hazard category.
    let category = category(object_value);

    // Retrieve hazard risk score.
    let risk_score = match with_risk {
        true => risk_score(object_value),
        false => None,
    };

    HazardData::new(name, description, category, risk_score)
}

// Creates the template context.
pub(crate) fn context(
    hazards: Vec<HazardData>,
    categories: Vec<CategoryData>,
) -> HashMap<&'static str, Value> {
    let mut context = HashMap::new();

    // Create context.
    context.insert("hazards", Value::from_serialize(hazards));
    context.insert("categories", Value::from_serialize(categories));

    context
}
