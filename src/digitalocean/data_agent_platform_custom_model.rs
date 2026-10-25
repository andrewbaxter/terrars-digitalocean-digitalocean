#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataAgentPlatformCustomModelData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    uuid: terrars::PrimField<std::string::String>,
}

struct DataAgentPlatformCustomModel_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAgentPlatformCustomModelData_>,
}

#[derive(Clone)]
pub struct DataAgentPlatformCustomModel(std::rc::Rc<DataAgentPlatformCustomModel_>);

impl DataAgentPlatformCustomModel {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `active_deployments` after provisioning.\nActive dedicated inference deployments referencing this custom model."]
    pub fn active_deployments(&self) -> terrars::ListRef<DataAgentPlatformCustomModel_ActiveDeploymentsRef> {
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
        "Get a reference to the value of field `input_modalities` after provisioning.\nInput modalities supported by the model (e.g. text, image)."]
    pub fn input_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license` after provisioning.\nLicense of the custom model, as reported by the source."]
    pub fn license(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nHuman-readable name of the custom model."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_modalities` after provisioning.\nOutput modalities produced by the model."]
    pub fn output_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nParameter-count summary reported by the importer."]
    pub fn parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_ref` after provisioning.\nReference to the source from which the custom model was imported."]
    pub fn source_ref(&self) -> terrars::ListRef<DataAgentPlatformCustomModel_SourceRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ref", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nSource type of the custom model (e.g. SOURCE_TYPE_HUGGINGFACE, SOURCE_TYPE_SPACES_BUCKET)."]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent status of the custom model (e.g. STATUS_IMPORTING, STATUS_READY, STATUS_FAILED)."]
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
        "Get a reference to the value of field `total_size_bytes` after provisioning.\nTotal size of the imported model artifacts in bytes (string-encoded int64)."]
    pub fn total_size_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the custom model was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the custom model to look up."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataAgentPlatformCustomModel {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAgentPlatformCustomModel { }

impl terrars::ToListMappable for DataAgentPlatformCustomModel {
    type O = terrars::ListRef<DataAgentPlatformCustomModelRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAgentPlatformCustomModel_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_custom_model".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAgentPlatformCustomModel {
    pub tf_id: std::string::String,
    #[doc = "UUID of the custom model to look up."]
    pub uuid: terrars::PrimField<std::string::String>,
}

impl BuildDataAgentPlatformCustomModel {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAgentPlatformCustomModel {
        let out = DataAgentPlatformCustomModel(std::rc::Rc::new(DataAgentPlatformCustomModel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAgentPlatformCustomModelData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                uuid: self.uuid,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAgentPlatformCustomModelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformCustomModelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAgentPlatformCustomModelRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `active_deployments` after provisioning.\nActive dedicated inference deployments referencing this custom model."]
    pub fn active_deployments(&self) -> terrars::ListRef<DataAgentPlatformCustomModel_ActiveDeploymentsRef> {
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
        "Get a reference to the value of field `input_modalities` after provisioning.\nInput modalities supported by the model (e.g. text, image)."]
    pub fn input_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `license` after provisioning.\nLicense of the custom model, as reported by the source."]
    pub fn license(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nHuman-readable name of the custom model."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_modalities` after provisioning.\nOutput modalities produced by the model."]
    pub fn output_modalities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_modalities", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nParameter-count summary reported by the importer."]
    pub fn parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_ref` after provisioning.\nReference to the source from which the custom model was imported."]
    pub fn source_ref(&self) -> terrars::ListRef<DataAgentPlatformCustomModel_SourceRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_ref", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nSource type of the custom model (e.g. SOURCE_TYPE_HUGGINGFACE, SOURCE_TYPE_SPACES_BUCKET)."]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nCurrent status of the custom model (e.g. STATUS_IMPORTING, STATUS_READY, STATUS_FAILED)."]
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
        "Get a reference to the value of field `total_size_bytes` after provisioning.\nTotal size of the imported model artifacts in bytes (string-encoded int64)."]
    pub fn total_size_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_size_bytes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nTimestamp when the custom model was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\nUUID of the custom model to look up."]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAgentPlatformCustomModel_ActiveDeployments_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_endpoint_fqdn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformCustomModel_ActiveDeployments_Endpoints {
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

impl terrars::ToListMappable for DataAgentPlatformCustomModel_ActiveDeployments_Endpoints {
    type O = terrars::BlockAssignable<DataAgentPlatformCustomModel_ActiveDeployments_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformCustomModel_ActiveDeployments_Endpoints {}

impl BuildDataAgentPlatformCustomModel_ActiveDeployments_Endpoints {
    pub fn build(self) -> DataAgentPlatformCustomModel_ActiveDeployments_Endpoints {
        DataAgentPlatformCustomModel_ActiveDeployments_Endpoints {
            private_endpoint_fqdn: core::default::Default::default(),
            public_endpoint_fqdn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
        DataAgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformCustomModel_ActiveDeployments_EndpointsRef {
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
pub struct DataAgentPlatformCustomModel_ActiveDeployments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints: std::option::Option<terrars::ListField<DataAgentPlatformCustomModel_ActiveDeployments_Endpoints>>,
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

impl DataAgentPlatformCustomModel_ActiveDeployments {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoints`.\n"]
    pub fn set_endpoints(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAgentPlatformCustomModel_ActiveDeployments_Endpoints>>,
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

impl terrars::ToListMappable for DataAgentPlatformCustomModel_ActiveDeployments {
    type O = terrars::BlockAssignable<DataAgentPlatformCustomModel_ActiveDeployments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformCustomModel_ActiveDeployments {}

impl BuildDataAgentPlatformCustomModel_ActiveDeployments {
    pub fn build(self) -> DataAgentPlatformCustomModel_ActiveDeployments {
        DataAgentPlatformCustomModel_ActiveDeployments {
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
pub struct DataAgentPlatformCustomModel_ActiveDeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformCustomModel_ActiveDeploymentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAgentPlatformCustomModel_ActiveDeploymentsRef {
        DataAgentPlatformCustomModel_ActiveDeploymentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformCustomModel_ActiveDeploymentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<DataAgentPlatformCustomModel_ActiveDeployments_EndpointsRef> {
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
pub struct DataAgentPlatformCustomModel_SourceRef {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAgentPlatformCustomModel_SourceRef {
    #[doc = "Set the field `access_type`.\n"]
    pub fn set_access_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `commit_sha`.\n"]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_id`.\n"]
    pub fn set_repo_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAgentPlatformCustomModel_SourceRef {
    type O = terrars::BlockAssignable<DataAgentPlatformCustomModel_SourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAgentPlatformCustomModel_SourceRef {}

impl BuildDataAgentPlatformCustomModel_SourceRef {
    pub fn build(self) -> DataAgentPlatformCustomModel_SourceRef {
        DataAgentPlatformCustomModel_SourceRef {
            access_type: core::default::Default::default(),
            bucket: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            prefix: core::default::Default::default(),
            region: core::default::Default::default(),
            repo_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAgentPlatformCustomModel_SourceRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAgentPlatformCustomModel_SourceRefRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAgentPlatformCustomModel_SourceRefRef {
        DataAgentPlatformCustomModel_SourceRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAgentPlatformCustomModel_SourceRefRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_type` after provisioning.\n"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\n"]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `repo_id` after provisioning.\n"]
    pub fn repo_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_id", self.base))
    }
}
