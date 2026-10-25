#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct LoadbalancerData_ {
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
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_lets_encrypt_dns_records: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_ids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    droplet_tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_backend_keepalive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_proxy_protocol: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_idle_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_stack: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_http_to_https: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_unit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_load_balancer_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_cipher_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_uuid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domains: std::option::Option<std::vec::Vec<Loadbalancer_Domains>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall: std::option::Option<std::vec::Vec<Loadbalancer_Firewall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarding_rule: std::option::Option<std::vec::Vec<Loadbalancer_ForwardingRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    glb_settings: std::option::Option<std::vec::Vec<Loadbalancer_GlbSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthcheck: std::option::Option<std::vec::Vec<Loadbalancer_Healthcheck>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sticky_sessions: std::option::Option<std::vec::Vec<Loadbalancer_StickySessions>>,
    dynamic: LoadbalancerDynamic,
}

struct Loadbalancer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LoadbalancerData_>,
}

#[derive(Clone)]
pub struct Loadbalancer(std::rc::Rc<Loadbalancer_>);

impl Loadbalancer {
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

    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_lets_encrypt_dns_records`.\n"]
    pub fn set_disable_lets_encrypt_dns_records(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_lets_encrypt_dns_records = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `droplet_ids`.\n"]
    pub fn set_droplet_ids(self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().droplet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `droplet_tag`.\n"]
    pub fn set_droplet_tag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().droplet_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_backend_keepalive`.\n"]
    pub fn set_enable_backend_keepalive(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_backend_keepalive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_proxy_protocol`.\n"]
    pub fn set_enable_proxy_protocol(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_proxy_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_idle_timeout_seconds`.\n"]
    pub fn set_http_idle_timeout_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_idle_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip`.\nOptional BYOIP address to assign on create. Must be an unassigned BYOIP on the account in the same region. When omitted, a system-allocated floating IP is provisioned."]
    pub fn set_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nthe type of network the load balancer is accessible from (EXTERNAL or INTERNAL)"]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_stack`.\nThe network stack determines the allocation of ipv4/ipv6 addresses to the load balancer. Enum: 'IPV4' 'DUALSTACK'"]
    pub fn set_network_stack(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_stack = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id`.\n"]
    pub fn set_project_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redirect_http_to_https`.\n"]
    pub fn set_redirect_http_to_https(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().redirect_http_to_https = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_unit`.\n"]
    pub fn set_size_unit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().size_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_uuid`.\n"]
    pub fn set_subnet_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnet_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_load_balancer_ids`.\nlist of load balancer IDs to put behind a global load balancer"]
    pub fn set_target_load_balancer_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().target_load_balancer_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_cipher_policy`.\nThe tls cipher policy to be used for the load balancer. Enum: 'DEFAULT' 'STRONG'"]
    pub fn set_tls_cipher_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tls_cipher_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nthe type of the load balancer (GLOBAL, REGIONAL, or REGIONAL_NETWORK)"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domains`.\n"]
    pub fn set_domains(self, v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_Domains>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().domains = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.domains = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `firewall`.\n"]
    pub fn set_firewall(self, v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_Firewall>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().firewall = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.firewall = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_ForwardingRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().forwarding_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.forwarding_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `glb_settings`.\n"]
    pub fn set_glb_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_GlbSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().glb_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.glb_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `healthcheck`.\n"]
    pub fn set_healthcheck(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_Healthcheck>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().healthcheck = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.healthcheck = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sticky_sessions`.\n"]
    pub fn set_sticky_sessions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_StickySessions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sticky_sessions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sticky_sessions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\n"]
    pub fn disable_lets_encrypt_dns_records(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_tag` after provisioning.\n"]
    pub fn droplet_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\n"]
    pub fn enable_backend_keepalive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\n"]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n"]
    pub fn http_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nOptional BYOIP address to assign on create. Must be an unassigned BYOIP on the account in the same region. When omitted, a system-allocated floating IP is provisioned."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nthe type of network the load balancer is accessible from (EXTERNAL or INTERNAL)"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_stack` after provisioning.\nThe network stack determines the allocation of ipv4/ipv6 addresses to the load balancer. Enum: 'IPV4' 'DUALSTACK'"]
    pub fn network_stack(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_stack", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_http_to_https` after provisioning.\n"]
    pub fn redirect_http_to_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_unit` after provisioning.\n"]
    pub fn size_unit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_uuid` after provisioning.\n"]
    pub fn subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_load_balancer_ids` after provisioning.\nlist of load balancer IDs to put behind a global load balancer"]
    pub fn target_load_balancer_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_load_balancer_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_cipher_policy` after provisioning.\nThe tls cipher policy to be used for the load balancer. Enum: 'DEFAULT' 'STRONG'"]
    pub fn tls_cipher_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_cipher_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nthe type of the load balancer (GLOBAL, REGIONAL, or REGIONAL_NETWORK)"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the load balancer"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glb_settings` after provisioning.\n"]
    pub fn glb_settings(&self) -> terrars::ListRef<Loadbalancer_GlbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glb_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> terrars::ListRef<Loadbalancer_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sticky_sessions` after provisioning.\n"]
    pub fn sticky_sessions(&self) -> terrars::ListRef<Loadbalancer_StickySessionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }
}

impl terrars::Referable for Loadbalancer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Loadbalancer { }

impl terrars::ToListMappable for Loadbalancer {
    type O = terrars::ListRef<LoadbalancerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Loadbalancer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_loadbalancer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadbalancer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLoadbalancer {
    pub fn build(self, stack: &mut terrars::Stack) -> Loadbalancer {
        let out = Loadbalancer(std::rc::Rc::new(Loadbalancer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LoadbalancerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                algorithm: core::default::Default::default(),
                disable_lets_encrypt_dns_records: core::default::Default::default(),
                droplet_ids: core::default::Default::default(),
                droplet_tag: core::default::Default::default(),
                enable_backend_keepalive: core::default::Default::default(),
                enable_proxy_protocol: core::default::Default::default(),
                http_idle_timeout_seconds: core::default::Default::default(),
                id: core::default::Default::default(),
                ip: core::default::Default::default(),
                name: self.name,
                network: core::default::Default::default(),
                network_stack: core::default::Default::default(),
                project_id: core::default::Default::default(),
                redirect_http_to_https: core::default::Default::default(),
                region: core::default::Default::default(),
                size: core::default::Default::default(),
                size_unit: core::default::Default::default(),
                subnet_uuid: core::default::Default::default(),
                target_load_balancer_ids: core::default::Default::default(),
                tls_cipher_policy: core::default::Default::default(),
                type_: core::default::Default::default(),
                vpc_uuid: core::default::Default::default(),
                domains: core::default::Default::default(),
                firewall: core::default::Default::default(),
                forwarding_rule: core::default::Default::default(),
                glb_settings: core::default::Default::default(),
                healthcheck: core::default::Default::default(),
                sticky_sessions: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadbalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LoadbalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadbalancerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\n"]
    pub fn disable_lets_encrypt_dns_records(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `droplet_tag` after provisioning.\n"]
    pub fn droplet_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\n"]
    pub fn enable_backend_keepalive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\n"]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n"]
    pub fn http_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\nOptional BYOIP address to assign on create. Must be an unassigned BYOIP on the account in the same region. When omitted, a system-allocated floating IP is provisioned."]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nthe type of network the load balancer is accessible from (EXTERNAL or INTERNAL)"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_stack` after provisioning.\nThe network stack determines the allocation of ipv4/ipv6 addresses to the load balancer. Enum: 'IPV4' 'DUALSTACK'"]
    pub fn network_stack(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_stack", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_http_to_https` after provisioning.\n"]
    pub fn redirect_http_to_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_unit` after provisioning.\n"]
    pub fn size_unit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_uuid` after provisioning.\n"]
    pub fn subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_load_balancer_ids` after provisioning.\nlist of load balancer IDs to put behind a global load balancer"]
    pub fn target_load_balancer_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_load_balancer_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls_cipher_policy` after provisioning.\nThe tls cipher policy to be used for the load balancer. Enum: 'DEFAULT' 'STRONG'"]
    pub fn tls_cipher_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_cipher_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nthe type of the load balancer (GLOBAL, REGIONAL, or REGIONAL_NETWORK)"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the load balancer"]
    pub fn urn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `glb_settings` after provisioning.\n"]
    pub fn glb_settings(&self) -> terrars::ListRef<Loadbalancer_GlbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glb_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> terrars::ListRef<Loadbalancer_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sticky_sessions` after provisioning.\n"]
    pub fn sticky_sessions(&self) -> terrars::ListRef<Loadbalancer_StickySessionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Loadbalancer_Domains {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_managed: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
}

impl Loadbalancer_Domains {
    #[doc = "Set the field `certificate_name`.\nname of certificate required for TLS handshaking"]
    pub fn set_certificate_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_managed`.\nflag indicating if domain is managed by DigitalOcean"]
    pub fn set_is_managed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_managed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_Domains {
    type O = terrars::BlockAssignable<Loadbalancer_Domains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_Domains {
    #[doc = "domain name"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLoadbalancer_Domains {
    pub fn build(self) -> Loadbalancer_Domains {
        Loadbalancer_Domains {
            certificate_name: core::default::Default::default(),
            is_managed: core::default::Default::default(),
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_DomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_DomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_DomainsRef {
        Loadbalancer_DomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_DomainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\ncertificate ID for TLS handshaking"]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `certificate_name` after provisioning.\nname of certificate required for TLS handshaking"]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_managed` after provisioning.\nflag indicating if domain is managed by DigitalOcean"]
    pub fn is_managed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_managed", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\ndomain name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_validation_error_reasons` after provisioning.\nlist of domain SSL validation errors"]
    pub fn ssl_validation_error_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_validation_error_reasons", self.base))
    }

    #[doc =
        "Get a reference to the value of field `verification_error_reasons` after provisioning.\nlist of domain verification errors"]
    pub fn verification_error_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_error_reasons", self.base))
    }
}

#[derive(Serialize)]
pub struct Loadbalancer_Firewall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Loadbalancer_Firewall {
    #[doc =
        "Set the field `allow`.\nthe rules for ALLOWING traffic to the LB (strings in the form: 'ip:1.2.3.4' or 'cidr:1.2.0.0/16')"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deny`.\nthe rules for DENYING traffic to the LB (strings in the form: 'ip:1.2.3.4' or 'cidr:1.2.0.0/16')"]
    pub fn set_deny(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.deny = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_Firewall {
    type O = terrars::BlockAssignable<Loadbalancer_Firewall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_Firewall {}

impl BuildLoadbalancer_Firewall {
    pub fn build(self) -> Loadbalancer_Firewall {
        Loadbalancer_Firewall {
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_FirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_FirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_FirewallRef {
        Loadbalancer_FirewallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_FirewallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow` after provisioning.\nthe rules for ALLOWING traffic to the LB (strings in the form: 'ip:1.2.3.4' or 'cidr:1.2.0.0/16')"]
    pub fn allow(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deny` after provisioning.\nthe rules for DENYING traffic to the LB (strings in the form: 'ip:1.2.3.4' or 'cidr:1.2.0.0/16')"]
    pub fn deny(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }
}

#[derive(Serialize)]
pub struct Loadbalancer_ForwardingRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_name: std::option::Option<terrars::PrimField<std::string::String>>,
    entry_port: terrars::PrimField<f64>,
    entry_protocol: terrars::PrimField<std::string::String>,
    target_port: terrars::PrimField<f64>,
    target_protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_passthrough: std::option::Option<terrars::PrimField<bool>>,
}

impl Loadbalancer_ForwardingRule {
    #[doc = "Set the field `certificate_id`.\n"]
    pub fn set_certificate_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_name`.\n"]
    pub fn set_certificate_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_passthrough`.\n"]
    pub fn set_tls_passthrough(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls_passthrough = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_ForwardingRule {
    type O = terrars::BlockAssignable<Loadbalancer_ForwardingRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_ForwardingRule {
    #[doc = ""]
    pub entry_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub entry_protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub target_protocol: terrars::PrimField<std::string::String>,
}

impl BuildLoadbalancer_ForwardingRule {
    pub fn build(self) -> Loadbalancer_ForwardingRule {
        Loadbalancer_ForwardingRule {
            certificate_id: core::default::Default::default(),
            certificate_name: core::default::Default::default(),
            entry_port: self.entry_port,
            entry_protocol: self.entry_protocol,
            target_port: self.target_port,
            target_protocol: self.target_protocol,
            tls_passthrough: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_ForwardingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_ForwardingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_ForwardingRuleRef {
        Loadbalancer_ForwardingRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_ForwardingRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\n"]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc = "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_port` after provisioning.\n"]
    pub fn entry_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_port", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_protocol` after provisioning.\n"]
    pub fn entry_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `target_port` after provisioning.\n"]
    pub fn target_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_port", self.base))
    }

    #[doc = "Get a reference to the value of field `target_protocol` after provisioning.\n"]
    pub fn target_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_passthrough` after provisioning.\n"]
    pub fn tls_passthrough(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_passthrough", self.base))
    }
}

#[derive(Serialize)]
pub struct Loadbalancer_GlbSettings_Cdn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl Loadbalancer_GlbSettings_Cdn {
    #[doc = "Set the field `is_enabled`.\ncache enable flag"]
    pub fn set_is_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_GlbSettings_Cdn {
    type O = terrars::BlockAssignable<Loadbalancer_GlbSettings_Cdn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_GlbSettings_Cdn {}

impl BuildLoadbalancer_GlbSettings_Cdn {
    pub fn build(self) -> Loadbalancer_GlbSettings_Cdn {
        Loadbalancer_GlbSettings_Cdn { is_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_GlbSettings_CdnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_GlbSettings_CdnRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_GlbSettings_CdnRef {
        Loadbalancer_GlbSettings_CdnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_GlbSettings_CdnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\ncache enable flag"]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct Loadbalancer_GlbSettingsDynamic {
    cdn: std::option::Option<terrars::DynamicBlock<Loadbalancer_GlbSettings_Cdn>>,
}

#[derive(Serialize)]
pub struct Loadbalancer_GlbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_priorities: std::option::Option<terrars::RecField<terrars::PrimField<f64>>>,
    target_port: terrars::PrimField<f64>,
    target_protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdn: std::option::Option<std::vec::Vec<Loadbalancer_GlbSettings_Cdn>>,
    dynamic: Loadbalancer_GlbSettingsDynamic,
}

impl Loadbalancer_GlbSettings {
    #[doc = "Set the field `failover_threshold`.\nfail-over threshold"]
    pub fn set_failover_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failover_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_priorities`.\nregion priority map"]
    pub fn set_region_priorities(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.region_priorities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdn`.\n"]
    pub fn set_cdn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Loadbalancer_GlbSettings_Cdn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cdn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cdn = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_GlbSettings {
    type O = terrars::BlockAssignable<Loadbalancer_GlbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_GlbSettings {
    #[doc = "target port rules"]
    pub target_port: terrars::PrimField<f64>,
    #[doc = "target protocol rules"]
    pub target_protocol: terrars::PrimField<std::string::String>,
}

impl BuildLoadbalancer_GlbSettings {
    pub fn build(self) -> Loadbalancer_GlbSettings {
        Loadbalancer_GlbSettings {
            failover_threshold: core::default::Default::default(),
            region_priorities: core::default::Default::default(),
            target_port: self.target_port,
            target_protocol: self.target_protocol,
            cdn: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_GlbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_GlbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_GlbSettingsRef {
        Loadbalancer_GlbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_GlbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failover_threshold` after provisioning.\nfail-over threshold"]
    pub fn failover_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `region_priorities` after provisioning.\nregion priority map"]
    pub fn region_priorities(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_priorities", self.base))
    }

    #[doc = "Get a reference to the value of field `target_port` after provisioning.\ntarget port rules"]
    pub fn target_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_port", self.base))
    }

    #[doc = "Get a reference to the value of field `target_protocol` after provisioning.\ntarget protocol rules"]
    pub fn target_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `cdn` after provisioning.\n"]
    pub fn cdn(&self) -> terrars::ListRef<Loadbalancer_GlbSettings_CdnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn", self.base))
    }
}

#[derive(Serialize)]
pub struct Loadbalancer_Healthcheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    port: terrars::PrimField<f64>,
    protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl Loadbalancer_Healthcheck {
    #[doc = "Set the field `check_interval_seconds`.\n"]
    pub fn set_check_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.check_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_timeout_seconds`.\n"]
    pub fn set_response_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.response_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_Healthcheck {
    type O = terrars::BlockAssignable<Loadbalancer_Healthcheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_Healthcheck {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildLoadbalancer_Healthcheck {
    pub fn build(self) -> Loadbalancer_Healthcheck {
        Loadbalancer_Healthcheck {
            check_interval_seconds: core::default::Default::default(),
            healthy_threshold: core::default::Default::default(),
            path: core::default::Default::default(),
            port: self.port,
            protocol: self.protocol,
            response_timeout_seconds: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_HealthcheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_HealthcheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_HealthcheckRef {
        Loadbalancer_HealthcheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_HealthcheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `check_interval_seconds` after provisioning.\n"]
    pub fn check_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_interval_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `response_timeout_seconds` after provisioning.\n"]
    pub fn response_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct Loadbalancer_StickySessions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_ttl_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Loadbalancer_StickySessions {
    #[doc = "Set the field `cookie_name`.\n"]
    pub fn set_cookie_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cookie_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cookie_ttl_seconds`.\n"]
    pub fn set_cookie_ttl_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cookie_ttl_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Loadbalancer_StickySessions {
    type O = terrars::BlockAssignable<Loadbalancer_StickySessions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancer_StickySessions {}

impl BuildLoadbalancer_StickySessions {
    pub fn build(self) -> Loadbalancer_StickySessions {
        Loadbalancer_StickySessions {
            cookie_name: core::default::Default::default(),
            cookie_ttl_seconds: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Loadbalancer_StickySessionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Loadbalancer_StickySessionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Loadbalancer_StickySessionsRef {
        Loadbalancer_StickySessionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Loadbalancer_StickySessionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_name` after provisioning.\n"]
    pub fn cookie_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_name", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie_ttl_seconds` after provisioning.\n"]
    pub fn cookie_ttl_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cookie_ttl_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct LoadbalancerDynamic {
    domains: std::option::Option<terrars::DynamicBlock<Loadbalancer_Domains>>,
    firewall: std::option::Option<terrars::DynamicBlock<Loadbalancer_Firewall>>,
    forwarding_rule: std::option::Option<terrars::DynamicBlock<Loadbalancer_ForwardingRule>>,
    glb_settings: std::option::Option<terrars::DynamicBlock<Loadbalancer_GlbSettings>>,
    healthcheck: std::option::Option<terrars::DynamicBlock<Loadbalancer_Healthcheck>>,
    sticky_sessions: std::option::Option<terrars::DynamicBlock<Loadbalancer_StickySessions>>,
}
