use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataLoadbalancerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

struct DataLoadbalancer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataLoadbalancerData>,
}

#[derive(Clone)]
pub struct DataLoadbalancer(Rc<DataLoadbalancer_>);

impl DataLoadbalancer {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\nid of the load balancer"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nname of the load balancer"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nalgorithm used to determine which backend Droplet will be selected by a client"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\nwhether to disable automatic DNS record creation for Let's Encrypt certificates that are added to the load balancer"]
    pub fn disable_lets_encrypt_dns_records(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_ids` after provisioning.\nids of the droplets assigned to the load balancer"]
    pub fn droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_tag` after provisioning.\nthe name of a tag corresponding to droplets assigned to the load balancer"]
    pub fn droplet_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\nwhether HTTP keepalive connections are maintained to target Droplets"]
    pub fn enable_backend_keepalive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nwhether PROXY Protocol should be used to pass information from connecting client requests to the backend service"]
    pub fn enable_proxy_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `forwarding_rule` after provisioning.\nlist of forwarding rules of the load balancer"]
    pub fn forwarding_rule(&self) -> SetRef<DataLoadbalancerForwardingRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.forwarding_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\nhealth check settings for the load balancer"]
    pub fn healthcheck(&self) -> ListRef<DataLoadbalancerHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n Specifies the idle timeout for HTTPS connections on the load balancer."]
    pub fn http_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nid of the load balancer"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\npublic-facing IP address of the load balancer"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the load balancer"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the load balancer is associated with."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect_http_to_https` after provisioning.\nwhether http requests will be redirected to https"]
    pub fn redirect_http_to_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nthe region that the load balancer is deployed in"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nthe size of the load balancer"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_unit` after provisioning.\nthe size of the load balancer."]
    pub fn size_unit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\ncurrent state of the Load Balancer"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sticky_sessions` after provisioning.\nsticky sessions settings for the load balancer"]
    pub fn sticky_sessions(&self) -> ListRef<DataLoadbalancerStickySessionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the load balancer"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the load balancer is located"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

impl Datasource for DataLoadbalancer {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataLoadbalancer {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataLoadbalancer {
    type O = ListRef<DataLoadbalancerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataLoadbalancer_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_loadbalancer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLoadbalancer {
    pub tf_id: String,
}

impl BuildDataLoadbalancer {
    pub fn build(self, stack: &mut Stack) -> DataLoadbalancer {
        let out = DataLoadbalancer(Rc::new(DataLoadbalancer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataLoadbalancerData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLoadbalancerRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadbalancerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLoadbalancerRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nalgorithm used to determine which backend Droplet will be selected by a client"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\nwhether to disable automatic DNS record creation for Let's Encrypt certificates that are added to the load balancer"]
    pub fn disable_lets_encrypt_dns_records(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_ids` after provisioning.\nids of the droplets assigned to the load balancer"]
    pub fn droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_tag` after provisioning.\nthe name of a tag corresponding to droplets assigned to the load balancer"]
    pub fn droplet_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\nwhether HTTP keepalive connections are maintained to target Droplets"]
    pub fn enable_backend_keepalive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\nwhether PROXY Protocol should be used to pass information from connecting client requests to the backend service"]
    pub fn enable_proxy_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `forwarding_rule` after provisioning.\nlist of forwarding rules of the load balancer"]
    pub fn forwarding_rule(&self) -> SetRef<DataLoadbalancerForwardingRuleElRef> {
        SetRef::new(self.shared().clone(), format!("{}.forwarding_rule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\nhealth check settings for the load balancer"]
    pub fn healthcheck(&self) -> ListRef<DataLoadbalancerHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n Specifies the idle timeout for HTTPS connections on the load balancer."]
    pub fn http_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nid of the load balancer"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\npublic-facing IP address of the load balancer"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the load balancer"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project that the load balancer is associated with."]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect_http_to_https` after provisioning.\nwhether http requests will be redirected to https"]
    pub fn redirect_http_to_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nthe region that the load balancer is deployed in"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nthe size of the load balancer"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_unit` after provisioning.\nthe size of the load balancer."]
    pub fn size_unit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\ncurrent state of the Load Balancer"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sticky_sessions` after provisioning.\nsticky sessions settings for the load balancer"]
    pub fn sticky_sessions(&self) -> ListRef<DataLoadbalancerStickySessionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the load balancer"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\nUUID of the VPC in which the load balancer is located"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancerForwardingRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entry_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_passthrough: Option<PrimField<bool>>,
}

impl DataLoadbalancerForwardingRuleEl {
    #[doc= "Set the field `certificate_id`.\n"]
    pub fn set_certificate_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_id = Some(v.into());
        self
    }

    #[doc= "Set the field `certificate_name`.\n"]
    pub fn set_certificate_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_name = Some(v.into());
        self
    }

    #[doc= "Set the field `entry_port`.\n"]
    pub fn set_entry_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.entry_port = Some(v.into());
        self
    }

    #[doc= "Set the field `entry_protocol`.\n"]
    pub fn set_entry_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.entry_protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `target_port`.\n"]
    pub fn set_target_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_port = Some(v.into());
        self
    }

    #[doc= "Set the field `target_protocol`.\n"]
    pub fn set_target_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `tls_passthrough`.\n"]
    pub fn set_tls_passthrough(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls_passthrough = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadbalancerForwardingRuleEl {
    type O = BlockAssignable<DataLoadbalancerForwardingRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadbalancerForwardingRuleElRef {
    fn new(shared: StackShared, base: String) -> DataLoadbalancerForwardingRuleElRef {
        DataLoadbalancerForwardingRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerForwardingRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `certificate_id` after provisioning.\n"]
    pub fn certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc= "Get a reference to the value of field `certificate_name` after provisioning.\n"]
    pub fn certificate_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.certificate_name", self.base))
    }

    #[doc= "Get a reference to the value of field `entry_port` after provisioning.\n"]
    pub fn entry_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_port", self.base))
    }

    #[doc= "Get a reference to the value of field `entry_protocol` after provisioning.\n"]
    pub fn entry_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.entry_protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `target_port` after provisioning.\n"]
    pub fn target_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_port", self.base))
    }

    #[doc= "Get a reference to the value of field `target_protocol` after provisioning.\n"]
    pub fn target_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `tls_passthrough` after provisioning.\n"]
    pub fn tls_passthrough(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.tls_passthrough", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancerHealthcheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_interval_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
}

impl DataLoadbalancerHealthcheckEl {
    #[doc= "Set the field `check_interval_seconds`.\n"]
    pub fn set_check_interval_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.check_interval_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }

    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `response_timeout_seconds`.\n"]
    pub fn set_response_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.response_timeout_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_threshold = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadbalancerHealthcheckEl {
    type O = BlockAssignable<DataLoadbalancerHealthcheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadbalancerHealthcheckElRef {
    fn new(shared: StackShared, base: String) -> DataLoadbalancerHealthcheckElRef {
        DataLoadbalancerHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerHealthcheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `check_interval_seconds` after provisioning.\n"]
    pub fn check_interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.check_interval_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `response_timeout_seconds` after provisioning.\n"]
    pub fn response_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.response_timeout_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLoadbalancerStickySessionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_ttl_seconds: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataLoadbalancerStickySessionsEl {
    #[doc= "Set the field `cookie_name`.\n"]
    pub fn set_cookie_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cookie_name = Some(v.into());
        self
    }

    #[doc= "Set the field `cookie_ttl_seconds`.\n"]
    pub fn set_cookie_ttl_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cookie_ttl_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataLoadbalancerStickySessionsEl {
    type O = BlockAssignable<DataLoadbalancerStickySessionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataLoadbalancerStickySessionsElRef {
    fn new(shared: StackShared, base: String) -> DataLoadbalancerStickySessionsElRef {
        DataLoadbalancerStickySessionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLoadbalancerStickySessionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cookie_name` after provisioning.\n"]
    pub fn cookie_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cookie_name", self.base))
    }

    #[doc= "Get a reference to the value of field `cookie_ttl_seconds` after provisioning.\n"]
    pub fn cookie_ttl_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cookie_ttl_seconds", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
