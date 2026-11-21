//! SigmaOS-as — AppStream catalog CLI.
//!
//! Usage:
//!   sigmaos-as status|list|kinds|categories
//!   sigmaos-as search <query...>
//!   sigmaos-as category <name>
//!   sigmaos-as show <id>
//!   sigmaos-as install|remove <id>
//!   sigmaos-as launchable
//!   sigmaos-as add <id> <kind> <name> [summary] [launch]
//!   sigmaos-as xml|desktop-entries            render outputs

use sigmaos_as::{Component, ComponentKind, SigmaOSAppStream};
use std::process::exit;

fn usage() -> ! {
    eprintln!("usage: sigmaos-as <status|list|kinds|categories|search|category|show|install|remove|launchable|add|xml|desktop-entries> [args]");
    exit(2);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() { usage(); }
    let state = SigmaOSAppStream::state_path();
    let mut m = SigmaOSAppStream::load_from(&state).unwrap_or_else(|_| SigmaOSAppStream::new());
    let cmd = args[0].as_str();
    let rest = &args[1..];
    let arg = |i: usize| -> &str { rest.get(i).map(|s| s.as_str()).unwrap_or_else(|| usage()) };

    let result: Result<(), String> = (|| -> Result<(), String> { match cmd {
        "status" => { println!("{}", m.status_line()); Ok(()) }
        "list" => {
            for c in m.components() {
                println!("{} {:<12} {:<28} {}", if c.installed {"*"} else {" "}, c.kind.label(), c.id, c.name);
            }
            Ok(())
        }
        "kinds" => {
            for k in [ComponentKind::DesktopApp, ComponentKind::Runtime, ComponentKind::Addon, ComponentKind::Font, ComponentKind::Service, ComponentKind::Driver] {
                println!("{:<12} {}", k.label(), m.of_kind(k).len());
            }
            Ok(())
        }
        "categories" => { for c in m.categories() { println!("{}", c); } Ok(()) }
        "search" => {
            let q = rest.join(" ");
            for (c, score) in m.search(&q) {
                println!("{:>3}  {:<12} {:<28} {}", score, c.kind.label(), c.id, c.name);
            }
            Ok(())
        }
        "category" => {
            for c in m.in_category(arg(0)) { println!("{} {:<28} {}", if c.installed {"*"} else {" "}, c.id, c.name); }
            Ok(())
        }
        "show" => {
            let c = m.component(arg(0)).ok_or_else(|| format!("no component {}", arg(0)))?;
            println!("id:          {}", c.id);
            println!("kind:        {}", c.kind.label());
            println!("name:        {}", c.name);
            println!("summary:     {}", c.summary);
            println!("description: {}", c.description);
            println!("version:     {}", c.version);
            println!("license:     {}", c.license);
            println!("developer:   {}", c.developer);
            println!("categories:  {}", c.categories.join(", "));
            println!("keywords:    {}", c.keywords.join(", "));
            println!("launch:      {}", c.launch.as_deref().unwrap_or("-"));
            println!("installed:   {}", c.installed);
            Ok(())
        }
        "install" | "remove" => {
            m.mark_installed(arg(0), cmd == "install").map_err(|e| e.to_string())?;
            println!("{} {}", if cmd=="install" {"installed"} else {"removed"}, arg(0)); Ok(())
        }
        "launchable" => {
            for c in m.launchable() { println!("{} -> {}", c.name, c.launch.as_deref().unwrap_or("")); }
            Ok(())
        }
        "add" => {
            let kind = ComponentKind::parse(arg(1)).ok_or_else(|| format!("bad kind {}", arg(1)))?;
            let c = Component { id: arg(0).into(), kind, name: arg(2).into(),
                summary: rest.get(3).cloned().unwrap_or_default(),
                description: String::new(), categories: vec![],
                icon: None, launch: rest.get(4).cloned(),
                version: "1.0".into(), license: "MIT".into(),
                developer: "SigmaOS".into(), keywords: vec![], installed: false };
            m.add(c).map_err(|e| e.to_string())?; println!("added {}", arg(0)); Ok(())
        }
        "xml" => { print!("{}", m.render_xml()); Ok(()) }
        "desktop-entries" => { print!("{}", m.render_desktop_entries()); Ok(()) }
        _ => usage(),
    } })();

    match result {
        Ok(()) => { if let Err(e) = m.save_to(&state) { eprintln!("sigmaos-as: warn: save: {e}"); } }
        Err(e) => { eprintln!("sigmaos-as: {e}"); exit(1); }
    }
}
