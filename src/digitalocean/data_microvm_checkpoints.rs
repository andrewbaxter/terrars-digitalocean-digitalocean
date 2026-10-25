#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataMicrovmCheckpointsData_ {
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
    microvm_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataMicrovmCheckpoints_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sort: std::option::Option<std::vec::Vec<DataMicrovmCheckpoints_Sort>>,
    dynamic: DataMicrovmCheckpointsDynamic,
}

struct DataMicrovmCheckpoints_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMicrovmCheckpointsData_>,
}

#[derive(Clone)]
pub struct DataMicrovmCheckpoints(std::rc::Rc<DataMicrovmCheckpoints_>);

impl DataMicrovmCheckpoints {
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
        "Set the field `microvm_id`.\nFilter checkpoints captured from this MicroVM UUID. Omit to list all checkpoints for the team."]
    pub fn set_microvm_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().microvm_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataMicrovmCheckpoints_Filter>>,
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
    pub fn set_sort(self, v: impl std::convert::Into<terrars::BlockAssignable<DataMicrovmCheckpoints_Sort>>) -> Self {
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

    #[doc = "Get a reference to the value of field `checkpoints` after provisioning.\n"]
    pub fn checkpoints(&self) -> terrars::ListRef<DataMicrovmCheckpoints_CheckpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `microvm_id` after provisioning.\nFilter checkpoints captured from this MicroVM UUID. Omit to list all checkpoints for the team."]
    pub fn microvm_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.microvm_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataMicrovmCheckpoints_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl terrars::Referable for DataMicrovmCheckpoints {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMicrovmCheckpoints { }

impl terrars::ToListMappable for DataMicrovmCheckpoints {
    type O = terrars::ListRef<DataMicrovmCheckpointsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMicrovmCheckpoints_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "digitalocean_microvm_checkpoints".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMicrovmCheckpoints {
    pub tf_id: std::string::String,
}

impl BuildDataMicrovmCheckpoints {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMicrovmCheckpoints {
        let out = DataMicrovmCheckpoints(std::rc::Rc::new(DataMicrovmCheckpoints_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMicrovmCheckpointsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                microvm_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                sort: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMicrovmCheckpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovmCheckpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMicrovmCheckpointsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `checkpoints` after provisioning.\n"]
    pub fn checkpoints(&self) -> terrars::ListRef<DataMicrovmCheckpoints_CheckpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.checkpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `microvm_id` after provisioning.\nFilter checkpoints captured from this MicroVM UUID. Omit to list all checkpoints for the team."]
    pub fn microvm_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.microvm_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> terrars::ListRef<DataMicrovmCheckpoints_SortRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataMicrovmCheckpointsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataMicrovmCheckpoints_Filter>>,
    sort: std::option::Option<terrars::DynamicBlock<DataMicrovmCheckpoints_Sort>>,
}

#[derive(Serialize)]
pub struct DataMicrovmCheckpoints_Checkpoints_Size {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
}

impl DataMicrovmCheckpoints_Checkpoints_Size {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataMicrovmCheckpoints_Checkpoints_Size {
    type O = terrars::BlockAssignable<DataMicrovmCheckpoints_Checkpoints_Size>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovmCheckpoints_Checkpoints_Size {}

impl BuildDataMicrovmCheckpoints_Checkpoints_Size {
    pub fn build(self) -> DataMicrovmCheckpoints_Checkpoints_Size {
        DataMicrovmCheckpoints_Checkpoints_Size {
            cpu: core::default::Default::default(),
            disk: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovmCheckpoints_Checkpoints_SizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovmCheckpoints_Checkpoints_SizeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovmCheckpoints_Checkpoints_SizeRef {
        DataMicrovmCheckpoints_Checkpoints_SizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovmCheckpoints_Checkpoints_SizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMicrovmCheckpoints_Checkpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microvm_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microvm_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::ListField<DataMicrovmCheckpoints_Checkpoints_Size>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMicrovmCheckpoints_Checkpoints {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_bytes`.\n"]
    pub fn set_disk_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_bytes`.\n"]
    pub fn set_memory_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `microvm_id`.\n"]
    pub fn set_microvm_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.microvm_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `microvm_name`.\n"]
    pub fn set_microvm_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.microvm_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMicrovmCheckpoints_Checkpoints_Size>>,
    ) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMicrovmCheckpoints_Checkpoints {
    type O = terrars::BlockAssignable<DataMicrovmCheckpoints_Checkpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovmCheckpoints_Checkpoints {}

impl BuildDataMicrovmCheckpoints_Checkpoints {
    pub fn build(self) -> DataMicrovmCheckpoints_Checkpoints {
        DataMicrovmCheckpoints_Checkpoints {
            created_at: core::default::Default::default(),
            disk_bytes: core::default::Default::default(),
            id: core::default::Default::default(),
            memory_bytes: core::default::Default::default(),
            microvm_id: core::default::Default::default(),
            microvm_name: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            size: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovmCheckpoints_CheckpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovmCheckpoints_CheckpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovmCheckpoints_CheckpointsRef {
        DataMicrovmCheckpoints_CheckpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovmCheckpoints_CheckpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_bytes` after provisioning.\n"]
    pub fn disk_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_bytes` after provisioning.\n"]
    pub fn memory_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `microvm_id` after provisioning.\n"]
    pub fn microvm_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.microvm_id", self.base))
    }

    #[doc = "Get a reference to the value of field `microvm_name` after provisioning.\n"]
    pub fn microvm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.microvm_name", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::ListRef<DataMicrovmCheckpoints_Checkpoints_SizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMicrovmCheckpoints_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_by: std::option::Option<terrars::PrimField<std::string::String>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataMicrovmCheckpoints_Filter {
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

impl terrars::ToListMappable for DataMicrovmCheckpoints_Filter {
    type O = terrars::BlockAssignable<DataMicrovmCheckpoints_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovmCheckpoints_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataMicrovmCheckpoints_Filter {
    pub fn build(self) -> DataMicrovmCheckpoints_Filter {
        DataMicrovmCheckpoints_Filter {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovmCheckpoints_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovmCheckpoints_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovmCheckpoints_FilterRef {
        DataMicrovmCheckpoints_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovmCheckpoints_FilterRef {
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
pub struct DataMicrovmCheckpoints_Sort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    key: terrars::PrimField<std::string::String>,
}

impl DataMicrovmCheckpoints_Sort {
    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMicrovmCheckpoints_Sort {
    type O = terrars::BlockAssignable<DataMicrovmCheckpoints_Sort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMicrovmCheckpoints_Sort {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataMicrovmCheckpoints_Sort {
    pub fn build(self) -> DataMicrovmCheckpoints_Sort {
        DataMicrovmCheckpoints_Sort {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct DataMicrovmCheckpoints_SortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMicrovmCheckpoints_SortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMicrovmCheckpoints_SortRef {
        DataMicrovmCheckpoints_SortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMicrovmCheckpoints_SortRef {
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
