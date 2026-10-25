#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDedicatedInferenceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
}

struct DataDedicatedInference_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDedicatedInferenceData_>,
}

#[derive(Clone)]
pub struct DataDedicatedInference(std::rc::Rc<DataDedicatedInference_>);

impl DataDedicatedInference {
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

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the dedicated inference endpoint was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_endpoint` after provisioning.\nWhether the public HTTPS endpoint is enabled."]
    pub fn enable_public_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the dedicated inference endpoint."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `model_deployments` after provisioning.\nThe list of model deployments running on the dedicated inference endpoint."]
    pub fn model_deployments(&self) -> terrars::ListRef<DataDedicatedInference_ModelDeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_deployments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the dedicated inference endpoint."]
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
        "Get a reference to the value of field `region` after provisioning.\nThe region where the dedicated inference endpoint is deployed."]
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
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nThe UUID of the VPC the dedicated inference endpoint is deployed in."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataDedicatedInference {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDedicatedInference { }

impl terrars::ToListMappable for DataDedicatedInference {
    type O = terrars::ListRef<DataDedicatedInferenceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDedicatedInference_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_dedicated_inference".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDedicatedInference {
    pub tf_id: std::string::String,
    #[doc = "The ID of the dedicated inference endpoint."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataDedicatedInference {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDedicatedInference {
        let out = DataDedicatedInference(std::rc::Rc::new(DataDedicatedInference_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDedicatedInferenceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDedicatedInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDedicatedInferenceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the dedicated inference endpoint was created."]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_endpoint` after provisioning.\nWhether the public HTTPS endpoint is enabled."]
    pub fn enable_public_endpoint(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe ID of the dedicated inference endpoint."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `model_deployments` after provisioning.\nThe list of model deployments running on the dedicated inference endpoint."]
    pub fn model_deployments(&self) -> terrars::ListRef<DataDedicatedInference_ModelDeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_deployments", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the dedicated inference endpoint."]
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
        "Get a reference to the value of field `region` after provisioning.\nThe region where the dedicated inference endpoint is deployed."]
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
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nThe UUID of the VPC the dedicated inference endpoint is deployed in."]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInference_ModelDeployments_Accelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInference_ModelDeployments_Accelerators {
    #[doc = "Set the field `accelerator_slug`.\n"]
    pub fn set_accelerator_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale`.\n"]
    pub fn set_scale(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInference_ModelDeployments_Accelerators {
    type O = terrars::BlockAssignable<DataDedicatedInference_ModelDeployments_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInference_ModelDeployments_Accelerators {}

impl BuildDataDedicatedInference_ModelDeployments_Accelerators {
    pub fn build(self) -> DataDedicatedInference_ModelDeployments_Accelerators {
        DataDedicatedInference_ModelDeployments_Accelerators {
            accelerator_slug: core::default::Default::default(),
            scale: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInference_ModelDeployments_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInference_ModelDeployments_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDedicatedInference_ModelDeployments_AcceleratorsRef {
        DataDedicatedInference_ModelDeployments_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInference_ModelDeployments_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_slug` after provisioning.\n"]
    pub fn accelerator_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInference_ModelDeployments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<terrars::ListField<DataDedicatedInference_ModelDeployments_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_model_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInference_ModelDeployments {
    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDedicatedInference_ModelDeployments_Accelerators>>,
    ) -> Self {
        self.accelerators = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_id`.\n"]
    pub fn set_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_provider`.\n"]
    pub fn set_model_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_slug`.\n"]
    pub fn set_model_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider_model_id`.\n"]
    pub fn set_provider_model_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.provider_model_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInference_ModelDeployments {
    type O = terrars::BlockAssignable<DataDedicatedInference_ModelDeployments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInference_ModelDeployments {}

impl BuildDataDedicatedInference_ModelDeployments {
    pub fn build(self) -> DataDedicatedInference_ModelDeployments {
        DataDedicatedInference_ModelDeployments {
            accelerators: core::default::Default::default(),
            model_id: core::default::Default::default(),
            model_provider: core::default::Default::default(),
            model_slug: core::default::Default::default(),
            provider_model_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInference_ModelDeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInference_ModelDeploymentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDedicatedInference_ModelDeploymentsRef {
        DataDedicatedInference_ModelDeploymentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInference_ModelDeploymentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DataDedicatedInference_ModelDeployments_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `model_provider` after provisioning.\n"]
    pub fn model_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `model_slug` after provisioning.\n"]
    pub fn model_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_model_id` after provisioning.\n"]
    pub fn provider_model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_model_id", self.base))
    }
}
