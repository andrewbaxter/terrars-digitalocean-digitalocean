use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataSizesData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataSizesFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<std::vec::Vec<DataSizesSortEl>>,
    dynamic: DataSizesDynamic,
}

struct DataSizes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSizesData>,
}

#[derive(Clone)]
pub struct DataSizes(std::rc::Rc<DataSizes_>);

impl DataSizes {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataSizesFilterEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sort`.\n"]
    pub fn set_sort(self, v: impl std::convert::Into<terrars::BlockAssignable<DataSizesSortEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sort = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sort = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sizes` after provisioning.\n"]
    pub fn sizes(&self) -> terrars::ListRef<DataSizesSizesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sizes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataSizesSortElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl terrars::Referable for DataSizes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSizes { }

impl terrars::ToListMappable for DataSizes {
    type O = terrars::ListRef<DataSizesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSizes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_sizes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSizes {
    pub tf_id: std::string::String,
}

impl BuildDataSizes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSizes {
        let out = DataSizes(std::rc::Rc::new(DataSizes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSizesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSizesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSizesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSizesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sizes` after provisioning.\n"]
    pub fn sizes(&self) -> terrars::ListRef<DataSizesSizesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sizes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataSizesSortElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSizesSizesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_hourly: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_monthly: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpus: std::option::Option<terrars::PrimField<f64>>,
}

impl DataSizesSizesEl {
    #[doc = "Set the field `available`.\n"]
    pub fn set_available(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.available = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_hourly`.\n"]
    pub fn set_price_hourly(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_hourly = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_monthly`.\n"]
    pub fn set_price_monthly(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.price_monthly = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer`.\n"]
    pub fn set_transfer(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transfer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vcpus`.\n"]
    pub fn set_vcpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vcpus = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSizesSizesEl {
    type O = terrars::BlockAssignable<DataSizesSizesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSizesSizesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSizesSizesElRef {
        DataSizesSizesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSizesSizesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available` after provisioning.\n"]
    pub fn available(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `price_hourly` after provisioning.\n"]
    pub fn price_hourly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_hourly", self.base))
    }

    #[doc = "Get a reference to the value of field `price_monthly` after provisioning.\n"]
    pub fn price_monthly(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_monthly", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer` after provisioning.\n"]
    pub fn transfer(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpus` after provisioning.\n"]
    pub fn vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpus", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSizesFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_by: std::option::Option<terrars::PrimField<std::string::String>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataSizesFilterEl {
    #[doc = "Set the field `all`.\n"]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_by`.\n"]
    pub fn set_match_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSizesFilterEl {
    type O = terrars::BlockAssignable<DataSizesFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSizesFilterEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSizesFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSizesFilterElRef {
        DataSizesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSizesFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_by` after provisioning.\n"]
    pub fn match_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_by", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSizesSortEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
}

impl DataSizesSortEl {
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSizesSortEl {
    type O = terrars::BlockAssignable<DataSizesSortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSizesSortEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSizesSortElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSizesSortElRef {
        DataSizesSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSizesSortElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataSizesDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataSizesFilterEl>>,
    sort: std::option::Option<terrars::DynamicBlock<DataSizesSortEl>>,
}
