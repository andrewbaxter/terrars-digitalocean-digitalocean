#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DropletData_ {
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
    backups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_agent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_partition_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    graceful_shutdown: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resize_disk: std::option::Option<terrars::PrimField<bool>>,
    size: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_policy: std::option::Option<std::vec::Vec<Droplet_BackupPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Droplet_Timeouts>,
    dynamic: DropletDynamic,
}

struct Droplet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DropletData_>,
}

#[derive(Clone)]
pub struct Droplet(std::rc::Rc<Droplet_>);

impl Droplet {
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

    #[doc = "Set the field `backups`.\n"]
    pub fn set_backups(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().backups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `droplet_agent`.\n"]
    pub fn set_droplet_agent(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().droplet_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gpu_partition_mode`.\nThe partition mode for a GPU Droplet. Omit to create a full GPU (equivalent to PARTITION_MODE_SPX_NPS1). Only supported on GPU sizes that advertise the mode in their supported_partition_modes."]
    pub fn set_gpu_partition_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gpu_partition_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `graceful_shutdown`.\n"]
    pub fn set_graceful_shutdown(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().graceful_shutdown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6`.\n"]
    pub fn set_ipv6(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_networking`.\n"]
    pub fn set_private_networking(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_networking`.\nEnables public networking for the Droplet. By default, this is always enabled on new droplets, but by explicitly setting it to false, you can create a droplet with public networking entirely disabled."]
    pub fn set_public_networking(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().public_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resize_disk`.\n"]
    pub fn set_resize_disk(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().resize_disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_keys`.\n"]
    pub fn set_ssh_keys(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ssh_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_ids`.\n"]
    pub fn set_volume_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().volume_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `backup_policy`.\n"]
    pub fn set_backup_policy(self, v: impl std::convert::Into<terrars::BlockAssignable<Droplet_BackupPolicy>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backup_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backup_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Droplet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `backups` after provisioning.\n"]
    pub fn backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_agent` after provisioning.\n"]
    pub fn droplet_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gpu_partition_mode` after provisioning.\nThe partition mode for a GPU Droplet. Omit to create a full GPU (equivalent to PARTITION_MODE_SPX_NPS1). Only supported on GPU sizes that advertise the mode in their supported_partition_modes."]
    pub fn gpu_partition_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `graceful_shutdown` after provisioning.\n"]
    pub fn graceful_shutdown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graceful_shutdown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_address_private` after provisioning.\n"]
    pub fn ipv4_address_private(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_networking` after provisioning.\n"]
    pub fn private_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_networking` after provisioning.\nEnables public networking for the Droplet. By default, this is always enabled on new droplets, but by explicitly setting it to false, you can create a droplet with public networking entirely disabled."]
    pub fn public_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resize_disk` after provisioning.\n"]
    pub fn resize_disk(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resize_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_ids` after provisioning.\n"]
    pub fn volume_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_policy` after provisioning.\n"]
    pub fn backup_policy(&self) -> terrars::ListRef<Droplet_BackupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Droplet_TimeoutsRef {
        Droplet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Droplet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Droplet { }

impl terrars::ToListMappable for Droplet {
    type O = terrars::ListRef<DropletRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Droplet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_droplet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDroplet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<std::string::String>,
}

impl BuildDroplet {
    pub fn build(self, stack: &mut terrars::Stack) -> Droplet {
        let out = Droplet(std::rc::Rc::new(Droplet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DropletData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                backups: core::default::Default::default(),
                droplet_agent: core::default::Default::default(),
                gpu_partition_mode: core::default::Default::default(),
                graceful_shutdown: core::default::Default::default(),
                id: core::default::Default::default(),
                image: self.image,
                ipv6: core::default::Default::default(),
                ipv6_address: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                name: self.name,
                private_networking: core::default::Default::default(),
                public_networking: core::default::Default::default(),
                region: core::default::Default::default(),
                resize_disk: core::default::Default::default(),
                size: self.size,
                ssh_keys: core::default::Default::default(),
                tags: core::default::Default::default(),
                user_data: core::default::Default::default(),
                volume_ids: core::default::Default::default(),
                vpc_uuid: core::default::Default::default(),
                backup_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DropletRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DropletRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DropletRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backups` after provisioning.\n"]
    pub fn backups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_agent` after provisioning.\n"]
    pub fn droplet_agent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gpu_partition_mode` after provisioning.\nThe partition mode for a GPU Droplet. Omit to create a full GPU (equivalent to PARTITION_MODE_SPX_NPS1). Only supported on GPU sizes that advertise the mode in their supported_partition_modes."]
    pub fn gpu_partition_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_partition_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `graceful_shutdown` after provisioning.\n"]
    pub fn graceful_shutdown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.graceful_shutdown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv4_address_private` after provisioning.\n"]
    pub fn ipv4_address_private(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_private", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_networking` after provisioning.\n"]
    pub fn private_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_networking", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `public_networking` after provisioning.\nEnables public networking for the Droplet. By default, this is always enabled on new droplets, but by explicitly setting it to false, you can create a droplet with public networking entirely disabled."]
    pub fn public_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_networking", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resize_disk` after provisioning.\n"]
    pub fn resize_disk(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resize_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_ids` after provisioning.\n"]
    pub fn volume_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.volume_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backup_policy` after provisioning.\n"]
    pub fn backup_policy(&self) -> terrars::ListRef<Droplet_BackupPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Droplet_TimeoutsRef {
        Droplet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DropletDynamic {
    backup_policy: std::option::Option<terrars::DynamicBlock<Droplet_BackupPolicy>>,
}

#[derive(Serialize)]
pub struct Droplet_BackupPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekday: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Droplet_BackupPolicy {
    #[doc = "Set the field `hour`.\n"]
    pub fn set_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plan`.\n"]
    pub fn set_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekday`.\n"]
    pub fn set_weekday(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weekday = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Droplet_BackupPolicy {
    type O = terrars::BlockAssignable<Droplet_BackupPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDroplet_BackupPolicy {}

impl BuildDroplet_BackupPolicy {
    pub fn build(self) -> Droplet_BackupPolicy {
        Droplet_BackupPolicy {
            hour: core::default::Default::default(),
            plan: core::default::Default::default(),
            weekday: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Droplet_BackupPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Droplet_BackupPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Droplet_BackupPolicyRef {
        Droplet_BackupPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Droplet_BackupPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hour` after provisioning.\n"]
    pub fn hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour", self.base))
    }

    #[doc = "Get a reference to the value of field `plan` after provisioning.\n"]
    pub fn plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan", self.base))
    }

    #[doc = "Get a reference to the value of field `weekday` after provisioning.\n"]
    pub fn weekday(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weekday", self.base))
    }
}

#[derive(Serialize)]
pub struct Droplet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Droplet_Timeouts {
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

impl terrars::ToListMappable for Droplet_Timeouts {
    type O = terrars::BlockAssignable<Droplet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDroplet_Timeouts {}

impl BuildDroplet_Timeouts {
    pub fn build(self) -> Droplet_Timeouts {
        Droplet_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Droplet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Droplet_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Droplet_TimeoutsRef {
        Droplet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Droplet_TimeoutsRef {
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
