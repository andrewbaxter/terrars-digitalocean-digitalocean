#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct VpcNatGatewayData_ {
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
    icmp_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egresses: std::option::Option<std::vec::Vec<VpcNatGateway_Egresses>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpcs: std::option::Option<std::vec::Vec<VpcNatGateway_Vpcs>>,
    dynamic: VpcNatGatewayDynamic,
}

struct VpcNatGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpcNatGatewayData_>,
}

#[derive(Clone)]
pub struct VpcNatGateway(std::rc::Rc<VpcNatGateway_>);

impl VpcNatGateway {
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

    #[doc = "Set the field `icmp_timeout_seconds`.\nICMP connection timeout (in seconds)"]
    pub fn set_icmp_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().icmp_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\nID of the project to which the VPC NAT Gateway will be assigned."]
    pub fn set_project_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_timeout_seconds`.\nTCP connection timeout (in seconds)"]
    pub fn set_tcp_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().tcp_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udp_timeout_seconds`.\nUDP connection timeout (in seconds)"]
    pub fn set_udp_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().udp_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egresses`.\n"]
    pub fn set_egresses(self, v: impl std::convert::Into<terrars::BlockAssignable<VpcNatGateway_Egresses>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpcs`.\n"]
    pub fn set_vpcs(self, v: impl std::convert::Into<terrars::BlockAssignable<VpcNatGateway_Vpcs>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpcs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpcs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nVPC NAT Gateway create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `icmp_timeout_seconds` after provisioning.\nICMP connection timeout (in seconds)"]
    pub fn icmp_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the VPC NAT Gateway"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the VPC NAT Gateway"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nID of the project to which the VPC NAT Gateway will be assigned."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the VPC NAT Gateway"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the VPC NAT Gateway"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the VPC NAT Gateway"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tcp_timeout_seconds` after provisioning.\nTCP connection timeout (in seconds)"]
    pub fn tcp_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the VPC NAT Gateway"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `udp_timeout_seconds` after provisioning.\nUDP connection timeout (in seconds)"]
    pub fn udp_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\nVPC NAT Gateway update timestamp"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egresses` after provisioning.\n"]
    pub fn egresses(&self) -> terrars::ListRef<VpcNatGateway_EgressesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egresses", self.extract_ref()))
    }
}

impl terrars::Referable for VpcNatGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpcNatGateway { }

impl terrars::ToListMappable for VpcNatGateway {
    type O = terrars::ListRef<VpcNatGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpcNatGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_vpc_nat_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcNatGateway {
    pub tf_id: std::string::String,
    #[doc = "Name of the VPC NAT Gateway"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Region of the VPC NAT Gateway"]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = "Size of the VPC NAT Gateway"]
    pub size: terrars::PrimField<f64>,
    #[doc = "Type of the VPC NAT Gateway"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildVpcNatGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> VpcNatGateway {
        let out = VpcNatGateway(std::rc::Rc::new(VpcNatGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpcNatGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                icmp_timeout_seconds: core::default::Default::default(),
                name: self.name,
                project_id: core::default::Default::default(),
                region: self.region,
                size: self.size,
                tcp_timeout_seconds: core::default::Default::default(),
                type_: self.type_,
                udp_timeout_seconds: core::default::Default::default(),
                egresses: core::default::Default::default(),
                vpcs: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcNatGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcNatGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcNatGatewayRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nVPC NAT Gateway create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `icmp_timeout_seconds` after provisioning.\nICMP connection timeout (in seconds)"]
    pub fn icmp_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icmp_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the VPC NAT Gateway"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the VPC NAT Gateway"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nID of the project to which the VPC NAT Gateway will be assigned."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion of the VPC NAT Gateway"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nSize of the VPC NAT Gateway"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nState of the VPC NAT Gateway"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tcp_timeout_seconds` after provisioning.\nTCP connection timeout (in seconds)"]
    pub fn tcp_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the VPC NAT Gateway"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `udp_timeout_seconds` after provisioning.\nUDP connection timeout (in seconds)"]
    pub fn udp_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\nVPC NAT Gateway update timestamp"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egresses` after provisioning.\n"]
    pub fn egresses(&self) -> terrars::ListRef<VpcNatGateway_EgressesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egresses", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct VpcNatGateway_Egresses_PublicGateways {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpcNatGateway_Egresses_PublicGateways {
    #[doc =
        "Set the field `ipv4`.\nIPv4 address. On create, set this to an unassigned BYOIP on the account in the same region. When omitted, a system-allocated reserved IP is provisioned."]
    pub fn set_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcNatGateway_Egresses_PublicGateways {
    type O = terrars::BlockAssignable<VpcNatGateway_Egresses_PublicGateways>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcNatGateway_Egresses_PublicGateways {}

impl BuildVpcNatGateway_Egresses_PublicGateways {
    pub fn build(self) -> VpcNatGateway_Egresses_PublicGateways {
        VpcNatGateway_Egresses_PublicGateways { ipv4: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VpcNatGateway_Egresses_PublicGatewaysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcNatGateway_Egresses_PublicGatewaysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcNatGateway_Egresses_PublicGatewaysRef {
        VpcNatGateway_Egresses_PublicGatewaysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcNatGateway_Egresses_PublicGatewaysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ipv4` after provisioning.\nIPv4 address. On create, set this to an unassigned BYOIP on the account in the same region. When omitted, a system-allocated reserved IP is provisioned."]
    pub fn ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpcNatGateway_EgressesDynamic {
    public_gateways: std::option::Option<terrars::DynamicBlock<VpcNatGateway_Egresses_PublicGateways>>,
}

#[derive(Serialize)]
pub struct VpcNatGateway_Egresses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_gateways: std::option::Option<std::vec::Vec<VpcNatGateway_Egresses_PublicGateways>>,
    dynamic: VpcNatGateway_EgressesDynamic,
}

impl VpcNatGateway_Egresses {
    #[doc = "Set the field `public_gateways`.\n"]
    pub fn set_public_gateways(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpcNatGateway_Egresses_PublicGateways>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_gateways = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_gateways = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpcNatGateway_Egresses {
    type O = terrars::BlockAssignable<VpcNatGateway_Egresses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcNatGateway_Egresses {}

impl BuildVpcNatGateway_Egresses {
    pub fn build(self) -> VpcNatGateway_Egresses {
        VpcNatGateway_Egresses {
            public_gateways: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpcNatGateway_EgressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcNatGateway_EgressesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcNatGateway_EgressesRef {
        VpcNatGateway_EgressesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcNatGateway_EgressesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct VpcNatGateway_Vpcs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_gateway: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    vpc_uuid: terrars::PrimField<std::string::String>,
}

impl VpcNatGateway_Vpcs {
    #[doc = "Set the field `default_gateway`.\nIndicates if this is the default VPC NAT Gateway in the VPC"]
    pub fn set_default_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_uuid`.\nID of the ingress subnet in the VPC"]
    pub fn set_subnet_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpcNatGateway_Vpcs {
    type O = terrars::BlockAssignable<VpcNatGateway_Vpcs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpcNatGateway_Vpcs {
    #[doc = "ID of the ingress VPC"]
    pub vpc_uuid: terrars::PrimField<std::string::String>,
}

impl BuildVpcNatGateway_Vpcs {
    pub fn build(self) -> VpcNatGateway_Vpcs {
        VpcNatGateway_Vpcs {
            default_gateway: core::default::Default::default(),
            subnet_uuid: core::default::Default::default(),
            vpc_uuid: self.vpc_uuid,
        }
    }
}

#[allow(dead_code)]
pub struct VpcNatGateway_VpcsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpcNatGateway_VpcsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpcNatGateway_VpcsRef {
        VpcNatGateway_VpcsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpcNatGateway_VpcsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_gateway` after provisioning.\nIndicates if this is the default VPC NAT Gateway in the VPC"]
    pub fn default_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_ip` after provisioning.\nGateway IP of the VPC NAT Gateway"]
    pub fn gateway_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnet_uuid` after provisioning.\nID of the ingress subnet in the VPC"]
    pub fn subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\nID of the ingress VPC"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpcNatGatewayDynamic {
    egresses: std::option::Option<terrars::DynamicBlock<VpcNatGateway_Egresses>>,
    vpcs: std::option::Option<terrars::DynamicBlock<VpcNatGateway_Vpcs>>,
}
