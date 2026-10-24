#![allow(unused_imports, unused_unsafe, stable_features, deprecated)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#![feature(strict_provenance)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;

pub mod src {
pub mod build {
pub mod reference {
pub mod protocols {
pub mod cursor_shape_v1_protocol;
pub mod tablet_v2_protocol;
pub mod wlr_layer_shell_unstable_v1_protocol;
pub mod wlr_output_power_management_unstable_v1_protocol;
pub mod xdg_output_unstable_v1_protocol;
pub mod xdg_shell_protocol;
} // mod protocols
} // mod reference
} // mod build
pub mod original {
pub mod common {
    pub use sway_common::ffi::*;
} // mod common
pub mod sway {
#[path = "commands.rs"]
pub mod commands_1;
#[path = "config.rs"]
pub mod config_1;
pub mod commands {
#[path = "bar.rs"]
pub mod bar_1;
#[path = "input.rs"]
pub mod input_1;
#[path = "output.rs"]
pub mod output_1;
#[path = "seat.rs"]
pub mod seat_1;
pub mod allow_tearing;
pub mod assign;
pub mod bar {
pub mod bind;
pub mod binding_mode_indicator;
pub mod colors;
pub mod font;
pub mod gaps;
pub mod height;
pub mod hidden_state;
pub mod icon_theme;
pub mod id;
pub mod mode;
pub mod modifier;
pub mod output;
pub mod pango_markup;
pub mod position;
pub mod separator_symbol;
pub mod status_command;
pub mod status_edge_padding;
pub mod status_padding;
pub mod strip_workspace_name;
pub mod strip_workspace_numbers;
pub mod swaybar_command;
pub mod tray_bind;
pub mod tray_output;
pub mod tray_padding;
pub mod workspace_buttons;
pub mod workspace_min_width;
pub mod wrap_scroll;
} // mod bar
pub mod bind;
pub mod border;
pub mod client;
pub mod create_output;
pub mod default_border;
pub mod default_floating_border;
pub mod default_orientation;
pub mod exec;
pub mod exec_always;
pub mod exit;
pub mod floating;
pub mod floating_minmax_size;
pub mod floating_modifier;
pub mod focus;
pub mod focus_follows_mouse;
pub mod focus_on_window_activation;
pub mod focus_wrapping;
pub mod font;
pub mod for_window;
pub mod force_display_urgency_hint;
pub mod force_focus_wrapping;
pub mod fullscreen;
pub mod gaps;
pub mod gesture;
pub mod hide_edge_borders;
pub mod include;
pub mod inhibit_idle;
pub mod input {
pub mod accel_profile;
pub mod calibration_matrix;
pub mod click_method;
pub mod clickfinger_button_map;
pub mod drag;
pub mod drag_lock;
pub mod dwt;
pub mod dwtp;
pub mod events;
pub mod left_handed;
pub mod map_from_region;
pub mod map_to_output;
pub mod map_to_region;
pub mod middle_emulation;
pub mod natural_scroll;
pub mod pointer_accel;
pub mod repeat_delay;
pub mod repeat_rate;
pub mod rotation_angle;
pub mod scroll_button;
pub mod scroll_button_lock;
pub mod scroll_factor;
pub mod scroll_method;
pub mod tap;
pub mod tap_button_map;
pub mod three_finger_drag;
pub mod tool_mode;
pub mod xkb_capslock;
pub mod xkb_file;
pub mod xkb_layout;
pub mod xkb_model;
pub mod xkb_numlock;
pub mod xkb_options;
pub mod xkb_rules;
pub mod xkb_switch_layout;
pub mod xkb_variant;
} // mod input
pub mod kill;
pub mod layout;
pub mod mark;
pub mod max_render_time;
pub mod mode;
pub mod mouse_warping;
pub mod new_float;
pub mod new_window;
pub mod no_focus;
pub mod nop;
pub mod opacity;
pub mod output {
pub mod adaptive_sync;
pub mod allow_tearing;
pub mod background;
pub mod color_profile;
pub mod disable;
pub mod dpms;
pub mod enable;
pub mod hdr;
pub mod max_render_time;
pub mod mode;
pub mod position;
pub mod power;
pub mod render_bit_depth;
pub mod scale;
pub mod scale_filter;
pub mod subpixel;
pub mod toggle;
pub mod transform;
pub mod unplug;
} // mod output
pub mod popup_during_fullscreen;
pub mod primary_selection;
pub mod r#move;
pub mod reload;
pub mod rename;
pub mod resize;
pub mod scratchpad;
pub mod seat {
pub mod attach;
pub mod cursor;
pub mod fallback;
pub mod hide_cursor;
pub mod idle;
pub mod keyboard_grouping;
pub mod pointer_constraint;
pub mod shortcuts_inhibitor;
pub mod xcursor_theme;
} // mod seat
pub mod set;
pub mod shortcuts_inhibitor;
pub mod show_marks;
pub mod smart_borders;
pub mod smart_gaps;
pub mod split;
pub mod sticky;
pub mod swap;
pub mod swaybg_command;
pub mod swaynag_command;
pub mod tiling_drag;
pub mod tiling_drag_threshold;
pub mod title_align;
pub mod title_format;
pub mod titlebar_border_thickness;
pub mod titlebar_padding;
pub mod unmark;
pub mod urgent;
pub mod workspace;
pub mod workspace_layout;
pub mod ws_auto_back_and_forth;
pub mod xwayland;
} // mod commands
pub mod config {
pub mod bar;
pub mod input;
pub mod output;
pub mod seat;
} // mod config
pub mod criteria;
pub mod decoration;
pub mod desktop {
pub mod idle_inhibit_v1;
pub mod launcher;
pub mod layer_shell;
pub mod output;
pub mod tearing;
pub mod transaction;
pub mod xdg_shell;
pub mod xwayland;
} // mod desktop
pub mod input {
pub mod cursor;
pub mod input_manager;
pub mod keyboard;
pub mod libinput;
pub mod seat;
pub mod seatop_default;
pub mod seatop_down;
pub mod seatop_move_floating;
pub mod seatop_move_tiling;
pub mod seatop_resize_floating;
pub mod seatop_resize_tiling;
pub mod switch;
pub mod tablet;
pub mod text_input;
} // mod input
pub mod ipc_json;
pub mod ipc_server;
pub mod lock;
pub mod realtime;
pub mod scene_descriptor;
pub mod server;
pub mod sway_text_node;
pub mod swaynag;
pub mod tree {
pub mod arrange;
pub mod container;
pub mod node;
pub mod output;
pub mod root;
pub mod view;
pub mod workspace;
} // mod tree
pub mod xdg_activation_v1;
pub mod xdg_decoration;
} // mod sway
} // mod original
} // mod src
