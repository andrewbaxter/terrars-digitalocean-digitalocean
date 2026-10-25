#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AgentPlatformCustomModelData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accept_terms_and_conditions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_modalities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_modalities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_gpu_region: std::option::Option<terrars::PrimField<std::string::String>>,
    source_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_ref: std::option::Option<std::vec::Vec<AgentPlatformCustomModel_SourceRef>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AgentPlatformCustomModel_Timeouts>,
    dynamic: AgentPlatformCustomModelDynamic,
}

struct AgentPlatformCustomModel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgentPlatformCustomModelData_>,
}

#[derive(Clone)]
pub struct AgentPlatformCustomModel(std::rc::Rc<AgentPlatformCustomModel_>);

impl AgentPlatformCustomModel {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc =
        "Set the field `accept_terms_and_conditions`.\nWhether the caller accepts the model provider's terms and conditions. Write-only."]
    pub fn set_accept_terms_and_conditions(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().accept_terms_and_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the custom model."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `input_modalities`.\nInput modalities supported by the model. Defaults to the values reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn set_input_modalities(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().input_modalities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `license`.\nLicense of the model. Defaults to the value reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn set_license(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().license = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `output_modalities`.\nOutput modalities produced by the model. Defaults to the values reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn set_output_modalities(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().output_modalities = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameters`.\nParameter-count summary for the model. Defaults to the value reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn set_parameters(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preferred_gpu_region`.\nPreferred GPU region where the model artifacts should be staged."]
    pub fn set_preferred_gpu_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().preferred_gpu_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nUser-defined tags associated with the custom model."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_ref`.\n"]
    pub fn set_source_ref(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformCustomModel_SourceRef>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_ref = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_ref = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AgentPlatformCustomModel_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `accept_terms_and_conditions` after provisioning.\nWhether the caller accepts the model provider's terms and conditions. Write-only."]
    pub fn accept_terms_and_conditions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_terms_and_conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `active_deployments` after provisioning.\nActive dedicated inference deployments referencing this custom model."]
    pub fn active_deployments(&self) -> terrars::ListRef<AgentPlatformCustomModel_ActiveDeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_deployments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `architecture` after provisioning.\nModel architecture as reported by the importer."]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `context_length` after provisioning.\nMaximum context length supported by the model."]
    pub fn context_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context_length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cost_estimate_per_month` after provisioning.\nEstimated monthly cost of running the custom model."]
    pub fn cost_estimate_per_month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cost_estimate_per_month", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the custom model was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the custom model."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nError message if the custom model import failed."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_count` after provisioning.\nNumber of files that make up the imported model."]
    pub fn file_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `input_modalities` after provisioning.\nInput modalities supported by the model. Defaults to the values reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn input_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license` after provisioning.\nLicense of the model. Defaults to the value reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn license(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-readable name for the custom model."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_modalities` after provisioning.\nOutput modalities produced by the model. Defaults to the values reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn output_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nParameter-count summary for the model. Defaults to the value reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preferred_gpu_region` after provisioning.\nPreferred GPU region where the model artifacts should be staged."]
    pub fn preferred_gpu_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_gpu_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nSource of the model to import. One of SOURCE_TYPE_HUGGINGFACE, SOURCE_TYPE_SPACES_BUCKET, SOURCE_TYPE_SDK_UPLOAD, SOURCE_TYPE_FINE_TUNING."]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nCurrent status of the custom model."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_region` after provisioning.\nRegion where the custom model artifacts are stored."]
    pub fn storage_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nUser-defined tags associated with the custom model."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `team_id` after provisioning.\nID of the team that owns the custom model."]
    pub fn team_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_size_bytes` after provisioning.\nTotal size of the imported model artifacts in bytes."]
    pub fn total_size_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the custom model was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the custom model."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ref` after provisioning.\n"]
    pub fn source_ref(&self) -> terrars::ListRef<AgentPlatformCustomModel_SourceRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ref", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgentPlatformCustomModel_TimeoutsRef {
        AgentPlatformCustomModel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AgentPlatformCustomModel {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgentPlatformCustomModel { }

impl terrars::ToListMappable for AgentPlatformCustomModel {
    type O = terrars::ListRef<AgentPlatformCustomModelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgentPlatformCustomModel_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_custom_model".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgentPlatformCustomModel {
    pub tf_id: std::string::String,
    #[doc = "A human-readable name for the custom model."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Source of the model to import. One of SOURCE_TYPE_HUGGINGFACE, SOURCE_TYPE_SPACES_BUCKET, SOURCE_TYPE_SDK_UPLOAD, SOURCE_TYPE_FINE_TUNING."]
    pub source_type: terrars::PrimField<std::string::String>,
}

impl BuildAgentPlatformCustomModel {
    pub fn build(self, stack: &mut terrars::Stack) -> AgentPlatformCustomModel {
        let out = AgentPlatformCustomModel(std::rc::Rc::new(AgentPlatformCustomModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgentPlatformCustomModelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                accept_terms_and_conditions: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                input_modalities: core::default::Default::default(),
                license: core::default::Default::default(),
                name: self.name,
                output_modalities: core::default::Default::default(),
                parameters: core::default::Default::default(),
                preferred_gpu_region: core::default::Default::default(),
                source_type: self.source_type,
                tags: core::default::Default::default(),
                source_ref: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgentPlatformCustomModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformCustomModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgentPlatformCustomModelRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accept_terms_and_conditions` after provisioning.\nWhether the caller accepts the model provider's terms and conditions. Write-only."]
    pub fn accept_terms_and_conditions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accept_terms_and_conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `active_deployments` after provisioning.\nActive dedicated inference deployments referencing this custom model."]
    pub fn active_deployments(&self) -> terrars::ListRef<AgentPlatformCustomModel_ActiveDeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_deployments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `architecture` after provisioning.\nModel architecture as reported by the importer."]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `context_length` after provisioning.\nMaximum context length supported by the model."]
    pub fn context_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context_length", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cost_estimate_per_month` after provisioning.\nEstimated monthly cost of running the custom model."]
    pub fn cost_estimate_per_month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cost_estimate_per_month", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nTimestamp when the custom model was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the custom model."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nError message if the custom model import failed."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `file_count` after provisioning.\nNumber of files that make up the imported model."]
    pub fn file_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `input_modalities` after provisioning.\nInput modalities supported by the model. Defaults to the values reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn input_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license` after provisioning.\nLicense of the model. Defaults to the value reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn license(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-readable name for the custom model."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_modalities` after provisioning.\nOutput modalities produced by the model. Defaults to the values reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn output_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nParameter-count summary for the model. Defaults to the value reported by the importer. Caller-supplied overrides are honored only for SOURCE_TYPE_SPACES_BUCKET imports."]
    pub fn parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preferred_gpu_region` after provisioning.\nPreferred GPU region where the model artifacts should be staged."]
    pub fn preferred_gpu_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_gpu_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nSource of the model to import. One of SOURCE_TYPE_HUGGINGFACE, SOURCE_TYPE_SPACES_BUCKET, SOURCE_TYPE_SDK_UPLOAD, SOURCE_TYPE_FINE_TUNING."]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nCurrent status of the custom model."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_region` after provisioning.\nRegion where the custom model artifacts are stored."]
    pub fn storage_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nUser-defined tags associated with the custom model."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `team_id` after provisioning.\nID of the team that owns the custom model."]
    pub fn team_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.team_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `total_size_bytes` after provisioning.\nTotal size of the imported model artifacts in bytes."]
    pub fn total_size_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the custom model was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the custom model."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ref` after provisioning.\n"]
    pub fn source_ref(&self) -> terrars::ListRef<AgentPlatformCustomModel_SourceRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ref", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AgentPlatformCustomModel_TimeoutsRef {
        AgentPlatformCustomModel_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AgentPlatformCustomModelDynamic {
    source_ref: std::option::Option<terrars::DynamicBlock<AgentPlatformCustomModel_SourceRef>>,
}

#[derive(Serialize)]
pub struct AgentPlatformCustomModel_ActiveDeployments_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_endpoint_fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformCustomModel_ActiveDeployments_Endpoints {
    #[doc = "Set the field `private_endpoint_fqdn`.\n"]
    pub fn set_private_endpoint_fqdn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_endpoint_fqdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_endpoint_fqdn`.\n"]
    pub fn set_public_endpoint_fqdn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_endpoint_fqdn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformCustomModel_ActiveDeployments_Endpoints {
    type O = terrars::BlockAssignable<AgentPlatformCustomModel_ActiveDeployments_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformCustomModel_ActiveDeployments_Endpoints {}

impl BuildAgentPlatformCustomModel_ActiveDeployments_Endpoints {
    pub fn build(self) -> AgentPlatformCustomModel_ActiveDeployments_Endpoints {
        AgentPlatformCustomModel_ActiveDeployments_Endpoints {
            private_endpoint_fqdn: core::default::Default::default(),
            public_endpoint_fqdn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
        AgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_endpoint_fqdn` after provisioning.\n"]
    pub fn private_endpoint_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_fqdn", self.base))
    }

    #[doc = "Get a reference to the value of field `public_endpoint_fqdn` after provisioning.\n"]
    pub fn public_endpoint_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_fqdn", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformCustomModel_ActiveDeployments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<AgentPlatformCustomModel_ActiveDeployments_Endpoints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformCustomModel_ActiveDeployments {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<AgentPlatformCustomModel_ActiveDeployments_Endpoints>>,
    ) -> Self {
        self.endpoints = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_slug`.\n"]
    pub fn set_region_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformCustomModel_ActiveDeployments {
    type O = terrars::BlockAssignable<AgentPlatformCustomModel_ActiveDeployments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformCustomModel_ActiveDeployments {}

impl BuildAgentPlatformCustomModel_ActiveDeployments {
    pub fn build(self) -> AgentPlatformCustomModel_ActiveDeployments {
        AgentPlatformCustomModel_ActiveDeployments {
            created_at: core::default::Default::default(),
            endpoints: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            region_slug: core::default::Default::default(),
            state: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformCustomModel_ActiveDeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformCustomModel_ActiveDeploymentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformCustomModel_ActiveDeploymentsRef {
        AgentPlatformCustomModel_ActiveDeploymentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformCustomModel_ActiveDeploymentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<AgentPlatformCustomModel_ActiveDeployments_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region_slug` after provisioning.\n"]
    pub fn region_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformCustomModel_SourceRef {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hf_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformCustomModel_SourceRef {
    #[doc =
        "Set the field `access_type`.\nAccess type for the source repository. One of ACCESS_TYPE_PUBLIC, ACCESS_TYPE_PRIVATE, ACCESS_TYPE_GATED."]
    pub fn set_access_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\nSpaces bucket name for SOURCE_TYPE_SPACES_BUCKET sources."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `commit_sha`.\nCommit SHA to pin for the import. If omitted, the API resolves and returns the SHA actually imported."]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hf_token`.\nHuggingFace token used to access ACCESS_TYPE_PRIVATE or ACCESS_TYPE_GATED repositories. Write-only."]
    pub fn set_hf_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hf_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\nKey prefix inside the source bucket."]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion of the source bucket."]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `repo_id`.\nRepository identifier (e.g. the HuggingFace repo). Required for SOURCE_TYPE_HUGGINGFACE sources."]
    pub fn set_repo_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformCustomModel_SourceRef {
    type O = terrars::BlockAssignable<AgentPlatformCustomModel_SourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformCustomModel_SourceRef {}

impl BuildAgentPlatformCustomModel_SourceRef {
    pub fn build(self) -> AgentPlatformCustomModel_SourceRef {
        AgentPlatformCustomModel_SourceRef {
            access_type: core::default::Default::default(),
            bucket: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            hf_token: core::default::Default::default(),
            prefix: core::default::Default::default(),
            region: core::default::Default::default(),
            repo_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformCustomModel_SourceRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformCustomModel_SourceRefRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformCustomModel_SourceRefRef {
        AgentPlatformCustomModel_SourceRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformCustomModel_SourceRefRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_type` after provisioning.\nAccess type for the source repository. One of ACCESS_TYPE_PUBLIC, ACCESS_TYPE_PRIVATE, ACCESS_TYPE_GATED."]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nSpaces bucket name for SOURCE_TYPE_SPACES_BUCKET sources."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `commit_sha` after provisioning.\nCommit SHA to pin for the import. If omitted, the API resolves and returns the SHA actually imported."]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hf_token` after provisioning.\nHuggingFace token used to access ACCESS_TYPE_PRIVATE or ACCESS_TYPE_GATED repositories. Write-only."]
    pub fn hf_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hf_token", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\nKey prefix inside the source bucket."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the source bucket."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_id` after provisioning.\nRepository identifier (e.g. the HuggingFace repo). Required for SOURCE_TYPE_HUGGINGFACE sources."]
    pub fn repo_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformCustomModel_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformCustomModel_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformCustomModel_Timeouts {
    type O = terrars::BlockAssignable<AgentPlatformCustomModel_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformCustomModel_Timeouts {}

impl BuildAgentPlatformCustomModel_Timeouts {
    pub fn build(self) -> AgentPlatformCustomModel_Timeouts {
        AgentPlatformCustomModel_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformCustomModel_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformCustomModel_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AgentPlatformCustomModel_TimeoutsRef {
        AgentPlatformCustomModel_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformCustomModel_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
