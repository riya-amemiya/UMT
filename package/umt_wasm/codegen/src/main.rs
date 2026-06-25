//! Generator for the umt_wasm bindings.
//!
//! It walks the local `umt_rust` source tree, finds every `pub fn umt_*`, and
//! emits a `#[wasm_bindgen]` wrapper for each function whose signature is
//! expressible across the wasm-bindgen ABI. Functions that need a hand-written
//! adapter (custom struct/enum types) or that cannot be exposed at all
//! (generics, closures, async, reference returns) are listed in the report
//! instead, so it is always visible what is and is not covered.
//!
//! umt_rust re-exports its functions inconsistently (some submodules are
//! `pub use`d up to the top module, some are reachable only at their own path),
//! so the call path for each function is resolved from the actual module graph
//! rather than assumed.
//!
//! Run it from the umt_wasm package with `bun run gen` (which also runs
//! `cargo fmt`), or directly with
//! `cargo run --manifest-path codegen/Cargo.toml`.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    FnArg, GenericArgument, GenericParam, Item, Pat, PathArguments, ReturnType, Type, UseTree,
};
use walkdir::WalkDir;

/// A function discovered in the umt_rust source, before classification.
struct Candidate {
    /// Module the function is defined in (e.g. `["math", "calculator"]`).
    module: Vec<String>,
    sig: syn::Signature,
}

/// The wasm-mapped signature parts of a function that can be wrapped.
struct Classified {
    params: Vec<TokenStream>,
    pre: Vec<TokenStream>,
    call_args: Vec<TokenStream>,
    ret_ty: Option<TokenStream>,
    is_result: bool,
}

/// A wrapper ready to emit, with its resolved public call path.
struct Wrapper {
    /// Public path the function is reachable at (e.g. `["validate"]`).
    call_path: Vec<String>,
    fn_name: String,
    classified: Classified,
}

/// A function that was skipped, with the reason, for the coverage report.
struct Skipped {
    module: String,
    fn_name: String,
    reason: String,
}

/// The `pub use` re-exports a module makes from its direct children.
#[derive(Default)]
struct Reexports {
    /// Children re-exported wholesale with `pub use child::*`.
    globs: HashSet<String>,
    /// `(child, name)` pairs re-exported with `pub use child::name`.
    named: HashSet<(String, String)>,
}

/// The module structure of umt_rust, used to resolve public call paths.
#[derive(Default)]
struct ModuleGraph {
    /// module path -> list of (child module name, declared `pub mod`).
    children: HashMap<Vec<String>, Vec<(String, bool)>>,
    /// module path -> the `pub use` re-exports declared in that module.
    reexports: HashMap<Vec<String>, Reexports>,
}

impl ModuleGraph {
    /// Is `path` reachable from the crate root through `pub mod` declarations?
    fn reachable(&self, path: &[String]) -> bool {
        for i in 0..path.len() {
            let parent = &path[..i];
            let child = &path[i];
            let ok = self
                .children
                .get(parent)
                .is_some_and(|v| v.iter().any(|(n, is_pub)| n == child && *is_pub));
            if !ok {
                return false;
            }
        }
        true
    }

    /// Does module `parent` re-export `fn_name` from its child `child`?
    fn reexports_fn(&self, parent: &[String], child: &str, fn_name: &str) -> bool {
        self.reexports.get(parent).is_some_and(|r| {
            r.globs.contains(child) || r.named.contains(&(child.to_string(), fn_name.to_string()))
        })
    }

    /// Shortest public path `fn_name` (defined in `defining`) is reachable at,
    /// following the chain of `pub use` re-exports up from its defining module.
    fn resolve(&self, defining: &[String], fn_name: &str) -> Option<Vec<String>> {
        let mut exposed = vec![defining.to_vec()];
        let mut cur = defining.to_vec();
        while !cur.is_empty() {
            let child = cur.last().unwrap().clone();
            let parent = cur[..cur.len() - 1].to_vec();
            if self.reexports_fn(&parent, &child, fn_name) {
                exposed.push(parent.clone());
                cur = parent;
            } else {
                break;
            }
        }
        exposed
            .into_iter()
            .filter(|p| !p.is_empty() && self.reachable(p))
            .min_by_key(Vec::len)
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rust_src = manifest_dir.join("../../umt_rust/src");
    let out_rs = manifest_dir.join("../src/generated.rs");
    let out_report = manifest_dir.join("../doc/generated.md");

    let rust_src = rust_src
        .canonicalize()
        .unwrap_or_else(|e| panic!("cannot find umt_rust src at {}: {e}", rust_src.display()));

    let mut graph = ModuleGraph::default();
    let mut candidates: Vec<Candidate> = Vec::new();
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new();

    for entry in WalkDir::new(&rust_src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let Some(file_mod) = file_module_path(&rust_src, path) else {
            continue;
        };
        let Ok(content) = fs::read_to_string(path) else {
            continue;
        };
        let Ok(file) = syn::parse_file(&content) else {
            continue;
        };
        for item in file.items {
            match item {
                Item::Mod(m) => {
                    let is_pub = matches!(m.vis, syn::Visibility::Public(_));
                    graph
                        .children
                        .entry(file_mod.clone())
                        .or_default()
                        .push((m.ident.to_string(), is_pub));
                }
                Item::Use(u) if matches!(u.vis, syn::Visibility::Public(_)) => {
                    let mut items = Vec::new();
                    collect_reexports(&u.tree, &mut items);
                    let entry = graph.reexports.entry(file_mod.clone()).or_default();
                    for (child, name) in items {
                        match name {
                            Some(n) => {
                                entry.named.insert((child, n));
                            }
                            None => {
                                entry.globs.insert(child);
                            }
                        }
                    }
                }
                Item::Fn(item_fn) => {
                    if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
                        continue;
                    }
                    let fn_name = item_fn.sig.ident.to_string();
                    if !fn_name.starts_with("umt_") {
                        continue;
                    }
                    if seen.insert((file_mod.join("::"), fn_name)) {
                        candidates.push(Candidate {
                            module: file_mod.clone(),
                            sig: item_fn.sig,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    let mut wrappers: Vec<Wrapper> = Vec::new();
    let mut skipped: Vec<Skipped> = Vec::new();

    for cand in candidates {
        let fn_name = cand.sig.ident.to_string();
        match classify(&cand.sig) {
            Err(reason) => skipped.push(Skipped {
                module: cand.module.join("::"),
                fn_name,
                reason,
            }),
            Ok(classified) => match graph.resolve(&cand.module, &fn_name) {
                Some(call_path) => wrappers.push(Wrapper {
                    call_path,
                    fn_name,
                    classified,
                }),
                None => skipped.push(Skipped {
                    module: cand.module.join("::"),
                    fn_name,
                    reason: "not reachable through a public path".into(),
                }),
            },
        }
    }

    // Deterministic order so js-name collision resolution is stable across runs.
    wrappers.sort_by(|a, b| {
        (a.call_path.join("::"), &a.fn_name).cmp(&(b.call_path.join("::"), &b.fn_name))
    });
    skipped.sort_by(|a, b| (&a.module, &a.fn_name).cmp(&(&b.module, &b.fn_name)));

    let mut used_js: BTreeSet<String> = BTreeSet::new();
    let mut items: Vec<TokenStream> = Vec::new();
    let mut report_rows: Vec<(String, String, String)> = Vec::new();

    for w in &wrappers {
        let stripped = w.fn_name.strip_prefix("umt_").unwrap_or(&w.fn_name);
        let mut js_name = stripped.to_lower_camel_case();
        if used_js.contains(&js_name) {
            // Disambiguate by prefixing the top module (e.g. `birthday` -> `simpleBirthday`).
            js_name = format!("{}_{}", w.call_path[0], stripped).to_lower_camel_case();
        }
        used_js.insert(js_name.clone());
        report_rows.push((w.call_path.join("::"), w.fn_name.clone(), js_name.clone()));
        items.push(emit_wrapper(w, &js_name));
    }

    let header = quote! {
        // @generated by codegen/ - do not edit by hand.
        // Run `bun run gen` (in package/umt_wasm) to regenerate.
        #![allow(clippy::all)]
        #![allow(clippy::pedantic)]

        use wasm_bindgen::prelude::*;
    };

    let file_tokens = quote! {
        #header
        #(#items)*
    };

    let parsed: syn::File = syn::parse2(file_tokens).expect("generated tokens must parse");
    let formatted = prettyplease::unparse(&parsed);
    fs::write(&out_rs, formatted).expect("write generated.rs");

    write_report(&out_report, &report_rows, &skipped);

    eprintln!(
        "generated {} wrappers, skipped {} functions -> {}",
        wrappers.len(),
        skipped.len(),
        out_rs.display()
    );
}

/// Module path that a source file's items belong to, relative to `src/`.
///
/// `src/lib.rs` -> `[]` (crate root), `src/date/mod.rs` -> `["date"]`,
/// `src/math/calculator.rs` -> `["math", "calculator"]`.
fn file_module_path(src_root: &Path, file: &Path) -> Option<Vec<String>> {
    let rel = file.strip_prefix(src_root).ok()?;
    let mut parts: Vec<String> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str().map(str::to_string))
        .collect();
    let last = parts.pop()?;
    let stem = last.strip_suffix(".rs")?;
    if stem != "mod" && stem != "lib" {
        parts.push(stem.to_string());
    }
    Some(parts)
}

/// Collect re-exports from a `pub use` tree as `(child, name)` pairs, where
/// `name` is `None` for a `child::*` glob and `Some(fn)` for `child::fn`.
fn collect_reexports(tree: &UseTree, out: &mut Vec<(String, Option<String>)>) {
    match tree {
        UseTree::Path(p) => {
            let ident = p.ident.to_string();
            if ident == "self" {
                collect_reexports(&p.tree, out);
                return;
            }
            if matches!(ident.as_str(), "crate" | "super") {
                return;
            }
            match &*p.tree {
                UseTree::Glob(_) => out.push((ident, None)),
                UseTree::Name(n) => out.push((ident, Some(n.ident.to_string()))),
                UseTree::Group(g) => {
                    for it in &g.items {
                        match it {
                            UseTree::Name(n) => {
                                out.push((ident.clone(), Some(n.ident.to_string())))
                            }
                            UseTree::Glob(_) => out.push((ident.clone(), None)),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        UseTree::Group(g) => {
            for it in &g.items {
                collect_reexports(it, out);
            }
        }
        _ => {}
    }
}

/// Map a function's signature onto the wasm boundary, or `Err(reason)`.
fn classify(sig: &syn::Signature) -> Result<Classified, String> {
    if sig.asyncness.is_some() {
        return Err("async function".into());
    }
    if sig
        .generics
        .params
        .iter()
        .any(|p| matches!(p, GenericParam::Type(_) | GenericParam::Const(_)))
    {
        return Err("generic type parameters".into());
    }

    let mut params = Vec::new();
    let mut pre = Vec::new();
    let mut call_args = Vec::new();

    for input in &sig.inputs {
        let FnArg::Typed(pt) = input else {
            return Err("has a self receiver".into());
        };
        let Pat::Ident(pat_ident) = &*pt.pat else {
            return Err("non-identifier parameter pattern".into());
        };
        let ident = pat_ident.ident.clone();
        let mapping = map_param(&ident, &pt.ty)
            .ok_or_else(|| format!("unsupported parameter type `{}`", type_str(&pt.ty)))?;
        let wasm_ty = mapping.wasm_ty;
        params.push(quote! { #ident: #wasm_ty });
        if let Some(stmt) = mapping.pre {
            pre.push(stmt);
        }
        call_args.push(mapping.call);
    }

    let ret = map_return(&sig.output)?;

    Ok(Classified {
        params,
        pre,
        call_args,
        ret_ty: ret.ty,
        is_result: ret.is_result,
    })
}

struct ParamMapping {
    wasm_ty: TokenStream,
    pre: Option<TokenStream>,
    call: TokenStream,
}

struct ReturnMapping {
    ty: Option<TokenStream>,
    is_result: bool,
}

/// Decide how a single parameter crosses the wasm boundary, or `None` if it can't.
fn map_param(ident: &syn::Ident, ty: &Type) -> Option<ParamMapping> {
    // By-value types pass straight through.
    if is_value_type(ty) {
        return Some(ParamMapping {
            wasm_ty: ty.to_token_stream(),
            pre: None,
            call: quote! { #ident },
        });
    }

    // `Option<&str>` becomes `Option<String>` with `.as_deref()` at the call.
    if let Some(inner) = option_inner(ty) {
        if is_str_ref(inner) {
            return Some(ParamMapping {
                wasm_ty: quote! { Option<String> },
                pre: None,
                call: quote! { #ident.as_deref() },
            });
        }
    }

    // References: `&str`, `&String`, and `&[..]` slices.
    if let Type::Reference(r) = ty {
        if r.mutability.is_some() {
            return None;
        }
        let elem = &*r.elem;
        if is_str_path(elem) {
            return Some(ParamMapping {
                wasm_ty: quote! { &str },
                pre: None,
                call: quote! { #ident },
            });
        }
        if ident_of(elem).as_deref() == Some("String") {
            return Some(ParamMapping {
                wasm_ty: quote! { String },
                pre: None,
                call: quote! { &#ident },
            });
        }
        if let Type::Slice(slice) = elem {
            let se = &*slice.elem;
            if is_str_ref(se) {
                let tmp = format_ident!("{}_refs", ident);
                return Some(ParamMapping {
                    wasm_ty: quote! { Vec<String> },
                    pre: Some(
                        quote! { let #tmp: Vec<&str> = #ident.iter().map(|s| s.as_str()).collect(); },
                    ),
                    call: quote! { &#tmp },
                });
            }
            if is_primitive_type(se) {
                return Some(ParamMapping {
                    wasm_ty: quote! { Vec<#se> },
                    pre: None,
                    call: quote! { &#ident },
                });
            }
            if ident_of(se).as_deref() == Some("String") {
                return Some(ParamMapping {
                    wasm_ty: quote! { Vec<String> },
                    pre: None,
                    call: quote! { &#ident },
                });
            }
        }
    }

    None
}

/// Decide how the return type crosses the wasm boundary, or `Err(reason)`.
fn map_return(output: &ReturnType) -> Result<ReturnMapping, String> {
    let ty = match output {
        ReturnType::Default => {
            return Ok(ReturnMapping {
                ty: None,
                is_result: false,
            })
        }
        ReturnType::Type(_, ty) => &**ty,
    };

    if is_unit(ty) {
        return Ok(ReturnMapping {
            ty: None,
            is_result: false,
        });
    }

    if is_value_type(ty) {
        return Ok(ReturnMapping {
            ty: Some(ty.to_token_stream()),
            is_result: false,
        });
    }

    if let Some((ok_ty, err_ty)) = result_inner(ty) {
        let ok_ok = is_value_type(ok_ty) || is_unit(ok_ty);
        let err_ok = is_str_path(err_ty)
            || is_str_ref(err_ty)
            || ident_of(err_ty).as_deref() == Some("String");
        if ok_ok && err_ok {
            let ok_tokens = if is_unit(ok_ty) {
                quote! { () }
            } else {
                ok_ty.to_token_stream()
            };
            return Ok(ReturnMapping {
                ty: Some(quote! { Result<#ok_tokens, JsValue> }),
                is_result: true,
            });
        }
    }

    Err(format!("unsupported return type `{}`", type_str(ty)))
}

fn emit_wrapper(w: &Wrapper, js_name: &str) -> TokenStream {
    let mod_idents: Vec<_> = w.call_path.iter().map(|s| format_ident!("{}", s)).collect();
    let inner = format_ident!("{}", w.fn_name);
    let stripped = w.fn_name.strip_prefix("umt_").unwrap_or(&w.fn_name);
    let wrapper_ident = format_ident!("{}_{}", w.call_path.join("_"), stripped);
    let params = &w.classified.params;
    let pre = &w.classified.pre;
    let call_args = &w.classified.call_args;
    let js_lit = proc_macro2::Literal::string(js_name);

    let call = quote! { umt_rust #(:: #mod_idents)* :: #inner(#(#call_args),*) };
    let call = if w.classified.is_result {
        quote! { #call.map_err(|e| JsValue::from_str(&e.to_string())) }
    } else {
        call
    };

    let ret = match &w.classified.ret_ty {
        Some(t) => quote! { -> #t },
        None => quote! {},
    };

    quote! {
        #[wasm_bindgen(js_name = #js_lit)]
        pub fn #wrapper_ident(#(#params),*) #ret {
            #(#pre)*
            #call
        }
    }
}

// ---- type predicates -------------------------------------------------------

const PRIMITIVES: &[&str] = &[
    "bool", "char", "i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64", "usize", "f32",
    "f64",
];

fn ident_of(ty: &Type) -> Option<String> {
    if let Type::Path(tp) = ty {
        if tp.qself.is_none() {
            return tp.path.segments.last().map(|s| s.ident.to_string());
        }
    }
    None
}

fn is_primitive_type(ty: &Type) -> bool {
    ident_of(ty).is_some_and(|id| PRIMITIVES.contains(&id.as_str()))
}

fn is_str_path(ty: &Type) -> bool {
    ident_of(ty).as_deref() == Some("str")
}

fn is_str_ref(ty: &Type) -> bool {
    if let Type::Reference(r) = ty {
        return r.mutability.is_none() && is_str_path(&r.elem);
    }
    false
}

fn is_unit(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(t) if t.elems.is_empty())
}

/// Generic args of the last path segment (e.g. the `T` in `Vec<T>`).
fn last_generic_args(ty: &Type) -> Vec<&Type> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                return ab
                    .args
                    .iter()
                    .filter_map(|a| match a {
                        GenericArgument::Type(t) => Some(t),
                        _ => None,
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

fn option_inner(ty: &Type) -> Option<&Type> {
    if ident_of(ty).as_deref() == Some("Option") {
        return last_generic_args(ty).into_iter().next();
    }
    None
}

fn result_inner(ty: &Type) -> Option<(&Type, &Type)> {
    if ident_of(ty).as_deref() == Some("Result") {
        let args = last_generic_args(ty);
        if args.len() == 2 {
            return Some((args[0], args[1]));
        }
    }
    None
}

/// A type that wasm-bindgen can move across the ABI by value: primitives,
/// `String`, `Vec<primitive|String>`, and `Option<primitive|String>`.
fn is_value_type(ty: &Type) -> bool {
    let Some(id) = ident_of(ty) else {
        return false;
    };
    if PRIMITIVES.contains(&id.as_str()) || id == "String" {
        return true;
    }
    if id == "Vec" || id == "Option" {
        if let Some(inner) = last_generic_args(ty).into_iter().next() {
            return is_primitive_type(inner) || ident_of(inner).as_deref() == Some("String");
        }
    }
    false
}

fn type_str(ty: &Type) -> String {
    ty.to_token_stream().to_string()
}

// ---- report ----------------------------------------------------------------

fn write_report(path: &Path, generated: &[(String, String, String)], skipped: &[Skipped]) {
    let mut out = String::new();
    out.push_str("# Generated wasm bindings\n\n");
    out.push_str("This file is produced by `codegen/`. Do not edit by hand.\n\n");
    out.push_str(&format!(
        "Generated {} wrappers. {} functions are not auto-exposable and are listed below for future hand-written adapters.\n\n",
        generated.len(),
        skipped.len()
    ));

    out.push_str("## Generated\n\n");
    out.push_str("| JS name | Rust source |\n|---|---|\n");
    for (module, fn_name, js_name) in generated {
        out.push_str(&format!(
            "| `{js_name}` | `umt_rust::{module}::{fn_name}` |\n"
        ));
    }

    out.push_str("\n## Skipped\n\n");
    out.push_str("| Rust source | Reason |\n|---|---|\n");
    for s in skipped {
        out.push_str(&format!(
            "| `umt_rust::{}::{}` | {} |\n",
            s.module, s.fn_name, s.reason
        ));
    }

    fs::write(path, out).expect("write report");
}
