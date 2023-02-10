use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDropletSnapshotData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    most_recent: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_regex: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataDropletSnapshot_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataDropletSnapshotData>,
}

#[derive(Clone)]
pub struct DataDropletSnapshot(Rc<DataDropletSnapshot_>);

impl DataDropletSnapshot {
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

    #[doc= "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_regex`.\n"]
    pub fn set_name_regex(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_regex = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_id` after provisioning.\n"]
    pub fn droplet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_disk_size` after provisioning.\n"]
    pub fn min_disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_disk_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }
}

impl Datasource for DataDropletSnapshot {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataDropletSnapshot {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataDropletSnapshot {
    type O = ListRef<DataDropletSnapshotRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataDropletSnapshot_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_droplet_snapshot".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDropletSnapshot {
    pub tf_id: String,
}

impl BuildDataDropletSnapshot {
    pub fn build(self, stack: &mut Stack) -> DataDropletSnapshot {
        let out = DataDropletSnapshot(Rc::new(DataDropletSnapshot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataDropletSnapshotData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                name: core::default::Default::default(),
                name_regex: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDropletSnapshotRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataDropletSnapshotRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDropletSnapshotRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `droplet_id` after provisioning.\n"]
    pub fn droplet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplet_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_disk_size` after provisioning.\n"]
    pub fn min_disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_disk_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_regex` after provisioning.\n"]
    pub fn name_regex(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_regex", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }
}
