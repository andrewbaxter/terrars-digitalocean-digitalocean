use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataTagData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataTag_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTagData>,
}

#[derive(Clone)]
pub struct DataTag(Rc<DataTag_>);

impl DataTag {
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

    #[doc= "Get a reference to the value of field `databases_count` after provisioning.\n"]
    pub fn databases_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.databases_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplets_count` after provisioning.\n"]
    pub fn droplets_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplets_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `images_count` after provisioning.\n"]
    pub fn images_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.images_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the tag"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_resource_count` after provisioning.\n"]
    pub fn total_resource_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_resource_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_snapshots_count` after provisioning.\n"]
    pub fn volume_snapshots_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_snapshots_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volumes_count` after provisioning.\n"]
    pub fn volumes_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_count", self.extract_ref()))
    }
}

impl Datasource for DataTag {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataTag {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataTag {
    type O = ListRef<DataTagRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataTag_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_tag".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTag {
    pub tf_id: String,
    #[doc= "name of the tag"]
    pub name: PrimField<String>,
}

impl BuildDataTag {
    pub fn build(self, stack: &mut Stack) -> DataTag {
        let out = DataTag(Rc::new(DataTag_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTagData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTagRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTagRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTagRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `databases_count` after provisioning.\n"]
    pub fn databases_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.databases_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplets_count` after provisioning.\n"]
    pub fn droplets_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplets_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `images_count` after provisioning.\n"]
    pub fn images_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.images_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the tag"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_resource_count` after provisioning.\n"]
    pub fn total_resource_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_resource_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volume_snapshots_count` after provisioning.\n"]
    pub fn volume_snapshots_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_snapshots_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `volumes_count` after provisioning.\n"]
    pub fn volumes_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_count", self.extract_ref()))
    }
}
