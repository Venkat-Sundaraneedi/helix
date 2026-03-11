use std::collections::HashMap;

use super::macros::keymap;
use super::{KeyTrie, Mode};
use helix_core::hashmap;

pub fn default() -> HashMap<Mode, KeyTrie> {
    let normal = keymap!({ "Normal mode"
        "h" | "left" => move_same_line_char_left,
        "j" | "down" => move_anchored_visual_line_down,
        "k" | "up" => move_anchored_visual_line_up,
        "l" | "right" => move_same_line_char_right,

        "t" => evil_find_till_char,
        "f" => evil_find_next_char,
        "T" => evil_till_prev_char,
        "F" => evil_find_prev_char,
        "r" => replace,
        "R" => replace_with_yanked,

        "~" => switch_case,


        //"w" => move_next_word_start,
        //"b" => move_prev_word_start,
        //"e" => move_next_word_end,

        //"W" => move_next_long_word_start,
        //"B" => move_prev_long_word_start,
        //"E" => move_next_long_word_end,

        "v" => evil_characterwise_select_mode,
        // TODO (redundant with count + gg anyway?): "G" => goto_line,
        "g" => { "Goto"
            "g" => evil_goto_line_or_first_line,
            "|" => goto_column,
            "f" => goto_file,
            "d" => goto_definition,
            "D" => goto_declaration,
            "y" => goto_type_definition,
            "r" => goto_reference,
            "i" => goto_implementation,
            "a" => goto_last_accessed_file,
            "m" => goto_last_modified_file,
            "." => goto_last_modification,
            "w" => evil_goto_word,
            "W" => evil_extend_to_word,
        },
        ":" => command_mode,

        "i" => insert_mode,
        "I" => insert_at_line_start,
        "a" => evil_append_mode,
        "A" => insert_at_line_end,
        "o" => open_below,
        "O" => open_above,

        "C" => copy_selection_on_next_line,
        "A-C" => copy_selection_on_prev_line,


        "s" => select_regex,
        "A-s" => split_selection_on_newline,
        "A-minus" => merge_selections,
        "A-_" => merge_consecutive_selections,
        "S" => split_selection,
        ";" => collapse_selection,
        "A-;" => flip_selections,
        "A-o" | "A-up" => expand_selection,
        "A-i" | "A-down" => shrink_selection,
        "A-I" | "A-S-down" => select_all_children,
        "A-p" | "A-left" => select_prev_sibling,
        "A-n" | "A-right" => select_next_sibling,
        // "A-e" => move_parent_node_end,
        "A-b" => move_parent_node_start,
        "A-a" => select_all_siblings,

        "m" => { "Match"
            "m" => match_brackets,
            "s" => surround_add,
            "r" => surround_replace,
            "d" => surround_delete,
            "a" => select_textobject_around,
            "i" => select_textobject_inner,
        },
        "[" => { "Left bracket"
            "d" => goto_prev_diag,
            "D" => goto_first_diag,
            "g" => goto_prev_change,
            "G" => goto_first_change,
            "f" => goto_prev_function,
            "t" => goto_prev_class,
            "a" => goto_prev_parameter,
            "c" => goto_prev_comment,
            "e" => goto_prev_entry,
            "T" => goto_prev_test,
            "p" => goto_prev_paragraph,
            "space" => add_newline_above,
        },
        "]" => { "Right bracket"
            "d" => goto_next_diag,
            "D" => goto_last_diag,
            "g" => goto_next_change,
            "G" => goto_last_change,
            "f" => goto_next_function,
            "t" => goto_next_class,
            "a" => goto_next_parameter,
            "c" => goto_next_comment,
            "e" => goto_next_entry,
            "T" => goto_next_test,
            "p" => goto_next_paragraph,
            "space" => add_newline_below,
        },

        "}" => evil_move_paragraph_forward,
        "{" => evil_move_paragraph_backward,

        "/" => search,
        "?" => rsearch,
        "n" => search_next,
        "N" => search_prev,

        "*" => evil_cursor_forward_search,
        "#" => evil_cursor_backward_search,

        "u" => undo,
        "U" => redo,
        "A-u" => earlier,
        "A-U" => later,

        "p" => paste_after,
        "P" => paste_before,

        "Q" => record_macro,
        "q" => replay_macro,

        ">" => indent,
        "<" => unindent,
        "=" => format_selections,
        "J" => join_selections,
        "A-J" => join_selections_space,
        "K" => keep_selections,
        "A-K" => remove_selections,

        "," => keep_primary_selection,
        "A-," => remove_primary_selection,

        "&" => align_selections,
        "_" => trim_selections,

        "(" => rotate_selections_backward,
        ")" => rotate_selections_forward,
        "A-(" => rotate_selection_contents_backward,
        "A-)" => rotate_selection_contents_forward,

        "A-:" => ensure_selections_forward,

        "esc" => normal_mode,
        "C-b" | "pageup" => page_up,
        "C-f" | "pagedown" => page_down,
        "C-u" => page_cursor_half_up,
        "C-d" => page_cursor_half_down,

        "C-w" => { "Window"
            "C-w" | "w" => rotate_view,
            "C-s" | "s" => hsplit,
            "C-v" | "v" => vsplit,
            "C-t" | "t" => transpose_view,
            "f" => goto_file_hsplit,
            "F" => goto_file_vsplit,
            "C-q" | "q" => wclose,
            "C-o" | "o" => wonly,
            "C-h" | "h" | "left" => jump_view_left,
            "C-j" | "j" | "down" => jump_view_down,
            "C-k" | "k" | "up" => jump_view_up,
            "C-l" | "l" | "right" => jump_view_right,
            "L" => swap_view_right,
            "K" => swap_view_up,
            "H" => swap_view_left,
            "J" => swap_view_down,
            "n" => { "New split scratch buffer"
                "C-s" | "s" => hsplit_new,
                "C-v" | "v" => vsplit_new,
            },
        },

        "space" => { "Space"
            "A" => file_explorer,
            "a" => file_explorer_in_current_buffer_directory,
            "e" => file_picker_in_current_directory,
            "E" => file_picker_in_current_buffer_directory,
            "j" => jumplist_picker,
            "g" => changed_file_picker,
            "G" => { "Debug (experimental)" sticky=true
                "l" => dap_launch,
                "r" => dap_restart,
                "b" => dap_toggle_breakpoint,
                "c" => dap_continue,
                "h" => dap_pause,
                "i" => dap_step_in,
                "o" => dap_step_out,
                "n" => dap_next,
                "v" => dap_variables,
                "t" => dap_terminate,
                "C-c" => dap_edit_condition,
                "C-l" => dap_edit_log,
                "s" => { "Switch"
                    "t" => dap_switch_thread,
                    "f" => dap_switch_stack_frame,
                    // sl, sb
                },
                "e" => dap_enable_exceptions,
                "E" => dap_disable_exceptions,
            },
            "h" => select_references_to_symbol_under_cursor,
            "k" => hover,
            ";" => toggle_comments,
            "c" => { "Code"
                "a" => code_action,
                "r" => rename_symbol,
                "s" => symbol_picker,
                "S" => workspace_symbol_picker,
            },

            "s" => { "Search"
                "k" => command_palette,
                "g" => global_search,
                "d" => diagnostics_picker,
                "D" => workspace_diagnostics_picker,
            },
            "f" => {"Find"
                "b" => buffer_picker,
            },
        },
        "z" => { "View"
            "z" | "c" => align_view_center,
            "t" => align_view_top,
            "b" => align_view_bottom,
            "m" => align_view_middle,
            "k" | "up" => scroll_up,
            "j" | "down" => scroll_down,
            "C-b" | "pageup" => page_up,
            "C-f" | "pagedown" => page_down,
            "C-u" | "backspace" => page_cursor_half_up,
            "C-d" | "space" => page_cursor_half_down,

            "/" => search,
            "?" => rsearch,
            "n" => search_next,
            "N" => search_prev,
        },
        "Z" => { "View" sticky=true
            "z" | "c" => align_view_center,
            "t" => align_view_top,
            "b" => align_view_bottom,
            "m" => align_view_middle,
            "k" | "up" => scroll_up,
            "j" | "down" => scroll_down,
            "C-b" | "pageup" => page_up,
            "C-f" | "pagedown" => page_down,
            "C-u" | "backspace" => page_cursor_half_up,
            "C-d" | "space" => page_cursor_half_down,

            "/" => search,
            "?" => rsearch,
            "n" => search_next,
            "N" => search_prev,
        },

        "\"" => select_register,
        "|" => shell_pipe,
        "A-|" => shell_pipe_to,
        "!" => shell_insert_output,
        "A-!" => shell_append_output,
        //"$" => shell_keep_pipe,
        "C-z" => suspend,

        "C-a" => increment,
        "C-x" => decrement,

        "c" => evil_change,
        "d" => evil_delete,
        "x" => evil_delete_immediate,
        "y" => evil_yank,
        "b" => evil_prev_word_start,
        "e" => evil_next_word_end,
        "w" => evil_next_word_start,
        "B" => evil_prev_long_word_start,
        "E" => evil_next_long_word_end,
        "W" => evil_next_long_word_start,

        "G" => evil_goto_line_or_last_line,
        "del" => delete_selection,

        // 0xgsvs custom keymaps
        "A-e" => match_brackets,
        "A-v" => evil_linewise_select_mode,
        "S-h" => goto_first_nonwhitespace,
        "S-l" => goto_line_end,
        "tab" => goto_next_buffer,
        "S-tab" => goto_previous_buffer,

    });
    let mut select = normal.clone();
    select.merge_nodes(keymap!({ "Select mode"
        "h" | "left" => extend_same_line_char_left,
        "j" | "down" => extend_anchored_visual_line_down,
        "k" | "up" => extend_anchored_visual_line_up,
        "l" | "right" => extend_same_line_char_right,

        "a" => select_textobject_around,
        "i" => select_textobject_inner,

        //"w" => extend_next_word_start,
        //"b" => extend_prev_word_start,
        //"e" => extend_next_word_end,
        //"W" => extend_next_long_word_start,
        //"B" => extend_prev_long_word_start,
        //"E" => extend_next_long_word_end,

        // "A-e" => extend_parent_node_end,
        "A-b" => extend_parent_node_start,

        //"n" => extend_search_next,
        //"N" => extend_search_prev,

        //"t" => extend_till_char,
        //"f" => extend_next_char,
        //"T" => extend_till_prev_char,
        //"F" => extend_prev_char,

        "home" => extend_to_line_start,
        "end" => extend_to_line_end,
        "esc" => exit_select_mode,

        "g" => { "Goto"
            "k" => extend_anchored_line_up,
            "j" => extend_anchored_line_down,
            "w" => extend_to_word,
        },

        "A-e" => match_brackets,
        "S-h" => goto_line_start,
        "S-l" => goto_line_end,
        "space" => {"Space"
            ";" => toggle_comments,
        },
    }));
    let insert = keymap!({ "Insert mode"
        "esc" => normal_mode,

        "C-s" => commit_undo_checkpoint,
        "C-x" => completion,
        "C-r" => insert_register,

        "C-w" | "A-backspace" => delete_word_backward,
        "A-d" | "A-del" => delete_word_forward,
        "C-d" => unindent,
        "C-t" => indent,
        "C-u" => kill_to_line_start,
        "C-k" => kill_to_line_end,
        "C-h" | "backspace" | "S-backspace" => delete_char_backward,
        "del" => delete_char_forward,
        "C-j" | "ret" => insert_newline,
        "tab" => smart_tab,
        "S-tab" => insert_tab,

        "up" => move_visual_line_up,
        "down" => move_visual_line_down,
        "left" => move_char_left,
        "right" => move_char_right,
        "pageup" => page_up,
        "pagedown" => page_down,
        "home" => goto_line_start,
        "end" => goto_line_end_newline,

        //0xgsvs custom keymaps
        "A-h" => move_char_left,
        "A-l" => move_char_right,
        "A-j" => move_line_down,
        "A-k" => move_line_up,
        "j" => {"normal mode"
            "k" => normal_mode,
        },
    });
    hashmap!(
        Mode::Normal => normal,
        Mode::Select => select,
        Mode::Insert => insert,
    )
}
