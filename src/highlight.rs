use std::collections::HashMap;
use std::rc::Rc;

use fnv::FnvHashMap;

use crate::color::*;
use neovim_lib::Value;
use crate::ui_model::Cell;

pub struct HighlightMap {
    highlights: FnvHashMap<u64, Rc<Highlight>>,
    default_hl: Rc<Highlight>,
    pub bg_color: Color,
    pub fg_color: Color,
    sp_color: Color,

    pmenu: Rc<Highlight>,
    pmenu_sel: Rc<Highlight>,
    cursor: Rc<Highlight>,
}

impl HighlightMap {
    pub fn new() -> Self {
        let default_hl = Rc::new(Highlight::new());
        HighlightMap {
            highlights: FnvHashMap::default(),
            bg_color: COLOR_BLACK,
            fg_color: COLOR_WHITE,
            sp_color: COLOR_RED,

            pmenu: default_hl.clone(),
            pmenu_sel: default_hl.clone(),
            cursor: default_hl.clone(),

            default_hl,
        }
    }

    pub fn default_hl(&self) -> Rc<Highlight> {
        self.default_hl.clone()
    }

    pub fn set_defaults(&mut self, fg: Color, bg: Color, sp: Color) {
        self.fg_color = fg;
        self.bg_color = bg;
        self.sp_color = sp;
    }

    pub fn get(&self, idx: Option<u64>) -> Rc<Highlight> {
        idx.and_then(|idx| self.highlights.get(&idx))
            .map(Rc::clone)
            .unwrap_or_else(|| {
                self.highlights
                    .get(&0)
                    .map(Rc::clone)
                    .unwrap_or_else(|| self.default_hl.clone())
            })
    }

    pub fn set(
        &mut self,
        idx: u64,
        hl: &HashMap<String, Value>,
        info: &Vec<HashMap<String, Value>>,
    ) {
        let hl = Rc::new(Highlight::from_value_map(&hl));

        for item in info {
            match item.get("hi_name").and_then(Value::as_str) {
                Some("Pmenu") => self.pmenu = hl.clone(),
                Some("PmenuSel") => self.pmenu_sel = hl.clone(),
                Some("Cursor") => self.cursor = hl.clone(),
                _ => (),
            }
        }

        self.highlights.insert(idx, hl);
    }

    pub fn cell_fg<'a>(&'a self, cell: &'a Cell) -> Option<&'a Color> {
        if !cell.hl.reverse {
            cell.hl.foreground.as_ref()
        } else {
            cell.hl.background.as_ref().or(Some(&self.bg_color))
        }
    }

    pub fn actual_cell_fg<'a>(&'a self, cell: &'a Cell) -> &'a Color {
        if !cell.hl.reverse {
            cell.hl.foreground.as_ref().unwrap_or(&self.fg_color)
        } else {
            cell.hl.background.as_ref().unwrap_or(&self.bg_color)
        }
    }

    pub fn cell_bg<'a>(&'a self, cell: &'a Cell) -> Option<&'a Color> {
        if !cell.hl.reverse {
            cell.hl.background.as_ref()
        } else {
            cell.hl.foreground.as_ref().or(Some(&self.fg_color))
        }
    }

    #[inline]
    pub fn actual_cell_sp<'a>(&'a self, cell: &'a Cell) -> &'a Color {
        cell.hl.special.as_ref().unwrap_or(&self.sp_color)
    }

    pub fn pmenu_bg(&self) -> &Color {
        if !self.pmenu.reverse {
            self.pmenu.background.as_ref().unwrap_or(&self.bg_color)
        } else {
            self.pmenu.foreground.as_ref().unwrap_or(&self.fg_color)
        }
    }

    pub fn pmenu_fg(&self) -> &Color {
        if !self.pmenu.reverse {
            self.pmenu.foreground.as_ref().unwrap_or(&self.fg_color)
        } else {
            self.pmenu.background.as_ref().unwrap_or(&self.bg_color)
        }
    }

    pub fn pmenu_bg_sel(&self) -> &Color {
        if !self.pmenu_sel.reverse {
            self.pmenu_sel.background.as_ref().unwrap_or(&self.bg_color)
        } else {
            self.pmenu_sel.foreground.as_ref().unwrap_or(&self.fg_color)
        }
    }

    pub fn pmenu_fg_sel(&self) -> &Color {
        if !self.pmenu_sel.reverse {
            self.pmenu_sel.foreground.as_ref().unwrap_or(&self.fg_color)
        } else {
            self.pmenu_sel.background.as_ref().unwrap_or(&self.bg_color)
        }
    }

    pub fn cursor_bg(&self) -> &Color {
        if !self.cursor.reverse {
            self.cursor.background.as_ref().unwrap_or(&self.bg_color)
        } else {
            self.cursor.foreground.as_ref().unwrap_or(&self.fg_color)
        }
    }
}

#[derive(Clone)]
pub struct Highlight {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub undercurl: bool,
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub special: Option<Color>,
    pub reverse: bool,
}

impl Highlight {
    pub fn new() -> Self {
        Highlight {
            foreground: None,
            background: None,
            special: None,
            italic: false,
            bold: false,
            underline: false,
            undercurl: false,
            reverse: false,
        }
    }

    pub fn from_value_map(attrs: &HashMap<String, Value>) -> Self {
        let mut model_attrs = Highlight::new();

        for (ref key, ref val) in attrs {
            match key.as_ref() {
                "foreground" => {
                    if let Some(fg) = val.as_u64() {
                        model_attrs.foreground = Some(Color::from_indexed_color(fg));
                    }
                }
                "background" => {
                    if let Some(bg) = val.as_u64() {
                        model_attrs.background = Some(Color::from_indexed_color(bg));
                    }
                }
                "special" => {
                    if let Some(bg) = val.as_u64() {
                        model_attrs.special = Some(Color::from_indexed_color(bg));
                    }
                }
                "reverse" => model_attrs.reverse = true,
                "bold" => model_attrs.bold = true,
                "italic" => model_attrs.italic = true,
                "underline" => model_attrs.underline = true,
                "undercurl" => model_attrs.undercurl = true,
                attr_key => error!("unknown attribute {}", attr_key),
            };
        }

        model_attrs
    }
}
