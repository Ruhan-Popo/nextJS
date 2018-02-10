//! Type definitions for the Next.js manifest formats.

pub(crate) mod client_reference_manifest;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use turbo_tasks::{trace::TraceRawVcs, FxIndexMap, FxIndexSet, RcStr, TaskInput};

use crate::next_config::{CrossOriginConfig, Rewrites, RouteHas};

#[derive(Serialize, Default, Debug)]
pub struct PagesManifest {
    #[serde(flatten)]
    pub pages: HashMap<RcStr, RcStr>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildManifest {
    pub dev_files: Vec<RcStr>,
    pub amp_dev_files: Vec<RcStr>,
    pub polyfill_files: Vec<RcStr>,
    pub low_priority_files: Vec<RcStr>,
    pub root_main_files: Vec<RcStr>,
    pub pages: HashMap<RcStr, Vec<RcStr>>,
    pub amp_first_pages: Vec<RcStr>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "version")]
#[allow(clippy::large_enum_variant)]
pub enum MiddlewaresManifest {
    #[serde(rename = "2")]
    MiddlewaresManifestV2(MiddlewaresManifestV2),
    #[serde(other)]
    Unsupported,
}

impl Default for MiddlewaresManifest {
    fn default() -> Self {
        Self::MiddlewaresManifestV2(Default::default())
    }
}

#[derive(
    Debug,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    TaskInput,
    TraceRawVcs,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase", default)]
pub struct MiddlewareMatcher {
    // When skipped next.js with fill that during merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regexp: Option<RcStr>,
    #[serde(skip_serializing_if = "bool_is_true")]
    pub locale: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has: Option<Vec<RouteHas>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<Vec<RouteHas>>,
    pub original_source: RcStr,
}

impl Default for MiddlewareMatcher {
    fn default() -> Self {
        Self {
            regexp: None,
            locale: true,
            has: None,
            missing: None,
            original_source: Default::default(),
        }
    }
}

fn bool_is_true(b: &bool) -> bool {
    *b
}

#[derive(Serialize, Default, Debug)]
pub struct EdgeFunctionDefinition {
    pub files: Vec<RcStr>,
    pub name: RcStr,
    pub page: RcStr,
    pub matchers: Vec<MiddlewareMatcher>,
    pub wasm: Vec<AssetBinding>,
    pub assets: Vec<AssetBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Regions>,
    pub env: FxIndexMap<RcStr, RcStr>,
}

#[derive(Serialize, Default, Debug)]
pub struct InstrumentationDefinition {
    pub files: Vec<RcStr>,
    pub name: RcStr,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub wasm: Vec<AssetBinding>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<AssetBinding>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetBinding {
    pub name: RcStr,
    pub file_path: RcStr,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Regions {
    Multiple(Vec<RcStr>),
    Single(RcStr),
}

#[derive(Serialize, Default, Debug)]
pub struct MiddlewaresManifestV2 {
    pub sorted_middleware: Vec<RcStr>,
    pub middleware: HashMap<RcStr, EdgeFunctionDefinition>,
    pub instrumentation: Option<InstrumentationDefinition>,
    pub functions: HashMap<RcStr, EdgeFunctionDefinition>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReactLoadableManifest {
    #[serde(flatten)]
    pub manifest: HashMap<RcStr, ReactLoadableManifestEntry>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReactLoadableManifestEntry {
    pub id: u32,
    pub files: Vec<RcStr>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NextFontManifest {
    pub pages: HashMap<RcStr, Vec<RcStr>>,
    pub app: HashMap<RcStr, Vec<RcStr>>,
    pub app_using_size_adjust: bool,
    pub pages_using_size_adjust: bool,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppPathsManifest {
    #[serde(flatten)]
    pub edge_server_app_paths: PagesManifest,
    #[serde(flatten)]
    pub node_server_app_paths: PagesManifest,
}

// A struct represent a single entry in react-loadable-manifest.json.
// The manifest is in a format of:
// { [`${origin} -> ${imported}`]: { id: `${origin} -> ${imported}`, files:
// string[] } }
#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadableManifest {
    pub id: RcStr,
    pub files: Vec<RcStr>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerReferenceManifest<'a> {
    /// A map from hashed action name to the runtime module we that exports it.
    pub node: HashMap<&'a str, ActionManifestEntry<'a>>,
    /// A map from hashed action name to the runtime module we that exports it.
    pub edge: HashMap<&'a str, ActionManifestEntry<'a>>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActionManifestEntry<'a> {
    /// A mapping from the page that uses the server action to the runtime
    /// module that exports it.
    pub workers: HashMap<&'a str, ActionManifestWorkerEntry<'a>>,

    pub layer: HashMap<&'a str, ActionLayer>,
}

#[derive(Serialize, Debug)]
pub struct ActionManifestWorkerEntry<'a> {
    #[serde(rename = "moduleId")]
    pub module_id: ActionManifestModuleId<'a>,
    #[serde(rename = "async")]
    pub is_async: bool,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ActionManifestModuleId<'a> {
    String(&'a str),
    Number(f64),
}

#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    TaskInput,
    TraceRawVcs,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum ActionLayer {
    Rsc,
    ActionBrowser,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientReferenceManifest {
    pub module_loading: ModuleLoading,
    /// Mapping of module path and export name to client module ID and required
    /// client chunks.
    pub client_modules: ManifestNode,
    /// Mapping of client module ID to corresponding SSR module ID and required
    /// SSR chunks.
    pub ssr_module_mapping: HashMap<ModuleId, ManifestNode>,
    /// Same as `ssr_module_mapping`, but for Edge SSR.
    #[serde(rename = "edgeSSRModuleMapping")]
    pub edge_ssr_module_mapping: HashMap<ModuleId, ManifestNode>,
    /// Mapping of client module ID to corresponding RSC module ID and required
    /// RSC chunks.
    pub rsc_module_mapping: HashMap<ModuleId, ManifestNode>,
    /// Same as `rsc_module_mapping`, but for Edge RSC.
    #[serde(rename = "edgeRscModuleMapping")]
    pub edge_rsc_module_mapping: HashMap<ModuleId, ManifestNode>,
    /// Mapping of server component path to required CSS client chunks.
    #[serde(rename = "entryCSSFiles")]
    pub entry_css_files: HashMap<RcStr, FxIndexSet<RcStr>>,
    /// Mapping of server component path to required JS client chunks.
    #[serde(rename = "entryJSFiles")]
    pub entry_js_files: HashMap<RcStr, FxIndexSet<RcStr>>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleLoading {
    pub prefix: RcStr,
    pub cross_origin: Option<CrossOriginConfig>,
}

#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestNode {
    /// Mapping of export name to manifest node entry.
    #[serde(flatten)]
    pub module_exports: HashMap<RcStr, ManifestNodeEntry>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestNodeEntry {
    /// Turbopack module ID.
    pub id: ModuleId,
    /// Export name.
    pub name: RcStr,
    /// Chunks for the module. JS and CSS.
    pub chunks: Vec<RcStr>,
    // TODO(WEB-434)
    pub r#async: bool,
}

#[derive(Serialize, Debug, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ModuleId {
    String(RcStr),
    Number(u64),
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FontManifest(pub Vec<FontManifestEntry>);

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FontManifestEntry {
    pub url: RcStr,
    pub content: RcStr,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppBuildManifest {
    pub pages: HashMap<RcStr, Vec<RcStr>>,
}

// TODO(alexkirsz) Unify with the one for dev.
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientBuildManifest<'a> {
    #[serde(rename = "__rewrites")]
    pub rewrites: &'a Rewrites,

    pub sorted_pages: &'a [RcStr],

    #[serde(flatten)]
    pub pages: HashMap<RcStr, Vec<&'a str>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middleware_matcher_serialization() {
        let matchers = vec![
            MiddlewareMatcher {
                regexp: None,
                locale: false,
                has: None,
                missing: None,
                original_source: "".into(),
            },
            MiddlewareMatcher {
                regexp: Some(".*".into()),
                locale: true,
                has: Some(vec![RouteHas::Query {
                    key: "foo".into(),
                    value: None,
                }]),
                missing: Some(vec![RouteHas::Query {
                    key: "bar".into(),
                    value: Some("value".into()),
                }]),
                original_source: "source".into(),
            },
        ];

        let serialized = serde_json::to_string(&matchers).unwrap();
        let deserialized: Vec<MiddlewareMatcher> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(matchers, deserialized);
    }
}