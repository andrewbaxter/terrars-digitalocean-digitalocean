use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DatabaseReplicaData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    cluster_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_network_uuid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
}

struct DatabaseReplica_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatabaseReplicaData>,
}

#[derive(Clone)]
pub struct DatabaseReplica(Rc<DatabaseReplica_>);

impl DatabaseReplica {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `private_network_uuid`.\n"]
    pub fn set_private_network_uuid(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().private_network_uuid = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().size = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_host` after provisioning.\n"]
    pub fn private_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_network_uuid` after provisioning.\n"]
    pub fn private_network_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_network_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_uri` after provisioning.\n"]
    pub fn private_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

impl Referable for DatabaseReplica {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DatabaseReplica { }

impl ToListMappable for DatabaseReplica {
    type O = ListRef<DatabaseReplicaRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DatabaseReplica_ {
    fn extract_resource_type(&self) -> String {
        "digitalocean_database_replica".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatabaseReplica {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDatabaseReplica {
    pub fn build(self, stack: &mut Stack) -> DatabaseReplica {
        let out = DatabaseReplica(Rc::new(DatabaseReplica_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatabaseReplicaData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: self.name,
                private_network_uuid: core::default::Default::default(),
                region: core::default::Default::default(),
                size: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatabaseReplicaRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatabaseReplicaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatabaseReplicaRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_host` after provisioning.\n"]
    pub fn private_host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_network_uuid` after provisioning.\n"]
    pub fn private_network_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_network_uuid", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private_uri` after provisioning.\n"]
    pub fn private_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.private_uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}
