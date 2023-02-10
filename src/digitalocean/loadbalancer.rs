use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct LoadbalancerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_lets_encrypt_dns_records: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    droplet_ids: Option<SetField<PrimField<f64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    droplet_tag: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_backend_keepalive: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_proxy_protocol: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_idle_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_http_to_https: Option<PrimField<bool>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size_unit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_uuid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    forwarding_rule: Option<Vec<LoadbalancerForwardingRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthcheck: Option<Vec<LoadbalancerHealthcheckEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticky_sessions: Option<Vec<LoadbalancerStickySessionsEl>>,
    dynamic: LoadbalancerDynamic,
}

struct Loadbalancer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LoadbalancerData>,
}

#[derive(Clone)]
pub struct Loadbalancer(Rc<Loadbalancer_>);

impl Loadbalancer {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `algorithm`.\n"]
    pub fn set_algorithm(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `disable_lets_encrypt_dns_records`.\n"]
    pub fn set_disable_lets_encrypt_dns_records(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_lets_encrypt_dns_records = Some(v.into());
        self
    }

    #[doc= "Set the field `droplet_ids`.\n"]
    pub fn set_droplet_ids(self, v: impl Into<SetField<PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().droplet_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `droplet_tag`.\n"]
    pub fn set_droplet_tag(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().droplet_tag = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_backend_keepalive`.\n"]
    pub fn set_enable_backend_keepalive(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_backend_keepalive = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_proxy_protocol`.\n"]
    pub fn set_enable_proxy_protocol(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_proxy_protocol = Some(v.into());
        self
    }

    #[doc= "Set the field `http_idle_timeout_seconds`.\n"]
    pub fn set_http_idle_timeout_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().http_idle_timeout_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project_id`.\n"]
    pub fn set_project_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project_id = Some(v.into());
        self
    }

    #[doc= "Set the field `redirect_http_to_https`.\n"]
    pub fn set_redirect_http_to_https(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().redirect_http_to_https = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().size = Some(v.into());
        self
    }

    #[doc= "Set the field `size_unit`.\n"]
    pub fn set_size_unit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().size_unit = Some(v.into());
        self
    }

    #[doc= "Set the field `vpc_uuid`.\n"]
    pub fn set_vpc_uuid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().vpc_uuid = Some(v.into());
        self
    }

    #[doc= "Set the field `forwarding_rule`.\n"]
    pub fn set_forwarding_rule(self, v: impl Into<BlockAssignable<LoadbalancerForwardingRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().forwarding_rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.forwarding_rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `healthcheck`.\n"]
    pub fn set_healthcheck(self, v: impl Into<BlockAssignable<LoadbalancerHealthcheckEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().healthcheck = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.healthcheck = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sticky_sessions`.\n"]
    pub fn set_sticky_sessions(self, v: impl Into<BlockAssignable<LoadbalancerStickySessionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sticky_sessions = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sticky_sessions = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\n"]
    pub fn disable_lets_encrypt_dns_records(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_tag` after provisioning.\n"]
    pub fn droplet_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\n"]
    pub fn enable_backend_keepalive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\n"]
    pub fn enable_proxy_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n"]
    pub fn http_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect_http_to_https` after provisioning.\n"]
    pub fn redirect_http_to_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_unit` after provisioning.\n"]
    pub fn size_unit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the load balancer"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> ListRef<LoadbalancerHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sticky_sessions` after provisioning.\n"]
    pub fn sticky_sessions(&self) -> ListRef<LoadbalancerStickySessionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }
}

impl Resource for Loadbalancer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for Loadbalancer {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for Loadbalancer {
    type O = ListRef<LoadbalancerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for Loadbalancer_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_loadbalancer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLoadbalancer {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub region: PrimField<String>,
}

impl BuildLoadbalancer {
    pub fn build(self, stack: &mut Stack) -> Loadbalancer {
        let out = Loadbalancer(Rc::new(Loadbalancer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(LoadbalancerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                algorithm: core::default::Default::default(),
                disable_lets_encrypt_dns_records: core::default::Default::default(),
                droplet_ids: core::default::Default::default(),
                droplet_tag: core::default::Default::default(),
                enable_backend_keepalive: core::default::Default::default(),
                enable_proxy_protocol: core::default::Default::default(),
                http_idle_timeout_seconds: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project_id: core::default::Default::default(),
                redirect_http_to_https: core::default::Default::default(),
                region: self.region,
                size: core::default::Default::default(),
                size_unit: core::default::Default::default(),
                vpc_uuid: core::default::Default::default(),
                forwarding_rule: core::default::Default::default(),
                healthcheck: core::default::Default::default(),
                sticky_sessions: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LoadbalancerRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadbalancerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LoadbalancerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disable_lets_encrypt_dns_records` after provisioning.\n"]
    pub fn disable_lets_encrypt_dns_records(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disable_lets_encrypt_dns_records", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_ids` after provisioning.\n"]
    pub fn droplet_ids(&self) -> SetRef<PrimExpr<f64>> {
        SetRef::new(self.shared().clone(), format!("{}.droplet_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_tag` after provisioning.\n"]
    pub fn droplet_tag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_tag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_backend_keepalive` after provisioning.\n"]
    pub fn enable_backend_keepalive(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_backend_keepalive", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_proxy_protocol` after provisioning.\n"]
    pub fn enable_proxy_protocol(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_proxy_protocol", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `http_idle_timeout_seconds` after provisioning.\n"]
    pub fn http_idle_timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.http_idle_timeout_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_id` after provisioning.\n"]
    pub fn project_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `redirect_http_to_https` after provisioning.\n"]
    pub fn redirect_http_to_https(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_to_https", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_unit` after provisioning.\n"]
    pub fn size_unit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_unit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\nthe uniform resource name for the load balancer"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_uuid` after provisioning.\n"]
    pub fn vpc_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `healthcheck` after provisioning.\n"]
    pub fn healthcheck(&self) -> ListRef<LoadbalancerHealthcheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.healthcheck", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sticky_sessions` after provisioning.\n"]
    pub fn sticky_sessions(&self) -> ListRef<LoadbalancerStickySessionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sticky_sessions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LoadbalancerForwardingRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_name: Option<PrimField<String>>,
    entry_port: PrimField<f64>,
    entry_protocol: PrimField<String>,
    target_port: PrimField<f64>,
    target_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tls_passthrough: Option<PrimField<bool>>,
}

impl LoadbalancerForwardingRuleEl {
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

    #[doc= "Set the field `tls_passthrough`.\n"]
    pub fn set_tls_passthrough(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.tls_passthrough = Some(v.into());
        self
    }
}

impl ToListMappable for LoadbalancerForwardingRuleEl {
    type O = BlockAssignable<LoadbalancerForwardingRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancerForwardingRuleEl {
    #[doc= ""]
    pub entry_port: PrimField<f64>,
    #[doc= ""]
    pub entry_protocol: PrimField<String>,
    #[doc= ""]
    pub target_port: PrimField<f64>,
    #[doc= ""]
    pub target_protocol: PrimField<String>,
}

impl BuildLoadbalancerForwardingRuleEl {
    pub fn build(self) -> LoadbalancerForwardingRuleEl {
        LoadbalancerForwardingRuleEl {
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

pub struct LoadbalancerForwardingRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadbalancerForwardingRuleElRef {
    fn new(shared: StackShared, base: String) -> LoadbalancerForwardingRuleElRef {
        LoadbalancerForwardingRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadbalancerForwardingRuleElRef {
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
pub struct LoadbalancerHealthcheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    check_interval_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    port: PrimField<f64>,
    protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
}

impl LoadbalancerHealthcheckEl {
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

impl ToListMappable for LoadbalancerHealthcheckEl {
    type O = BlockAssignable<LoadbalancerHealthcheckEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancerHealthcheckEl {
    #[doc= ""]
    pub port: PrimField<f64>,
    #[doc= ""]
    pub protocol: PrimField<String>,
}

impl BuildLoadbalancerHealthcheckEl {
    pub fn build(self) -> LoadbalancerHealthcheckEl {
        LoadbalancerHealthcheckEl {
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

pub struct LoadbalancerHealthcheckElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadbalancerHealthcheckElRef {
    fn new(shared: StackShared, base: String) -> LoadbalancerHealthcheckElRef {
        LoadbalancerHealthcheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadbalancerHealthcheckElRef {
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
pub struct LoadbalancerStickySessionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_ttl_seconds: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl LoadbalancerStickySessionsEl {
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

impl ToListMappable for LoadbalancerStickySessionsEl {
    type O = BlockAssignable<LoadbalancerStickySessionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLoadbalancerStickySessionsEl {}

impl BuildLoadbalancerStickySessionsEl {
    pub fn build(self) -> LoadbalancerStickySessionsEl {
        LoadbalancerStickySessionsEl {
            cookie_name: core::default::Default::default(),
            cookie_ttl_seconds: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct LoadbalancerStickySessionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for LoadbalancerStickySessionsElRef {
    fn new(shared: StackShared, base: String) -> LoadbalancerStickySessionsElRef {
        LoadbalancerStickySessionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LoadbalancerStickySessionsElRef {
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

#[derive(Serialize, Default)]
struct LoadbalancerDynamic {
    forwarding_rule: Option<DynamicBlock<LoadbalancerForwardingRuleEl>>,
    healthcheck: Option<DynamicBlock<LoadbalancerHealthcheckEl>>,
    sticky_sessions: Option<DynamicBlock<LoadbalancerStickySessionsEl>>,
}
