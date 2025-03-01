use super::CssData;
use crate::css::{
    AtRule, AtRuleBodyItem, Comment, CssString, Import, Item, Property, Rule,
    Selectors, Value,
};
use crate::Invalid;

type Result<T = ()> = std::result::Result<T, Invalid>;

pub trait CssDestination {
    fn head(&mut self) -> &mut CssData;

    fn start_rule(&mut self, selectors: Selectors) -> Result<RuleDest>;
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest;
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest>;

    fn push_import(&mut self, import: Import);
    fn push_comment(&mut self, c: Comment);
    fn push_item(&mut self, item: Item) -> Result;
    fn push_property(&mut self, name: String, value: Value) -> Result;
    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result;

    /// Nop in default implementation, adds a spacer in CssHead.
    fn separate(&mut self) {}
}

pub struct RuleDest<'a> {
    parent: &'a mut dyn CssDestination,
    rule: Rule,
    trail: Vec<Item>,
}

impl<'a> RuleDest<'a> {
    pub fn new(
        parent: &'a mut dyn CssDestination,
        selectors: Selectors,
    ) -> Self {
        RuleDest {
            parent,
            rule: Rule::new(selectors),
            trail: Default::default(),
        }
    }
}
impl<'a> Drop for RuleDest<'a> {
    fn drop(&mut self) {
        fn end(dest: &mut RuleDest) -> Result<()> {
            dest.parent.push_item(dest.rule.clone().into())?;
            let mut t = Vec::new();
            std::mem::swap(&mut dest.trail, &mut t);
            for item in t {
                dest.parent.push_item(item)?;
            }
            dest.parent.separate();
            Ok(())
        }
        if let Err(err) = end(self) {
            eprintln!("Error in ending RuleDest: {}", err);
        }
    }
}

impl<'a> CssDestination for RuleDest<'a> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }
    fn start_rule(&mut self, selectors: Selectors) -> Result<RuleDest> {
        Ok(RuleDest::new(self, selectors))
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest {
        let selectors = self.rule.selectors.clone();
        AtRuleDest {
            parent: self,
            name,
            args,
            rule: Some(Rule::new(selectors)),
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.rule.push(import.into());
    }

    fn push_comment(&mut self, c: Comment) {
        self.rule.push(c.into())
    }

    fn push_item(&mut self, item: Item) -> Result {
        match item {
            Item::AtRule(r) => match r.try_into() {
                Ok(item) => self.rule.push(item),
                Err(r) => self.trail.push(r.into()),
            },
            item => self.trail.push(item),
        }
        Ok(())
    }

    fn push_property(&mut self, name: String, value: Value) -> Result {
        self.rule.push(Property::new(name, value).into());
        Ok(())
    }

    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result {
        self.rule
            .push(crate::css::BodyItem::CustomProperty(name, value));
        Ok(())
    }
}

pub struct NsRuleDest<'a> {
    parent: &'a mut dyn CssDestination,
    name: String,
}

impl<'a> CssDestination for NsRuleDest<'a> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }
    fn start_rule(&mut self, _selectors: Selectors) -> Result<RuleDest> {
        Err(Invalid::InNsRule)
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest {
        AtRuleDest {
            parent: self,
            name,
            args,
            rule: None,
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.parent.push_import(import)
    }
    fn push_comment(&mut self, c: Comment) {
        self.parent.push_comment(c)
    }
    fn push_item(&mut self, _item: Item) -> Result {
        Err(Invalid::InNsRule)
    }
    fn push_property(&mut self, name: String, value: Value) -> Result {
        self.parent
            .push_property(format!("{}-{}", self.name, name), value)
    }
    fn push_custom_property(&mut self, _: String, _: CssString) -> Result {
        Err(Invalid::InNsRule)
    }
}

pub struct AtRuleDest<'a> {
    parent: &'a mut dyn CssDestination,
    name: String,
    args: Value,
    rule: Option<Rule>,
    body: Vec<AtRuleBodyItem>,
}
impl<'a> AtRuleDest<'a> {
    pub fn new(
        parent: &'a mut dyn CssDestination,
        name: String,
        args: Value,
    ) -> Self {
        AtRuleDest {
            parent,
            name,
            args,
            rule: None,
            body: Vec::new(),
        }
    }
}

impl<'a> Drop for AtRuleDest<'a> {
    fn drop(&mut self) {
        let mut body = Vec::new();
        std::mem::swap(&mut self.body, &mut body);
        let mut name = String::new();
        std::mem::swap(&mut self.name, &mut name);
        let mut args = Value::Null;
        std::mem::swap(&mut self.args, &mut args);
        if let Some(rule) = &self.rule {
            body.insert(0, rule.clone().into());
        }
        let result = AtRule::new(name, args, Some(body));
        if let Err(err) = self.parent.push_item(result.into()) {
            eprintln!("Error ending AtRuleDest: {}", err);
        }
        self.parent.separate();
    }
}
impl<'a> CssDestination for AtRuleDest<'a> {
    fn head(&mut self) -> &mut CssData {
        self.parent.head()
    }
    fn start_rule(&mut self, selectors: Selectors) -> Result<RuleDest> {
        Ok(RuleDest::new(self, selectors))
    }
    fn start_atrule(&mut self, name: String, args: Value) -> AtRuleDest {
        let rule = self.rule.as_ref().map(|r| Rule::new(r.selectors.clone()));
        AtRuleDest {
            parent: self,
            name,
            args,
            rule,
            body: Vec::new(),
        }
    }
    fn start_nsrule(&mut self, name: String) -> Result<NsRuleDest> {
        Ok(NsRuleDest { parent: self, name })
    }

    fn push_import(&mut self, import: Import) {
        self.body.push(import.into());
    }

    fn push_comment(&mut self, c: Comment) {
        if let Some(rule) = &mut self.rule {
            rule.push(c.into());
        } else {
            self.body.push(c.into());
        }
    }

    fn push_item(&mut self, item: Item) -> Result {
        self.body.push(match item {
            Item::Comment(c) => c.into(),
            Item::Import(i) => i.into(),
            Item::Rule(r) => r.into(),
            Item::AtRule(r) => r.into(),
            Item::Separator => return Ok(()), // Not pushed?
        });
        Ok(())
    }

    fn push_property(&mut self, name: String, value: Value) -> Result {
        let prop = Property::new(name, value);
        if let Some(rule) = &mut self.rule {
            rule.push(prop.into());
        } else {
            self.body.push(prop.into());
        }
        Ok(())
    }

    fn push_custom_property(
        &mut self,
        name: String,
        value: CssString,
    ) -> Result {
        if let Some(rule) = &mut self.rule {
            rule.push(crate::css::BodyItem::CustomProperty(name, value));
            Ok(())
        } else {
            Err(Invalid::GlobalCustomProperty)
        }
    }
}
