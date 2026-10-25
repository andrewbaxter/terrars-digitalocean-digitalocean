#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderDigitalocean;

#[derive(Serialize)]
struct AgentPlatformKnowledgeBaseDataSourceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    knowledge_base_uuid: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spaces_data_source: std::option::Option<std::vec::Vec<AgentPlatformKnowledgeBaseDataSource_SpacesDataSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_crawler_data_source: std
    ::option
    ::Option<std::vec::Vec<AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource>>,
    dynamic: AgentPlatformKnowledgeBaseDataSourceDynamic,
}

struct AgentPlatformKnowledgeBaseDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AgentPlatformKnowledgeBaseDataSourceData_>,
}

#[derive(Clone)]
pub struct AgentPlatformKnowledgeBaseDataSource(std::rc::Rc<AgentPlatformKnowledgeBaseDataSource_>);

impl AgentPlatformKnowledgeBaseDataSource {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderDigitalocean) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `spaces_data_source`.\n"]
    pub fn set_spaces_data_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBaseDataSource_SpacesDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spaces_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spaces_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_crawler_data_source`.\n"]
    pub fn set_web_crawler_data_source(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().web_crawler_data_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.web_crawler_data_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spaces_data_source` after provisioning.\n"]
    pub fn spaces_data_source(&self) -> terrars::ListRef<AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spaces_data_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_crawler_data_source` after provisioning.\n"]
    pub fn web_crawler_data_source(
        &self,
    ) -> terrars::ListRef<AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_crawler_data_source", self.extract_ref()))
    }
}

impl terrars::Referable for AgentPlatformKnowledgeBaseDataSource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AgentPlatformKnowledgeBaseDataSource { }

impl terrars::ToListMappable for AgentPlatformKnowledgeBaseDataSource {
    type O = terrars::ListRef<AgentPlatformKnowledgeBaseDataSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AgentPlatformKnowledgeBaseDataSource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "digitalocean_agent_platform_knowledge_base_data_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAgentPlatformKnowledgeBaseDataSource {
    pub tf_id: std::string::String,
    #[doc = "UUID of the Knowledge Base"]
    pub knowledge_base_uuid: terrars::PrimField<std::string::String>,
}

impl BuildAgentPlatformKnowledgeBaseDataSource {
    pub fn build(self, stack: &mut terrars::Stack) -> AgentPlatformKnowledgeBaseDataSource {
        let out = AgentPlatformKnowledgeBaseDataSource(std::rc::Rc::new(AgentPlatformKnowledgeBaseDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AgentPlatformKnowledgeBaseDataSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                knowledge_base_uuid: self.knowledge_base_uuid,
                spaces_data_source: core::default::Default::default(),
                web_crawler_data_source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AgentPlatformKnowledgeBaseDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBaseDataSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AgentPlatformKnowledgeBaseDataSourceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `knowledge_base_uuid` after provisioning.\nUUID of the Knowledge Base"]
    pub fn knowledge_base_uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.knowledge_base_uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spaces_data_source` after provisioning.\n"]
    pub fn spaces_data_source(&self) -> terrars::ListRef<AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spaces_data_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `web_crawler_data_source` after provisioning.\n"]
    pub fn web_crawler_data_source(
        &self,
    ) -> terrars::ListRef<AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_crawler_data_source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBaseDataSource_SpacesDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    item_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AgentPlatformKnowledgeBaseDataSource_SpacesDataSource {
    #[doc = "Set the field `bucket_name`.\nThe name of the Spaces bucket"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `item_path`.\nThe path to the item in the bucket"]
    pub fn set_item_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.item_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the Spaces bucket"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBaseDataSource_SpacesDataSource {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBaseDataSource_SpacesDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBaseDataSource_SpacesDataSource {}

impl BuildAgentPlatformKnowledgeBaseDataSource_SpacesDataSource {
    pub fn build(self) -> AgentPlatformKnowledgeBaseDataSource_SpacesDataSource {
        AgentPlatformKnowledgeBaseDataSource_SpacesDataSource {
            bucket_name: core::default::Default::default(),
            item_path: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef {
        AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBaseDataSource_SpacesDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the Spaces bucket"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `item_path` after provisioning.\nThe path to the item in the bucket"]
    pub fn item_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.item_path", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe region of the Spaces bucket"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawling_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    embed_media: std::option::Option<terrars::PrimField<bool>>,
}

impl AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {
    #[doc = "Set the field `base_url`.\nThe base URL to crawl"]
    pub fn set_base_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.base_url = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `crawling_option`.\nOptions for specifying how URLs found on pages should be handled. \n- UNKNOWN: Default unknown value\n- SCOPED: Only include the base URL.\n- PATH: Crawl the base URL and linked pages within the URL path.\n- DOMAIN: Crawl the base URL and linked pages within the same domain.\n- SUBDOMAINS: Crawl the base URL and linked pages for any subdomain."]
    pub fn set_crawling_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawling_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `embed_media`.\nWhether to embed media content"]
    pub fn set_embed_media(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.embed_media = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {
    type O = terrars::BlockAssignable<AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {}

impl BuildAgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {
    pub fn build(self) -> AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {
        AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource {
            base_url: core::default::Default::default(),
            crawling_option: core::default::Default::default(),
            embed_media: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef {
        AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_url` after provisioning.\nThe base URL to crawl"]
    pub fn base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_url", self.base))
    }

    #[doc =
        "Get a reference to the value of field `crawling_option` after provisioning.\nOptions for specifying how URLs found on pages should be handled. \n- UNKNOWN: Default unknown value\n- SCOPED: Only include the base URL.\n- PATH: Crawl the base URL and linked pages within the URL path.\n- DOMAIN: Crawl the base URL and linked pages within the same domain.\n- SUBDOMAINS: Crawl the base URL and linked pages for any subdomain."]
    pub fn crawling_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawling_option", self.base))
    }

    #[doc = "Get a reference to the value of field `embed_media` after provisioning.\nWhether to embed media content"]
    pub fn embed_media(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.embed_media", self.base))
    }
}

#[derive(Serialize, Default)]
struct AgentPlatformKnowledgeBaseDataSourceDynamic {
    spaces_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBaseDataSource_SpacesDataSource>>,
    web_crawler_data_source: std
    ::option
    ::Option<terrars::DynamicBlock<AgentPlatformKnowledgeBaseDataSource_WebCrawlerDataSource>>,
}
