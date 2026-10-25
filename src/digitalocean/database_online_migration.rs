#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseOnlineMigrationData_ {
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
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_dbs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<DatabaseOnlineMigration_Source>>,
    dynamic: DatabaseOnlineMigrationDynamic,
}

struct DatabaseOnlineMigration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatabaseOnlineMigrationData_>,
}

#[derive(Clone)]
pub struct DatabaseOnlineMigration(std::rc::Rc<DatabaseOnlineMigration_>);

impl DatabaseOnlineMigration {
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

    #[doc = "Set the field `disable_ssl`.\nDisables SSL encryption when connecting to the source database"]
    pub fn set_disable_ssl(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_dbs`.\nThe list of databases to be ignored during the migration"]
    pub fn set_ignore_dbs(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ignore_dbs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatabaseOnlineMigration_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the online migration was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_ssl` after provisioning.\nDisables SSL encryption when connecting to the source database"]
    pub fn disable_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_ssl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the migration"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_dbs` after provisioning.\nThe list of databases to be ignored during the migration"]
    pub fn ignore_dbs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ignore_dbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the online migration"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DatabaseOnlineMigration_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for DatabaseOnlineMigration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatabaseOnlineMigration { }

impl terrars::ToListMappable for DatabaseOnlineMigration {
    type O = terrars::ListRef<DatabaseOnlineMigrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatabaseOnlineMigration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_database_online_migration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseOnlineMigration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseOnlineMigration {
    pub fn build(self, stack: &mut terrars::Stack) -> DatabaseOnlineMigration {
        let out = DatabaseOnlineMigration(std::rc::Rc::new(DatabaseOnlineMigration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatabaseOnlineMigrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                disable_ssl: core::default::Default::default(),
                ignore_dbs: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseOnlineMigrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseOnlineMigrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseOnlineMigrationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_at` after provisioning.\nThe date and time when the online migration was created"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_ssl` after provisioning.\nDisables SSL encryption when connecting to the source database"]
    pub fn disable_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_ssl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe ID of the migration"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ignore_dbs` after provisioning.\nThe list of databases to be ignored during the migration"]
    pub fn ignore_dbs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ignore_dbs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe status of the online migration"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<DatabaseOnlineMigration_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatabaseOnlineMigrationDynamic {
    source: std::option::Option<terrars::DynamicBlock<DatabaseOnlineMigration_Source>>,
}

#[derive(Serialize)]
pub struct DatabaseOnlineMigration_Source {
    db_name: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
    username: terrars::PrimField<std::string::String>,
}

impl DatabaseOnlineMigration_Source { }

impl terrars::ToListMappable for DatabaseOnlineMigration_Source {
    type O = terrars::BlockAssignable<DatabaseOnlineMigration_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatabaseOnlineMigration_Source {
    #[doc = "The name of the default database"]
    pub db_name: terrars::PrimField<std::string::String>,
    #[doc = "The FQDN pointing to the database cluster's current primary node"]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = "The password of the database."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = "The port on which the database cluster is listening"]
    pub port: terrars::PrimField<f64>,
    #[doc = "The default user of the database"]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildDatabaseOnlineMigration_Source {
    pub fn build(self) -> DatabaseOnlineMigration_Source {
        DatabaseOnlineMigration_Source {
            db_name: self.db_name,
            host: self.host,
            password: self.password,
            port: self.port,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct DatabaseOnlineMigration_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatabaseOnlineMigration_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatabaseOnlineMigration_SourceRef {
        DatabaseOnlineMigration_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatabaseOnlineMigration_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\nThe name of the default database"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe FQDN pointing to the database cluster's current primary node"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nThe password of the database."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port on which the database cluster is listening"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nThe default user of the database"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
