use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataLoadbalancerData {
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
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLoadbalancer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLoadbalancerData>,
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

    #[doc = "Set the field `type_`.\nthe type of the load balancer (GLOBAL or REGIONAL)"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
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
    pub fn firewall(&self) -> terrars::ListRef<DataLoadbalancerFirewallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nlist of forwarding rules of the load balancer"]
    pub fn forwarding_rule(&self) -> terrars::SetRef<DataLoadbalancerForwardingRuleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.forwarding_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthcheck` after provisioning.\nhealth check settings for the load balancer"]
    pub fn healthcheck(&self) -> terrars::ListRef<DataLoadbalancerHealthcheckElRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the load balancer"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
    pub fn sticky_sessions(&self) -> terrars::ListRef<DataLoadbalancerStickySessionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nthe type of the load balancer (GLOBAL or REGIONAL)"]
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
            data: std::cell::RefCell::new(DataLoadbalancerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                type_: core::default::Default::default(),
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
    pub fn firewall(&self) -> terrars::ListRef<DataLoadbalancerFirewallElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `forwarding_rule` after provisioning.\nlist of forwarding rules of the load balancer"]
    pub fn forwarding_rule(&self) -> terrars::SetRef<DataLoadbalancerForwardingRuleElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.forwarding_rule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `healthcheck` after provisioning.\nhealth check settings for the load balancer"]
    pub fn healthcheck(&self) -> terrars::ListRef<DataLoadbalancerHealthcheckElRef> {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the load balancer"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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
    pub fn sticky_sessions(&self) -> terrars::ListRef<DataLoadbalancerStickySessionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nthe type of the load balancer (GLOBAL or REGIONAL)"]
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
pub struct DataLoadbalancerFirewallEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataLoadbalancerFirewallEl {
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

impl terrars::ToListMappable for DataLoadbalancerFirewallEl {
    type O = terrars::BlockAssignable<DataLoadbalancerFirewallEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancerFirewallEl {}

impl BuildDataLoadbalancerFirewallEl {
    pub fn build(self) -> DataLoadbalancerFirewallEl {
        DataLoadbalancerFirewallEl {
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
        }
    }
}

pub struct DataLoadbalancerFirewallElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancerFirewallElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancerFirewallElRef {
        DataLoadbalancerFirewallElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerFirewallElRef {
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
pub struct DataLoadbalancerForwardingRuleEl {
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

impl DataLoadbalancerForwardingRuleEl {
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

impl terrars::ToListMappable for DataLoadbalancerForwardingRuleEl {
    type O = terrars::BlockAssignable<DataLoadbalancerForwardingRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancerForwardingRuleEl {}

impl BuildDataLoadbalancerForwardingRuleEl {
    pub fn build(self) -> DataLoadbalancerForwardingRuleEl {
        DataLoadbalancerForwardingRuleEl {
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

pub struct DataLoadbalancerForwardingRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancerForwardingRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancerForwardingRuleElRef {
        DataLoadbalancerForwardingRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerForwardingRuleElRef {
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
pub struct DataLoadbalancerHealthcheckEl {
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

impl DataLoadbalancerHealthcheckEl {
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

impl terrars::ToListMappable for DataLoadbalancerHealthcheckEl {
    type O = terrars::BlockAssignable<DataLoadbalancerHealthcheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancerHealthcheckEl {}

impl BuildDataLoadbalancerHealthcheckEl {
    pub fn build(self) -> DataLoadbalancerHealthcheckEl {
        DataLoadbalancerHealthcheckEl {
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

pub struct DataLoadbalancerHealthcheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancerHealthcheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancerHealthcheckElRef {
        DataLoadbalancerHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerHealthcheckElRef {
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
pub struct DataLoadbalancerStickySessionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie_ttl_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLoadbalancerStickySessionsEl {
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

impl terrars::ToListMappable for DataLoadbalancerStickySessionsEl {
    type O = terrars::BlockAssignable<DataLoadbalancerStickySessionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLoadbalancerStickySessionsEl {}

impl BuildDataLoadbalancerStickySessionsEl {
    pub fn build(self) -> DataLoadbalancerStickySessionsEl {
        DataLoadbalancerStickySessionsEl {
            cookie_name: core::default::Default::default(),
            cookie_ttl_seconds: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataLoadbalancerStickySessionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLoadbalancerStickySessionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLoadbalancerStickySessionsElRef {
        DataLoadbalancerStickySessionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerStickySessionsElRef {
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
