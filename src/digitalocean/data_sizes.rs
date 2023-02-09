use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataSizesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataSizesFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataSizesSortEl>>,
    dynamic: DataSizesDynamic,
}

struct DataSizes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSizesData>,
}

#[derive(Clone)]
pub struct DataSizes(Rc<DataSizes_>);

impl DataSizes {
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataSizesFilterEl>>) -> Self {
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
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataSizesSortEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `sizes` after provisioning.\n"]
    pub fn sizes(&self) -> ListRef<DataSizesSizesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sizes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataSizesSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Datasource for DataSizes {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataSizes {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataSizes {
    type O = ListRef<DataSizesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSizes_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_sizes".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSizes {
    pub tf_id: String,
}

impl BuildDataSizes {
    pub fn build(self, stack: &mut Stack) -> DataSizes {
        let out = DataSizes(Rc::new(DataSizes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSizesData {
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

pub struct DataSizesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSizesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSizesRef {
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

    #[doc= "Get a reference to the value of field `sizes` after provisioning.\n"]
    pub fn sizes(&self) -> ListRef<DataSizesSizesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sizes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataSizesSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSizesSizesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    available: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_hourly: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_monthly: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcpus: Option<PrimField<f64>>,
}

impl DataSizesSizesEl {
    #[doc= "Set the field `available`.\n"]
    pub fn set_available(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.available = Some(v.into());
        self
    }

    #[doc= "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk = Some(v.into());
        self
    }

    #[doc= "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory = Some(v.into());
        self
    }

    #[doc= "Set the field `price_hourly`.\n"]
    pub fn set_price_hourly(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.price_hourly = Some(v.into());
        self
    }

    #[doc= "Set the field `price_monthly`.\n"]
    pub fn set_price_monthly(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.price_monthly = Some(v.into());
        self
    }

    #[doc= "Set the field `regions`.\n"]
    pub fn set_regions(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }

    #[doc= "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.slug = Some(v.into());
        self
    }

    #[doc= "Set the field `transfer`.\n"]
    pub fn set_transfer(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.transfer = Some(v.into());
        self
    }

    #[doc= "Set the field `vcpus`.\n"]
    pub fn set_vcpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.vcpus = Some(v.into());
        self
    }
}

impl ToListMappable for DataSizesSizesEl {
    type O = BlockAssignable<DataSizesSizesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSizesSizesEl {}

impl BuildDataSizesSizesEl {
    pub fn build(self) -> DataSizesSizesEl {
        DataSizesSizesEl {
            available: core::default::Default::default(),
            disk: core::default::Default::default(),
            memory: core::default::Default::default(),
            price_hourly: core::default::Default::default(),
            price_monthly: core::default::Default::default(),
            regions: core::default::Default::default(),
            slug: core::default::Default::default(),
            transfer: core::default::Default::default(),
            vcpus: core::default::Default::default(),
        }
    }
}

pub struct DataSizesSizesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSizesSizesElRef {
    fn new(shared: StackShared, base: String) -> DataSizesSizesElRef {
        DataSizesSizesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSizesSizesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `available` after provisioning.\n"]
    pub fn available(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.available", self.base))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc= "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.base))
    }

    #[doc= "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.base))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }

    #[doc= "Get a reference to the value of field `transfer` after provisioning.\n"]
    pub fn transfer(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.transfer", self.base))
    }

    #[doc= "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSizesFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataSizesFilterEl {
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

impl ToListMappable for DataSizesFilterEl {
    type O = BlockAssignable<DataSizesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSizesFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataSizesFilterEl {
    pub fn build(self) -> DataSizesFilterEl {
        DataSizesFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataSizesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSizesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataSizesFilterElRef {
        DataSizesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSizesFilterElRef {
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
pub struct DataSizesSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataSizesSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataSizesSortEl {
    type O = BlockAssignable<DataSizesSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSizesSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataSizesSortEl {
    pub fn build(self) -> DataSizesSortEl {
        DataSizesSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataSizesSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSizesSortElRef {
    fn new(shared: StackShared, base: String) -> DataSizesSortElRef {
        DataSizesSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSizesSortElRef {
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
struct DataSizesDynamic {
    filter: Option<DynamicBlock<DataSizesFilterEl>>,
    sort: Option<DynamicBlock<DataSizesSortEl>>,
}
