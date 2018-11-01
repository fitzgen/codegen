// Matcher

use mergedtree::{self, MergedArena};
use patternmatcher::{self, Node, NodeType, NodeID};

#[derive(Clone)]
pub struct Opt {
    current_entity: String,
    func_str: String,
    //scope_stack: Vec<ScopeType>,
    scope_stack: Vec<ScopeStack>,
}

#[derive(Clone)]
pub struct ScopeStack {
    scope_type: ScopeType,
    level: usize,
}

#[derive(Clone)]
pub enum ScopeType {
    scope_match,
    scope_case,
    scope_func,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            current_entity: String::from("inst"),
            func_str: String::from(""),
            scope_stack: Vec::new(),
        }
    }

    pub fn generate_header(&mut self) {
        self.func_str.push_str("fn matcher(pos: &mut FuncCursor, inst: Inst)");
    }

    pub fn append(&mut self, input: String) {
        self.func_str.push_str(&input);
    }

    pub fn set_entity(&mut self, entity: String) {
        self.current_entity = entity;
    }

    pub fn does_level_exist_in_stack(&mut self, find_level: usize) -> usize {
        let mut index = 0;
        for i in 0 .. self.scope_stack.len() {
            if find_level == self.scope_stack[i].level {
                index = i;
                break;
            } else {
                continue;
            }
        }
        index
    }

    pub fn pop_and_exit_scope_from(&mut self, from: usize) {
        for i in from .. self.scope_stack.len() {
            let stack_elem = self.scope_stack.pop();
            match stack_elem {
                Some(elem) => {
                    self.exit_scope(elem.scope_type, elem.level);
                },
                None => {},
            }
        }
    }

    pub fn enter_scope(&mut self, scope: ScopeType, current_level: usize) {
        // check for the level in current stack
        // if level is new - not found in stack - push it directly.
        // if level already exists in stack, pop the stack until that level and then
        // push the new level.
        println!("Current stack before entering scope is: -------");
        for x in 0 .. self.scope_stack.len() {
           println!("stack levels pushed so far = {}", self.scope_stack[x].level);
        }
        println!("find the level number = {} in stack", current_level);
        let index = self.does_level_exist_in_stack(current_level);
        println!("Found index from stack == {}", index);
        if index != 0 {
            // index exists
            // pop first
            self.pop_and_exit_scope_from(index);
        }
        // push the level
        self.scope_stack.push(ScopeStack {scope_type: scope.clone(), level: current_level });
        // append the string
        match scope {
            ScopeType::scope_match => {
                self.append(String::from(" {\n"));
            },
            ScopeType::scope_func => {
                self.append(String::from(" {\n"));
            },
            ScopeType::scope_case => {
                self.append(String::from(" => {\n"));
            },
            _ => {
                panic!("Error: No such scope type exists");
            },
        }
    }

    pub fn exit_scope(&mut self, scope: ScopeType, level: usize) {
        match scope {
            ScopeType::scope_match => {
                self.append(String::from("\n}"));
            },
            ScopeType::scope_func => {
                self.append(String::from("\n}"));
            },
            ScopeType::scope_case => {
                self.append(String::from("\n},"));
            },
            _ => {
                panic!("Error: No such scope type exists");
            },
        }
    }

    pub fn is_leaf_node(&mut self, node: Node) -> bool {
        println!("check leaf node =========\n\n");
        match node.next {
            Some(x) => false,
            None => true,
        }
    }
}

pub fn generate_matcher(mut arena: MergedArena) -> String {
    let mut opt_func = Opt::new();

    for node in 0 .. arena.merged_tree.len() {
        println!("Node ==== ============================================================");
        println!("\t\t Node Id = {}", arena.merged_tree[node].id);
        println!("\t\t Node Level = {}", arena.merged_tree[node].level);
        println!("\t\t Node arg flag = {}", arena.merged_tree[node].arg_flag);
        match arena.merged_tree[node].node_type {
            NodeType::match_root => {
                opt_func.generate_header();
                let current_level = arena.merged_tree[node].level;
                opt_func.enter_scope(ScopeType::scope_func, current_level);
                //set the level of root->next nodes to 0+1
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_level(next_node.clone(), current_level + 1);
                        arena.update_node_level_in_arena(updated_node.clone());
                    }
                }
            },
            NodeType::match_instdata => {
                println!("\t\t Instdata node type");
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_level(next_node.clone(), current_level+1);
                        arena.update_node_level_in_arena(updated_node.clone());
                    }
                }
                if !arena.merged_tree[node].arg_flag {
                    println!("\t\t\t Instdata node: arg_flag is NOT true\n");
                    opt_func.append(String::from("match pos.func.dfg"));
                    opt_func.append(String::from("["));
                    let mut opt_clone = opt_func.clone();
                    let mut ent = opt_clone.current_entity;
                    opt_func.append(ent);
                    opt_func.append(String::from("]"));

                    opt_func.enter_scope(ScopeType::scope_match, current_level);

                    match arena.merged_tree[node].node_value.as_ref() {
                        // TODO: Add more types of instdata here
                        // FIXME: Later make sure if Var case is handled well.
                        // Example:
                        // %0 = var
                        // infer %0
                        "Var" => {},
                        "Binary" => {
                            opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                            opt_func.enter_scope(ScopeType::scope_case, current_level);
                        },
                        _ => {
                            panic!("Error: This instruction data type is not yet handled");
                        },
                    }
                } else {
                    //handle this case separately
                    println!("\t\t\t Instdata node: arg_flag is true\n");
                    opt_func.append(String::from("\nValDef::"));
                    match arena.merged_tree[node].node_value.as_ref() {
                        "Var" => {
                            println!("\t\t\t entering valdef::param here");
                            opt_func.append(String::from("Param(_, _)"));
                            opt_func.enter_scope(ScopeType::scope_case, current_level);
                            opt_func.set_entity(String::from(""));
                        },
                        _ => {
                            println!("\t\t\t entering valdef::result here");
                            opt_func.append(String::from("Result(arg_ty, _)"));
                            opt_func.enter_scope(ScopeType::scope_case, current_level);
                            opt_func.set_entity(String::from("arg_ty"));
                        },
                    }
                    match opt_func.current_entity.as_ref() {
                        "" => {},
                        _ => {
                            opt_func.append(String::from("match pos.func.dfg"));
                            opt_func.append(String::from("["));
                            let mut opt_clone = opt_func.clone();
                            let mut ent = opt_clone.current_entity;
                            opt_func.append(ent);
                            opt_func.append(String::from("]"));
                            opt_func.enter_scope(ScopeType::scope_match, current_level);
                            //
                            //match node_value
                            // TODO: InstructionData::node_value stuff
                            // enter scope case
                            match arena.merged_tree[node].node_value.as_ref() {
                                // TODO: Add more types of instdata here
                                // FIXME: Later make sure if Var case is handled well.
                                // Example:
                                // %0 = var
                                // infer %0
                                "Var" => {},
                                "Binary" => {
                                    opt_func.append(String::from("InstructionData::Binary { opcode, args }"));
                                    opt_func.enter_scope(ScopeType::scope_case, current_level);
                                },
                                _ => {
                                    panic!("Error: This instruction data type is not yet handled");
                                },
                            }
                        },
                    }
                }
            },
            NodeType::match_opcode => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_level(next_node.clone(), current_level+1);
                        arena.update_node_level_in_arena(updated_node.clone());
                    }
                }
                // match the actual opcode types
                // FIXME: Later add more opcodes here
                match arena.merged_tree[node].node_value.as_ref() {
                    "Var" => {},
                    "Iadd" => {
                        opt_func.append(String::from("match_opcode"));
                        opt_func.enter_scope(ScopeType::scope_match, current_level);
                        opt_func.append(String::from("Opcode::Iadd"));
                        opt_func.enter_scope(ScopeType::scope_case, current_level);
                    },
                    _ => {
                        panic!("Error: this opcode type is not yet handled");
                    },
                }
            },
            NodeType::match_args => {
                let current_level = arena.merged_tree[node].level;
                //set the level of root->next nodes to 0+1
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_level(next_node.clone(), current_level+1);
                        arena.update_node_level_in_arena(updated_node.clone());
                    }
                }
                // create a default match string
                opt_func.append(String::from("match pos.func.dfg.val_def"));
                opt_func.append(String::from("("));
                opt_func.append(arena.merged_tree[node].node_value.clone());
                opt_func.append(String::from(")"));

                opt_func.enter_scope(ScopeType::scope_match, current_level);
 
                // set the arg_flag to true for next nodes of match_args
                if let Some(next_nodes) = arena.merged_tree[node].next.clone() {
                    for n in 0 .. next_nodes.len() {
                        let id = next_nodes[n].index;
                        let mut next_node = arena.find_node_with_id_in_arena(id);
                        let updated_node = arena.update_node_with_arg_flag(next_node.clone(), true);
                        arena.update_node_arg_flag_in_arena(updated_node.clone());
                    }
                }
            },
            _ => {
                panic!("\n\nmatch type not handled yet!\n");
            },
        }
    }

    // exit func scope
    for s in 0 .. opt_func.scope_stack.len() {
        match opt_func.scope_stack.pop() {
            Some(elem) => {
                opt_func.exit_scope(elem.scope_type, elem.level);
            },
            None => {},
        }
        //let elem_ty = opt_func.scope_stack.pop();
        //match elem_ty {
        //    Some(ty) => {
        //        opt_func.exit_scope(ty);
        //    },
        //    None => {},
        //}
    }

    opt_func.func_str
}
