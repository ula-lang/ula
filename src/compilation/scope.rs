use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct Scope {
    scope_data: Arc<ScopeData>
}

#[derive(Debug, Default)]
struct ScopeData {
    parent: Option<Arc<ScopeData>>,
    classes: RwLock<HashSet<String>>,
    enums: RwLock<HashSet<String>>,
    functions: RwLock<HashSet<String>>,
    variables: RwLock<HashSet<String>>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_child(&self) -> Self {
        Self {
            scope_data: Arc::new(ScopeData::new(Some(self.scope_data.clone())))
        }
    }

    pub fn is_defined(&self, identifier: &str) -> bool {
        self.is_class(identifier)
            || self.is_enum(identifier)
            || self.is_function(identifier)
            || self.is_variable(identifier)
    }

    pub fn is_class(&self, identifier: &str) -> bool {
        self.scope_data.is_class(identifier)
    }

    pub fn is_enum(&self, identifier: &str) -> bool {
        self.scope_data.is_enum(identifier)
    }

    pub fn is_function(&self, identifier: &str) -> bool {
        self.scope_data.is_function(identifier)
    }

    pub fn is_variable(&self, identifier: &str) -> bool {
        self.scope_data.is_variable(identifier)
    }

    pub fn add_class<T>(&self, identifier: T) where T: ToString {
        self.scope_data.add_class(identifier.to_string())
    }

    pub fn add_enum<T>(&self, identifier: T) where T: ToString {
        self.scope_data.add_enum(identifier.to_string())
    }

    pub fn add_function<T>(&self, identifier: T) where T: ToString {
        self.scope_data.add_function(identifier.to_string())
    }

    pub fn add_variable<T>(&self, identifier: T) where T: ToString {
        self.scope_data.add_variable(identifier.to_string())
    }

    pub fn flatten(&self) -> Scope {
        Self {
            scope_data: Arc::new(self.scope_data.flatten())
        }
    }
}

impl ScopeData {
    pub fn new(parent: Option<Arc<ScopeData>>) -> Self {
        Self {
            parent,
            classes: RwLock::default(),
            enums: RwLock::default(),
            functions: RwLock::default(),
            variables: RwLock::default(),
        }
    }

    pub fn is_class(&self, identifier: &str) -> bool {
        self.check(identifier, &self.classes, Self::is_class)
    }

    pub fn is_enum(&self, identifier: &str) -> bool {
        self.check(identifier, &self.enums, Self::is_enum)
    }

    pub fn is_function(&self, identifier: &str) -> bool {
        self.check(identifier, &self.functions, Self::is_function)
    }

    pub fn is_variable(&self, identifier: &str) -> bool {
        self.check(identifier, &self.variables, Self::is_variable)
    }

    pub fn add_class(&self, identifier: String) {
        if self.is_class(&identifier) {
            return;
        }

        self.classes.write().unwrap().insert(identifier.to_string());
    }

    pub fn add_enum(&self, identifier: String) {
        if self.is_enum(&identifier) {
            return;
        }

        self.enums.write().unwrap().insert(identifier.to_string());
    }

    pub fn add_function(&self, identifier: String) {
        if self.is_function(&identifier) {
            return;
        }

        self.functions.write().unwrap().insert(identifier.to_string());
    }

    pub fn add_variable(&self, identifier: String) {
        if self.is_variable(&identifier) {
            return;
        }

        self.variables.write().unwrap().insert(identifier.to_string());
    }

    pub fn flatten(&self) -> Self {
        let scope_data = match &self.parent {
            Some(parent) => parent.flatten(),

            None => ScopeData::new(None)
        };

        self.classes.read().unwrap().iter()
            .for_each(|identifier| scope_data.add_class(identifier.to_owned()));

        self.enums.read().unwrap().iter()
            .for_each(|identifier| scope_data.add_enum(identifier.to_owned()));

        self.functions.read().unwrap().iter()
            .for_each(|identifier| scope_data.add_function(identifier.to_owned()));

        self.variables.read().unwrap().iter()
            .for_each(|identifier| scope_data.add_variable(identifier.to_owned()));

        scope_data
    }

    fn check<T>(&self, ident: &str, local_data: &RwLock<HashSet<String>>, parent_func: T) -> bool where T: Fn(&ScopeData, &str) -> bool {
        match local_data.read().unwrap().contains(ident) {
            true => true,

            false => match &self.parent {
                &Some(ref scope_data) => parent_func(scope_data, ident),

                &None => false
            }
        }
    }
}

mod tests {
    use super::Scope;

    #[test]
    fn test_local_classes() {
        let scope = Scope::new();

        assert_eq!(scope.is_class("Foo"), false);

        scope.add_class("Foo");

        assert_eq!(scope.is_class("Foo"), true);
        assert_eq!(scope.is_class("Bar"), false);

        scope.add_class("Bar");

        assert_eq!(scope.is_class("Foo") && scope.is_class("Bar"), true);

        assert_eq!(scope.is_class("Baz"), false);
    }

    #[test]
    fn test_local_enums() {
        let scope = Scope::new();

        assert_eq!(scope.is_enum("Foo"), false);

        scope.add_enum("Foo");

        assert_eq!(scope.is_enum("Foo"), true);
        assert_eq!(scope.is_enum("Bar"), false);

        scope.add_enum("Bar");

        assert_eq!(scope.is_enum("Foo") && scope.is_enum("Bar"), true);

        assert_eq!(scope.is_enum("Baz"), false);
    }

    #[test]
    fn test_local_functions() {
        let scope = Scope::new();

        assert_eq!(scope.is_function("foo"), false);

        scope.add_function("foo");

        assert_eq!(scope.is_function("foo"), true);
        assert_eq!(scope.is_function("bar"), false);

        scope.add_function("bar");

        assert_eq!(scope.is_function("foo") && scope.is_function("bar"), true);

        assert_eq!(scope.is_function("baz"), false);
    }

    #[test]
    fn test_local_variables() {
        let scope = Scope::new();

        assert_eq!(scope.is_variable("foo"), false);

        scope.add_variable("foo");

        assert_eq!(scope.is_variable("foo"), true);
        assert_eq!(scope.is_variable("bar"), false);

        scope.add_variable("bar");

        assert_eq!(scope.is_variable("foo") && scope.is_variable("bar"), true);

        assert_eq!(scope.is_variable("baz"), false);
    }

    #[test]
    fn test_global_classes() {
        let global_scope = Scope::new();
        let local_scope = global_scope.create_child();

        assert_eq!(global_scope.is_class("Foo") || local_scope.is_class("Foo"), false);

        global_scope.add_class("Foo");

        assert_eq!(global_scope.is_class("Foo") && !global_scope.is_class("Bar"), true);
        assert_eq!(local_scope.is_class("Foo") && !local_scope.is_class("Bar"), true);

        local_scope.add_class("Bar");

        assert_eq!(global_scope.is_class("Foo") && !global_scope.is_class("Bar"), true);
        assert_eq!(local_scope.is_class("Foo") && local_scope.is_class("Bar"), true);

        assert_eq!(global_scope.is_class("Baz") || local_scope.is_class("Baz"), false);
    }

    #[test]
    fn test_global_enums() {
        let global_scope = Scope::new();
        let local_scope = global_scope.create_child();

        assert_eq!(global_scope.is_enum("Foo") || local_scope.is_enum("Foo"), false);

        global_scope.add_enum("Foo");

        assert_eq!(global_scope.is_enum("Foo") && !global_scope.is_enum("Bar"), true);
        assert_eq!(local_scope.is_enum("Foo") && !local_scope.is_enum("Bar"), true);

        local_scope.add_enum("Bar");

        assert_eq!(global_scope.is_enum("Foo") && !global_scope.is_enum("Bar"), true);
        assert_eq!(local_scope.is_enum("Foo") && local_scope.is_enum("Bar"), true);

        assert_eq!(global_scope.is_enum("Baz") || local_scope.is_enum("Baz"), false);
    }

    #[test]
    fn test_global_functions() {
        let global_scope = Scope::new();
        let local_scope = global_scope.create_child();

        assert_eq!(global_scope.is_function("foo") || local_scope.is_function("foo"), false);

        global_scope.add_function("foo");

        assert_eq!(global_scope.is_function("foo") && !global_scope.is_function("bar"), true);
        assert_eq!(local_scope.is_function("foo") && !local_scope.is_function("bar"), true);

        local_scope.add_function("bar");

        assert_eq!(global_scope.is_function("foo") && !global_scope.is_function("bar"), true);
        assert_eq!(local_scope.is_function("foo") && local_scope.is_function("bar"), true);

        assert_eq!(global_scope.is_function("baz") || local_scope.is_function("baz"), false);
    }

    #[test]
    fn test_global_variables() {
        let global_scope = Scope::new();
        let local_scope = global_scope.create_child();

        assert_eq!(global_scope.is_variable("foo") || local_scope.is_variable("foo"), false);

        global_scope.add_variable("foo");

        assert_eq!(global_scope.is_variable("foo") && !global_scope.is_variable("bar"), true);
        assert_eq!(local_scope.is_variable("foo") && !local_scope.is_variable("bar"), true);

        local_scope.add_variable("bar");

        assert_eq!(global_scope.is_variable("foo") && !global_scope.is_variable("bar"), true);
        assert_eq!(local_scope.is_variable("foo") && local_scope.is_variable("bar"), true);

        assert_eq!(global_scope.is_variable("baz") || local_scope.is_variable("baz"), false);
    }
}