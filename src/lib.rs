// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-as — AppStream software metadata catalog.
//!
//! Models the AppStream component database: software components
//! (desktop apps, runtimes, addons, fonts) with ids, names, summaries,
//! categories, icons, launch commands, versions, licenses. Supports
//! search, category browsing, and cache export — the metadata layer a
//! software center / launcher queries. JSON persistence.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ComponentKind { DesktopApp, Runtime, Addon, Font, Service, Driver }
impl ComponentKind {
    pub fn label(self) -> &'static str {
        match self { Self::DesktopApp=>"desktop-app", Self::Runtime=>"runtime", Self::Addon=>"addon", Self::Font=>"font", Self::Service=>"service", Self::Driver=>"driver" }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s { "desktop-app"=>Some(Self::DesktopApp), "runtime"=>Some(Self::Runtime),
            "addon"=>Some(Self::Addon), "font"=>Some(Self::Font),
            "service"=>Some(Self::Service), "driver"=>Some(Self::Driver), _=>None }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Component {
    pub id: String,               // e.g. "org.sigmaos.Terminal"
    pub kind: ComponentKind,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub categories: Vec<String>,
    pub icon: Option<String>,
    pub launch: Option<String>,   // desktop-entry Exec or command
    pub version: String,
    pub license: String,
    pub developer: String,
    pub keywords: Vec<String>,
    pub installed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AsError {
    Duplicate(String),
    NoSuchComponent(String),
    BadKind(String),
    EmptyId,
}
impl fmt::Display for AsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duplicate(id)=>write!(f,"duplicate component {id}"),
            Self::NoSuchComponent(id)=>write!(f,"no component {id}"),
            Self::BadKind(k)=>write!(f,"bad component kind {k}"),
            Self::EmptyId=>write!(f,"empty component id"),
        }
    }
}
impl std::error::Error for AsError {}

pub struct SigmaOSAppStream {
    components: BTreeMap<String, Component>,
}

impl SigmaOSAppStream {
    pub fn new() -> Self {
        let mut m = Self { components: BTreeMap::new() };
        m.seed();
        m
    }

    fn seed(&mut self) {
        let defs = vec![
            ("org.sigmaos.Terminal", ComponentKind::DesktopApp, "Terminal", "Wayland terminal emulator", "Fast GPU-accelerated terminal", vec!["System","Terminal"], Some("utilities-terminal".into()), Some("foot".into()), "1.0", "GPL-3.0", "SigmaOS", vec!["terminal","shell","console"], true),
            ("org.sigmaos.Files", ComponentKind::DesktopApp, "Files", "File manager", "Browse and manage files", vec!["System","FileTools"], Some("system-file-manager".into()), Some("sigmaos-fm".into()), "1.0", "GPL-3.0", "SigmaOS", vec!["files","manager","browse"], true),
            ("org.sigmaos.TextEditor", ComponentKind::DesktopApp, "Text Editor", "Code and text editor", "Edit text and code files", vec!["TextEditor","Development"], Some("accessories-text-editor".into()), Some("nvim".into()), "1.0", "Apache-2.0", "SigmaOS", vec!["editor","text","code"], true),
            ("org.sigmaos.Browser", ComponentKind::DesktopApp, "Browser", "Web browser", "Browse the web", vec!["Network","WebBrowser"], Some("web-browser".into()), Some("chromium".into()), "1.0", "BSD-3-Clause", "SigmaOS", vec!["browser","web","internet"], true),
            ("org.sigmaos.Calculator", ComponentKind::DesktopApp, "Calculator", "Desktop calculator", "Perform calculations", vec!["Utility"], Some("accessories-calculator".into()), Some("sigmaos-calc".into()), "1.0", "GPL-3.0", "SigmaOS", vec!["calc","math"], true),
            ("org.sigmaos.Player", ComponentKind::DesktopApp, "Media Player", "Audio/video player", "Play media files", vec!["AudioVideo","Player"], Some("multimedia-player".into()), Some("sigmaos-player".into()), "1.0", "GPL-3.0", "SigmaOS", vec!["player","media","audio","video"], true),
            ("org.sigmaos.Platform.Gtk", ComponentKind::Runtime, "SigmaOS GTK Platform", "GTK runtime libraries", "Shared libraries for GTK apps", vec!["Runtime"], None, None, "4", "LGPL-2.1", "SigmaOS", vec!["runtime","gtk"], false),
            ("org.sigmaos.Platform.Qt", ComponentKind::Runtime, "SigmaOS Qt Platform", "Qt runtime libraries", "Shared libraries for Qt apps", vec!["Runtime"], None, None, "6", "LGPL-3.0", "SigmaOS", vec!["runtime","qt"], false),
            ("org.sigmaos.Platform.Base", ComponentKind::Runtime, "SigmaOS Base Platform", "Base runtime", "Base runtime for sandboxed apps", vec!["Runtime"], None, None, "1", "MIT", "SigmaOS", vec!["runtime","base"], false),
            ("org.sigmaos.AudioCodec", ComponentKind::Addon, "SigmaOS Audio Codec", "Audio codec addon", "Additional audio codecs", vec!["Addon"], None, None, "1.0", "MIT", "SigmaOS", vec!["codec","audio"], false),
            ("org.sigmaos.IconTheme", ComponentKind::Font, "SigmaOS Icon Theme", "Icon theme", "Desktop icon theme", vec!["IconTheme"], None, None, "1.0", "GPL-3.0", "SigmaOS", vec!["icons","theme"], true),
            ("org.sigmaos.NetService", ComponentKind::Service, "SigmaOS Network Service", "Networking daemon", "Network management service", vec!["System","Service"], None, None, "1.0", "GPL-3.0", "SigmaOS", vec!["network","service","daemon"], true),
            ("org.sigmaos.VideoDriver", ComponentKind::Driver, "SigmaOS Video Driver", "GPU driver", "Kernel video driver", vec!["Hardware","Driver"], None, None, "1.0", "MIT", "SigmaOS", vec!["driver","gpu","video"], false),
        ];
        for (id, kind, name, summary, desc, cats, icon, launch, ver, lic, dev, kw, inst) in defs {
            self.components.insert(id.into(), Component {
                id: id.into(), kind, name: name.into(), summary: summary.into(),
                description: desc.into(), categories: cats.into_iter().map(String::from).collect(),
                icon, launch, version: ver.into(), license: lic.into(),
                developer: dev.into(), keywords: kw.into_iter().map(String::from).collect(),
                installed: inst,
            });
        }
    }

    pub fn component(&self, id: &str) -> Option<&Component> { self.components.get(id) }
    pub fn components(&self) -> impl Iterator<Item = &Component> { self.components.values() }
    pub fn len(&self) -> usize { self.components.len() }

    pub fn of_kind(&self, kind: ComponentKind) -> Vec<&Component> {
        self.components.values().filter(|c| c.kind == kind).collect()
    }
    pub fn installed(&self) -> Vec<&Component> {
        self.components.values().filter(|c| c.installed).collect()
    }
    pub fn available(&self) -> Vec<&Component> {
        self.components.values().filter(|c| !c.installed).collect()
    }

    pub fn categories(&self) -> Vec<String> {
        let mut cats = std::collections::BTreeSet::new();
        for c in self.components.values() { for cat in &c.categories { cats.insert(cat.clone()); } }
        cats.into_iter().collect()
    }

    pub fn in_category(&self, cat: &str) -> Vec<&Component> {
        self.components.values().filter(|c| c.categories.iter().any(|x| x == cat)).collect()
    }

    // ---- search ----
    /// Weighted search: id > name > keywords > summary > description.
    pub fn search(&self, query: &str) -> Vec<(&Component, u32)> {
        let q = query.to_lowercase();
        let terms: Vec<&str> = q.split_whitespace().collect();
        if terms.is_empty() { return Vec::new(); }
        let mut scored: Vec<(&Component, u32)> = self.components.values().filter_map(|c| {
            let mut score = 0u32;
            for t in &terms {
                if c.id.to_lowercase().contains(t) { score += 10; }
                if c.name.to_lowercase().contains(t) { score += 8; }
                if c.keywords.iter().any(|k| k.contains(t)) { score += 6; }
                if c.summary.to_lowercase().contains(t) { score += 4; }
                if c.description.to_lowercase().contains(t) { score += 2; }
                if c.categories.iter().any(|cat| cat.to_lowercase().contains(t)) { score += 1; }
            }
            if score > 0 { Some((c, score)) } else { None }
        }).collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.name.cmp(&b.0.name)));
        scored
    }

    /// Launchable components (have a launch command).
    pub fn launchable(&self) -> Vec<&Component> {
        self.components.values().filter(|c| c.launch.is_some() && c.installed).collect()
    }

    // ---- mutation ----
    pub fn add(&mut self, c: Component) -> Result<(), AsError> {
        if c.id.is_empty() { return Err(AsError::EmptyId); }
        if self.components.contains_key(&c.id) { return Err(AsError::Duplicate(c.id.clone())); }
        self.components.insert(c.id.clone(), c); Ok(())
    }

    pub fn mark_installed(&mut self, id: &str, on: bool) -> Result<(), AsError> {
        let c = self.components.get_mut(id).ok_or_else(|| AsError::NoSuchComponent(id.into()))?;
        c.installed = on; Ok(())
    }

    /// Export the catalog as AppStream-ish XML collection.
    pub fn render_xml(&self) -> String {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<components>\n");
        for c in self.components.values() {
            out.push_str(&format!("  <component type=\"{}\">\n", c.kind.label()));
            out.push_str(&format!("    <id>{}</id>\n", c.id));
            out.push_str(&format!("    <name>{}</name>\n", c.name));
            out.push_str(&format!("    <summary>{}</summary>\n", c.summary));
            out.push_str(&format!("    <version>{}</version>\n", c.version));
            out.push_str(&format!("    <developer><name>{}</name></developer>\n", c.developer));
            out.push_str(&format!("    <launchable type=\"desktop-id\">{}</launchable>\n", c.id));
            out.push_str("  </component>\n");
        }
        out.push_str("</components>\n");
        out
    }

    /// Render a desktop-entry-style listing for launchers.
    pub fn render_desktop_entries(&self) -> String {
        let mut out = String::new();
        for c in self.launchable() {
            if let Some(cmd) = &c.launch {
                out.push_str(&format!("[Desktop Entry]\nType=Application\nName={}\nExec={}\nIcon={}\nCategories={}\n\n",
                    c.name, cmd, c.icon.as_deref().unwrap_or(""), c.categories.join(";") + ";"));
            }
        }
        out
    }

    /// Persist the catalog (incl. install flags) to JSON.
    pub fn save_to(&self, path: &std::path::Path) -> Result<(), String> {
        let v: Vec<&Component> = self.components.values().collect();
        let owned: Vec<Component> = v.into_iter().cloned().collect();
        let j = serde_json::to_string_pretty(&owned).map_err(|e| e.to_string())?;
        if let Some(p) = path.parent() { std::fs::create_dir_all(p).map_err(|e| e.to_string())?; }
        std::fs::write(path, j).map_err(|e| e.to_string())
    }

    pub fn load_from(path: &std::path::Path) -> Result<Self, String> {
        let txt = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let items: Vec<Component> = serde_json::from_str(&txt).map_err(|e| e.to_string())?;
        let mut m = Self::new();
        m.components.clear();
        for c in items { m.components.insert(c.id.clone(), c); }
        Ok(m)
    }

    pub fn state_path() -> std::path::PathBuf {
        if let Ok(p) = std::env::var("SIGMAOS_APPSTREAM_FILE") { return std::path::PathBuf::from(p); }
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        std::path::PathBuf::from(home).join(".local/share/sigmaos/appstream.json")
    }

    pub fn status_line(&self) -> String {
        format!("appstream: {} components ({} installed, {} launchable), {} categories",
            self.len(), self.installed().len(), self.launchable().len(), self.categories().len())
    }
}

impl Default for SigmaOSAppStream { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_catalog() {
        let m = SigmaOSAppStream::new();
        assert!(m.len() >= 12);
        assert!(m.component("org.sigmaos.Terminal").is_some());
        assert!(m.installed().len() > 0);
        assert!(m.available().len() > 0);
        assert!(m.categories().contains(&"System".to_string()));
    }

    #[test]
    fn kind_filter() {
        let m = SigmaOSAppStream::new();
        assert!(m.of_kind(ComponentKind::Runtime).len() >= 3);
        assert!(m.of_kind(ComponentKind::DesktopApp).len() >= 5);
        assert!(m.of_kind(ComponentKind::Driver).len() == 1);
    }

    #[test]
    fn search_ranking() {
        let m = SigmaOSAppStream::new();
        let r = m.search("terminal");
        assert!(!r.is_empty());
        assert_eq!(r[0].0.name, "Terminal"); // name match outranks keyword matches
        let r2 = m.search("codec audio");
        assert!(r2.iter().any(|(c,_)| c.id == "org.sigmaos.AudioCodec"));
        assert!(m.search("").is_empty());
        assert!(m.search("zzzznothing").is_empty());
    }

    #[test]
    fn category_browse() {
        let m = SigmaOSAppStream::new();
        let sys = m.in_category("System");
        assert!(sys.iter().any(|c| c.name == "Terminal"));
        assert!(sys.iter().any(|c| c.kind == ComponentKind::Service));
    }

    #[test]
    fn add_and_install() {
        let mut m = SigmaOSAppStream::new();
        let c = Component { id: "org.test.App".into(), kind: ComponentKind::DesktopApp,
            name: "Test App".into(), summary: "s".into(), description: "d".into(),
            categories: vec!["Utility".into()], icon: None, launch: Some("test".into()),
            version: "1".into(), license: "MIT".into(), developer: "T".into(),
            keywords: vec![], installed: false };
        m.add(c).unwrap();
        assert!(m.add(Component { id: "org.test.App".into(), kind: ComponentKind::Addon,
            name: "x".into(), summary: String::new(), description: String::new(), categories: vec![],
            icon: None, launch: None, version: String::new(), license: String::new(),
            developer: String::new(), keywords: vec![], installed: false }).is_err());
        m.mark_installed("org.test.App", true).unwrap();
        assert!(m.component("org.test.App").unwrap().installed);
        assert!(m.launchable().iter().any(|c| c.id == "org.test.App"));
        assert!(m.mark_installed("nope", true).is_err());
    }

    #[test]
    fn render_outputs() {
        let m = SigmaOSAppStream::new();
        let xml = m.render_xml();
        assert!(xml.contains("<components>"));
        assert!(xml.contains("org.sigmaos.Terminal"));
        assert!(xml.contains("desktop-app"));
        let de = m.render_desktop_entries();
        assert!(de.contains("[Desktop Entry]"));
        assert!(de.contains("Exec=foot"));
        assert!(de.contains("Categories=System;Terminal;"));
    }
}
