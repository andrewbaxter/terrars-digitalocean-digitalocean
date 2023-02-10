use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct FirewallData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    droplet_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inbound_rule: Option<Vec<FirewallInboundRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outbound_rule: Option<Vec<FirewallOutboundRuleEl>>,
    dynamic: FirewallDynamic,
}

struct Firewall_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<FirewallData>,
}

#[derive(Clone)]
pub struct Firewall(Rc<Firewall_>);

impl Firewall {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `droplet_ids`.\n"]
    pub fn set_droplet_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().droplet_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `inbound_rule`.\n"]
    pub fn set_inbound_rule(self, v: impl Into<BlockAssignable<FirewallInboundRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inbound_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inbound_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `outbound_rule`.\n"]
    pub fn set_outbound_rule(self, v: impl Into<BlockAssignable<FirewallOutboundRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().outbound_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.outbound_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pending_changes` after provisioning.\n"]
    pub fn pending_changes(&self) -> ListRef<FirewallPendingChangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pending_changes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for Firewall {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Firewall { }

impl ToListMappable for Firewall {
    type O = ListRef<FirewallRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Firewall_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_firewall".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirewall {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildFirewall {
    pub fn build(self, stack: &mut Stack) -> Firewall {
        let out = Firewall(Rc::new(Firewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(FirewallData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                droplet_ids: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                inbound_rule: core::default::Default::default(),
                outbound_rule: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirewallRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirewallRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirewallRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pending_changes` after provisioning.\n"]
    pub fn pending_changes(&self) -> ListRef<FirewallPendingChangesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.pending_changes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FirewallPendingChangesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    droplet_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    removing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
}

impl FirewallPendingChangesEl {
    #[doc= "Set the field `droplet_id`.\n"]
    pub fn set_droplet_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.droplet_id = Some(v.into());
        self
    }

    #[doc= "Set the field `removing`.\n"]
    pub fn set_removing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.removing = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }
}

impl ToListMappable for FirewallPendingChangesEl {
    type O = BlockAssignable<FirewallPendingChangesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirewallPendingChangesEl {}

impl BuildFirewallPendingChangesEl {
    pub fn build(self) -> FirewallPendingChangesEl {
        FirewallPendingChangesEl {
            droplet_id: core::default::Default::default(),
            removing: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct FirewallPendingChangesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirewallPendingChangesElRef {
    fn new(shared: StackShared, base: String) -> FirewallPendingChangesElRef {
        FirewallPendingChangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirewallPendingChangesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `droplet_id` after provisioning.\n"]
    pub fn droplet_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_id", self.base))
    }

    #[doc= "Get a reference to the value of field `removing` after provisioning.\n"]
    pub fn removing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.removing", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct FirewallInboundRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<PrimField<String>>,
    protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_droplet_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_kubernetes_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_load_balancer_uids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_tags: Option<SetField<PrimField<String>>>,
}

impl FirewallInboundRuleEl {
    #[doc= "Set the field `port_range`.\n"]
    pub fn set_port_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_range = Some(v.into());
        self
    }

    #[doc= "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `source_droplet_ids`.\n"]
    pub fn set_source_droplet_ids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.source_droplet_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `source_kubernetes_ids`.\n"]
    pub fn set_source_kubernetes_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_kubernetes_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `source_load_balancer_uids`.\n"]
    pub fn set_source_load_balancer_uids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_load_balancer_uids = Some(v.into());
        self
    }

    #[doc= "Set the field `source_tags`.\n"]
    pub fn set_source_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.source_tags = Some(v.into());
        self
    }
}

impl ToListMappable for FirewallInboundRuleEl {
    type O = BlockAssignable<FirewallInboundRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirewallInboundRuleEl {
    #[doc= ""]
    pub protocol: PrimField<String>,
}

impl BuildFirewallInboundRuleEl {
    pub fn build(self) -> FirewallInboundRuleEl {
        FirewallInboundRuleEl {
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

pub struct FirewallInboundRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirewallInboundRuleElRef {
    fn new(shared: StackShared, base: String) -> FirewallInboundRuleElRef {
        FirewallInboundRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirewallInboundRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc= "Get a reference to the value of field `source_droplet_ids` after provisioning.\n"]
    pub fn source_droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.source_droplet_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `source_kubernetes_ids` after provisioning.\n"]
    pub fn source_kubernetes_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.source_kubernetes_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `source_load_balancer_uids` after provisioning.\n"]
    pub fn source_load_balancer_uids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.source_load_balancer_uids", self.base))
    }

    #[doc= "Get a reference to the value of field `source_tags` after provisioning.\n"]
    pub fn source_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.source_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct FirewallOutboundRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_addresses: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_droplet_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_kubernetes_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_load_balancer_uids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination_tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port_range: Option<PrimField<String>>,
    protocol: PrimField<String>,
}

impl FirewallOutboundRuleEl {
    #[doc= "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.destination_addresses = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_droplet_ids`.\n"]
    pub fn set_destination_droplet_ids(mut self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.destination_droplet_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_kubernetes_ids`.\n"]
    pub fn set_destination_kubernetes_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.destination_kubernetes_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_load_balancer_uids`.\n"]
    pub fn set_destination_load_balancer_uids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.destination_load_balancer_uids = Some(v.into());
        self
    }

    #[doc= "Set the field `destination_tags`.\n"]
    pub fn set_destination_tags(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.destination_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `port_range`.\n"]
    pub fn set_port_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port_range = Some(v.into());
        self
    }
}

impl ToListMappable for FirewallOutboundRuleEl {
    type O = BlockAssignable<FirewallOutboundRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirewallOutboundRuleEl {
    #[doc= ""]
    pub protocol: PrimField<String>,
}

impl BuildFirewallOutboundRuleEl {
    pub fn build(self) -> FirewallOutboundRuleEl {
        FirewallOutboundRuleEl {
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

pub struct FirewallOutboundRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for FirewallOutboundRuleElRef {
    fn new(shared: StackShared, base: String) -> FirewallOutboundRuleElRef {
        FirewallOutboundRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirewallOutboundRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc= "Get a reference to the value of field `destination_droplet_ids` after provisioning.\n"]
    pub fn destination_droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.destination_droplet_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `destination_kubernetes_ids` after provisioning.\n"]
    pub fn destination_kubernetes_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.destination_kubernetes_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `destination_load_balancer_uids` after provisioning.\n"]
    pub fn destination_load_balancer_uids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.destination_load_balancer_uids", self.base))
    }

    #[doc= "Get a reference to the value of field `destination_tags` after provisioning.\n"]
    pub fn destination_tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.destination_tags", self.base))
    }

    #[doc= "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirewallDynamic {
    inbound_rule: Option<DynamicBlock<FirewallInboundRuleEl>>,
    outbound_rule: Option<DynamicBlock<FirewallOutboundRuleEl>>,
}
