#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DropletAutoscaleData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<DropletAutoscale_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_template: std::option::Option<std::vec::Vec<DropletAutoscale_DropletTemplate>>,
    dynamic: DropletAutoscaleDynamic,
}

struct DropletAutoscale_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DropletAutoscaleData_>,
}

#[derive(Clone)]
pub struct DropletAutoscale(std::rc::Rc<DropletAutoscale_>);

impl DropletAutoscale {
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

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(self, v: impl std::convert::Into<terrars::BlockAssignable<DropletAutoscale_Config>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `droplet_template`.\n"]
    pub fn set_droplet_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DropletAutoscale_DropletTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().droplet_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.droplet_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDroplet autoscale pool create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_utilization` after provisioning.\n"]
    pub fn current_utilization(&self) -> terrars::ListRef<DropletAutoscale_CurrentUtilizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_utilization", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DropletAutoscale_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_template` after provisioning.\n"]
    pub fn droplet_template(&self) -> terrars::ListRef<DropletAutoscale_DropletTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.droplet_template", self.extract_ref()))
    }
}

impl terrars::Referable for DropletAutoscale {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DropletAutoscale { }

impl terrars::ToListMappable for DropletAutoscale {
    type O = terrars::ListRef<DropletAutoscaleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DropletAutoscale_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_droplet_autoscale".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDropletAutoscale {
    pub tf_id: std::string::String,
    #[doc = "Name of the Droplet autoscale pool"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDropletAutoscale {
    pub fn build(self, stack: &mut terrars::Stack) -> DropletAutoscale {
        let out = DropletAutoscale(std::rc::Rc::new(DropletAutoscale_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DropletAutoscaleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                config: core::default::Default::default(),
                droplet_template: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DropletAutoscaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DropletAutoscaleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DropletAutoscaleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nDroplet autoscale pool create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_utilization` after provisioning.\n"]
    pub fn current_utilization(&self) -> terrars::ListRef<DropletAutoscale_CurrentUtilizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.current_utilization", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DropletAutoscale_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_template` after provisioning.\n"]
    pub fn droplet_template(&self) -> terrars::ListRef<DropletAutoscale_DropletTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.droplet_template", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DropletAutoscaleDynamic {
    config: std::option::Option<terrars::DynamicBlock<DropletAutoscale_Config>>,
    droplet_template: std::option::Option<terrars::DynamicBlock<DropletAutoscale_DropletTemplate>>,
}

#[derive(Serialize)]
pub struct DropletAutoscale_CurrentUtilization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
}

impl DropletAutoscale_CurrentUtilization {
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

impl terrars::ToListMappable for DropletAutoscale_CurrentUtilization {
    type O = terrars::BlockAssignable<DropletAutoscale_CurrentUtilization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDropletAutoscale_CurrentUtilization {}

impl BuildDropletAutoscale_CurrentUtilization {
    pub fn build(self) -> DropletAutoscale_CurrentUtilization {
        DropletAutoscale_CurrentUtilization {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DropletAutoscale_CurrentUtilizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DropletAutoscale_CurrentUtilizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DropletAutoscale_CurrentUtilizationRef {
        DropletAutoscale_CurrentUtilizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DropletAutoscale_CurrentUtilizationRef {
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
pub struct DropletAutoscale_Config {
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

impl DropletAutoscale_Config {
    #[doc = "Set the field `cooldown_minutes`.\nCooldown duration"]
    pub fn set_cooldown_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cooldown_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_instances`.\nMax number of members"]
    pub fn set_max_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_instances`.\nMin number of members"]
    pub fn set_min_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_cpu_utilization`.\nCPU target threshold"]
    pub fn set_target_cpu_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_cpu_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_memory_utilization`.\nMemory target threshold"]
    pub fn set_target_memory_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_memory_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_number_instances`.\nTarget number of members"]
    pub fn set_target_number_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_number_instances = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DropletAutoscale_Config {
    type O = terrars::BlockAssignable<DropletAutoscale_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDropletAutoscale_Config {}

impl BuildDropletAutoscale_Config {
    pub fn build(self) -> DropletAutoscale_Config {
        DropletAutoscale_Config {
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
pub struct DropletAutoscale_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DropletAutoscale_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DropletAutoscale_ConfigRef {
        DropletAutoscale_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DropletAutoscale_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cooldown_minutes` after provisioning.\nCooldown duration"]
    pub fn cooldown_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cooldown_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `max_instances` after provisioning.\nMax number of members"]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `min_instances` after provisioning.\nMin number of members"]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `target_cpu_utilization` after provisioning.\nCPU target threshold"]
    pub fn target_cpu_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_cpu_utilization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_memory_utilization` after provisioning.\nMemory target threshold"]
    pub fn target_memory_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_memory_utilization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_number_instances` after provisioning.\nTarget number of members"]
    pub fn target_number_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_number_instances", self.base))
    }
}

#[derive(Serialize)]
pub struct DropletAutoscale_DropletTemplate {
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_networking: std::option::Option<terrars::PrimField<bool>>,
    region: terrars::PrimField<std::string::String>,
    size: terrars::PrimField<std::string::String>,
    ssh_keys: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    with_droplet_agent: std::option::Option<terrars::PrimField<bool>>,
}

impl DropletAutoscale_DropletTemplate {
    #[doc = "Set the field `ipv6`.\nEnable droplet IPv6"]
    pub fn set_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nDroplet project ID"]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_networking`.\nEnables public networking for the Droplet. By default, this is always enabled on new Droplets, but by explicitly setting it to false, you can create a Droplet with public networking entirely disabled."]
    pub fn set_public_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.public_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nDroplet tags"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\nDroplet user data"]
    pub fn set_user_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\nDroplet VPC UUID"]
    pub fn set_vpc_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `with_droplet_agent`.\nEnable droplet agent"]
    pub fn set_with_droplet_agent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.with_droplet_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DropletAutoscale_DropletTemplate {
    type O = terrars::BlockAssignable<DropletAutoscale_DropletTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDropletAutoscale_DropletTemplate {
    #[doc = "Droplet image"]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = "Droplet region"]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = "Droplet size"]
    pub size: terrars::PrimField<std::string::String>,
    #[doc = "Droplet SSH keys"]
    pub ssh_keys: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDropletAutoscale_DropletTemplate {
    pub fn build(self) -> DropletAutoscale_DropletTemplate {
        DropletAutoscale_DropletTemplate {
            image: self.image,
            ipv6: core::default::Default::default(),
            project_id: core::default::Default::default(),
            public_networking: core::default::Default::default(),
            region: self.region,
            size: self.size,
            ssh_keys: self.ssh_keys,
            tags: core::default::Default::default(),
            user_data: core::default::Default::default(),
            vpc_uuid: core::default::Default::default(),
            with_droplet_agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DropletAutoscale_DropletTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DropletAutoscale_DropletTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DropletAutoscale_DropletTemplateRef {
        DropletAutoscale_DropletTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DropletAutoscale_DropletTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nDroplet image"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\nEnable droplet IPv6"]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\nDroplet project ID"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_networking` after provisioning.\nEnables public networking for the Droplet. By default, this is always enabled on new Droplets, but by explicitly setting it to false, you can create a Droplet with public networking entirely disabled."]
    pub fn public_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_networking", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nDroplet region"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nDroplet size"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_keys` after provisioning.\nDroplet SSH keys"]
    pub fn ssh_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssh_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nDroplet tags"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\nDroplet user data"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\nDroplet VPC UUID"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `with_droplet_agent` after provisioning.\nEnable droplet agent"]
    pub fn with_droplet_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.with_droplet_agent", self.base))
    }
}
