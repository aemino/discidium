use std::collections::HashMap;

use hyper::Method;

use super::RequestBuilder;

#[derive(Debug, Clone)]
pub enum PathComponent {
    Const(String),
    Variant(String),
}

#[derive(Debug, Clone)]
pub struct Route {
    path: Vec<PathComponent>,
    variant_map: HashMap<String, String>,
}

// TODO: It would be nice to make some of these methods `const fn`s once they are more stabilized.

impl Route {
    pub fn from_static(src: &'static str) -> Self {
        Self {
            path: Self::parse_path(src).collect(),
            variant_map: Default::default(),
        }
    }

    fn parse_path(src: &'static str) -> impl Iterator<Item = PathComponent> {
        src.split('/')
            .filter(|comp| !comp.is_empty())
            .map(Self::parse_path_component)
    }

    fn parse_path_component(src: &str) -> PathComponent {
        if src.starts_with(":") {
            PathComponent::Variant(src[1..].to_string())
        } else {
            PathComponent::Const(src.to_string())
        }
    }

    pub fn push(&mut self, src: &'static str) -> &mut Self {
        self.path.extend(Self::parse_path(src));

        self
    }

    pub fn join(&self, src: &'static str) -> Self {
        let mut route = self.clone();
        route.push(src);

        route
    }

    pub fn var(&mut self, key: &'static str, val: impl ToString) -> &mut Self {
        self.variant_map.insert(key.to_string(), val.to_string());

        self
    }

    pub fn with_var(&self, key: &'static str, val: impl ToString) -> Self {
        let mut route = self.clone();
        route.var(key, val);

        route
    }

    pub fn get(self) -> RequestBuilder {
        RequestBuilder::new(self, Method::GET)
    }

    pub fn put(self) -> RequestBuilder {
        RequestBuilder::new(self, Method::PUT)
    }

    pub fn post(self) -> RequestBuilder {
        RequestBuilder::new(self, Method::POST)
    }

    pub fn patch(self) -> RequestBuilder {
        RequestBuilder::new(self, Method::PATCH)
    }

    pub fn delete(self) -> RequestBuilder {
        RequestBuilder::new(self, Method::DELETE)
    }
}

impl ToString for Route {
    fn to_string(&self) -> String {
        let mut res = String::new();

        for path_comp in self.path.iter() {
            res.push('/');

            match path_comp {
                PathComponent::Variant(key) => res.push_str(
                    self.variant_map
                        .get(key)
                        .unwrap_or_else(|| panic!("Expected path variant {} to be populated", key)),
                ),
                PathComponent::Const(raw) => res.push_str(&raw),
            }
        }

        res
    }
}
