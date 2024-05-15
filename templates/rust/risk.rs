{% extends "rs.ontology" %}

{%- block struct_risk %}
    /// Risk score.
    /// 
    /// Currently, the value is [`None`] in the following cases:
    /// - When a hazard has no risk score.
    /// - When a risk score exists but is an empty string.
    /// 
    /// This will change when we decide whether to remove or keep the risk score.
    /// In the latter case, one of the following conditions will apply:
    /// - The risk score will have a numeric value.
    /// - The risk score will not exist at all, thus eliminating the empty string problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<u8>,
{%- endblock %}

{%- block new_param_risk %}, risk_score: Option<u8>{%- endblock %}

{%- block new_value_risk %}
            risk_score,
{%- endblock %}

{% block fn_risk %}

    /// Returns an [`Hazard`] risk score.
    pub const fn risk_score(&self) -> Option<u8> {
        match self {
        {%- for hazard in hazards %}
            Self::{{ hazard.name }} => {% if hazard.risk_score == none %}None{% else %}Some({{ hazard.risk_score }}){% endif %},
        {%- endfor %}
        }
    }
{%- endblock %}

{%- block serialize_risk %}
            risk_score: self.risk_score(),
{%- endblock %}