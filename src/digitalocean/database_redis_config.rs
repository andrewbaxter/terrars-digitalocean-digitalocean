use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseRedisConfigData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    acl_channels_default: Option<PrimField<String>>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    io_threads: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfu_decay_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lfu_log_factor: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maxmemory_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_keyspace_events: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_databases: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    persistence: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pubsub_client_output_buffer_limit: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
}

struct DatabaseRedisConfig_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseRedisConfigData>,
}

#[derive(Clone)]
pub struct DatabaseRedisConfig(Rc<DatabaseRedisConfig_>);

impl DatabaseRedisConfig {
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

    #[doc= "Set the field `acl_channels_default`.\n"]
    pub fn set_acl_channels_default(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().acl_channels_default = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `io_threads`.\n"]
    pub fn set_io_threads(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().io_threads = Some(v.into());
        self
    }

    #[doc= "Set the field `lfu_decay_time`.\n"]
    pub fn set_lfu_decay_time(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lfu_decay_time = Some(v.into());
        self
    }

    #[doc= "Set the field `lfu_log_factor`.\n"]
    pub fn set_lfu_log_factor(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lfu_log_factor = Some(v.into());
        self
    }

    #[doc= "Set the field `maxmemory_policy`.\n"]
    pub fn set_maxmemory_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().maxmemory_policy = Some(v.into());
        self
    }

    #[doc= "Set the field `notify_keyspace_events`.\n"]
    pub fn set_notify_keyspace_events(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().notify_keyspace_events = Some(v.into());
        self
    }

    #[doc= "Set the field `number_of_databases`.\n"]
    pub fn set_number_of_databases(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_databases = Some(v.into());
        self
    }

    #[doc= "Set the field `persistence`.\n"]
    pub fn set_persistence(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().persistence = Some(v.into());
        self
    }

    #[doc= "Set the field `pubsub_client_output_buffer_limit`.\n"]
    pub fn set_pubsub_client_output_buffer_limit(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().pubsub_client_output_buffer_limit = Some(v.into());
        self
    }

    #[doc= "Set the field `ssl`.\n"]
    pub fn set_ssl(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ssl = Some(v.into());
        self
    }

    #[doc= "Set the field `timeout`.\n"]
    pub fn set_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `acl_channels_default` after provisioning.\n"]
    pub fn acl_channels_default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl_channels_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `io_threads` after provisioning.\n"]
    pub fn io_threads(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.io_threads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfu_decay_time` after provisioning.\n"]
    pub fn lfu_decay_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfu_decay_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfu_log_factor` after provisioning.\n"]
    pub fn lfu_log_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfu_log_factor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maxmemory_policy` after provisioning.\n"]
    pub fn maxmemory_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maxmemory_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_keyspace_events` after provisioning.\n"]
    pub fn notify_keyspace_events(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_keyspace_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number_of_databases` after provisioning.\n"]
    pub fn number_of_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_databases", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `persistence` after provisioning.\n"]
    pub fn persistence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persistence", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pubsub_client_output_buffer_limit` after provisioning.\n"]
    pub fn pubsub_client_output_buffer_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_client_output_buffer_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}

impl Referable for DatabaseRedisConfig {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatabaseRedisConfig { }

impl ToListMappable for DatabaseRedisConfig {
    type O = ListRef<DatabaseRedisConfigRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatabaseRedisConfig_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_database_redis_config".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseRedisConfig {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
}

impl BuildDatabaseRedisConfig {
    pub fn build(self, stack: &mut Stack) -> DatabaseRedisConfig {
        let out = DatabaseRedisConfig(Rc::new(DatabaseRedisConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatabaseRedisConfigData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                acl_channels_default: core::default::Default::default(),
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                io_threads: core::default::Default::default(),
                lfu_decay_time: core::default::Default::default(),
                lfu_log_factor: core::default::Default::default(),
                maxmemory_policy: core::default::Default::default(),
                notify_keyspace_events: core::default::Default::default(),
                number_of_databases: core::default::Default::default(),
                persistence: core::default::Default::default(),
                pubsub_client_output_buffer_limit: core::default::Default::default(),
                ssl: core::default::Default::default(),
                timeout: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseRedisConfigRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseRedisConfigRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseRedisConfigRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `acl_channels_default` after provisioning.\n"]
    pub fn acl_channels_default(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.acl_channels_default", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `io_threads` after provisioning.\n"]
    pub fn io_threads(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.io_threads", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfu_decay_time` after provisioning.\n"]
    pub fn lfu_decay_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfu_decay_time", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lfu_log_factor` after provisioning.\n"]
    pub fn lfu_log_factor(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.lfu_log_factor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maxmemory_policy` after provisioning.\n"]
    pub fn maxmemory_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maxmemory_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `notify_keyspace_events` after provisioning.\n"]
    pub fn notify_keyspace_events(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.notify_keyspace_events", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `number_of_databases` after provisioning.\n"]
    pub fn number_of_databases(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_databases", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `persistence` after provisioning.\n"]
    pub fn persistence(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.persistence", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `pubsub_client_output_buffer_limit` after provisioning.\n"]
    pub fn pubsub_client_output_buffer_limit(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pubsub_client_output_buffer_limit", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }
}
