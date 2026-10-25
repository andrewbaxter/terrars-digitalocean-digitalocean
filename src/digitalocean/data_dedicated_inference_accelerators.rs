#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDedicatedInferenceAcceleratorsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    dedicated_inference_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataDedicatedInferenceAccelerators_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<std::vec::Vec<DataDedicatedInferenceAccelerators_Sort>>,
    dynamic: DataDedicatedInferenceAcceleratorsDynamic,
}

struct DataDedicatedInferenceAccelerators_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDedicatedInferenceAcceleratorsData_>,
}

#[derive(Clone)]
pub struct DataDedicatedInferenceAccelerators(std::rc::Rc<DataDedicatedInferenceAccelerators_>);

impl DataDedicatedInferenceAccelerators {
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
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataDedicatedInferenceAccelerators_Filter>>,
    ) -> Self {
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
    pub fn set_sort(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataDedicatedInferenceAccelerators_Sort>>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DataDedicatedInferenceAccelerators_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dedicated_inference_id` after provisioning.\nThe ID of the dedicated inference endpoint to list accelerators for."]
    pub fn dedicated_inference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_inference_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataDedicatedInferenceAccelerators_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl terrars::Referable for DataDedicatedInferenceAccelerators {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDedicatedInferenceAccelerators { }

impl terrars::ToListMappable for DataDedicatedInferenceAccelerators {
    type O = terrars::ListRef<DataDedicatedInferenceAcceleratorsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDedicatedInferenceAccelerators_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_dedicated_inference_accelerators".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDedicatedInferenceAccelerators {
    pub tf_id: std::string::String,
    #[doc = "The ID of the dedicated inference endpoint to list accelerators for."]
    pub dedicated_inference_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDedicatedInferenceAccelerators {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDedicatedInferenceAccelerators {
        let out = DataDedicatedInferenceAccelerators(std::rc::Rc::new(DataDedicatedInferenceAccelerators_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDedicatedInferenceAcceleratorsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                dedicated_inference_id: self.dedicated_inference_id,
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

pub struct DataDedicatedInferenceAcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceAcceleratorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDedicatedInferenceAcceleratorsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DataDedicatedInferenceAccelerators_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dedicated_inference_id` after provisioning.\nThe ID of the dedicated inference endpoint to list accelerators for."]
    pub fn dedicated_inference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_inference_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataDedicatedInferenceAccelerators_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataDedicatedInferenceAcceleratorsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataDedicatedInferenceAccelerators_Filter>>,
    sort: std::option::Option<terrars::DynamicBlock<DataDedicatedInferenceAccelerators_Sort>>,
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceAccelerators_Accelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInferenceAccelerators_Accelerators {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceAccelerators_Accelerators {
    type O = terrars::BlockAssignable<DataDedicatedInferenceAccelerators_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceAccelerators_Accelerators {}

impl BuildDataDedicatedInferenceAccelerators_Accelerators {
    pub fn build(self) -> DataDedicatedInferenceAccelerators_Accelerators {
        DataDedicatedInferenceAccelerators_Accelerators {
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            slug: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceAccelerators_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceAccelerators_AcceleratorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDedicatedInferenceAccelerators_AcceleratorsRef {
        DataDedicatedInferenceAccelerators_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceAccelerators_AcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceAccelerators_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_by: std::option::Option<terrars::PrimField<std::string::String>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInferenceAccelerators_Filter {
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

impl terrars::ToListMappable for DataDedicatedInferenceAccelerators_Filter {
    type O = terrars::BlockAssignable<DataDedicatedInferenceAccelerators_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceAccelerators_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataDedicatedInferenceAccelerators_Filter {
    pub fn build(self) -> DataDedicatedInferenceAccelerators_Filter {
        DataDedicatedInferenceAccelerators_Filter {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceAccelerators_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceAccelerators_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDedicatedInferenceAccelerators_FilterRef {
        DataDedicatedInferenceAccelerators_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceAccelerators_FilterRef {
    #[allow(dead_code)]
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
pub struct DataDedicatedInferenceAccelerators_Sort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
}

impl DataDedicatedInferenceAccelerators_Sort {
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceAccelerators_Sort {
    type O = terrars::BlockAssignable<DataDedicatedInferenceAccelerators_Sort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceAccelerators_Sort {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataDedicatedInferenceAccelerators_Sort {
    pub fn build(self) -> DataDedicatedInferenceAccelerators_Sort {
        DataDedicatedInferenceAccelerators_Sort {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceAccelerators_SortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceAccelerators_SortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDedicatedInferenceAccelerators_SortRef {
        DataDedicatedInferenceAccelerators_SortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceAccelerators_SortRef {
    #[allow(dead_code)]
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
