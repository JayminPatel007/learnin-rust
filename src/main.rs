mod structure;
mod data_types;
mod operators;
mod scope_and_shadowing;
mod constants;
mod stack_and_heap_memory;
mod if_statement;
mod while_and_loop;
mod for_loop;
mod match_statement;

fn main() {
    data_types::data_types();
    operators::bitwise_operators();
    operators::logical_operator();
    operators::arithmetic_operators();
    scope_and_shadowing::scope_and_shadowing();
    constants::constants();
    stack_and_heap_memory::stack_and_heap_memory();
    if_statement::if_statement();
    while_and_loop::while_and_loop();
    for_loop::for_loop();
    match_statement::match_statement();
    structure::structures();
}
