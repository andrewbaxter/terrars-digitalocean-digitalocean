use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataRegionsData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataRegionsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataRegionsSortEl>>,
    dynamic: DataRegionsDynamic,
}

struct DataRegions_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataRegionsData>,
}

#[derive(Clone)]
pub struct DataRegions(Rc<DataRegions_>);

impl DataRegions {
    fn shared(&self) -> &StackShared {
        &self.0.shared
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataRegionsFilterEl>>) -> Self {
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
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataRegionsSortEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<DataRegionsRegionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataRegionsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Datasource for DataRegions {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataRegions {
    type O = ListRef<DataRegionsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataRegions_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_regions".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRegions {
    pub tf_id: String,
}

impl BuildDataRegions {
    pub fn build(self, stack: &mut Stack) -> DataRegions {
        let out = DataRegions(Rc::new(DataRegions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataRegionsData {
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

pub struct DataRegionsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRegionsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRegionsRef {
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

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> ListRef<DataRegionsRegionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataRegionsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataRegionsRegionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    available: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<PrimField<String>>,
}

impl DataRegionsRegionsEl {
    #[doc= "Set the field `available`.\n"]
    pub fn set_available(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.available = Some(v.into());
        self
    }

    #[doc= "Set the field `features`.\n"]
    pub fn set_features(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.features = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `sizes`.\n"]
    pub fn set_sizes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.sizes = Some(v.into());
        self
    }

    #[doc= "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slug = Some(v.into());
        self
    }
}

impl ToListMappable for DataRegionsRegionsEl {
    type O = BlockAssignable<DataRegionsRegionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegionsRegionsEl {}

impl BuildDataRegionsRegionsEl {
    pub fn build(self) -> DataRegionsRegionsEl {
        DataRegionsRegionsEl {
            available: core::default::Default::default(),
            features: core::default::Default::default(),
            name: core::default::Default::default(),
            sizes: core::default::Default::default(),
            slug: core::default::Default::default(),
        }
    }
}

pub struct DataRegionsRegionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRegionsRegionsElRef {
    fn new(shared: StackShared, base: String) -> DataRegionsRegionsElRef {
        DataRegionsRegionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegionsRegionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `available` after provisioning.\n"]
    pub fn available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.available", self.base))
    }

    #[doc= "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.features", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `sizes` after provisioning.\n"]
    pub fn sizes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.sizes", self.base))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }
}

#[derive(Serialize)]
pub struct DataRegionsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataRegionsFilterEl {
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

impl ToListMappable for DataRegionsFilterEl {
    type O = BlockAssignable<DataRegionsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegionsFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataRegionsFilterEl {
    pub fn build(self) -> DataRegionsFilterEl {
        DataRegionsFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataRegionsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRegionsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataRegionsFilterElRef {
        DataRegionsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegionsFilterElRef {
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
pub struct DataRegionsSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataRegionsSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataRegionsSortEl {
    type O = BlockAssignable<DataRegionsSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataRegionsSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataRegionsSortEl {
    pub fn build(self) -> DataRegionsSortEl {
        DataRegionsSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataRegionsSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataRegionsSortElRef {
    fn new(shared: StackShared, base: String) -> DataRegionsSortElRef {
        DataRegionsSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataRegionsSortElRef {
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
struct DataRegionsDynamic {
    filter: Option<DynamicBlock<DataRegionsFilterEl>>,
    sort: Option<DynamicBlock<DataRegionsSortEl>>,
}
