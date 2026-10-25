#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDropletAutoscaleData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDropletAutoscale_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDropletAutoscaleData_>,
}

#[derive(Clone)]
pub struct DataDropletAutoscale(std::rc::Rc<DataDropletAutoscale_>);

impl DataDropletAutoscale {
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

    #[doc = "Set the field `id`.\nID of the Droplet autoscale pool"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the Droplet autoscale pool"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DataDropletAutoscale_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDroplet autoscale pool create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_utilization` after provisioning.\n"]
    pub fn current_utilization(&self) -> terrars::ListRef<DataDropletAutoscale_CurrentUtilizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_utilization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_template` after provisioning.\n"]
    pub fn droplet_template(&self) -> terrars::ListRef<DataDropletAutoscale_DropletTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.droplet_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the Droplet autoscale pool"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Droplet autoscale pool"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nDroplet autoscale pool status"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nDroplet autoscale pool update timestamp"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

impl terrars::Referable for DataDropletAutoscale {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDropletAutoscale { }

impl terrars::ToListMappable for DataDropletAutoscale {
    type O = terrars::ListRef<DataDropletAutoscaleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDropletAutoscale_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_droplet_autoscale".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDropletAutoscale {
    pub tf_id: std::string::String,
}

impl BuildDataDropletAutoscale {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDropletAutoscale {
        let out = DataDropletAutoscale(std::rc::Rc::new(DataDropletAutoscale_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDropletAutoscaleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDropletAutoscaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDropletAutoscaleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDropletAutoscaleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DataDropletAutoscale_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDroplet autoscale pool create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_utilization` after provisioning.\n"]
    pub fn current_utilization(&self) -> terrars::ListRef<DataDropletAutoscale_CurrentUtilizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_utilization", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_template` after provisioning.\n"]
    pub fn droplet_template(&self) -> terrars::ListRef<DataDropletAutoscale_DropletTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.droplet_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the Droplet autoscale pool"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the Droplet autoscale pool"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nDroplet autoscale pool status"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `updated_at` after provisioning.\nDroplet autoscale pool update timestamp"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDropletAutoscale_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cooldown_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_cpu_utilization: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_memory_utilization: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_number_instances: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDropletAutoscale_Config {
    #[doc = "Set the field `cooldown_minutes`.\n"]
    pub fn set_cooldown_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooldown_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_instances`.\n"]
    pub fn set_max_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instances`.\n"]
    pub fn set_min_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_cpu_utilization`.\n"]
    pub fn set_target_cpu_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_cpu_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_memory_utilization`.\n"]
    pub fn set_target_memory_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_memory_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_number_instances`.\n"]
    pub fn set_target_number_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_number_instances = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDropletAutoscale_Config {
    type O = terrars::BlockAssignable<DataDropletAutoscale_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDropletAutoscale_Config {}

impl BuildDataDropletAutoscale_Config {
    pub fn build(self) -> DataDropletAutoscale_Config {
        DataDropletAutoscale_Config {
            cooldown_minutes: core::default::Default::default(),
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            target_cpu_utilization: core::default::Default::default(),
            target_memory_utilization: core::default::Default::default(),
            target_number_instances: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDropletAutoscale_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDropletAutoscale_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDropletAutoscale_ConfigRef {
        DataDropletAutoscale_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDropletAutoscale_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cooldown_minutes` after provisioning.\n"]
    pub fn cooldown_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\n"]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\n"]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `target_cpu_utilization` after provisioning.\n"]
    pub fn target_cpu_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_cpu_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `target_memory_utilization` after provisioning.\n"]
    pub fn target_memory_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_memory_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `target_number_instances` after provisioning.\n"]
    pub fn target_number_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_number_instances", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDropletAutoscale_CurrentUtilization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDropletAutoscale_CurrentUtilization {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDropletAutoscale_CurrentUtilization {
    type O = terrars::BlockAssignable<DataDropletAutoscale_CurrentUtilization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDropletAutoscale_CurrentUtilization {}

impl BuildDataDropletAutoscale_CurrentUtilization {
    pub fn build(self) -> DataDropletAutoscale_CurrentUtilization {
        DataDropletAutoscale_CurrentUtilization {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDropletAutoscale_CurrentUtilizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDropletAutoscale_CurrentUtilizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDropletAutoscale_CurrentUtilizationRef {
        DataDropletAutoscale_CurrentUtilizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDropletAutoscale_CurrentUtilizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDropletAutoscale_DropletTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_droplet_agent: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDropletAutoscale_DropletTemplate {
    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\n"]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_networking`.\n"]
    pub fn set_public_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.public_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_keys`.\n"]
    pub fn set_ssh_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ssh_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_droplet_agent`.\n"]
    pub fn set_with_droplet_agent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.with_droplet_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDropletAutoscale_DropletTemplate {
    type O = terrars::BlockAssignable<DataDropletAutoscale_DropletTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDropletAutoscale_DropletTemplate {}

impl BuildDataDropletAutoscale_DropletTemplate {
    pub fn build(self) -> DataDropletAutoscale_DropletTemplate {
        DataDropletAutoscale_DropletTemplate {
            image: core::default::Default::default(),
            ipv6: core::default::Default::default(),
            project_id: core::default::Default::default(),
            public_networking: core::default::Default::default(),
            region: core::default::Default::default(),
            size: core::default::Default::default(),
            ssh_keys: core::default::Default::default(),
            tags: core::default::Default::default(),
            user_data: core::default::Default::default(),
            vpc_uuid: core::default::Default::default(),
            with_droplet_agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDropletAutoscale_DropletTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDropletAutoscale_DropletTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDropletAutoscale_DropletTemplateRef {
        DataDropletAutoscale_DropletTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDropletAutoscale_DropletTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_networking` after provisioning.\n"]
    pub fn public_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_networking", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssh_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `with_droplet_agent` after provisioning.\n"]
    pub fn with_droplet_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_droplet_agent", self.base))
    }
}
