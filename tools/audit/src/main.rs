use quote::quote;
use std::{collections::BTreeMap, collections::BTreeSet, env, fs, path::Path};
use syn::{Block, Item};
fn block<'a>(items: &'a mut [Item], node: &[usize]) -> &'a mut Block {
    let (index, rest) = node.split_first().unwrap();
    match &mut items[*index] {
        Item::Fn(f) if rest.is_empty() => &mut f.block,
        Item::Impl(i) if rest.len() == 1 => match &mut i.items[rest[0]] {
            syn::ImplItem::Fn(f) => &mut f.block,
            _ => panic!("planned method absent"),
        },
        Item::Mod(m) => block(&mut m.content.as_mut().unwrap().1, rest),
        _ => panic!("planned target absent"),
    }
}
/// The fn a planned injection lands in, by name, so the final comparison can
/// leave the kinds this audit does not reconstruct (RETURN, OVERWRITE, …) to
/// the compiler and the tests, and compare everything else with pristine.
fn fn_name(items: &[Item], node: &[usize]) -> String {
    match &items[node[0]] {
        Item::Fn(f) => f.sig.ident.to_string(),
        Item::Impl(i) => match &i.items[node[1]] {
            syn::ImplItem::Fn(f) => f.sig.ident.to_string(),
            _ => panic!("planned method absent"),
        },
        _ => panic!("planned target absent"),
    }
}
fn canonical(items: &[Item]) -> String {
    prettyplease::unparse(&syn::File {
        shebang: None,
        attrs: vec![],
        items: items.to_vec(),
    })
}
/// Strips the named fns wherever they sit, so both trees pair up.
fn strip(items: Vec<Item>, skip: &BTreeSet<String>) -> Vec<Item> {
    items
        .into_iter()
        .filter_map(|item| match item {
            Item::Fn(f) if skip.contains(&f.sig.ident.to_string()) => None,
            Item::Impl(mut i) => {
                i.items.retain(|impl_item| match impl_item {
                    syn::ImplItem::Fn(f) => !skip.contains(&f.sig.ident.to_string()),
                    _ => true,
                });
                Some(Item::Impl(i))
            }
            Item::Mod(mut m) => {
                if let Some((_, inner)) = &mut m.content {
                    *inner = strip(std::mem::take(inner), skip);
                }
                Some(Item::Mod(m))
            }
            other => Some(other),
        })
        .collect()
}
fn main() {
    let args: Vec<_> = env::args().collect();
    let plan: serde_json::Value = serde_json::from_slice(&fs::read(&args[3]).unwrap()).unwrap();
    let mut trees = BTreeMap::new();
    let mut skipped = BTreeMap::<String, BTreeSet<String>>::new();
    for p in plan["injections"].as_array().unwrap() {
        let file = p["file"].as_str().unwrap().to_owned();
        let kind = p["kind"].as_str().unwrap().to_owned();
        let node: Vec<usize> = p["node"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_u64().unwrap() as usize)
            .collect();
        let helper: syn::ItemFn = syn::parse_str(p["helper_source"].as_str().unwrap()).unwrap();
        let helper_name = helper.sig.ident.to_string();
        let tree = trees.entry(file.clone()).or_insert_with(|| {
            syn::parse_file(&fs::read_to_string(Path::new(&args[2]).join(&file)).unwrap()).unwrap()
        });
        let fn_id = fn_name(&tree.items, &node);
        let b = block(&mut tree.items, &node);
        if kind == "HEAD" {
            assert_eq!(p["cancellable"], false);
            let name = &helper.sig.ident;
            let args: Vec<syn::Expr> = p["bindings"]
                .as_array()
                .unwrap()
                .iter()
                .map(|b| syn::parse_str(b["value"].as_str().unwrap()).unwrap())
                .collect();
            let expected: syn::Stmt = syn::parse2(quote!({#helper #name(#(#args),*)};)).unwrap();
            let actual = b.stmts.first().expect("HEAD missing");
            let actual_fn: Item = syn::parse_quote!(fn audit() {#actual});
            let expected_fn: Item = syn::parse_quote!(fn audit() {#expected});
            assert_eq!(
                canonical(&[actual_fn]),
                canonical(&[expected_fn]),
                "HEAD differs from planned helper/bindings"
            );
            b.stmts.remove(0);
        } else {
            // RETURN, OVERWRITE and the other body kinds: the compiler
            // checked them; here the helper's name must appear in the
            // target's body, and the fn leaves the pristine comparison.
            let actual_fn: Item = syn::parse_quote!(fn audit() #b);
            assert!(
                canonical(&[actual_fn]).contains(&helper_name),
                "planned helper absent from its fn"
            );
            skipped.entry(file).or_default().insert(fn_id);
        }
    }
    for m in plan["modules"].as_array().unwrap() {
        let file = m["file"].as_str().unwrap();
        let tree = trees.entry(file.to_owned()).or_insert_with(|| {
            syn::parse_file(&fs::read_to_string(Path::new(&args[2]).join(file)).unwrap()).unwrap()
        });
        let index = tree
            .items
            .iter()
            .position(|i| matches!(i,Item::Mod(v) if v.ident==m["name"].as_str().unwrap()))
            .expect("support module missing");
        let Item::Mod(module) = tree.items.remove(index) else {
            unreachable!()
        };
        let payload = syn::parse_file(m["source"].as_str().unwrap()).unwrap();
        assert!(module.attrs.is_empty());
        let public = matches!(&module.vis, syn::Visibility::Public(_))
            || matches!(&module.vis, syn::Visibility::Restricted(v) if v.path.is_ident("crate"));
        assert!(public, "support module visibility");
        let mut expected = module.clone();
        expected.content.as_mut().unwrap().1 = payload.items;
        assert!(
            canonical(&[Item::Mod(module)]) == canonical(&[Item::Mod(expected)]),
            "support module differs from planned source"
        );
    }
    for (file, tree) in trees {
        let skip = skipped.remove(&file).unwrap_or_default();
        let mut staged = tree.clone();
        staged.items = strip(staged.items, &skip);
        let mut original =
            syn::parse_file(&fs::read_to_string(Path::new(&args[1]).join(&file)).unwrap()).unwrap();
        original.items = strip(original.items, &skip);
        assert!(
            prettyplease::unparse(&original) == prettyplease::unparse(&staged),
            "unexpected source change in {file}"
        );
    }
    println!("all original ASTs preserved apart from the planned additions and support modules");
}
