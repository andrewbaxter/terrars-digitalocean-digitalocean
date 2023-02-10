use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDatabaseReplicaData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
}

struct DataDatabaseReplica_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDatabaseReplicaData>,
}

#[derive(Clone)]
pub struct DataDatabaseReplica(Rc<DataDatabaseReplica_>);

impl DataDatabaseReplica {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

impl Datasource for DataDatabaseReplica {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataDatabaseReplica {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataDatabaseReplica {
    type O = ListRef<DataDatabaseReplicaRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataDatabaseReplica_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_database_replica".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDatabaseReplica {
    pub tf_id: String,
    #[doc= ""]
    pub cluster_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataDatabaseReplica {
    pub fn build(self, stack: &mut Stack) -> DataDatabaseReplica {
        let out = DataDatabaseReplica(Rc::new(DataDatabaseReplica_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDatabaseReplicaData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDatabaseReplicaRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDatabaseReplicaRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDatabaseReplicaRef {
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
