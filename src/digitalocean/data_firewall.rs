#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataFirewallData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_ids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    firewall_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_rule: std::option::Option<std::vec::Vec<DataFirewall_InboundRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_rule: std::option::Option<std::vec::Vec<DataFirewall_OutboundRule>>,
    dynamic: DataFirewallDynamic,
}

struct DataFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFirewallData_>,
}

#[derive(Clone)]
pub struct DataFirewall(std::rc::Rc<DataFirewall_>);

impl DataFirewall {
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

    #[doc = "Set the field `droplet_ids`.\n"]
    pub fn set_droplet_ids(self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().droplet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `inbound_rule`.\n"]
    pub fn set_inbound_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFirewall_InboundRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inbound_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inbound_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `outbound_rule`.\n"]
    pub fn set_outbound_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFirewall_OutboundRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outbound_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outbound_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_id` after provisioning.\n"]
    pub fn firewall_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_changes` after provisioning.\n"]
    pub fn pending_changes(&self) -> terrars::ListRef<DataFirewall_PendingChangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pending_changes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataFirewall {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataFirewall { }

impl terrars::ToListMappable for DataFirewall {
    type O = terrars::ListRef<DataFirewallRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataFirewall_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_firewall".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFirewall {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firewall_id: terrars::PrimField<std::string::String>,
}

impl BuildDataFirewall {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFirewall {
        let out = DataFirewall(std::rc::Rc::new(DataFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFirewallData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                droplet_ids: core::default::Default::default(),
                firewall_id: self.firewall_id,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                inbound_rule: core::default::Default::default(),
                outbound_rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFirewallRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_id` after provisioning.\n"]
    pub fn firewall_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_changes` after provisioning.\n"]
    pub fn pending_changes(&self) -> terrars::ListRef<DataFirewall_PendingChangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pending_changes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataFirewallDynamic {
    inbound_rule: std::option::Option<terrars::DynamicBlock<DataFirewall_InboundRule>>,
    outbound_rule: std::option::Option<terrars::DynamicBlock<DataFirewall_OutboundRule>>,
}

#[derive(Serialize)]
pub struct DataFirewall_PendingChanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    removing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFirewall_PendingChanges {
    #[doc = "Set the field `droplet_id`.\n"]
    pub fn set_droplet_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.droplet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `removing`.\n"]
    pub fn set_removing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.removing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFirewall_PendingChanges {
    type O = terrars::BlockAssignable<DataFirewall_PendingChanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFirewall_PendingChanges {}

impl BuildDataFirewall_PendingChanges {
    pub fn build(self) -> DataFirewall_PendingChanges {
        DataFirewall_PendingChanges {
            droplet_id: core::default::Default::default(),
            removing: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFirewall_PendingChangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFirewall_PendingChangesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFirewall_PendingChangesRef {
        DataFirewall_PendingChangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFirewall_PendingChangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `droplet_id` after provisioning.\n"]
    pub fn droplet_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `removing` after provisioning.\n"]
    pub fn removing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.removing", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFirewall_InboundRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_droplet_ids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_kubernetes_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_load_balancer_uids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFirewall_InboundRule {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_droplet_ids`.\n"]
    pub fn set_source_droplet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.source_droplet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_kubernetes_ids`.\n"]
    pub fn set_source_kubernetes_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_kubernetes_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_load_balancer_uids`.\n"]
    pub fn set_source_load_balancer_uids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_load_balancer_uids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_tags`.\n"]
    pub fn set_source_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFirewall_InboundRule {
    type O = terrars::BlockAssignable<DataFirewall_InboundRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFirewall_InboundRule {
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildDataFirewall_InboundRule {
    pub fn build(self) -> DataFirewall_InboundRule {
        DataFirewall_InboundRule {
            action: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: self.protocol,
            source_addresses: core::default::Default::default(),
            source_droplet_ids: core::default::Default::default(),
            source_kubernetes_ids: core::default::Default::default(),
            source_load_balancer_uids: core::default::Default::default(),
            source_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFirewall_InboundRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFirewall_InboundRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFirewall_InboundRuleRef {
        DataFirewall_InboundRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFirewall_InboundRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_droplet_ids` after provisioning.\n"]
    pub fn source_droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_droplet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `source_kubernetes_ids` after provisioning.\n"]
    pub fn source_kubernetes_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_kubernetes_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `source_load_balancer_uids` after provisioning.\n"]
    pub fn source_load_balancer_uids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_load_balancer_uids", self.base))
    }

    #[doc = "Get a reference to the value of field `source_tags` after provisioning.\n"]
    pub fn source_tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFirewall_OutboundRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_droplet_ids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_kubernetes_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_load_balancer_uids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<terrars::PrimField<std::string::String>>,
    protocol: terrars::PrimField<std::string::String>,
}

impl DataFirewall_OutboundRule {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_droplet_ids`.\n"]
    pub fn set_destination_droplet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.destination_droplet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_kubernetes_ids`.\n"]
    pub fn set_destination_kubernetes_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_kubernetes_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_load_balancer_uids`.\n"]
    pub fn set_destination_load_balancer_uids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_load_balancer_uids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_tags`.\n"]
    pub fn set_destination_tags(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFirewall_OutboundRule {
    type O = terrars::BlockAssignable<DataFirewall_OutboundRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFirewall_OutboundRule {
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildDataFirewall_OutboundRule {
    pub fn build(self) -> DataFirewall_OutboundRule {
        DataFirewall_OutboundRule {
            action: core::default::Default::default(),
            destination_addresses: core::default::Default::default(),
            destination_droplet_ids: core::default::Default::default(),
            destination_kubernetes_ids: core::default::Default::default(),
            destination_load_balancer_uids: core::default::Default::default(),
            destination_tags: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: self.protocol,
        }
    }
}

#[allow(dead_code)]
pub struct DataFirewall_OutboundRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFirewall_OutboundRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFirewall_OutboundRuleRef {
        DataFirewall_OutboundRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFirewall_OutboundRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_droplet_ids` after provisioning.\n"]
    pub fn destination_droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destination_droplet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_kubernetes_ids` after provisioning.\n"]
    pub fn destination_kubernetes_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destination_kubernetes_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_load_balancer_uids` after provisioning.\n"]
    pub fn destination_load_balancer_uids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destination_load_balancer_uids", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_tags` after provisioning.\n"]
    pub fn destination_tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.destination_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}
