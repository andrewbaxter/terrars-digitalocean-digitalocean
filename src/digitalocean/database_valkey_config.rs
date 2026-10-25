#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseValkeyConfigData_ {
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
    acl_channels_default: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequent_snapshots: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    io_threads: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lfu_decay_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lfu_log_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notify_keyspace_events: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_databases: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistence: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_client_output_buffer_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valkey_active_expire_effort: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valkey_maxmemory_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DatabaseValkeyConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseValkeyConfigData_>,
}

#[derive(Clone)]
pub struct DatabaseValkeyConfig(std::rc::Rc<DatabaseValkeyConfig_>);

impl DatabaseValkeyConfig {
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

    #[doc =
        "Set the field `acl_channels_default`.\nDetermines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default."]
    pub fn set_acl_channels_default(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().acl_channels_default = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `frequent_snapshots`.\nFrequent RDB snapshots. When enabled, Valkey will create frequent local RDB snapshots. When disabled, Valkey will only take RDB snapshots when a backup is created, based on the backup schedule. This setting is ignored when valkey_persistence is set to off."]
    pub fn set_frequent_snapshots(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().frequent_snapshots = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `io_threads`.\nThe number of IO threads used by Valkey. Must be between 1 and 32."]
    pub fn set_io_threads(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().io_threads = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lfu_decay_time`.\nThe decay time for Valkey's LFU cache eviction. Must be between 1 and 120."]
    pub fn set_lfu_decay_time(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lfu_decay_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `lfu_log_factor`.\nThe log factor for Valkey's LFU (Least Frequently Used) cache eviction. Must be between 1 and 100."]
    pub fn set_lfu_log_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().lfu_log_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notify_keyspace_events`.\nSet notify-keyspace-events option. Requires at least K or E and accepts any combination of the following options. Setting the parameter to \"\" disables notifications.\n\nK — Keyspace events\nE — Keyevent events\ng — Generic commands (e.g. DEL, EXPIRE, RENAME, ...)\n$ — String commands\nl — List commands\ns — Set commands\nh — Hash commands\nz — Sorted set commands\nt — Stream commands\nd — Module key type events\nx — Expired events\ne — Evicted events\nm — Key miss events\nn — New key events\nA — Alias for \"g$lshztxed\""]
    pub fn set_notify_keyspace_events(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().notify_keyspace_events = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `number_of_databases`.\nThe number of logical databases in the Valkey cluster. Must be between 1 and 128."]
    pub fn set_number_of_databases(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().number_of_databases = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `persistence`.\nWhen persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked."]
    pub fn set_persistence(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().persistence = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `pubsub_client_output_buffer_limit`.\nSet output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan."]
    pub fn set_pubsub_client_output_buffer_limit(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().pubsub_client_output_buffer_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl`.\nWhether to enable SSL/TLS for connections to the Valkey cluster."]
    pub fn set_ssl(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nThe timeout (in seconds) for Valkey client connections."]
    pub fn set_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `valkey_active_expire_effort`.\nActive expire effort. Valkey reclaims expired keys both when accessed and in the background. The background process scans for expired keys to free memory. Increasing the active-expire-effort setting (default 1, max 10) uses more CPU to reclaim expired keys faster, reducing memory usage but potentially increasing latency."]
    pub fn set_valkey_active_expire_effort(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().valkey_active_expire_effort = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `valkey_maxmemory_policy`.\nA string specifying the desired eviction policy for a Caching or Valkey cluster."]
    pub fn set_valkey_maxmemory_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().valkey_maxmemory_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `acl_channels_default` after provisioning.\nDetermines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default."]
    pub fn acl_channels_default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_channels_default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nA unique identifier for the database cluster."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `frequent_snapshots` after provisioning.\nFrequent RDB snapshots. When enabled, Valkey will create frequent local RDB snapshots. When disabled, Valkey will only take RDB snapshots when a backup is created, based on the backup schedule. This setting is ignored when valkey_persistence is set to off."]
    pub fn frequent_snapshots(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequent_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `io_threads` after provisioning.\nThe number of IO threads used by Valkey. Must be between 1 and 32."]
    pub fn io_threads(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.io_threads", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lfu_decay_time` after provisioning.\nThe decay time for Valkey's LFU cache eviction. Must be between 1 and 120."]
    pub fn lfu_decay_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfu_decay_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lfu_log_factor` after provisioning.\nThe log factor for Valkey's LFU (Least Frequently Used) cache eviction. Must be between 1 and 100."]
    pub fn lfu_log_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfu_log_factor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notify_keyspace_events` after provisioning.\nSet notify-keyspace-events option. Requires at least K or E and accepts any combination of the following options. Setting the parameter to \"\" disables notifications.\n\nK — Keyspace events\nE — Keyevent events\ng — Generic commands (e.g. DEL, EXPIRE, RENAME, ...)\n$ — String commands\nl — List commands\ns — Set commands\nh — Hash commands\nz — Sorted set commands\nt — Stream commands\nd — Module key type events\nx — Expired events\ne — Evicted events\nm — Key miss events\nn — New key events\nA — Alias for \"g$lshztxed\""]
    pub fn notify_keyspace_events(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify_keyspace_events", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number_of_databases` after provisioning.\nThe number of logical databases in the Valkey cluster. Must be between 1 and 128."]
    pub fn number_of_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_databases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence` after provisioning.\nWhen persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked."]
    pub fn persistence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_client_output_buffer_limit` after provisioning.\nSet output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan."]
    pub fn pubsub_client_output_buffer_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pubsub_client_output_buffer_limit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nWhether to enable SSL/TLS for connections to the Valkey cluster."]
    pub fn ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) for Valkey client connections."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valkey_active_expire_effort` after provisioning.\nActive expire effort. Valkey reclaims expired keys both when accessed and in the background. The background process scans for expired keys to free memory. Increasing the active-expire-effort setting (default 1, max 10) uses more CPU to reclaim expired keys faster, reducing memory usage but potentially increasing latency."]
    pub fn valkey_active_expire_effort(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valkey_active_expire_effort", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valkey_maxmemory_policy` after provisioning.\nA string specifying the desired eviction policy for a Caching or Valkey cluster."]
    pub fn valkey_maxmemory_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valkey_maxmemory_policy", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseValkeyConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseValkeyConfig { }

impl terrars::ToListMappable for DatabaseValkeyConfig {
    type O = terrars::ListRef<DatabaseValkeyConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseValkeyConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_valkey_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseValkeyConfig {
    pub tf_id: std::string::String,
    #[doc = "A unique identifier for the database cluster."]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseValkeyConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseValkeyConfig {
        let out = DatabaseValkeyConfig(std::rc::Rc::new(DatabaseValkeyConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseValkeyConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                acl_channels_default: core::default::Default::default(),
                cluster_id: self.cluster_id,
                frequent_snapshots: core::default::Default::default(),
                id: core::default::Default::default(),
                io_threads: core::default::Default::default(),
                lfu_decay_time: core::default::Default::default(),
                lfu_log_factor: core::default::Default::default(),
                notify_keyspace_events: core::default::Default::default(),
                number_of_databases: core::default::Default::default(),
                persistence: core::default::Default::default(),
                pubsub_client_output_buffer_limit: core::default::Default::default(),
                ssl: core::default::Default::default(),
                timeout: core::default::Default::default(),
                valkey_active_expire_effort: core::default::Default::default(),
                valkey_maxmemory_policy: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseValkeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseValkeyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseValkeyConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `acl_channels_default` after provisioning.\nDetermines default pub/sub channels' ACL for new users if ACL is not supplied. When this option is not defined, all_channels is assumed to keep backward compatibility. This option doesn't affect Valkey configuration acl-pubsub-default."]
    pub fn acl_channels_default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_channels_default", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nA unique identifier for the database cluster."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `frequent_snapshots` after provisioning.\nFrequent RDB snapshots. When enabled, Valkey will create frequent local RDB snapshots. When disabled, Valkey will only take RDB snapshots when a backup is created, based on the backup schedule. This setting is ignored when valkey_persistence is set to off."]
    pub fn frequent_snapshots(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequent_snapshots", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `io_threads` after provisioning.\nThe number of IO threads used by Valkey. Must be between 1 and 32."]
    pub fn io_threads(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.io_threads", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lfu_decay_time` after provisioning.\nThe decay time for Valkey's LFU cache eviction. Must be between 1 and 120."]
    pub fn lfu_decay_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfu_decay_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `lfu_log_factor` after provisioning.\nThe log factor for Valkey's LFU (Least Frequently Used) cache eviction. Must be between 1 and 100."]
    pub fn lfu_log_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lfu_log_factor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notify_keyspace_events` after provisioning.\nSet notify-keyspace-events option. Requires at least K or E and accepts any combination of the following options. Setting the parameter to \"\" disables notifications.\n\nK — Keyspace events\nE — Keyevent events\ng — Generic commands (e.g. DEL, EXPIRE, RENAME, ...)\n$ — String commands\nl — List commands\ns — Set commands\nh — Hash commands\nz — Sorted set commands\nt — Stream commands\nd — Module key type events\nx — Expired events\ne — Evicted events\nm — Key miss events\nn — New key events\nA — Alias for \"g$lshztxed\""]
    pub fn notify_keyspace_events(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notify_keyspace_events", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `number_of_databases` after provisioning.\nThe number of logical databases in the Valkey cluster. Must be between 1 and 128."]
    pub fn number_of_databases(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_databases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `persistence` after provisioning.\nWhen persistence is 'rdb', Valkey does RDB dumps each 10 minutes if any key is changed. Also RDB dumps are done according to backup schedule for backup purposes. When persistence is 'off', no RDB dumps and backups are done, so data can be lost at any moment if service is restarted for any reason, or if service is powered off. Also service can't be forked."]
    pub fn persistence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistence", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_client_output_buffer_limit` after provisioning.\nSet output buffer limit for pub / sub clients in MB. The value is the hard limit, the soft limit is 1/4 of the hard limit. When setting the limit, be mindful of the available memory in the selected service plan."]
    pub fn pubsub_client_output_buffer_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.pubsub_client_output_buffer_limit", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `ssl` after provisioning.\nWhether to enable SSL/TLS for connections to the Valkey cluster."]
    pub fn ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nThe timeout (in seconds) for Valkey client connections."]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valkey_active_expire_effort` after provisioning.\nActive expire effort. Valkey reclaims expired keys both when accessed and in the background. The background process scans for expired keys to free memory. Increasing the active-expire-effort setting (default 1, max 10) uses more CPU to reclaim expired keys faster, reducing memory usage but potentially increasing latency."]
    pub fn valkey_active_expire_effort(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valkey_active_expire_effort", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `valkey_maxmemory_policy` after provisioning.\nA string specifying the desired eviction policy for a Caching or Valkey cluster."]
    pub fn valkey_maxmemory_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valkey_maxmemory_policy", self.extract_ref()))
    }
}
