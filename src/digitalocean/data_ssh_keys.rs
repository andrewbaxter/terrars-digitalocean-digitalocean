use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataSshKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataSshKeysFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataSshKeysSortEl>>,
    dynamic: DataSshKeysDynamic,
}

struct DataSshKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSshKeysData>,
}

#[derive(Clone)]
pub struct DataSshKeys(Rc<DataSshKeys_>);

impl DataSshKeys {
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataSshKeysFilterEl>>) -> Self {
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
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataSshKeysSortEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> ListRef<DataSshKeysSshKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataSshKeysSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Datasource for DataSshKeys {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataSshKeys {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataSshKeys {
    type O = ListRef<DataSshKeysRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataSshKeys_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_ssh_keys".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSshKeys {
    pub tf_id: String,
}

impl BuildDataSshKeys {
    pub fn build(self, stack: &mut Stack) -> DataSshKeys {
        let out = DataSshKeys(Rc::new(DataSshKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSshKeysData {
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

pub struct DataSshKeysRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSshKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSshKeysRef {
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

    #[doc= "Get a reference to the value of field `ssh_keys` after provisioning.\n"]
    pub fn ssh_keys(&self) -> ListRef<DataSshKeysSshKeysElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ssh_keys", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataSshKeysSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSshKeysSshKeysEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    fingerprint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_key: Option<PrimField<String>>,
}

impl DataSshKeysSshKeysEl {
    #[doc= "Set the field `fingerprint`.\n"]
    pub fn set_fingerprint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.fingerprint = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `public_key`.\n"]
    pub fn set_public_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataSshKeysSshKeysEl {
    type O = BlockAssignable<DataSshKeysSshKeysEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSshKeysSshKeysEl {}

impl BuildDataSshKeysSshKeysEl {
    pub fn build(self) -> DataSshKeysSshKeysEl {
        DataSshKeysSshKeysEl {
            fingerprint: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            public_key: core::default::Default::default(),
        }
    }
}

pub struct DataSshKeysSshKeysElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSshKeysSshKeysElRef {
    fn new(shared: StackShared, base: String) -> DataSshKeysSshKeysElRef {
        DataSshKeysSshKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSshKeysSshKeysElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\n"]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.base))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `public_key` after provisioning.\n"]
    pub fn public_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSshKeysFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataSshKeysFilterEl {
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

impl ToListMappable for DataSshKeysFilterEl {
    type O = BlockAssignable<DataSshKeysFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSshKeysFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataSshKeysFilterEl {
    pub fn build(self) -> DataSshKeysFilterEl {
        DataSshKeysFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataSshKeysFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSshKeysFilterElRef {
    fn new(shared: StackShared, base: String) -> DataSshKeysFilterElRef {
        DataSshKeysFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSshKeysFilterElRef {
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
pub struct DataSshKeysSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataSshKeysSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataSshKeysSortEl {
    type O = BlockAssignable<DataSshKeysSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSshKeysSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataSshKeysSortEl {
    pub fn build(self) -> DataSshKeysSortEl {
        DataSshKeysSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataSshKeysSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSshKeysSortElRef {
    fn new(shared: StackShared, base: String) -> DataSshKeysSortElRef {
        DataSshKeysSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSshKeysSortElRef {
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
struct DataSshKeysDynamic {
    filter: Option<DynamicBlock<DataSshKeysFilterEl>>,
    sort: Option<DynamicBlock<DataSshKeysSortEl>>,
}
