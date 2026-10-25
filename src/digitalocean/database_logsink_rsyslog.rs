#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseLogsinkRsyslogData_ {
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
    ca_cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logline: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
    server: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    structured_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls: std::option::Option<terrars::PrimField<bool>>,
}

struct DatabaseLogsinkRsyslog_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseLogsinkRsyslogData_>,
}

#[derive(Clone)]
pub struct DatabaseLogsinkRsyslog(std::rc::Rc<DatabaseLogsinkRsyslog_>);

impl DatabaseLogsinkRsyslog {
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

    #[doc = "Set the field `ca_cert`.\nCA certificate for TLS verification (PEM format)"]
    pub fn set_ca_cert(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ca_cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_cert`.\nClient certificate for mTLS (PEM format)"]
    pub fn set_client_cert(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_key`.\nClient private key for mTLS (PEM format)"]
    pub fn set_client_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `format`.\nLog format: rfc5424, rfc3164, or custom"]
    pub fn set_format(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logline`.\nCustom logline template (required when format is 'custom')"]
    pub fn set_logline(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logline = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `structured_data`.\nStructured data for rsyslog"]
    pub fn set_structured_data(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().structured_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls`.\nEnable TLS encryption for rsyslog connection"]
    pub fn set_tls(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().tls = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ca_cert` after provisioning.\nCA certificate for TLS verification (PEM format)"]
    pub fn ca_cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_cert` after provisioning.\nClient certificate for mTLS (PEM format)"]
    pub fn client_cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nClient private key for mTLS (PEM format)"]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nUUID of the source database cluster that will forward logs"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nLog format: rfc5424, rfc3164, or custom"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nComposite ID of the logsink resource"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logline` after provisioning.\nCustom logline template (required when format is 'custom')"]
    pub fn logline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logsink_id` after provisioning.\nThe API sink_id returned by DigitalOcean"]
    pub fn logsink_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logsink_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the logsink"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number for the rsyslog server (1-65535)"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server` after provisioning.\nHostname or IP address of the rsyslog server"]
    pub fn server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `structured_data` after provisioning.\nStructured data for rsyslog"]
    pub fn structured_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.structured_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nEnable TLS encryption for rsyslog connection"]
    pub fn tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseLogsinkRsyslog {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseLogsinkRsyslog { }

impl terrars::ToListMappable for DatabaseLogsinkRsyslog {
    type O = terrars::ListRef<DatabaseLogsinkRsyslogRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseLogsinkRsyslog_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_logsink_rsyslog".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseLogsinkRsyslog {
    pub tf_id: std::string::String,
    #[doc = "UUID of the source database cluster that will forward logs"]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc = "Display name for the logsink"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Port number for the rsyslog server (1-65535)"]
    pub port: terrars::PrimField<f64>,
    #[doc = "Hostname or IP address of the rsyslog server"]
    pub server: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseLogsinkRsyslog {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseLogsinkRsyslog {
        let out = DatabaseLogsinkRsyslog(std::rc::Rc::new(DatabaseLogsinkRsyslog_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseLogsinkRsyslogData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                ca_cert: core::default::Default::default(),
                client_cert: core::default::Default::default(),
                client_key: core::default::Default::default(),
                cluster_id: self.cluster_id,
                format: core::default::Default::default(),
                logline: core::default::Default::default(),
                name: self.name,
                port: self.port,
                server: self.server,
                structured_data: core::default::Default::default(),
                tls: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseLogsinkRsyslogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseLogsinkRsyslogRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseLogsinkRsyslogRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_cert` after provisioning.\nCA certificate for TLS verification (PEM format)"]
    pub fn ca_cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_cert` after provisioning.\nClient certificate for mTLS (PEM format)"]
    pub fn client_cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cert", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `client_key` after provisioning.\nClient private key for mTLS (PEM format)"]
    pub fn client_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cluster_id` after provisioning.\nUUID of the source database cluster that will forward logs"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `format` after provisioning.\nLog format: rfc5424, rfc3164, or custom"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nComposite ID of the logsink resource"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logline` after provisioning.\nCustom logline template (required when format is 'custom')"]
    pub fn logline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logline", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logsink_id` after provisioning.\nThe API sink_id returned by DigitalOcean"]
    pub fn logsink_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logsink_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nDisplay name for the logsink"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number for the rsyslog server (1-65535)"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `server` after provisioning.\nHostname or IP address of the rsyslog server"]
    pub fn server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `structured_data` after provisioning.\nStructured data for rsyslog"]
    pub fn structured_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.structured_data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tls` after provisioning.\nEnable TLS encryption for rsyslog connection"]
    pub fn tls(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls", self.extract_ref()))
    }
}
