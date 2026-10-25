#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataDedicatedInferenceSizesData_ {
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
}

struct DataDedicatedInferenceSizes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDedicatedInferenceSizesData_>,
}

#[derive(Clone)]
pub struct DataDedicatedInferenceSizes(std::rc::Rc<DataDedicatedInferenceSizes_>);

impl DataDedicatedInferenceSizes {
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

    #[doc =
        "Get a reference to the value of field `enabled_regions` after provisioning.\nThe list of regions where dedicated inference endpoints can be deployed."]
    pub fn enabled_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sizes` after provisioning.\nThe list of available GPU sizes with their configurations and pricing."]
    pub fn sizes(&self) -> terrars::ListRef<DataDedicatedInferenceSizes_SizesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sizes", self.extract_ref()))
    }
}

impl terrars::Referable for DataDedicatedInferenceSizes {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDedicatedInferenceSizes { }

impl terrars::ToListMappable for DataDedicatedInferenceSizes {
    type O = terrars::ListRef<DataDedicatedInferenceSizesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDedicatedInferenceSizes_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_dedicated_inference_sizes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDedicatedInferenceSizes {
    pub tf_id: std::string::String,
}

impl BuildDataDedicatedInferenceSizes {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDedicatedInferenceSizes {
        let out = DataDedicatedInferenceSizes(std::rc::Rc::new(DataDedicatedInferenceSizes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDedicatedInferenceSizesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDedicatedInferenceSizesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceSizesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDedicatedInferenceSizesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `enabled_regions` after provisioning.\nThe list of regions where dedicated inference endpoints can be deployed."]
    pub fn enabled_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_regions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sizes` after provisioning.\nThe list of available GPU sizes with their configurations and pricing."]
    pub fn sizes(&self) -> terrars::ListRef<DataDedicatedInferenceSizes_SizesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sizes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceSizes_Sizes_Disks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInferenceSizes_Sizes_Disks {
    #[doc = "Set the field `size_gb`.\n"]
    pub fn set_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceSizes_Sizes_Disks {
    type O = terrars::BlockAssignable<DataDedicatedInferenceSizes_Sizes_Disks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceSizes_Sizes_Disks {}

impl BuildDataDedicatedInferenceSizes_Sizes_Disks {
    pub fn build(self) -> DataDedicatedInferenceSizes_Sizes_Disks {
        DataDedicatedInferenceSizes_Sizes_Disks {
            size_gb: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceSizes_Sizes_DisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceSizes_Sizes_DisksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDedicatedInferenceSizes_Sizes_DisksRef {
        DataDedicatedInferenceSizes_Sizes_DisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceSizes_Sizes_DisksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\n"]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceSizes_Sizes_Gpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vram_gb: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDedicatedInferenceSizes_Sizes_Gpu {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `slug`.\n"]
    pub fn set_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vram_gb`.\n"]
    pub fn set_vram_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vram_gb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceSizes_Sizes_Gpu {
    type O = terrars::BlockAssignable<DataDedicatedInferenceSizes_Sizes_Gpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceSizes_Sizes_Gpu {}

impl BuildDataDedicatedInferenceSizes_Sizes_Gpu {
    pub fn build(self) -> DataDedicatedInferenceSizes_Sizes_Gpu {
        DataDedicatedInferenceSizes_Sizes_Gpu {
            count: core::default::Default::default(),
            slug: core::default::Default::default(),
            vram_gb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceSizes_Sizes_GpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceSizes_Sizes_GpuRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDedicatedInferenceSizes_Sizes_GpuRef {
        DataDedicatedInferenceSizes_Sizes_GpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceSizes_Sizes_GpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `slug` after provisioning.\n"]
    pub fn slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.slug", self.base))
    }

    #[doc = "Get a reference to the value of field `vram_gb` after provisioning.\n"]
    pub fn vram_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vram_gb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceSizes_Sizes_SizeCategory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDedicatedInferenceSizes_Sizes_SizeCategory {
    #[doc = "Set the field `fleet_name`.\n"]
    pub fn set_fleet_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fleet_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceSizes_Sizes_SizeCategory {
    type O = terrars::BlockAssignable<DataDedicatedInferenceSizes_Sizes_SizeCategory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceSizes_Sizes_SizeCategory {}

impl BuildDataDedicatedInferenceSizes_Sizes_SizeCategory {
    pub fn build(self) -> DataDedicatedInferenceSizes_Sizes_SizeCategory {
        DataDedicatedInferenceSizes_Sizes_SizeCategory {
            fleet_name: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceSizes_Sizes_SizeCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceSizes_Sizes_SizeCategoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDedicatedInferenceSizes_Sizes_SizeCategoryRef {
        DataDedicatedInferenceSizes_Sizes_SizeCategoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceSizes_Sizes_SizeCategoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fleet_name` after provisioning.\n"]
    pub fn fleet_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDedicatedInferenceSizes_Sizes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disks: std::option::Option<terrars::ListField<DataDedicatedInferenceSizes_Sizes_Disks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu: std::option::Option<terrars::ListField<DataDedicatedInferenceSizes_Sizes_Gpu>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gpu_slug: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_per_hour: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_category: std::option::Option<terrars::ListField<DataDedicatedInferenceSizes_Sizes_SizeCategory>>,
}

impl DataDedicatedInferenceSizes_Sizes {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `currency`.\n"]
    pub fn set_currency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disks`.\n"]
    pub fn set_disks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDedicatedInferenceSizes_Sizes_Disks>>,
    ) -> Self {
        self.disks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu`.\n"]
    pub fn set_gpu(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDedicatedInferenceSizes_Sizes_Gpu>>,
    ) -> Self {
        self.gpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gpu_slug`.\n"]
    pub fn set_gpu_slug(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gpu_slug = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `price_per_hour`.\n"]
    pub fn set_price_per_hour(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_per_hour = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size_category`.\n"]
    pub fn set_size_category(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDedicatedInferenceSizes_Sizes_SizeCategory>>,
    ) -> Self {
        self.size_category = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDedicatedInferenceSizes_Sizes {
    type O = terrars::BlockAssignable<DataDedicatedInferenceSizes_Sizes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDedicatedInferenceSizes_Sizes {}

impl BuildDataDedicatedInferenceSizes_Sizes {
    pub fn build(self) -> DataDedicatedInferenceSizes_Sizes {
        DataDedicatedInferenceSizes_Sizes {
            cpu: core::default::Default::default(),
            currency: core::default::Default::default(),
            disks: core::default::Default::default(),
            gpu: core::default::Default::default(),
            gpu_slug: core::default::Default::default(),
            memory: core::default::Default::default(),
            price_per_hour: core::default::Default::default(),
            regions: core::default::Default::default(),
            size_category: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDedicatedInferenceSizes_SizesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDedicatedInferenceSizes_SizesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDedicatedInferenceSizes_SizesRef {
        DataDedicatedInferenceSizes_SizesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDedicatedInferenceSizes_SizesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\n"]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }

    #[doc = "Get a reference to the value of field `disks` after provisioning.\n"]
    pub fn disks(&self) -> terrars::ListRef<DataDedicatedInferenceSizes_Sizes_DisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disks", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu` after provisioning.\n"]
    pub fn gpu(&self) -> terrars::ListRef<DataDedicatedInferenceSizes_Sizes_GpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gpu", self.base))
    }

    #[doc = "Get a reference to the value of field `gpu_slug` after provisioning.\n"]
    pub fn gpu_slug(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gpu_slug", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `price_per_hour` after provisioning.\n"]
    pub fn price_per_hour(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_per_hour", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `size_category` after provisioning.\n"]
    pub fn size_category(&self) -> terrars::ListRef<DataDedicatedInferenceSizes_Sizes_SizeCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_category", self.base))
    }
}
