#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataVpcNatGatewayData_ {
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

struct DataVpcNatGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcNatGatewayData_>,
}

#[derive(Clone)]
pub struct DataVpcNatGateway(std::rc::Rc<DataVpcNatGateway_>);

impl DataVpcNatGateway {
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

    #[doc = "Set the field `id`.\nID of the VPC NAT Gateway"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the VPC NAT Gateway"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nVPC NAT Gateway create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egresses` after provisioning.\nList of egresses"]
    pub fn egresses(&self) -> terrars::ListRef<DataVpcNatGateway_EgressesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egresses", self.extract_ref()))
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
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the VPC NAT Gateway is associated with."]
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

    #[doc = "Get a reference to the value of field `vpcs` after provisioning.\nList of ingress VPCs"]
    pub fn vpcs(&self) -> terrars::SetRef<DataVpcNatGateway_VpcsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpcs", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcNatGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcNatGateway { }

impl terrars::ToListMappable for DataVpcNatGateway {
    type O = terrars::ListRef<DataVpcNatGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcNatGateway_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_vpc_nat_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcNatGateway {
    pub tf_id: std::string::String,
}

impl BuildDataVpcNatGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcNatGateway {
        let out = DataVpcNatGateway(std::rc::Rc::new(DataVpcNatGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcNatGatewayData_ {
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

pub struct DataVpcNatGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcNatGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcNatGatewayRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\nVPC NAT Gateway create timestamp"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egresses` after provisioning.\nList of egresses"]
    pub fn egresses(&self) -> terrars::ListRef<DataVpcNatGateway_EgressesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egresses", self.extract_ref()))
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
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the VPC NAT Gateway is associated with."]
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

    #[doc = "Get a reference to the value of field `vpcs` after provisioning.\nList of ingress VPCs"]
    pub fn vpcs(&self) -> terrars::SetRef<DataVpcNatGateway_VpcsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpcs", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcNatGateway_Egresses_PublicGateways {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcNatGateway_Egresses_PublicGateways {
    #[doc = "Set the field `ipv4`.\n"]
    pub fn set_ipv4(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4 = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcNatGateway_Egresses_PublicGateways {
    type O = terrars::BlockAssignable<DataVpcNatGateway_Egresses_PublicGateways>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcNatGateway_Egresses_PublicGateways {}

impl BuildDataVpcNatGateway_Egresses_PublicGateways {
    pub fn build(self) -> DataVpcNatGateway_Egresses_PublicGateways {
        DataVpcNatGateway_Egresses_PublicGateways { ipv4: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcNatGateway_Egresses_PublicGatewaysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcNatGateway_Egresses_PublicGatewaysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcNatGateway_Egresses_PublicGatewaysRef {
        DataVpcNatGateway_Egresses_PublicGatewaysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcNatGateway_Egresses_PublicGatewaysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4` after provisioning.\n"]
    pub fn ipv4(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcNatGateway_Egresses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_gateways: std::option::Option<terrars::SetField<DataVpcNatGateway_Egresses_PublicGateways>>,
}

impl DataVpcNatGateway_Egresses {
    #[doc = "Set the field `public_gateways`.\n"]
    pub fn set_public_gateways(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataVpcNatGateway_Egresses_PublicGateways>>,
    ) -> Self {
        self.public_gateways = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcNatGateway_Egresses {
    type O = terrars::BlockAssignable<DataVpcNatGateway_Egresses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcNatGateway_Egresses {}

impl BuildDataVpcNatGateway_Egresses {
    pub fn build(self) -> DataVpcNatGateway_Egresses {
        DataVpcNatGateway_Egresses { public_gateways: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataVpcNatGateway_EgressesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcNatGateway_EgressesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcNatGateway_EgressesRef {
        DataVpcNatGateway_EgressesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcNatGateway_EgressesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `public_gateways` after provisioning.\n"]
    pub fn public_gateways(&self) -> terrars::SetRef<DataVpcNatGateway_Egresses_PublicGatewaysRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.public_gateways", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcNatGateway_Vpcs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_gateway: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcNatGateway_Vpcs {
    #[doc = "Set the field `default_gateway`.\n"]
    pub fn set_default_gateway(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.default_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_ip`.\n"]
    pub fn set_gateway_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_uuid`.\n"]
    pub fn set_subnet_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_uuid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcNatGateway_Vpcs {
    type O = terrars::BlockAssignable<DataVpcNatGateway_Vpcs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcNatGateway_Vpcs {}

impl BuildDataVpcNatGateway_Vpcs {
    pub fn build(self) -> DataVpcNatGateway_Vpcs {
        DataVpcNatGateway_Vpcs {
            default_gateway: core::default::Default::default(),
            gateway_ip: core::default::Default::default(),
            subnet_uuid: core::default::Default::default(),
            vpc_uuid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcNatGateway_VpcsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcNatGateway_VpcsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcNatGateway_VpcsRef {
        DataVpcNatGateway_VpcsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcNatGateway_VpcsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_gateway` after provisioning.\n"]
    pub fn default_gateway(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_ip` after provisioning.\n"]
    pub fn gateway_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_uuid` after provisioning.\n"]
    pub fn subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_uuid", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.base))
    }
}
