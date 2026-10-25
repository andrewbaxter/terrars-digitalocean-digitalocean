#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataLoadbalancerData_ {
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

struct DataLoadbalancer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadbalancerData_>,
}

#[derive(Clone)]
pub struct DataLoadbalancer(std::rc::Rc<DataLoadbalancer_>);

impl DataLoadbalancer {
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

    #[doc = "Set the field `id`.\nid of the load balancer"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nname of the load balancer"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nalgorithm used to determine which backend Droplet will be selected by a client"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\nwhether to disable automatic DNS record creation for Let's Encrypt certificates that are added to the load balancer"]
    pub fn disable_lets_encrypt_dns_records(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nthe list of domains required to ingress traffic to global load balancer"]
    pub fn domains(&self) -> terrars::SetRef<DataLoadbalancer_DomainsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `droplet_ids` after provisioning.\nids of the droplets assigned to the load balancer"]
    pub fn droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `droplet_tag` after provisioning.\nthe name of a tag corresponding to droplets assigned to the load balancer"]
    pub fn droplet_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\nwhether HTTP keepalive connections are maintained to target Droplets"]
    pub fn enable_backend_keepalive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nwhether PROXY Protocol should be used to pass information from connecting client requests to the backend service"]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `firewall` after provisioning.\nthe firewall rules for allowing/denying traffic to the load balancer"]
    pub fn firewall(&self) -> terrars::ListRef<DataLoadbalancer_FirewallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nlist of forwarding rules of the load balancer"]
    pub fn forwarding_rule(&self) -> terrars::SetRef<DataLoadbalancer_ForwardingRuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.forwarding_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `glb_settings` after provisioning.\nconfiguration options for global load balancer"]
    pub fn glb_settings(&self) -> terrars::ListRef<DataLoadbalancer_GlbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glb_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthcheck` after provisioning.\nhealth check settings for the load balancer"]
    pub fn healthcheck(&self) -> terrars::ListRef<DataLoadbalancer_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n Specifies the idle timeout for HTTPS connections on the load balancer."]
    pub fn http_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nid of the load balancer"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\npublic-facing IP address of the load balancer"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the load balancer"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nthe type of network the load balancer is accessible from (EXTERNAL or INTERNAL)"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the load balancer is associated with."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redirect_http_to_https` after provisioning.\nwhether http requests will be redirected to https"]
    pub fn redirect_http_to_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nthe region that the load balancer is deployed in"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nthe size of the load balancer"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_unit` after provisioning.\nthe size of the load balancer."]
    pub fn size_unit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\ncurrent state of the Load Balancer"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sticky_sessions` after provisioning.\nsticky sessions settings for the load balancer"]
    pub fn sticky_sessions(&self) -> terrars::ListRef<DataLoadbalancer_StickySessionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_uuid` after provisioning.\nUUID of the VPC subnet in which the load balancer is located"]
    pub fn subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_load_balancer_ids` after provisioning.\nlist of load balancer IDs to put behind a global load balancer"]
    pub fn target_load_balancer_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_load_balancer_ids", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the load balancer is located"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataLoadbalancer {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLoadbalancer { }

impl terrars::ToListMappable for DataLoadbalancer {
    type O = terrars::ListRef<DataLoadbalancerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLoadbalancer_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_loadbalancer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadbalancer {
    pub tf_id: std::string::String,
}

impl BuildDataLoadbalancer {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLoadbalancer {
        let out = DataLoadbalancer(std::rc::Rc::new(DataLoadbalancer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLoadbalancerData_ {
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

pub struct DataLoadbalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadbalancerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nalgorithm used to determine which backend Droplet will be selected by a client"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\nwhether to disable automatic DNS record creation for Let's Encrypt certificates that are added to the load balancer"]
    pub fn disable_lets_encrypt_dns_records(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `domains` after provisioning.\nthe list of domains required to ingress traffic to global load balancer"]
    pub fn domains(&self) -> terrars::SetRef<DataLoadbalancer_DomainsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `droplet_ids` after provisioning.\nids of the droplets assigned to the load balancer"]
    pub fn droplet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `droplet_tag` after provisioning.\nthe name of a tag corresponding to droplets assigned to the load balancer"]
    pub fn droplet_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\nwhether HTTP keepalive connections are maintained to target Droplets"]
    pub fn enable_backend_keepalive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nwhether PROXY Protocol should be used to pass information from connecting client requests to the backend service"]
    pub fn enable_proxy_protocol(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `firewall` after provisioning.\nthe firewall rules for allowing/denying traffic to the load balancer"]
    pub fn firewall(&self) -> terrars::ListRef<DataLoadbalancer_FirewallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nlist of forwarding rules of the load balancer"]
    pub fn forwarding_rule(&self) -> terrars::SetRef<DataLoadbalancer_ForwardingRuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.forwarding_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `glb_settings` after provisioning.\nconfiguration options for global load balancer"]
    pub fn glb_settings(&self) -> terrars::ListRef<DataLoadbalancer_GlbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.glb_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthcheck` after provisioning.\nhealth check settings for the load balancer"]
    pub fn healthcheck(&self) -> terrars::ListRef<DataLoadbalancer_HealthcheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n Specifies the idle timeout for HTTPS connections on the load balancer."]
    pub fn http_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nid of the load balancer"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ip` after provisioning.\npublic-facing IP address of the load balancer"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6` after provisioning.\n"]
    pub fn ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the load balancer"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nthe type of network the load balancer is accessible from (EXTERNAL or INTERNAL)"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the load balancer is associated with."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redirect_http_to_https` after provisioning.\nwhether http requests will be redirected to https"]
    pub fn redirect_http_to_https(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nthe region that the load balancer is deployed in"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nthe size of the load balancer"]
    pub fn size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `size_unit` after provisioning.\nthe size of the load balancer."]
    pub fn size_unit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\ncurrent state of the Load Balancer"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sticky_sessions` after provisioning.\nsticky sessions settings for the load balancer"]
    pub fn sticky_sessions(&self) -> terrars::ListRef<DataLoadbalancer_StickySessionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subnet_uuid` after provisioning.\nUUID of the VPC subnet in which the load balancer is located"]
    pub fn subnet_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_uuid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_load_balancer_ids` after provisioning.\nlist of load balancer IDs to put behind a global load balancer"]
    pub fn target_load_balancer_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_load_balancer_ids", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the load balancer is located"]
    pub fn vpc_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancer_Domains {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_managed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_validation_error_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_error_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLoadbalancer_Domains {
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

    #[doc = "Set the field `is_managed`.\n"]
    pub fn set_is_managed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_managed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_validation_error_reasons`.\n"]
    pub fn set_ssl_validation_error_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ssl_validation_error_reasons = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verification_error_reasons`.\n"]
    pub fn set_verification_error_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.verification_error_reasons = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadbalancer_Domains {
    type O = terrars::BlockAssignable<DataLoadbalancer_Domains>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_Domains {}

impl BuildDataLoadbalancer_Domains {
    pub fn build(self) -> DataLoadbalancer_Domains {
        DataLoadbalancer_Domains {
            certificate_id: core::default::Default::default(),
            certificate_name: core::default::Default::default(),
            is_managed: core::default::Default::default(),
            name: core::default::Default::default(),
            ssl_validation_error_reasons: core::default::Default::default(),
            verification_error_reasons: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_DomainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_DomainsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_DomainsRef {
        DataLoadbalancer_DomainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_DomainsRef {
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

    #[doc = "Get a reference to the value of field `is_managed` after provisioning.\n"]
    pub fn is_managed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_managed", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_validation_error_reasons` after provisioning.\n"]
    pub fn ssl_validation_error_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_validation_error_reasons", self.base))
    }

    #[doc = "Get a reference to the value of field `verification_error_reasons` after provisioning.\n"]
    pub fn verification_error_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verification_error_reasons", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancer_Firewall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataLoadbalancer_Firewall {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allow = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.deny = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadbalancer_Firewall {
    type O = terrars::BlockAssignable<DataLoadbalancer_Firewall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_Firewall {}

impl BuildDataLoadbalancer_Firewall {
    pub fn build(self) -> DataLoadbalancer_Firewall {
        DataLoadbalancer_Firewall {
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_FirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_FirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_FirewallRef {
        DataLoadbalancer_FirewallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_FirewallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancer_ForwardingRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_passthrough: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLoadbalancer_ForwardingRule {
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

    #[doc = "Set the field `entry_port`.\n"]
    pub fn set_entry_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.entry_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_protocol`.\n"]
    pub fn set_entry_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entry_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_port`.\n"]
    pub fn set_target_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_protocol`.\n"]
    pub fn set_target_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_passthrough`.\n"]
    pub fn set_tls_passthrough(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls_passthrough = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadbalancer_ForwardingRule {
    type O = terrars::BlockAssignable<DataLoadbalancer_ForwardingRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_ForwardingRule {}

impl BuildDataLoadbalancer_ForwardingRule {
    pub fn build(self) -> DataLoadbalancer_ForwardingRule {
        DataLoadbalancer_ForwardingRule {
            certificate_id: core::default::Default::default(),
            certificate_name: core::default::Default::default(),
            entry_port: core::default::Default::default(),
            entry_protocol: core::default::Default::default(),
            target_port: core::default::Default::default(),
            target_protocol: core::default::Default::default(),
            tls_passthrough: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_ForwardingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_ForwardingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_ForwardingRuleRef {
        DataLoadbalancer_ForwardingRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_ForwardingRuleRef {
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
pub struct DataLoadbalancer_GlbSettings_Cdn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLoadbalancer_GlbSettings_Cdn {
    #[doc = "Set the field `is_enabled`.\n"]
    pub fn set_is_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadbalancer_GlbSettings_Cdn {
    type O = terrars::BlockAssignable<DataLoadbalancer_GlbSettings_Cdn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_GlbSettings_Cdn {}

impl BuildDataLoadbalancer_GlbSettings_Cdn {
    pub fn build(self) -> DataLoadbalancer_GlbSettings_Cdn {
        DataLoadbalancer_GlbSettings_Cdn { is_enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_GlbSettings_CdnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_GlbSettings_CdnRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_GlbSettings_CdnRef {
        DataLoadbalancer_GlbSettings_CdnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_GlbSettings_CdnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\n"]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancer_GlbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdn: std::option::Option<terrars::ListField<DataLoadbalancer_GlbSettings_Cdn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_priorities: std::option::Option<terrars::RecField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLoadbalancer_GlbSettings {
    #[doc = "Set the field `cdn`.\n"]
    pub fn set_cdn(mut self, v: impl std::convert::Into<terrars::ListField<DataLoadbalancer_GlbSettings_Cdn>>) -> Self {
        self.cdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `failover_threshold`.\n"]
    pub fn set_failover_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failover_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_priorities`.\n"]
    pub fn set_region_priorities(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.region_priorities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_port`.\n"]
    pub fn set_target_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_protocol`.\n"]
    pub fn set_target_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLoadbalancer_GlbSettings {
    type O = terrars::BlockAssignable<DataLoadbalancer_GlbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_GlbSettings {}

impl BuildDataLoadbalancer_GlbSettings {
    pub fn build(self) -> DataLoadbalancer_GlbSettings {
        DataLoadbalancer_GlbSettings {
            cdn: core::default::Default::default(),
            failover_threshold: core::default::Default::default(),
            region_priorities: core::default::Default::default(),
            target_port: core::default::Default::default(),
            target_protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_GlbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_GlbSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_GlbSettingsRef {
        DataLoadbalancer_GlbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_GlbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cdn` after provisioning.\n"]
    pub fn cdn(&self) -> terrars::ListRef<DataLoadbalancer_GlbSettings_CdnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn", self.base))
    }

    #[doc = "Get a reference to the value of field `failover_threshold` after provisioning.\n"]
    pub fn failover_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `region_priorities` after provisioning.\n"]
    pub fn region_priorities(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.region_priorities", self.base))
    }

    #[doc = "Get a reference to the value of field `target_port` after provisioning.\n"]
    pub fn target_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_port", self.base))
    }

    #[doc = "Get a reference to the value of field `target_protocol` after provisioning.\n"]
    pub fn target_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancer_Healthcheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLoadbalancer_Healthcheck {
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

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataLoadbalancer_Healthcheck {
    type O = terrars::BlockAssignable<DataLoadbalancer_Healthcheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_Healthcheck {}

impl BuildDataLoadbalancer_Healthcheck {
    pub fn build(self) -> DataLoadbalancer_Healthcheck {
        DataLoadbalancer_Healthcheck {
            check_interval_seconds: core::default::Default::default(),
            healthy_threshold: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            response_timeout_seconds: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_HealthcheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_HealthcheckRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_HealthcheckRef {
        DataLoadbalancer_HealthcheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_HealthcheckRef {
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
pub struct DataLoadbalancer_StickySessions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_ttl_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLoadbalancer_StickySessions {
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

impl terrars::ToListMappable for DataLoadbalancer_StickySessions {
    type O = terrars::BlockAssignable<DataLoadbalancer_StickySessions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancer_StickySessions {}

impl BuildDataLoadbalancer_StickySessions {
    pub fn build(self) -> DataLoadbalancer_StickySessions {
        DataLoadbalancer_StickySessions {
            cookie_name: core::default::Default::default(),
            cookie_ttl_seconds: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLoadbalancer_StickySessionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancer_StickySessionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancer_StickySessionsRef {
        DataLoadbalancer_StickySessionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancer_StickySessionsRef {
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
