use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataSpacesBucketsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataSpacesBucketsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<DataSpacesBucketsSortEl>>,
    dynamic: DataSpacesBucketsDynamic,
}

struct DataSpacesBuckets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSpacesBucketsData>,
}

#[derive(Clone)]
pub struct DataSpacesBuckets(Rc<DataSpacesBuckets_>);

impl DataSpacesBuckets {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataSpacesBucketsFilterEl>>) -> Self {
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
    pub fn set_sort(self, v: impl Into<BlockAssignable<DataSpacesBucketsSortEl>>) -> Self {
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

    #[doc= "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> ListRef<DataSpacesBucketsBucketsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.buckets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataSpacesBucketsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

impl Referable for DataSpacesBuckets {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSpacesBuckets { }

impl ToListMappable for DataSpacesBuckets {
    type O = ListRef<DataSpacesBucketsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSpacesBuckets_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_spaces_buckets".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSpacesBuckets {
    pub tf_id: String,
}

impl BuildDataSpacesBuckets {
    pub fn build(self, stack: &mut Stack) -> DataSpacesBuckets {
        let out = DataSpacesBuckets(Rc::new(DataSpacesBuckets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSpacesBucketsData {
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

pub struct DataSpacesBucketsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSpacesBucketsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSpacesBucketsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> ListRef<DataSpacesBucketsBucketsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.buckets", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sort` after provisioning.\n"]
    pub fn sort(&self) -> ListRef<DataSpacesBucketsSortElRef> {
        ListRef::new(self.shared().clone(), format!("{}.sort", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSpacesBucketsBucketsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_domain_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    urn: Option<PrimField<String>>,
}

impl DataSpacesBucketsBucketsEl {
    #[doc= "Set the field `bucket_domain_name`.\n"]
    pub fn set_bucket_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_domain_name = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc= "Set the field `urn`.\n"]
    pub fn set_urn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.urn = Some(v.into());
        self
    }
}

impl ToListMappable for DataSpacesBucketsBucketsEl {
    type O = BlockAssignable<DataSpacesBucketsBucketsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpacesBucketsBucketsEl {}

impl BuildDataSpacesBucketsBucketsEl {
    pub fn build(self) -> DataSpacesBucketsBucketsEl {
        DataSpacesBucketsBucketsEl {
            bucket_domain_name: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            name: core::default::Default::default(),
            region: core::default::Default::default(),
            urn: core::default::Default::default(),
        }
    }
}

pub struct DataSpacesBucketsBucketsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSpacesBucketsBucketsElRef {
    fn new(shared: StackShared, base: String) -> DataSpacesBucketsBucketsElRef {
        DataSpacesBucketsBucketsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpacesBucketsBucketsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_domain_name` after provisioning.\n"]
    pub fn bucket_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_domain_name", self.base))
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc= "Get a reference to the value of field `urn` after provisioning.\n"]
    pub fn urn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.urn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSpacesBucketsFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    all: Option<PrimField<bool>>,
    key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    match_by: Option<PrimField<String>>,
    values: ListField<PrimField<String>>,
}

impl DataSpacesBucketsFilterEl {
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

impl ToListMappable for DataSpacesBucketsFilterEl {
    type O = BlockAssignable<DataSpacesBucketsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpacesBucketsFilterEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub values: ListField<PrimField<String>>,
}

impl BuildDataSpacesBucketsFilterEl {
    pub fn build(self) -> DataSpacesBucketsFilterEl {
        DataSpacesBucketsFilterEl {
            all: core::default::Default::default(),
            key: self.key,
            match_by: core::default::Default::default(),
            values: self.values,
        }
    }
}

pub struct DataSpacesBucketsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSpacesBucketsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataSpacesBucketsFilterElRef {
        DataSpacesBucketsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpacesBucketsFilterElRef {
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
pub struct DataSpacesBucketsSortEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<PrimField<String>>,
    key: PrimField<String>,
}

impl DataSpacesBucketsSortEl {
    #[doc= "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.direction = Some(v.into());
        self
    }
}

impl ToListMappable for DataSpacesBucketsSortEl {
    type O = BlockAssignable<DataSpacesBucketsSortEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpacesBucketsSortEl {
    #[doc= ""]
    pub key: PrimField<String>,
}

impl BuildDataSpacesBucketsSortEl {
    pub fn build(self) -> DataSpacesBucketsSortEl {
        DataSpacesBucketsSortEl {
            direction: core::default::Default::default(),
            key: self.key,
        }
    }
}

pub struct DataSpacesBucketsSortElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSpacesBucketsSortElRef {
    fn new(shared: StackShared, base: String) -> DataSpacesBucketsSortElRef {
        DataSpacesBucketsSortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpacesBucketsSortElRef {
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
struct DataSpacesBucketsDynamic {
    filter: Option<DynamicBlock<DataSpacesBucketsFilterEl>>,
    sort: Option<DynamicBlock<DataSpacesBucketsSortEl>>,
}
