extern crate pipl_engine;
#[macro_use]
mod helpers;
mod cases {
    mod simplest_reaction;
    mod multi_step_reaction;
    mod simplest_mobility;
    mod repeating_read_prefix;
    mod repeating_send_prefix;
    mod terminate_parallel;
    mod terminate_choice;
    mod no_names;
    mod polyadic;
    mod new_names_in_read;
    mod new_names_in_send;
    mod new_names_in_repeating_read;
    mod new_names_in_repeating_send;
    mod new_names_in_prefix_do_not_affect_channel;
    mod new_names_before_parallel;
    mod new_names_before_choice;
    mod new_names_in_parallel_prefixes;
    mod new_names_in_choice_prefixes;
}
