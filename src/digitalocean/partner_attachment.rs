#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct PartnerAttachmentData_ {
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
    connection_bandwidth_in_mbps: terrars::PrimField<f64>,
    naas_provider: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redundancy_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    vpc_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp: std::option::Option<std::vec::Vec<PartnerAttachment_Bgp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PartnerAttachment_Timeouts>,
    dynamic: PartnerAttachmentDynamic,
}

struct PartnerAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PartnerAttachmentData_>,
}

#[derive(Clone)]
pub struct PartnerAttachment(std::rc::Rc<PartnerAttachment_>);

impl PartnerAttachment {
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

    #[doc = "Set the field `parent_uuid`.\nThe UUID of the Parent Partner Attachment "]
    pub fn set_parent_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parent_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redundancy_zone`.\nThe redundancy zone for the NaaS"]
    pub fn set_redundancy_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().redundancy_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp`.\n"]
    pub fn set_bgp(self, v: impl std::convert::Into<terrars::BlockAssignable<PartnerAttachment_Bgp>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bgp = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bgp = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PartnerAttachment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `children` after provisioning.\nThe children uuids of Partner Attachment"]
    pub fn children(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_bandwidth_in_mbps` after provisioning.\nThe connection bandwidth in Mbps"]
    pub fn connection_bandwidth_in_mbps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_bandwidth_in_mbps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the Partner Attachment was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Partner Attachment"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `naas_provider` after provisioning.\nThe NaaS provider"]
    pub fn naas_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.naas_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Partner Attachment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent_uuid` after provisioning.\nThe UUID of the Parent Partner Attachment "]
    pub fn parent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redundancy_zone` after provisioning.\nThe redundancy zone for the NaaS"]
    pub fn redundancy_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redundancy_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region where the Partner Attachment will be created"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Partner Attachment"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_ids` after provisioning.\nThe list of VPC IDs to attach the Partner Attachment to"]
    pub fn vpc_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> terrars::ListRef<PartnerAttachment_BgpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PartnerAttachment_TimeoutsRef {
        PartnerAttachment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PartnerAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PartnerAttachment { }

impl terrars::ToListMappable for PartnerAttachment {
    type O = terrars::ListRef<PartnerAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PartnerAttachment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_partner_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPartnerAttachment {
    pub tf_id: std::string::String,
    #[doc = "The connection bandwidth in Mbps"]
    pub connection_bandwidth_in_mbps: terrars::PrimField<f64>,
    #[doc = "The NaaS provider"]
    pub naas_provider: terrars::PrimField<std::string::String>,
    #[doc = "The name of the Partner Attachment"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The region where the Partner Attachment will be created"]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = "The list of VPC IDs to attach the Partner Attachment to"]
    pub vpc_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPartnerAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> PartnerAttachment {
        let out = PartnerAttachment(std::rc::Rc::new(PartnerAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PartnerAttachmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_bandwidth_in_mbps: self.connection_bandwidth_in_mbps,
                naas_provider: self.naas_provider,
                name: self.name,
                parent_uuid: core::default::Default::default(),
                redundancy_zone: core::default::Default::default(),
                region: self.region,
                vpc_ids: self.vpc_ids,
                bgp: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PartnerAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PartnerAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PartnerAttachmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `children` after provisioning.\nThe children uuids of Partner Attachment"]
    pub fn children(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connection_bandwidth_in_mbps` after provisioning.\nThe connection bandwidth in Mbps"]
    pub fn connection_bandwidth_in_mbps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_bandwidth_in_mbps", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the Partner Attachment was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Partner Attachment"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `naas_provider` after provisioning.\nThe NaaS provider"]
    pub fn naas_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.naas_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Partner Attachment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent_uuid` after provisioning.\nThe UUID of the Parent Partner Attachment "]
    pub fn parent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redundancy_zone` after provisioning.\nThe redundancy zone for the NaaS"]
    pub fn redundancy_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redundancy_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region where the Partner Attachment will be created"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the Partner Attachment"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `vpc_ids` after provisioning.\nThe list of VPC IDs to attach the Partner Attachment to"]
    pub fn vpc_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> terrars::ListRef<PartnerAttachment_BgpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PartnerAttachment_TimeoutsRef {
        PartnerAttachment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PartnerAttachment_Bgp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_router_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_router_asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_router_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PartnerAttachment_Bgp {
    #[doc = "Set the field `auth_key`.\n"]
    pub fn set_auth_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_router_ip`.\n"]
    pub fn set_local_router_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_router_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_router_asn`.\n"]
    pub fn set_peer_router_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.peer_router_asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_router_ip`.\n"]
    pub fn set_peer_router_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_router_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PartnerAttachment_Bgp {
    type O = terrars::BlockAssignable<PartnerAttachment_Bgp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPartnerAttachment_Bgp {}

impl BuildPartnerAttachment_Bgp {
    pub fn build(self) -> PartnerAttachment_Bgp {
        PartnerAttachment_Bgp {
            auth_key: core::default::Default::default(),
            local_router_ip: core::default::Default::default(),
            peer_router_asn: core::default::Default::default(),
            peer_router_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PartnerAttachment_BgpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PartnerAttachment_BgpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PartnerAttachment_BgpRef {
        PartnerAttachment_BgpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PartnerAttachment_BgpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_key` after provisioning.\n"]
    pub fn auth_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_key", self.base))
    }

    #[doc = "Get a reference to the value of field `local_router_ip` after provisioning.\n"]
    pub fn local_router_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_router_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_router_asn` after provisioning.\n"]
    pub fn peer_router_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_router_asn", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_router_ip` after provisioning.\n"]
    pub fn peer_router_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_router_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct PartnerAttachment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PartnerAttachment_Timeouts {
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
}

impl terrars::ToListMappable for PartnerAttachment_Timeouts {
    type O = terrars::BlockAssignable<PartnerAttachment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPartnerAttachment_Timeouts {}

impl BuildPartnerAttachment_Timeouts {
    pub fn build(self) -> PartnerAttachment_Timeouts {
        PartnerAttachment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PartnerAttachment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PartnerAttachment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PartnerAttachment_TimeoutsRef {
        PartnerAttachment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PartnerAttachment_TimeoutsRef {
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
}

#[derive(Serialize, Default)]
struct PartnerAttachmentDynamic {
    bgp: std::option::Option<terrars::DynamicBlock<PartnerAttachment_Bgp>>,
}
