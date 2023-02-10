use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct DataImageData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slug: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
}

struct DataImage_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataImageData>,
}

#[derive(Clone)]
pub struct DataImage(Rc<DataImage_>);

impl DataImage {
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

    #[doc= "Set the field `id`.\nid of the image"]
    pub fn set_id(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nname of the image"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `slug`.\nslug of the image"]
    pub fn set_slug(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().slug = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().source = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\na description of the image"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `distribution` after provisioning.\ndistribution of the OS of the image"]
    pub fn distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `error_message` after provisioning.\nerror message associated with the image"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nid of the image"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nslug or id of the image"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_disk_size` after provisioning.\nminimum disk size required by the image"]
    pub fn min_disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_disk_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the image"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private` after provisioning.\nIs the image private or non-private"]
    pub fn private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\nlist of the regions that the image is available in"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_gigabytes` after provisioning.\nsize in GB of the image"]
    pub fn size_gigabytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gigabytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\nslug of the image"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nstatus of the image"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\ntags applied to the image"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\ntype of the image"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Datasource for DataImage {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataImage {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataImage {
    type O = ListRef<DataImageRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(&self))
    }
}

impl Datasource_ for DataImage_ {
    fn extract_datasource_type(&self) -> String {
        "digitalocean_image".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataImage {
    pub tf_id: String,
}

impl BuildDataImage {
    pub fn build(self, stack: &mut Stack) -> DataImage {
        let out = DataImage(Rc::new(DataImage_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataImageData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                slug: core::default::Default::default(),
                source: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataImageRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataImageRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataImageRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\na description of the image"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `distribution` after provisioning.\ndistribution of the OS of the image"]
    pub fn distribution(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.distribution", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `error_message` after provisioning.\nerror message associated with the image"]
    pub fn error_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\nid of the image"]
    pub fn id(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\nslug or id of the image"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_disk_size` after provisioning.\nminimum disk size required by the image"]
    pub fn min_disk_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_disk_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nname of the image"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `private` after provisioning.\nIs the image private or non-private"]
    pub fn private(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.private", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `regions` after provisioning.\nlist of the regions that the image is available in"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size_gigabytes` after provisioning.\nsize in GB of the image"]
    pub fn size_gigabytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size_gigabytes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `slug` after provisioning.\nslug of the image"]
    pub fn slug(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.slug", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\nstatus of the image"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\ntags applied to the image"]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\ntype of the image"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}
