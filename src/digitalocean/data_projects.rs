use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataProjectsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataProjectsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataProjectsSortEl>>,
    dynamic: DataProjectsDynamic,
}

struct DataProjects_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataProjectsData>,
}

#[derive(Clone)]
pub struct DataProjects(Rc<DataProjects_>);

impl DataProjects {
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataProjectsFilterEl>>) -> Self {
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
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataProjectsSortEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `projects` after provisioning.\n"]
    pub fn projects(&self) -> ListRef<DataProjectsProjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataProjectsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Datasource for DataProjects {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataProjects {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataProjects {
    type O = ListRef<DataProjectsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataProjects_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_projects".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataProjects {
    pub tf_id: String,
}

impl BuildDataProjects {
    pub fn build(self, stack: &mut Stack) -> DataProjects {
        let out = DataProjects(Rc::new(DataProjects_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataProjectsData {
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

pub struct DataProjectsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataProjectsRef {
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

    #[doc= "Get a reference to the value of field `projects` after provisioning.\n"]
    pub fn projects(&self) -> ListRef<DataProjectsProjectsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.projects", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataProjectsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataProjectsProjectsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_default: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_uuid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purpose: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_at: Option<PrimField<String>>,
}

impl DataProjectsProjectsEl {
    #[doc= "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_at = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.environment = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `is_default`.\n"]
    pub fn set_is_default(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_default = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `owner_id`.\n"]
    pub fn set_owner_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.owner_id = Some(v.into());
        self
    }

    #[doc= "Set the field `owner_uuid`.\n"]
    pub fn set_owner_uuid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner_uuid = Some(v.into());
        self
    }

    #[doc= "Set the field `purpose`.\n"]
    pub fn set_purpose(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.purpose = Some(v.into());
        self
    }

    #[doc= "Set the field `resources`.\n"]
    pub fn set_resources(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.resources = Some(v.into());
        self
    }

    #[doc= "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsProjectsEl {
    type O = BlockAssignable<DataProjectsProjectsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsProjectsEl {}

impl BuildDataProjectsProjectsEl {
    pub fn build(self) -> DataProjectsProjectsEl {
        DataProjectsProjectsEl {
            created_at: core::default::Default::default(),
            description: core::default::Default::default(),
            environment: core::default::Default::default(),
            id: core::default::Default::default(),
            is_default: core::default::Default::default(),
            name: core::default::Default::default(),
            owner_id: core::default::Default::default(),
            owner_uuid: core::default::Default::default(),
            purpose: core::default::Default::default(),
            resources: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

pub struct DataProjectsProjectsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsProjectsElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsProjectsElRef {
        DataProjectsProjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsProjectsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `is_default` after provisioning.\n"]
    pub fn is_default(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_default", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc= "Get a reference to the value of field `owner_uuid` after provisioning.\n"]
    pub fn owner_uuid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_uuid", self.base))
    }

    #[doc= "Get a reference to the value of field `purpose` after provisioning.\n"]
    pub fn purpose(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.purpose", self.base))
    }

    #[doc= "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc= "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataProjectsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataProjectsFilterEl {
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

impl ToListMappable for DataProjectsFilterEl {
    type O = BlockAssignable<DataProjectsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataProjectsFilterEl {
    pub fn build(self) -> DataProjectsFilterEl {
        DataProjectsFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataProjectsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsFilterElRef {
        DataProjectsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsFilterElRef {
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
pub struct DataProjectsSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataProjectsSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataProjectsSortEl {
    type O = BlockAssignable<DataProjectsSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataProjectsSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataProjectsSortEl {
    pub fn build(self) -> DataProjectsSortEl {
        DataProjectsSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataProjectsSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataProjectsSortElRef {
    fn new(shared: StackShared, base: String) -> DataProjectsSortElRef {
        DataProjectsSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataProjectsSortElRef {
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
struct DataProjectsDynamic {
    filter: Option<DynamicBlock<DataProjectsFilterEl>>,
    sort: Option<DynamicBlock<DataProjectsSortEl>>,
}
