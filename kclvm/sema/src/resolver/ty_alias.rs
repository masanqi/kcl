use indexmap::IndexMap;
use kclvm_ast::walker::MutSelfMutWalker;
use kclvm_ast::{ast, walk_if_mut, walk_list_mut};

#[derive(Default)]
struct TypeAliasTransformer {
    pub type_alias_mapping: IndexMap<String, String>,
}

impl<'ctx> MutSelfMutWalker<'ctx> for TypeAliasTransformer {
    fn walk_rule_stmt(&mut self, rule_stmt: &'ctx mut ast::RuleStmt) {
        // walk_list_mut!(self, walk_identifier, rule_stmt.parent_rules);
        // walk_list_mut!(self, walk_call_expr, rule_stmt.decorators);
        walk_if_mut!(self, walk_arguments, rule_stmt.args);
        walk_if_mut!(self, walk_identifier, rule_stmt.for_host_name);
        walk_list_mut!(self, walk_check_expr, rule_stmt.checks);
    }
    fn walk_schema_stmt(&mut self, schema_stmt: &'ctx mut ast::SchemaStmt) {
        // walk_if_mut!(self, walk_identifier, schema_stmt.parent_name);
        // walk_if_mut!(self, walk_identifier, schema_stmt.for_host_name);
        walk_if_mut!(self, walk_arguments, schema_stmt.args);
        if let Some(schema_index_signature) = schema_stmt.index_signature.as_deref_mut() {
            let value = &mut schema_index_signature.node.value;
            if let Some(type_alias) = self
                .type_alias_mapping
                .get(&schema_index_signature.node.key_type.node)
            {
                schema_index_signature.node.key_type.node = type_alias.clone();
            }
            if let Some(type_alias) = self
                .type_alias_mapping
                .get(&schema_index_signature.node.value_type.node)
            {
                schema_index_signature.node.value_type.node = type_alias.clone();
            }
            walk_if_mut!(self, walk_expr, value);
        }
        walk_list_mut!(self, walk_identifier, schema_stmt.mixins);
        // walk_list_mut!(self, walk_call_expr, schema_stmt.decorators);
        walk_list_mut!(self, walk_stmt, schema_stmt.body);
        walk_list_mut!(self, walk_check_expr, schema_stmt.checks);
    }
    fn walk_schema_attr(&mut self, schema_attr: &'ctx mut ast::SchemaAttr) {
        // walk_list_mut!(self, walk_call_expr, schema_attr.decorators);
        if let Some(type_alias) = self.type_alias_mapping.get(&schema_attr.type_str.node) {
            schema_attr.type_str.node = type_alias.clone();
        }
        walk_if_mut!(self, walk_expr, schema_attr.value);
    }
    fn walk_assign_stmt(&mut self, assign_stmt: &'ctx mut ast::AssignStmt) {
        if let Some(ty_str) = &mut assign_stmt.type_annotation {
            if let Some(type_alias) = self.type_alias_mapping.get(&ty_str.node) {
                ty_str.node = type_alias.clone();
            }
        }
        self.walk_expr(&mut assign_stmt.value.node);
    }
    fn walk_unification_stmt(&mut self, unification_stmt: &'ctx mut ast::UnificationStmt) {
        self.walk_schema_expr(&mut unification_stmt.value.node);
    }
    fn walk_import_stmt(&mut self, _: &'ctx mut ast::ImportStmt) {}
    fn walk_lambda_expr(&mut self, lambda_expr: &'ctx mut ast::LambdaExpr) {
        walk_if_mut!(self, walk_arguments, lambda_expr.args);
        walk_list_mut!(self, walk_stmt, lambda_expr.body);
        if let Some(ty_str) = &mut lambda_expr.return_type_str {
            if let Some(type_alias) = self.type_alias_mapping.get(ty_str) {
                *ty_str = type_alias.clone();
            }
        }
    }
    fn walk_arguments(&mut self, arguments: &'ctx mut ast::Arguments) {
        walk_list_mut!(self, walk_identifier, arguments.args);
        for type_annotation in (&mut arguments.type_annotation_list.iter_mut()).flatten() {
            if let Some(type_alias) = self.type_alias_mapping.get(&type_annotation.node) {
                type_annotation.node = type_alias.clone();
            }
        }
        for default in arguments.defaults.iter_mut() {
            if let Some(d) = default.as_deref_mut() {
                self.walk_expr(&mut d.node)
            }
        }
    }
    fn walk_identifier(&mut self, identifier: &'ctx mut ast::Identifier) {
        if let Some(type_alias) = self.type_alias_mapping.get(&identifier.get_name()) {
            if type_alias.starts_with('@') {
                let splits: Vec<&str> = type_alias.rsplitn(2, '.').collect();
                identifier.pkgpath = splits[1].to_string();
                identifier.names = vec![splits[1].to_string(), splits[0].to_string()];
            } else {
                let names = type_alias.split('.').collect::<Vec<&str>>();
                identifier.names = names.iter().map(|n| n.to_string()).collect();
            }
        }
    }
}

/// Replace type alias.
fn fix_type_alias_identifier<'ctx>(
    module: &'ctx mut ast::Module,
    type_alias_mapping: IndexMap<String, String>,
) {
    let mut type_alias_transformer = TypeAliasTransformer { type_alias_mapping };
    type_alias_transformer.walk_module(module);
}

/// Process type alias.
pub fn process_program_type_alias(
    program: &mut ast::Program,
    type_alias_mapping: IndexMap<String, IndexMap<String, String>>,
) {
    for (pkgpath, modules) in program.pkgs.iter_mut() {
        for module in modules.iter_mut() {
            if let Some(type_alias_mapping) = type_alias_mapping.get(pkgpath) {
                fix_type_alias_identifier(module, type_alias_mapping.clone());
            }
        }
    }
}