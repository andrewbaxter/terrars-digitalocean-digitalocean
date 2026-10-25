#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DedicatedInferenceData_ {
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
    enable_public_endpoint: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hugging_face_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_deployments: std::option::Option<std::vec::Vec<DedicatedInference_ModelDeployments>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DedicatedInference_Timeouts>,
    dynamic: DedicatedInferenceDynamic,
}

struct DedicatedInference_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DedicatedInferenceData_>,
}

#[derive(Clone)]
pub struct DedicatedInference(std::rc::Rc<DedicatedInference_>);

impl DedicatedInference {
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
        "Set the field `enable_public_endpoint`.\nWhether to enable a public HTTPS endpoint for the dedicated inference endpoint. This field is immutable after creation."]
    pub fn set_enable_public_endpoint(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_public_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hugging_face_token`.\nA HuggingFace token for accessing gated models."]
    pub fn set_hugging_face_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hugging_face_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\nThe UUID of the VPC to deploy the dedicated inference endpoint into."]
    pub fn set_vpc_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_deployments`.\n"]
    pub fn set_model_deployments(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DedicatedInference_ModelDeployments>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_deployments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_deployments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DedicatedInference_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the dedicated inference endpoint was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_endpoint` after provisioning.\nWhether to enable a public HTTPS endpoint for the dedicated inference endpoint. This field is immutable after creation."]
    pub fn enable_public_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hugging_face_token` after provisioning.\nA HuggingFace token for accessing gated models."]
    pub fn hugging_face_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugging_face_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-readable name for the dedicated inference endpoint."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_fqdn` after provisioning.\nThe fully-qualified domain name of the private endpoint."]
    pub fn private_endpoint_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_fqdn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_endpoint_fqdn` after provisioning.\nThe fully-qualified domain name of the public endpoint, if enabled."]
    pub fn public_endpoint_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_fqdn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region slug where the dedicated inference endpoint will be deployed."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the dedicated inference endpoint."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time when the dedicated inference endpoint was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nThe UUID of the VPC to deploy the dedicated inference endpoint into."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_deployments` after provisioning.\n"]
    pub fn model_deployments(&self) -> terrars::ListRef<DedicatedInference_ModelDeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_deployments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DedicatedInference_TimeoutsRef {
        DedicatedInference_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DedicatedInference {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DedicatedInference { }

impl terrars::ToListMappable for DedicatedInference {
    type O = terrars::ListRef<DedicatedInferenceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DedicatedInference_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_dedicated_inference".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDedicatedInference {
    pub tf_id: std::string::String,
    #[doc = "A human-readable name for the dedicated inference endpoint."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The region slug where the dedicated inference endpoint will be deployed."]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildDedicatedInference {
    pub fn build(self, stack: &mut terrars::Stack) -> DedicatedInference {
        let out = DedicatedInference(std::rc::Rc::new(DedicatedInference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DedicatedInferenceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enable_public_endpoint: core::default::Default::default(),
                hugging_face_token: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: self.region,
                vpc_uuid: core::default::Default::default(),
                model_deployments: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DedicatedInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DedicatedInferenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DedicatedInferenceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the dedicated inference endpoint was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_endpoint` after provisioning.\nWhether to enable a public HTTPS endpoint for the dedicated inference endpoint. This field is immutable after creation."]
    pub fn enable_public_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hugging_face_token` after provisioning.\nA HuggingFace token for accessing gated models."]
    pub fn hugging_face_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hugging_face_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA human-readable name for the dedicated inference endpoint."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_endpoint_fqdn` after provisioning.\nThe fully-qualified domain name of the private endpoint."]
    pub fn private_endpoint_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_endpoint_fqdn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_endpoint_fqdn` after provisioning.\nThe fully-qualified domain name of the public endpoint, if enabled."]
    pub fn public_endpoint_fqdn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_fqdn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region slug where the dedicated inference endpoint will be deployed."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe current status of the dedicated inference endpoint."]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nThe date and time when the dedicated inference endpoint was last updated."]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nThe UUID of the VPC to deploy the dedicated inference endpoint into."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_deployments` after provisioning.\n"]
    pub fn model_deployments(&self) -> terrars::ListRef<DedicatedInference_ModelDeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_deployments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DedicatedInference_TimeoutsRef {
        DedicatedInference_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DedicatedInferenceDynamic {
    model_deployments: std::option::Option<terrars::DynamicBlock<DedicatedInference_ModelDeployments>>,
}

#[derive(Serialize)]
pub struct DedicatedInference_ModelDeployments_Accelerators {
    accelerator_slug: terrars::PrimField<std::string::String>,
    scale: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DedicatedInference_ModelDeployments_Accelerators { }

impl terrars::ToListMappable for DedicatedInference_ModelDeployments_Accelerators {
    type O = terrars::BlockAssignable<DedicatedInference_ModelDeployments_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDedicatedInference_ModelDeployments_Accelerators {
    #[doc = "The slug identifier for the GPU accelerator type."]
    pub accelerator_slug: terrars::PrimField<std::string::String>,
    #[doc = "The number of accelerator units to allocate."]
    pub scale: terrars::PrimField<f64>,
    #[doc = "The accelerator type."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDedicatedInference_ModelDeployments_Accelerators {
    pub fn build(self) -> DedicatedInference_ModelDeployments_Accelerators {
        DedicatedInference_ModelDeployments_Accelerators {
            accelerator_slug: self.accelerator_slug,
            scale: self.scale,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DedicatedInference_ModelDeployments_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DedicatedInference_ModelDeployments_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DedicatedInference_ModelDeployments_AcceleratorsRef {
        DedicatedInference_ModelDeployments_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DedicatedInference_ModelDeployments_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_slug` after provisioning.\nThe slug identifier for the GPU accelerator type."]
    pub fn accelerator_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scale` after provisioning.\nThe number of accelerator units to allocate."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nThe accelerator type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DedicatedInference_ModelDeploymentsDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<DedicatedInference_ModelDeployments_Accelerators>>,
}

#[derive(Serialize)]
pub struct DedicatedInference_ModelDeployments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    model_provider: terrars::PrimField<std::string::String>,
    model_slug: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<DedicatedInference_ModelDeployments_Accelerators>>,
    dynamic: DedicatedInference_ModelDeploymentsDynamic,
}

impl DedicatedInference_ModelDeployments {
    #[doc = "Set the field `model_id`.\nThe unique ID of the model."]
    pub fn set_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provider_model_id`.\nThe provider-specific model ID. Required when model_provider is 'hugging_face', optional for 'modelcatalog'."]
    pub fn set_provider_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider_model_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DedicatedInference_ModelDeployments_Accelerators>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DedicatedInference_ModelDeployments {
    type O = terrars::BlockAssignable<DedicatedInference_ModelDeployments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDedicatedInference_ModelDeployments {
    #[doc = "The provider of the model (e.g. 'modelcatalog', 'hugging_face')."]
    pub model_provider: terrars::PrimField<std::string::String>,
    #[doc = "The slug identifier for the model to deploy."]
    pub model_slug: terrars::PrimField<std::string::String>,
}

impl BuildDedicatedInference_ModelDeployments {
    pub fn build(self) -> DedicatedInference_ModelDeployments {
        DedicatedInference_ModelDeployments {
            model_id: core::default::Default::default(),
            model_provider: self.model_provider,
            model_slug: self.model_slug,
            provider_model_id: core::default::Default::default(),
            accelerators: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DedicatedInference_ModelDeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DedicatedInference_ModelDeploymentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DedicatedInference_ModelDeploymentsRef {
        DedicatedInference_ModelDeploymentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DedicatedInference_ModelDeploymentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\nThe unique ID of the model."]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_provider` after provisioning.\nThe provider of the model (e.g. 'modelcatalog', 'hugging_face')."]
    pub fn model_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_provider", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_slug` after provisioning.\nThe slug identifier for the model to deploy."]
    pub fn model_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_slug", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provider_model_id` after provisioning.\nThe provider-specific model ID. Required when model_provider is 'hugging_face', optional for 'modelcatalog'."]
    pub fn provider_model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DedicatedInference_ModelDeployments_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }
}

#[derive(Serialize)]
pub struct DedicatedInference_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DedicatedInference_Timeouts {
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

impl terrars::ToListMappable for DedicatedInference_Timeouts {
    type O = terrars::BlockAssignable<DedicatedInference_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDedicatedInference_Timeouts {}

impl BuildDedicatedInference_Timeouts {
    pub fn build(self) -> DedicatedInference_Timeouts {
        DedicatedInference_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DedicatedInference_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DedicatedInference_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DedicatedInference_TimeoutsRef {
        DedicatedInference_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DedicatedInference_TimeoutsRef {
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
