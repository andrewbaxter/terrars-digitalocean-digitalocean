use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDatabaseUserData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

struct DataDatabaseUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDatabaseUserData>,
}

#[derive(Clone)]
pub struct DataDatabaseUser(std::rc::Rc<DataDatabaseUser_>);

impl DataDatabaseUser {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DataDatabaseUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl terrars::Referable for DataDatabaseUser {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDatabaseUser { }

impl terrars::ToListMappable for DataDatabaseUser {
    type O = terrars::ListRef<DataDatabaseUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDatabaseUser_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_database_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDatabaseUser {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataDatabaseUser {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDatabaseUser {
        let out = DataDatabaseUser(std::rc::Rc::new(DataDatabaseUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDatabaseUserData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDatabaseUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatabaseUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDatabaseUserRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> terrars::ListRef<DataDatabaseUserSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDatabaseUserSettingsElAclEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDatabaseUserSettingsElAclEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.permission = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDatabaseUserSettingsElAclEl {
    type O = terrars::BlockAssignable<DataDatabaseUserSettingsElAclEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatabaseUserSettingsElAclEl {}

impl BuildDataDatabaseUserSettingsElAclEl {
    pub fn build(self) -> DataDatabaseUserSettingsElAclEl {
        DataDatabaseUserSettingsElAclEl {
            id: core::default::Default::default(),
            permission: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

pub struct DataDatabaseUserSettingsElAclElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatabaseUserSettingsElAclElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDatabaseUserSettingsElAclElRef {
        DataDatabaseUserSettingsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatabaseUserSettingsElAclElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDatabaseUserSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<terrars::ListField<DataDatabaseUserSettingsElAclEl>>,
}

impl DataDatabaseUserSettingsEl {
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(mut self, v: impl std::convert::Into<terrars::ListField<DataDatabaseUserSettingsElAclEl>>) -> Self {
        self.acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDatabaseUserSettingsEl {
    type O = terrars::BlockAssignable<DataDatabaseUserSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatabaseUserSettingsEl {}

impl BuildDataDatabaseUserSettingsEl {
    pub fn build(self) -> DataDatabaseUserSettingsEl {
        DataDatabaseUserSettingsEl { acl: core::default::Default::default() }
    }
}

pub struct DataDatabaseUserSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatabaseUserSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDatabaseUserSettingsElRef {
        DataDatabaseUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatabaseUserSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::ListRef<DataDatabaseUserSettingsElAclElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }
}
