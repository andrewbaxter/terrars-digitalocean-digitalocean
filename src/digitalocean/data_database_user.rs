use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDatabaseUserData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataDatabaseUser_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDatabaseUserData>,
}

#[derive(Clone)]
pub struct DataDatabaseUser(Rc<DataDatabaseUser_>);

impl DataDatabaseUser {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderDigitalocean) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<DataDatabaseUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

impl Referable for DataDatabaseUser {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataDatabaseUser { }

impl ToListMappable for DataDatabaseUser {
    type O = ListRef<DataDatabaseUserRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataDatabaseUser_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_database_user".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDatabaseUser {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataDatabaseUser {
    pub fn build(self, stack: &mut Stack) -> DataDatabaseUser {
        let out = DataDatabaseUser(Rc::new(DataDatabaseUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDatabaseUserData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DataDatabaseUserRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDatabaseUserRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mysql_auth_plugin` after provisioning.\n"]
    pub fn mysql_auth_plugin(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mysql_auth_plugin", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `settings` after provisioning.\n"]
    pub fn settings(&self) -> ListRef<DataDatabaseUserSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDatabaseUserSettingsElAclEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<PrimField<String>>,
}

impl DataDatabaseUserSettingsElAclEl {
    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `permission`.\n"]
    pub fn set_permission(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.permission = Some(v.into());
        self
    }

    #[doc= "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.topic = Some(v.into());
        self
    }
}

impl ToListMappable for DataDatabaseUserSettingsElAclEl {
    type O = BlockAssignable<DataDatabaseUserSettingsElAclEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataDatabaseUserSettingsElAclElRef {
    fn new(shared: StackShared, base: String) -> DataDatabaseUserSettingsElAclElRef {
        DataDatabaseUserSettingsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatabaseUserSettingsElAclElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.permission", self.base))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDatabaseUserSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    acl: Option<ListField<DataDatabaseUserSettingsElAclEl>>,
}

impl DataDatabaseUserSettingsEl {
    #[doc= "Set the field `acl`.\n"]
    pub fn set_acl(mut self, v: impl Into<ListField<DataDatabaseUserSettingsElAclEl>>) -> Self {
        self.acl = Some(v.into());
        self
    }
}

impl ToListMappable for DataDatabaseUserSettingsEl {
    type O = BlockAssignable<DataDatabaseUserSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataDatabaseUserSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataDatabaseUserSettingsElRef {
        DataDatabaseUserSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatabaseUserSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> ListRef<DataDatabaseUserSettingsElAclElRef> {
        ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }
}
