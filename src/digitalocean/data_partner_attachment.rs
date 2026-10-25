#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataPartnerAttachmentData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redundancy_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp: std::option::Option<std::vec::Vec<DataPartnerAttachment_Bgp>>,
    dynamic: DataPartnerAttachmentDynamic,
}

struct DataPartnerAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPartnerAttachmentData_>,
}

#[derive(Clone)]
pub struct DataPartnerAttachment(std::rc::Rc<DataPartnerAttachment_>);

impl DataPartnerAttachment {
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

    #[doc = "Set the field `id`.\nThe ID of the Partner Attachment"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the Partner Attachment"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redundancy_zone`.\n"]
    pub fn set_redundancy_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().redundancy_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp`.\n"]
    pub fn set_bgp(self, v: impl std::convert::Into<terrars::BlockAssignable<DataPartnerAttachment_Bgp>>) -> Self {
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

    #[doc =
        "Get a reference to the value of field `children` after provisioning.\nThe children uuids of the Partner Attachment."]
    pub fn children(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_bandwidth_in_mbps` after provisioning.\n"]
    pub fn connection_bandwidth_in_mbps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_bandwidth_in_mbps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Partner Attachment"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `naas_provider` after provisioning.\n"]
    pub fn naas_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.naas_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Partner Attachment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_uuid` after provisioning.\n"]
    pub fn parent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redundancy_zone` after provisioning.\n"]
    pub fn redundancy_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redundancy_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_ids` after provisioning.\n"]
    pub fn vpc_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> terrars::ListRef<DataPartnerAttachment_BgpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }
}

impl terrars::Referable for DataPartnerAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPartnerAttachment { }

impl terrars::ToListMappable for DataPartnerAttachment {
    type O = terrars::ListRef<DataPartnerAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPartnerAttachment_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_partner_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPartnerAttachment {
    pub tf_id: std::string::String,
}

impl BuildDataPartnerAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPartnerAttachment {
        let out = DataPartnerAttachment(std::rc::Rc::new(DataPartnerAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPartnerAttachmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                redundancy_zone: core::default::Default::default(),
                bgp: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPartnerAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPartnerAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPartnerAttachmentRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `children` after provisioning.\nThe children uuids of the Partner Attachment."]
    pub fn children(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.children", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_bandwidth_in_mbps` after provisioning.\n"]
    pub fn connection_bandwidth_in_mbps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_bandwidth_in_mbps", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the Partner Attachment"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `naas_provider` after provisioning.\n"]
    pub fn naas_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.naas_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Partner Attachment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent_uuid` after provisioning.\n"]
    pub fn parent_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redundancy_zone` after provisioning.\n"]
    pub fn redundancy_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redundancy_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_ids` after provisioning.\n"]
    pub fn vpc_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp` after provisioning.\n"]
    pub fn bgp(&self) -> terrars::ListRef<DataPartnerAttachment_BgpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bgp", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataPartnerAttachmentDynamic {
    bgp: std::option::Option<terrars::DynamicBlock<DataPartnerAttachment_Bgp>>,
}

#[derive(Serialize)]
pub struct DataPartnerAttachment_Bgp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_router_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_router_asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_router_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPartnerAttachment_Bgp {
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

impl terrars::ToListMappable for DataPartnerAttachment_Bgp {
    type O = terrars::BlockAssignable<DataPartnerAttachment_Bgp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPartnerAttachment_Bgp {}

impl BuildDataPartnerAttachment_Bgp {
    pub fn build(self) -> DataPartnerAttachment_Bgp {
        DataPartnerAttachment_Bgp {
            local_router_ip: core::default::Default::default(),
            peer_router_asn: core::default::Default::default(),
            peer_router_ip: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPartnerAttachment_BgpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPartnerAttachment_BgpRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPartnerAttachment_BgpRef {
        DataPartnerAttachment_BgpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPartnerAttachment_BgpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
