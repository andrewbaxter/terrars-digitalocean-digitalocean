use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataTagsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataTagsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataTagsSortEl>>,
    dynamic: DataTagsDynamic,
}

struct DataTags_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataTagsData>,
}

#[derive(Clone)]
pub struct DataTags(Rc<DataTags_>);

impl DataTags {
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

    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataTagsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `sort`.\n"]
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataTagsSortEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataTagsTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataTagsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Datasource for DataTags {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataTags {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataTags {
    type O = ListRef<DataTagsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataTags_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_tags".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTags {
    pub tf_id: String,
}

impl BuildDataTags {
    pub fn build(self, stack: &mut Stack) -> DataTags {
        let out = DataTags(Rc::new(DataTags_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataTagsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTagsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTagsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTagsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> ListRef<DataTagsTagsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataTagsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTagsTagsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    databases_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    droplets_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    images_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_resource_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_snapshots_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes_count: Option<PrimField<f64>>,
}

impl DataTagsTagsEl {
    #[doc= "Set the field `databases_count`.\n"]
    pub fn set_databases_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.databases_count = Some(v.into());
        self
    }

    #[doc= "Set the field `droplets_count`.\n"]
    pub fn set_droplets_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.droplets_count = Some(v.into());
        self
    }

    #[doc= "Set the field `images_count`.\n"]
    pub fn set_images_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.images_count = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `total_resource_count`.\n"]
    pub fn set_total_resource_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_resource_count = Some(v.into());
        self
    }

    #[doc= "Set the field `volume_snapshots_count`.\n"]
    pub fn set_volume_snapshots_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volume_snapshots_count = Some(v.into());
        self
    }

    #[doc= "Set the field `volumes_count`.\n"]
    pub fn set_volumes_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.volumes_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataTagsTagsEl {
    type O = BlockAssignable<DataTagsTagsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTagsTagsEl {}

impl BuildDataTagsTagsEl {
    pub fn build(self) -> DataTagsTagsEl {
        DataTagsTagsEl {
            databases_count: core::default::Default::default(),
            droplets_count: core::default::Default::default(),
            images_count: core::default::Default::default(),
            name: core::default::Default::default(),
            total_resource_count: core::default::Default::default(),
            volume_snapshots_count: core::default::Default::default(),
            volumes_count: core::default::Default::default(),
        }
    }
}

pub struct DataTagsTagsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTagsTagsElRef {
    fn new(shared: StackShared, base: String) -> DataTagsTagsElRef {
        DataTagsTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTagsTagsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `databases_count` after provisioning.\n"]
    pub fn databases_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.databases_count", self.base))
    }

    #[doc= "Get a reference to the value of field `droplets_count` after provisioning.\n"]
    pub fn droplets_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.droplets_count", self.base))
    }

    #[doc= "Get a reference to the value of field `images_count` after provisioning.\n"]
    pub fn images_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.images_count", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `total_resource_count` after provisioning.\n"]
    pub fn total_resource_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_resource_count", self.base))
    }

    #[doc= "Get a reference to the value of field `volume_snapshots_count` after provisioning.\n"]
    pub fn volume_snapshots_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volume_snapshots_count", self.base))
    }

    #[doc= "Get a reference to the value of field `volumes_count` after provisioning.\n"]
    pub fn volumes_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.volumes_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTagsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataTagsFilterEl {
    #[doc= "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.all = Some(v.into());
        self
    }

    #[doc= "Set the field `match_by`.\n"]
    pub fn set_match_by(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.match_by = Some(v.into());
        self
    }
}

impl ToListMappable for DataTagsFilterEl {
    type O = BlockAssignable<DataTagsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTagsFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataTagsFilterEl {
    pub fn build(self) -> DataTagsFilterEl {
        DataTagsFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataTagsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTagsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataTagsFilterElRef {
        DataTagsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTagsFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `match_by` after provisioning.\n"]
    pub fn match_by(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.match_by", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTagsSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataTagsSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataTagsSortEl {
    type O = BlockAssignable<DataTagsSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTagsSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataTagsSortEl {
    pub fn build(self) -> DataTagsSortEl {
        DataTagsSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataTagsSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataTagsSortElRef {
    fn new(shared: StackShared, base: String) -> DataTagsSortElRef {
        DataTagsSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTagsSortElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataTagsDynamic {
    filter: Option<DynamicBlock<DataTagsFilterEl>>,
    sort: Option<DynamicBlock<DataTagsSortEl>>,
}
