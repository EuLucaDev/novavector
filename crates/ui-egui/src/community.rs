//! Community and project links: the Suporte button (header, About, Home) and the link list.
//! Icons are Lucide's (ISC); Suporte's own logo is a trademark, so the button uses a chat icon
//! on Suporte's colour with the word "Suporte".

use egui::{Color32, CornerRadius, Sense, Ui, vec2};

use crate::theme::{self, Tokens};
use crate::{NovavectorApp, icons};

/// Suporte's brand colour (a colour, not a logo).
const DISCORD: Color32 = Color32::from_rgb(0x58, 0x65, 0xF2);

/// The Suporte button's label, height and width.
fn discord_layout(ui: &Ui, large: bool) -> (std::sync::Arc<egui::Galley>, f32, f32) {
    let (h, font, label) = if large { (36.0, theme::semibold(14.0), tl!("Get support")) } else { (24.0, theme::semibold(12.0), "Suporte") };
    let galley = ui.painter().layout_no_wrap(label.to_string(), font, Color32::WHITE);
    let w = galley.size().x + h * 0.55 + h * 0.9;
    (galley, h, w)
}

/// The Suporte button's width (the app bar drops it when space runs out).
pub fn discord_width(ui: &Ui, large: bool) -> f32 {
    discord_layout(ui, large).2
}

/// A Suporte pill button: compact for the header, large for About / Home.
pub fn discord_button(app: &mut NovavectorApp, ui: &mut Ui, large: bool) -> egui::Response {
    let (galley, h, w) = discord_layout(ui, large);
    let icon = h * 0.55;
    let (r, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    let fill = if resp.hovered() { DISCORD.gamma_multiply(0.85) } else { DISCORD };
    ui.painter().rect_filled(r, CornerRadius::same((h / 2.0) as u8), fill);
    let ir = egui::Rect::from_center_size(r.left_center() + vec2(h * 0.35 + icon / 2.0, 0.0), vec2(icon, icon));
    icons::paint(ui, "message-circle", ir, Color32::WHITE);
    ui.painter().galley(egui::pos2(ir.right() + h * 0.2, r.center().y - galley.size().y / 2.0), galley, Color32::WHITE);
    let resp = resp.on_hover_text(tl!("Talk to the Nova Foods IT team (workflowind.agcunae.com/inbox)"));
    if resp.clicked() {
        app.open_link("help.discord");
    }
    resp
}

/// One link row: icon, label, opens the command's URL.
fn link(app: &mut NovavectorApp, ui: &mut Ui, icon: &str, label: &str, cmd: &str, url: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        icons::icon(ui, icon, 16.0, t.icon);
        let r = ui.add(egui::Button::new(egui::RichText::new(label).color(t.accent).size(13.0)).frame(false)).on_hover_text(url);
        if r.clicked() {
            app.open_link(cmd);
        }
    });
}

/// The Suporte button and the website / app page / GitHub links.
pub fn links(app: &mut NovavectorApp, ui: &mut Ui) {
    discord_button(app, ui, true);
    ui.add_space(8.0);
    let l = app.session.execute("help.links", &serde_json::json!({})).unwrap_or_default();
    let s = |k: &str| l[k].as_str().unwrap_or("").to_string();
    link(app, ui, "globe", tl!("Nova Foods website"), "help.website", &s("website"));
    link(app, ui, "external-link", tl!("Nova Vector on workflowind.agcunae.com/apps"), "help.appPage", &s("appPage"));
    link(app, ui, "git-branch", tl!("Source code on GitHub"), "help.github", &s("github"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_clicks_open_the_right_urls() {
        use std::sync::{Arc, Mutex};
        let opened = Arc::new(Mutex::new(vec![]));
        let o = opened.clone();
        let services = crate::Services { open_url: Some(Box::new(move |u: &str| o.lock().unwrap().push(u.to_string()))), ..Default::default() };
        let mut app = NovavectorApp::new(novavector_engine::Session::new(), services);
        for id in ["help.discord", "help.website", "help.appPage", "help.github"] {
            crate::menus::invoke(&mut app, id, serde_json::json!({}));
        }
        assert_eq!(
            *opened.lock().unwrap(),
            [
                "https://workflowind.agcunae.com/inbox",
                "https://workflowind.agcunae.com/apps",
                "https://workflowind.agcunae.com/apps/novavector",
                "https://github.com/EuLucaDev/novavector"
            ]
        );
        // The control channel / MCP path returns the URL without opening a browser.
        let v = app.run("help.discord", serde_json::json!({})).unwrap();
        assert_eq!(v["url"], "https://workflowind.agcunae.com/inbox");
        assert_eq!(opened.lock().unwrap().len(), 4);
        let ctx = egui::Context::default();
        theme::install_fonts(&ctx);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| links(&mut app, ui));
        out.textures_delta.clear();
    }
}
