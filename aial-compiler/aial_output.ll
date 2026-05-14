; AIAL-generated LLVM IR
target triple = "x86_64-unknown-linux-gnu"

declare i64 @aial_rt_strcat(i64, i64)
declare i64 @aial_rt_ctx_open_memory(i64)
declare void @aial_rt_ctx_save_message(i64, i64, i64, i64)
declare i64 @aial_rt_ctx_load_messages(i64, i64, i64)
declare void @aial_rt_ctx_close_memory(i64)
declare i64 @aial_rt_ctx_new(i64, i64, i64, i64)
declare i64 @aial_rt_ai_stream_start(i64, i64, i64, double, i64, i64)
declare i64 @aial_rt_ai_stream_read(i64)
declare i64 @aial_rt_map_new()
declare i64 @aial_rt_map_has(i64, i64)
declare i64 @aial_rt_map_get(i64, i64)
declare void @aial_rt_map_set(i64, i64, i64)
declare void @aial_rt_println(i64)
declare void @aial_rt_print(i64)
declare void @aial_rt_array_push(i64, i64)
declare i64 @aial_rt_array_len(i64)
declare i64 @aial_rt_array_get(i64, i64)
declare i64 @aial_rt_array_new()
declare void @aial_rt_io_raw_mode(i64)
declare i64 @aial_rt_key_exists(i64)
declare i64 @aial_rt_io_readkey()
declare i64 @aial_rt_strlen(i64)
declare void @aial_rt_time_sleep(i64)
declare i64 @aial_rt_str_eq(i64, i64)
declare i64 @aial_rt_starts_with(i64, i64)
declare i64 @aial_rt_strslice(i64, i64, i64)
declare i64 @aial_rt_key_set(i64, i64)
declare i64 @aial_rt_ctx_last_error()
declare i64 @aial_rt_io_readkey_timeout(i64)
declare i64 @aial_rt_ctx_add_message(i64, i64, i64)
declare i64 @aial_rt_token_estimate(i64)
declare i64 @aial_rt_strchr(i64, i64)
declare void @aial_rt_string_register(i64, i8*)

@.str0 = private unnamed_addr constant [5 x i8] c"[0m\00", align 1
@.str1 = private unnamed_addr constant [6 x i8] c"[36m\00", align 1
@.str2 = private unnamed_addr constant [6 x i8] c"[33m\00", align 1
@.str3 = private unnamed_addr constant [6 x i8] c"[32m\00", align 1
@.str4 = private unnamed_addr constant [6 x i8] c"[35m\00", align 1
@.str5 = private unnamed_addr constant [6 x i8] c"[90m\00", align 1
@.str6 = private unnamed_addr constant [5 x i8] c"[1m\00", align 1
@.str7 = private unnamed_addr constant [9 x i8] c"[40;37m\00", align 1
@.str8 = private unnamed_addr constant [4 x i8] c"╔\00", align 1
@.str9 = private unnamed_addr constant [4 x i8] c"╗\00", align 1
@.str10 = private unnamed_addr constant [4 x i8] c"╚\00", align 1
@.str11 = private unnamed_addr constant [4 x i8] c"╝\00", align 1
@.str12 = private unnamed_addr constant [4 x i8] c"═\00", align 1
@.str13 = private unnamed_addr constant [4 x i8] c"║\00", align 1
@.str14 = private unnamed_addr constant [55 x i8] c"======================================================\00", align 1
@.str15 = private unnamed_addr constant [12 x i8] c"deep_tui.db\00", align 1
@.str16 = private unnamed_addr constant [221 x i8] c"You are Deep TUI, a terminal AI coding assistant built with the AIAL language. You are powered by DeepSeek-V4. You answer in the same language as the user. Be concise and accurate. Provide code blocks with language tags.\00", align 1
@.str17 = private unnamed_addr constant [42 x i8] c" Deep TUI | ^Q quit | ^L clear | ^D line \00", align 1
@.str18 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str19 = private unnamed_addr constant [6 x i8] c"/help\00", align 1
@.str20 = private unnamed_addr constant [95 x i8] c"Commands: /help  /key  /about  /clear  /quit\0A  ^Q quit  ^L clear  ^D multiline  ↑↓ history\00", align 1
@.str21 = private unnamed_addr constant [5 x i8] c"/key\00", align 1
@.str22 = private unnamed_addr constant [186 x i8] c"Set API key:\0A    /key set sk-your-key-here\0A\0A  The key is stored securely by the runtime.\0A\0A  Fallback: export DEEPSEEK_API_KEY=sk-...\0A\0A  Get a key: https://platform.deepseek.com/api_keys\00", align 1
@.str23 = private unnamed_addr constant [7 x i8] c"/about\00", align 1
@.str24 = private unnamed_addr constant [98 x i8] c"Deep TUI v0.3 — AIAL Terminal AI Chat\0A  github.com/gordonlu/aial\0A  github.com/gordonlu/deep-tui\00", align 1
@.str25 = private unnamed_addr constant [11 x i8] c"no_api_key\00", align 1
@.str26 = private unnamed_addr constant [219 x i8] c"No API key found.\0A\0A  Set in TUI: /key set sk-your-key-here\0A  Env var:    export DEEPSEEK_API_KEY=sk-...\0A  CLI:        aial key add --provider deepseek --key YOUR_KEY\0A\0A  Get a key: https://platform.deepseek.com/api_keys\00", align 1
@.str27 = private unnamed_addr constant [8 x i8] c"welcome\00", align 1
@.str28 = private unnamed_addr constant [72 x i8] c"Welcome! Type /help for commands. Use /key set to configure API access.\00", align 1
@.str29 = private unnamed_addr constant [241 x i8] c"────────────────────────────────────────────────────────────────────────────────\00", align 1
@.str30 = private unnamed_addr constant [65 x i8] c"  ^Q quit    ^L clear    ^D multiline    ↑↓ history    /help\00", align 1
@.str31 = private unnamed_addr constant [241 x i8] c"────────────────────────────────────────────────────────────────────────────────\00", align 1
@.str32 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str33 = private unnamed_addr constant [5 x i8] c"  > \00", align 1
@.str34 = private unnamed_addr constant [241 x i8] c"────────────────────────────────────────────────────────────────────────────────\00", align 1
@.str35 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str36 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str37 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str38 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str39 = private unnamed_addr constant [11 x i8] c"🧑 You: \00", align 1
@.str40 = private unnamed_addr constant [10 x i8] c"🤖 AI: \00", align 1
@.str41 = private unnamed_addr constant [10 x i8] c"  ── \00", align 1
@.str42 = private unnamed_addr constant [6 x i8] c"[31m\00", align 1
@.str43 = private unnamed_addr constant [14 x i8] c"  ✗ ERROR: \00", align 1
@.str44 = private unnamed_addr constant [11 x i8] c"     💡 \00", align 1
@.str45 = private unnamed_addr constant [19 x i8] c"[38;2;252;238;10m\00", align 1
@.str46 = private unnamed_addr constant [19 x i8] c"[38;2;245;230;20m\00", align 1
@.str47 = private unnamed_addr constant [19 x i8] c"[38;2;235;225;30m\00", align 1
@.str48 = private unnamed_addr constant [19 x i8] c"[38;2;220;220;40m\00", align 1
@.str49 = private unnamed_addr constant [19 x i8] c"[38;2;200;215;50m\00", align 1
@.str50 = private unnamed_addr constant [19 x i8] c"[38;2;180;215;60m\00", align 1
@.str51 = private unnamed_addr constant [19 x i8] c"[38;2;150;210;70m\00", align 1
@.str52 = private unnamed_addr constant [19 x i8] c"[38;2;120;210;80m\00", align 1
@.str53 = private unnamed_addr constant [18 x i8] c"[38;2;90;210;90m\00", align 1
@.str54 = private unnamed_addr constant [19 x i8] c"[38;2;60;210;100m\00", align 1
@.str55 = private unnamed_addr constant [19 x i8] c"[38;2;30;215;110m\00", align 1
@.str56 = private unnamed_addr constant [18 x i8] c"[38;2;0;220;120m\00", align 1
@.str57 = private unnamed_addr constant [18 x i8] c"[38;2;57;255;20m\00", align 1
@.str58 = private unnamed_addr constant [18 x i8] c"[38;2;57;255;20m\00", align 1
@.str59 = private unnamed_addr constant [154 x i8] c"██████╗ ███████╗███████╗██████╗     ████████╗██╗   ██╗██╗\00", align 1
@.str60 = private unnamed_addr constant [158 x i8] c"██╔══██╗██╔════╝██╔════╝██╔══██╗    ╚══██╔══╝██║   ██║██║\00", align 1
@.str61 = private unnamed_addr constant [134 x i8] c"██║  ██║█████╗  █████╗  ██████╔╝       ██║   ██║   ██║██║\00", align 1
@.str62 = private unnamed_addr constant [132 x i8] c"██║  ██║██╔══╝  ██╔══╝  ██╔═══╝        ██║   ██║   ██║██║\00", align 1
@.str63 = private unnamed_addr constant [142 x i8] c"██████╔╝███████╗███████╗██║            ██║   ╚██████╔╝██║\00", align 1
@.str64 = private unnamed_addr constant [136 x i8] c"╚═════╝ ╚══════╝╚══════╝╚═╝            ╚═╝    ╚═════╝ ╚═╝\00", align 1
@.str65 = private unnamed_addr constant [58 x i8] c"                                                         \00", align 1
@.str66 = private unnamed_addr constant [163 x i8] c"████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██╗ █████╗ ██╗\00", align 1
@.str67 = private unnamed_addr constant [175 x i8] c"╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║██╔══██╗██║\00", align 1
@.str68 = private unnamed_addr constant [163 x i8] c"   ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║███████║██║\00", align 1
@.str69 = private unnamed_addr constant [165 x i8] c"   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║██╔══██║██║\00", align 1
@.str70 = private unnamed_addr constant [170 x i8] c"   ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║██║  ██║███████╗\00", align 1
@.str71 = private unnamed_addr constant [162 x i8] c"   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝\00", align 1
@.str72 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str73 = private unnamed_addr constant [81 x i8] c"     DEEP TUI  —  AIAL Terminal AI Chat   |   v0.4   |   build 2026-05-13-1545\00", align 1
@.str74 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str75 = private unnamed_addr constant [3 x i8] c"7\00", align 1
@.str76 = private unnamed_addr constant [9 x i8] c"[999;1H\00", align 1
@.str77 = private unnamed_addr constant [13 x i8] c" Deep TUI | \00", align 1
@.str78 = private unnamed_addr constant [26 x i8] c" | msgs: ~N | tokens: ~N \00", align 1
@.str79 = private unnamed_addr constant [4 x i8] c"[K\00", align 1
@.str80 = private unnamed_addr constant [3 x i8] c"8\00", align 1
@.str81 = private unnamed_addr constant [6 x i8] c"ENTER\00", align 1
@.str82 = private unnamed_addr constant [10 x i8] c"BACKSPACE\00", align 1
@.str83 = private unnamed_addr constant [4 x i8] c"ESC\00", align 1
@.str84 = private unnamed_addr constant [3 x i8] c"UP\00", align 1
@.str85 = private unnamed_addr constant [5 x i8] c"DOWN\00", align 1
@.str86 = private unnamed_addr constant [5 x i8] c"LEFT\00", align 1
@.str87 = private unnamed_addr constant [6 x i8] c"RIGHT\00", align 1
@.str88 = private unnamed_addr constant [7 x i8] c"CTRL_Q\00", align 1
@.str89 = private unnamed_addr constant [7 x i8] c"CTRL_L\00", align 1
@.str90 = private unnamed_addr constant [7 x i8] c"CTRL_D\00", align 1
@.str91 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str92 = private unnamed_addr constant [48 x i8] c"Run /key for API setup, /help for all commands.\00", align 1
@.str93 = private unnamed_addr constant [9 x i8] c"deepseek\00", align 1
@.str94 = private unnamed_addr constant [22 x i8] c"API key found. Ready!\00", align 1
@.str95 = private unnamed_addr constant [23 x i8] c"No API key configured.\00", align 1
@.str96 = private unnamed_addr constant [52 x i8] c"Type /key set YOUR_KEY  or  export DEEPSEEK_API_KEY\00", align 1
@.str97 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str98 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str99 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str100 = private unnamed_addr constant [48 x i8] c"multi-line ON — Enter for newline, ^D to send\00", align 1
@.str101 = private unnamed_addr constant [15 x i8] c"multi-line OFF\00", align 1
@.str102 = private unnamed_addr constant [9 x i8] c"\0D[K  > \00", align 1
@.str103 = private unnamed_addr constant [9 x i8] c"\0D[K  > \00", align 1
@.str104 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str105 = private unnamed_addr constant [5 x i8] c"\0D[K\00", align 1
@.str106 = private unnamed_addr constant [5 x i8] c"  > \00", align 1
@.str107 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str108 = private unnamed_addr constant [4 x i8] c"\0A+ \00", align 1
@.str109 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str110 = private unnamed_addr constant [10 x i8] c"/key set \00", align 1
@.str111 = private unnamed_addr constant [9 x i8] c"deepseek\00", align 1
@.str112 = private unnamed_addr constant [41 x i8] c"Key saved securely. Restart recommended.\00", align 1
@.str113 = private unnamed_addr constant [20 x i8] c"Failed to save key.\00", align 1
@.str114 = private unnamed_addr constant [33 x i8] c"Usage: /key set sk-your-key-here\00", align 1
@.str115 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str116 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str117 = private unnamed_addr constant [6 x i8] c"/quit\00", align 1
@.str118 = private unnamed_addr constant [7 x i8] c"/clear\00", align 1
@.str119 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str120 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str121 = private unnamed_addr constant [9 x i8] c"\0D[1A[K\00", align 1
@.str122 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str123 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str124 = private unnamed_addr constant [5 x i8] c"user\00", align 1
@.str125 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str126 = private unnamed_addr constant [8 x i8] c"[error:\00", align 1
@.str127 = private unnamed_addr constant [7 x i8] c"[error\00", align 1
@.str128 = private unnamed_addr constant [7 x i8] c"Error:\00", align 1
@.str129 = private unnamed_addr constant [7 x i8] c"error:\00", align 1
@.str130 = private unnamed_addr constant [8 x i8] c"{\22error\00", align 1
@.str131 = private unnamed_addr constant [19 x i8] c"[error: no API key\00", align 1
@.str132 = private unnamed_addr constant [18 x i8] c"Error: No API key\00", align 1
@.str133 = private unnamed_addr constant [11 x i8] c"No API key\00", align 1
@.str134 = private unnamed_addr constant [11 x i8] c"no_api_key\00", align 1
@.str135 = private unnamed_addr constant [50 x i8] c"No response from API. Check your key and network.\00", align 1
@.str136 = private unnamed_addr constant [19 x i8] c"Error: no response\00", align 1
@.str137 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str138 = private unnamed_addr constant [7 x i8] c"Error:\00", align 1
@.str139 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str140 = private unnamed_addr constant [10 x i8] c"assistant\00", align 1
@.str141 = private unnamed_addr constant [5 x i8] c"user\00", align 1
@.str142 = private unnamed_addr constant [10 x i8] c"assistant\00", align 1
@.str143 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str144 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str145 = private unnamed_addr constant [8 x i8] c"[D [D\00", align 1
@.str146 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str147 = private unnamed_addr constant [24 x i8] c"Session saved. Goodbye!\00", align 1
@.str148 = private unnamed_addr constant [5 x i8] c"main\00", align 1

define i64 @theme_reset() {

b0:
  %v0 = add i64 0, 0
  ret i64 %v0
}

define i64 @theme_cyan() {

b2:
  %v0 = add i64 0, 1
  ret i64 %v0
}

define i64 @theme_yellow() {

b4:
  %v0 = add i64 0, 2
  ret i64 %v0
}

define i64 @theme_green() {

b6:
  %v0 = add i64 0, 3
  ret i64 %v0
}

define i64 @theme_magenta() {

b8:
  %v0 = add i64 0, 4
  ret i64 %v0
}

define i64 @theme_dim() {

b10:
  %v0 = add i64 0, 5
  ret i64 %v0
}

define i64 @theme_bold() {

b12:
  %v0 = add i64 0, 6
  ret i64 %v0
}

define i64 @color_ai() {

b14:
  %v0 = call i64 @theme_cyan()
  ret i64 %v0
}

define i64 @color_user() {

b16:
  %v0 = call i64 @theme_yellow()
  ret i64 %v0
}

define i64 @color_system() {

b18:
  %v0 = call i64 @theme_dim()
  ret i64 %v0
}

define i64 @color_title() {

b20:
  %v0 = call i64 @theme_cyan()
  ret i64 %v0
}

define i64 @color_border() {

b22:
  %v0 = call i64 @theme_cyan()
  ret i64 %v0
}

define i64 @color_input() {

b24:
  %v0 = call i64 @theme_green()
  ret i64 %v0
}

define i64 @color_bar() {

b26:
  %v0 = add i64 0, 7
  ret i64 %v0
}

define i64 @color_bar_reset() {

b28:
  %v0 = call i64 @theme_reset()
  ret i64 %v0
}

define i64 @box_tl() {

b30:
  %v0 = add i64 0, 8
  ret i64 %v0
}

define i64 @box_tr() {

b32:
  %v0 = add i64 0, 9
  ret i64 %v0
}

define i64 @box_bl() {

b34:
  %v0 = add i64 0, 10
  ret i64 %v0
}

define i64 @box_br() {

b36:
  %v0 = add i64 0, 11
  ret i64 %v0
}

define i64 @box_h() {

b38:
  %v0 = add i64 0, 12
  ret i64 %v0
}

define i64 @box_v() {

b40:
  %v0 = add i64 0, 13
  ret i64 %v0
}

define i64 @hr() {

b42:
  %v0 = add i64 0, 14
  ret i64 %v0
}

define i64 @box_top() {

b44:
  %v0 = call i64 @box_tl()
  %v1 = call i64 @hr()
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  %v3 = call i64 @box_tr()
  %v4 = call i64 @aial_rt_strcat(i64 %v2, i64 %v3)
  ret i64 %v4
}

define i64 @box_bot() {

b46:
  %v0 = call i64 @box_bl()
  %v1 = call i64 @hr()
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  %v3 = call i64 @box_br()
  %v4 = call i64 @aial_rt_strcat(i64 %v2, i64 %v3)
  ret i64 %v4
}

define i64 @box_line(i64 %arg0) {

b48:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %v0 = call i64 @box_v()
  %lptr66 = inttoptr i64 %arg0_ptr to i64*
  %v1 = load i64, i64* %lptr66
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  %v3 = call i64 @box_v()
  %v4 = call i64 @aial_rt_strcat(i64 %v2, i64 %v3)
  ret i64 %v4
}

define i64 @colored(i64 %arg0, i64 %arg1) {

b50:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %lptr70 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr70
  %lptr71 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr71
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  %v3 = call i64 @theme_reset()
  %v4 = call i64 @aial_rt_strcat(i64 %v2, i64 %v3)
  ret i64 %v4
}

define i64 @mem_open() {

b52:
  %v0 = add i64 0, 15
  %v1 = call i64 @aial_rt_ctx_open_memory(i64 %v0)
  ret i64 %v1
}

define void @mem_save(i64 %arg0, i64 %arg1, i64 %arg2, i64 %arg3) {

b54:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %arg2_addr = alloca i64
  store i64 %arg2, i64* %arg2_addr
  %arg2_ptr = ptrtoint i64* %arg2_addr to i64
  %arg3_addr = alloca i64
  store i64 %arg3, i64* %arg3_addr
  %arg3_ptr = ptrtoint i64* %arg3_addr to i64
  %lptr77 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr77
  %lptr78 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr78
  %lptr79 = inttoptr i64 %arg2_ptr to i64*
  %v2 = load i64, i64* %lptr79
  %lptr80 = inttoptr i64 %arg3_ptr to i64*
  %v3 = load i64, i64* %lptr80
  call void @aial_rt_ctx_save_message(i64 %v0, i64 %v1, i64 %v2, i64 %v3)
  %v4 = add i64 0, 0
  ret void
}

define i64 @mem_load(i64 %arg0, i64 %arg1, i64 %arg2) {

b55:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %arg2_addr = alloca i64
  store i64 %arg2, i64* %arg2_addr
  %arg2_ptr = ptrtoint i64* %arg2_addr to i64
  %lptr82 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr82
  %lptr83 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr83
  %lptr84 = inttoptr i64 %arg2_ptr to i64*
  %v2 = load i64, i64* %lptr84
  %v3 = call i64 @aial_rt_ctx_load_messages(i64 %v0, i64 %v1, i64 %v2)
  ret i64 %v3
}

define void @mem_close(i64 %arg0) {

b57:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %lptr86 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr86
  call void @aial_rt_ctx_close_memory(i64 %v0)
  %v1 = add i64 0, 0
  ret void
}

define i64 @chat_system_prompt() {

b58:
  %v0 = add i64 0, 16
  ret i64 %v0
}

define i64 @chat_context_new() {

b60:
  %v0 = add i64 0, 18
  %v1 = add i64 0, 4096
  %v2 = add i64 0, 18
  %v3 = add i64 0, 0
  %v4 = call i64 @chat_system_prompt()
  %v5 = add i64 0, 32768
  %v6 = call i64 @aial_rt_ctx_new(i64 %v4, i64 %v5, i64 %v2, i64 %v3)
  ret i64 %v6
}

define i64 @chat_send(i64 %arg0, i64 %arg1) {

b62:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %v0 = add i64 0, 0
  %v1 = add i64 0, 0
  %v2 = add i64 0, 18
  %v3 = fadd double 0.0, 0.000000
  %v4 = add i64 0, 0
  %v5 = add i64 0, 0
  %v6 = add i64 0, 0
  %lptr103 = inttoptr i64 %arg0_ptr to i64*
  %v7 = load i64, i64* %lptr103
  %lptr104 = inttoptr i64 %arg1_ptr to i64*
  %v8 = load i64, i64* %lptr104
  %v9 = add i1 0, 1
  %v10 = add i64 0, 1024
  %v11 = fadd double 0.0, 1.000000
  %v12 = call i64 @aial_rt_ai_stream_start(i64 %v6, i64 %v7, i64 %v8, double %v11, i64 %v10, i64 %v5)
  ret i64 %v12
}

define i64 @chat_read_token(i64 %arg0) {

b64:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %lptr109 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr109
  %v1 = call i64 @aial_rt_ai_stream_read(i64 %v0)
  ret i64 %v1
}

define i64 @stats_bar_text(i64 %arg0) {

b66:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %v0 = add i64 0, 17
  ret i64 %v0
}

define i64 @cmd_cache_new() {

b68:
  %v0 = call i64 @aial_rt_map_new()
  ret i64 %v0
}

define i64 @cmd_cache_get(i64 %arg0, i64 %arg1) {

b70:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %lptr113 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr113
  %lptr114 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr114
  %x2 = call i64 @aial_rt_map_has(i64 %v0, i64 %v1)
  %v2 = trunc i64 %x2 to i1
  br i1 %v2, label %b71, label %b72

b71:
  %lptr116 = inttoptr i64 %arg0_ptr to i64*
  %v3 = load i64, i64* %lptr116
  %lptr117 = inttoptr i64 %arg1_ptr to i64*
  %v4 = load i64, i64* %lptr117
  %v5 = call i64 @aial_rt_map_get(i64 %v3, i64 %v4)
  ret i64 %v5

b73:
  %v6 = add i64 0, 18
  ret i64 %v6

b72:
  br label %b73

b74:
  br label %b73
}

define void @cmd_cache_set(i64 %arg0, i64 %arg1, i64 %arg2) {

b76:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %arg2_addr = alloca i64
  store i64 %arg2, i64* %arg2_addr
  %arg2_ptr = ptrtoint i64* %arg2_addr to i64
  %lptr120 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr120
  %lptr121 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr121
  %lptr122 = inttoptr i64 %arg2_ptr to i64*
  %v2 = load i64, i64* %lptr122
  call void @aial_rt_map_set(i64 %v0, i64 %v1, i64 %v2)
  %v3 = add i64 0, 0
  ret void
}

define void @cmd_cache_populate(i64 %arg0) {

b77:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %lptr124 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr124
  %v1 = add i64 0, 19
  %v2 = add i64 0, 20
  call void @aial_rt_map_set(i64 %v0, i64 %v1, i64 %v2)
  %v3 = add i64 0, 0
  %lptr128 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr128
  %v5 = add i64 0, 21
  %v6 = add i64 0, 22
  call void @aial_rt_map_set(i64 %v4, i64 %v5, i64 %v6)
  %v7 = add i64 0, 0
  %lptr132 = inttoptr i64 %arg0_ptr to i64*
  %v8 = load i64, i64* %lptr132
  %v9 = add i64 0, 23
  %v10 = add i64 0, 24
  call void @aial_rt_map_set(i64 %v8, i64 %v9, i64 %v10)
  %v11 = add i64 0, 0
  %lptr136 = inttoptr i64 %arg0_ptr to i64*
  %v12 = load i64, i64* %lptr136
  %v13 = add i64 0, 25
  %v14 = add i64 0, 26
  call void @aial_rt_map_set(i64 %v12, i64 %v13, i64 %v14)
  %v15 = add i64 0, 0
  %lptr140 = inttoptr i64 %arg0_ptr to i64*
  %v16 = load i64, i64* %lptr140
  %v17 = add i64 0, 27
  %v18 = add i64 0, 28
  call void @aial_rt_map_set(i64 %v16, i64 %v17, i64 %v18)
  %v19 = add i64 0, 0
  ret void
}

define void @header_draw() {

b78:
  %v0 = call i64 @theme_dim()
  %aptr145 = alloca i64
  %v1 = ptrtoint i64* %aptr145 to i64
  %sptr146 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr146
  %v2 = add i64 0, 0
  %v3 = call i64 @theme_reset()
  %aptr148 = alloca i64
  %v4 = ptrtoint i64* %aptr148 to i64
  %sptr149 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr149
  %v5 = add i64 0, 0
  %v6 = add i64 0, 29
  %aptr151 = alloca i64
  %v7 = ptrtoint i64* %aptr151 to i64
  %sptr152 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr152
  %v8 = add i64 0, 0
  %lptr153 = inttoptr i64 %v1 to i64*
  %v9 = load i64, i64* %lptr153
  %lptr154 = inttoptr i64 %v7 to i64*
  %v10 = load i64, i64* %lptr154
  %v11 = call i64 @aial_rt_strcat(i64 %v9, i64 %v10)
  %lptr156 = inttoptr i64 %v4 to i64*
  %v12 = load i64, i64* %lptr156
  %v13 = call i64 @aial_rt_strcat(i64 %v11, i64 %v12)
  call void @aial_rt_println(i64 %v13)
  %v14 = add i64 0, 0
  %v15 = add i64 0, 30
  call void @aial_rt_println(i64 %v15)
  %v16 = add i64 0, 0
  %lptr161 = inttoptr i64 %v1 to i64*
  %v17 = load i64, i64* %lptr161
  %lptr162 = inttoptr i64 %v7 to i64*
  %v18 = load i64, i64* %lptr162
  %v19 = call i64 @aial_rt_strcat(i64 %v17, i64 %v18)
  %lptr164 = inttoptr i64 %v4 to i64*
  %v20 = load i64, i64* %lptr164
  %v21 = call i64 @aial_rt_strcat(i64 %v19, i64 %v20)
  call void @aial_rt_println(i64 %v21)
  %v22 = add i64 0, 0
  ret void
}

define void @input_draw_border() {

b79:
  %v0 = call i64 @theme_dim()
  %aptr168 = alloca i64
  %v1 = ptrtoint i64* %aptr168 to i64
  %sptr169 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr169
  %v2 = add i64 0, 0
  %v3 = call i64 @color_input()
  %aptr171 = alloca i64
  %v4 = ptrtoint i64* %aptr171 to i64
  %sptr172 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr172
  %v5 = add i64 0, 0
  %v6 = call i64 @theme_reset()
  %aptr174 = alloca i64
  %v7 = ptrtoint i64* %aptr174 to i64
  %sptr175 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr175
  %v8 = add i64 0, 0
  %v9 = add i64 0, 29
  %aptr177 = alloca i64
  %v10 = ptrtoint i64* %aptr177 to i64
  %sptr178 = inttoptr i64 %v10 to i64*
  store i64 %v9, i64* %sptr178
  %v11 = add i64 0, 0
  %v12 = add i64 0, 32
  %lptr180 = inttoptr i64 %v1 to i64*
  %v13 = load i64, i64* %lptr180
  %v14 = call i64 @aial_rt_strcat(i64 %v12, i64 %v13)
  %lptr182 = inttoptr i64 %v10 to i64*
  %v15 = load i64, i64* %lptr182
  %v16 = call i64 @aial_rt_strcat(i64 %v14, i64 %v15)
  %lptr184 = inttoptr i64 %v7 to i64*
  %v17 = load i64, i64* %lptr184
  %v18 = call i64 @aial_rt_strcat(i64 %v16, i64 %v17)
  call void @aial_rt_println(i64 %v18)
  %v19 = add i64 0, 0
  %lptr187 = inttoptr i64 %v4 to i64*
  %v20 = load i64, i64* %lptr187
  %v21 = add i64 0, 33
  %v22 = call i64 @aial_rt_strcat(i64 %v20, i64 %v21)
  %lptr190 = inttoptr i64 %v7 to i64*
  %v23 = load i64, i64* %lptr190
  %v24 = call i64 @aial_rt_strcat(i64 %v22, i64 %v23)
  call void @aial_rt_print(i64 %v24)
  %v25 = add i64 0, 0
  ret void
}

define void @input_draw_bottom() {

b80:
  %v0 = call i64 @theme_dim()
  %aptr194 = alloca i64
  %v1 = ptrtoint i64* %aptr194 to i64
  %sptr195 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr195
  %v2 = add i64 0, 0
  %v3 = call i64 @theme_reset()
  %aptr197 = alloca i64
  %v4 = ptrtoint i64* %aptr197 to i64
  %sptr198 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr198
  %v5 = add i64 0, 0
  %v6 = add i64 0, 29
  %aptr200 = alloca i64
  %v7 = ptrtoint i64* %aptr200 to i64
  %sptr201 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr201
  %v8 = add i64 0, 0
  %v9 = add i64 0, 18
  call void @aial_rt_println(i64 %v9)
  %v10 = add i64 0, 0
  %lptr204 = inttoptr i64 %v1 to i64*
  %v11 = load i64, i64* %lptr204
  %lptr205 = inttoptr i64 %v7 to i64*
  %v12 = load i64, i64* %lptr205
  %v13 = call i64 @aial_rt_strcat(i64 %v11, i64 %v12)
  %lptr207 = inttoptr i64 %v4 to i64*
  %v14 = load i64, i64* %lptr207
  %v15 = call i64 @aial_rt_strcat(i64 %v13, i64 %v14)
  call void @aial_rt_println(i64 %v15)
  %v16 = add i64 0, 0
  ret void
}

define void @history_push(i64 %arg0, i64 %arg1) {

b81:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %lptr210 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr210
  %lptr211 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr211
  call void @aial_rt_array_push(i64 %v0, i64 %v1)
  %v2 = add i64 0, 0
  ret void
}

define i64 @history_recall(i64 %arg0, i64 %arg1) {

b82:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %lptr213 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr213
  %v1 = call i64 @aial_rt_array_len(i64 %v0)
  %aptr215 = alloca i64
  %v2 = ptrtoint i64* %aptr215 to i64
  %sptr216 = inttoptr i64 %v2 to i64*
  store i64 %v1, i64* %sptr216
  %v3 = add i64 0, 0
  %lptr217 = inttoptr i64 %v2 to i64*
  %v4 = load i64, i64* %lptr217
  %v5 = add i64 0, 0
  %v6 = icmp eq i64 %v4, %v5
  br i1 %v6, label %b83, label %b84

b83:
  %v7 = add i64 0, 18
  ret i64 %v7

b85:
  %lptr221 = inttoptr i64 %arg1_ptr to i64*
  %v8 = load i64, i64* %lptr221
  %v9 = add i64 0, 0
  %v10 = icmp slt i64 %v8, %v9
  br i1 %v10, label %b87, label %b88

b87:
  %v11 = add i64 0, 18
  ret i64 %v11

b89:
  %lptr225 = inttoptr i64 %arg1_ptr to i64*
  %v12 = load i64, i64* %lptr225
  %lptr226 = inttoptr i64 %v2 to i64*
  %v13 = load i64, i64* %lptr226
  %v14 = icmp sge i64 %v12, %v13
  br i1 %v14, label %b91, label %b92

b91:
  %v15 = add i64 0, 18
  ret i64 %v15

b93:
  %lptr229 = inttoptr i64 %arg0_ptr to i64*
  %v16 = load i64, i64* %lptr229
  %lptr230 = inttoptr i64 %arg1_ptr to i64*
  %v17 = load i64, i64* %lptr230
  %v18 = call i64 @aial_rt_array_get(i64 %v16, i64 %v17)
  ret i64 %v18

b84:
  br label %b85

b86:
  br label %b85

b88:
  br label %b89

b90:
  br label %b89

b92:
  br label %b93

b94:
  br label %b93
}

define i64 @history_size(i64 %arg0) {

b96:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %lptr232 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr232
  %v1 = call i64 @aial_rt_array_len(i64 %v0)
  ret i64 %v1
}

define void @chat_show_user(i64 %arg0) {

b98:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %v0 = call i64 @color_user()
  %v1 = add i64 0, 39
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  %v3 = call i64 @theme_reset()
  %v4 = call i64 @aial_rt_strcat(i64 %v2, i64 %v3)
  call void @aial_rt_print(i64 %v4)
  %v5 = add i64 0, 0
  %lptr240 = inttoptr i64 %arg0_ptr to i64*
  %v6 = load i64, i64* %lptr240
  call void @aial_rt_println(i64 %v6)
  %v7 = add i64 0, 0
  ret void
}

define void @chat_show_ai_prefix() {

b99:
  %v0 = call i64 @color_ai()
  %v1 = add i64 0, 40
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  %v3 = call i64 @theme_reset()
  %v4 = call i64 @aial_rt_strcat(i64 %v2, i64 %v3)
  call void @aial_rt_print(i64 %v4)
  %v5 = add i64 0, 0
  ret void
}

define void @chat_show_system(i64 %arg0) {

b100:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %v0 = call i64 @color_system()
  %v1 = add i64 0, 41
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  call void @aial_rt_print(i64 %v2)
  %v3 = add i64 0, 0
  %lptr252 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr252
  call void @aial_rt_print(i64 %v4)
  %v5 = add i64 0, 0
  %v6 = call i64 @theme_reset()
  call void @aial_rt_println(i64 %v6)
  %v7 = add i64 0, 0
  ret void
}

define void @chat_show_error(i64 %arg0) {

b101:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %v0 = add i64 0, 42
  %v1 = add i64 0, 43
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  call void @aial_rt_print(i64 %v2)
  %v3 = add i64 0, 0
  %lptr260 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr260
  call void @aial_rt_print(i64 %v4)
  %v5 = add i64 0, 0
  %v6 = call i64 @theme_reset()
  call void @aial_rt_println(i64 %v6)
  %v7 = add i64 0, 0
  ret void
}

define void @chat_show_hint(i64 %arg0) {

b102:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %v0 = call i64 @theme_dim()
  %v1 = add i64 0, 44
  %v2 = call i64 @aial_rt_strcat(i64 %v0, i64 %v1)
  call void @aial_rt_print(i64 %v2)
  %v3 = add i64 0, 0
  %lptr268 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr268
  call void @aial_rt_print(i64 %v4)
  %v5 = add i64 0, 0
  %v6 = call i64 @theme_reset()
  call void @aial_rt_println(i64 %v6)
  %v7 = add i64 0, 0
  ret void
}

define void @chat_show_welcome() {

b103:
  %v0 = call i64 @theme_reset()
  %aptr273 = alloca i64
  %v1 = ptrtoint i64* %aptr273 to i64
  %sptr274 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr274
  %v2 = add i64 0, 0
  %v3 = add i64 0, 45
  %aptr276 = alloca i64
  %v4 = ptrtoint i64* %aptr276 to i64
  %sptr277 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr277
  %v5 = add i64 0, 0
  %v6 = add i64 0, 46
  %aptr279 = alloca i64
  %v7 = ptrtoint i64* %aptr279 to i64
  %sptr280 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr280
  %v8 = add i64 0, 0
  %v9 = add i64 0, 47
  %aptr282 = alloca i64
  %v10 = ptrtoint i64* %aptr282 to i64
  %sptr283 = inttoptr i64 %v10 to i64*
  store i64 %v9, i64* %sptr283
  %v11 = add i64 0, 0
  %v12 = add i64 0, 48
  %aptr285 = alloca i64
  %v13 = ptrtoint i64* %aptr285 to i64
  %sptr286 = inttoptr i64 %v13 to i64*
  store i64 %v12, i64* %sptr286
  %v14 = add i64 0, 0
  %v15 = add i64 0, 49
  %aptr288 = alloca i64
  %v16 = ptrtoint i64* %aptr288 to i64
  %sptr289 = inttoptr i64 %v16 to i64*
  store i64 %v15, i64* %sptr289
  %v17 = add i64 0, 0
  %v18 = add i64 0, 50
  %aptr291 = alloca i64
  %v19 = ptrtoint i64* %aptr291 to i64
  %sptr292 = inttoptr i64 %v19 to i64*
  store i64 %v18, i64* %sptr292
  %v20 = add i64 0, 0
  %v21 = add i64 0, 51
  %aptr294 = alloca i64
  %v22 = ptrtoint i64* %aptr294 to i64
  %sptr295 = inttoptr i64 %v22 to i64*
  store i64 %v21, i64* %sptr295
  %v23 = add i64 0, 0
  %v24 = add i64 0, 52
  %aptr297 = alloca i64
  %v25 = ptrtoint i64* %aptr297 to i64
  %sptr298 = inttoptr i64 %v25 to i64*
  store i64 %v24, i64* %sptr298
  %v26 = add i64 0, 0
  %v27 = add i64 0, 53
  %aptr300 = alloca i64
  %v28 = ptrtoint i64* %aptr300 to i64
  %sptr301 = inttoptr i64 %v28 to i64*
  store i64 %v27, i64* %sptr301
  %v29 = add i64 0, 0
  %v30 = add i64 0, 54
  %aptr303 = alloca i64
  %v31 = ptrtoint i64* %aptr303 to i64
  %sptr304 = inttoptr i64 %v31 to i64*
  store i64 %v30, i64* %sptr304
  %v32 = add i64 0, 0
  %v33 = add i64 0, 55
  %aptr306 = alloca i64
  %v34 = ptrtoint i64* %aptr306 to i64
  %sptr307 = inttoptr i64 %v34 to i64*
  store i64 %v33, i64* %sptr307
  %v35 = add i64 0, 0
  %v36 = add i64 0, 56
  %aptr309 = alloca i64
  %v37 = ptrtoint i64* %aptr309 to i64
  %sptr310 = inttoptr i64 %v37 to i64*
  store i64 %v36, i64* %sptr310
  %v38 = add i64 0, 0
  %v39 = add i64 0, 57
  %aptr312 = alloca i64
  %v40 = ptrtoint i64* %aptr312 to i64
  %sptr313 = inttoptr i64 %v40 to i64*
  store i64 %v39, i64* %sptr313
  %v41 = add i64 0, 0
  %v42 = add i64 0, 57
  %aptr315 = alloca i64
  %v43 = ptrtoint i64* %aptr315 to i64
  %sptr316 = inttoptr i64 %v43 to i64*
  store i64 %v42, i64* %sptr316
  %v44 = add i64 0, 0
  %lptr317 = inttoptr i64 %v4 to i64*
  %v45 = load i64, i64* %lptr317
  %v46 = add i64 0, 59
  %v47 = call i64 @aial_rt_strcat(i64 %v45, i64 %v46)
  call void @aial_rt_println(i64 %v47)
  %v48 = add i64 0, 0
  %lptr321 = inttoptr i64 %v7 to i64*
  %v49 = load i64, i64* %lptr321
  %v50 = add i64 0, 60
  %v51 = call i64 @aial_rt_strcat(i64 %v49, i64 %v50)
  call void @aial_rt_println(i64 %v51)
  %v52 = add i64 0, 0
  %lptr325 = inttoptr i64 %v10 to i64*
  %v53 = load i64, i64* %lptr325
  %v54 = add i64 0, 61
  %v55 = call i64 @aial_rt_strcat(i64 %v53, i64 %v54)
  call void @aial_rt_println(i64 %v55)
  %v56 = add i64 0, 0
  %lptr329 = inttoptr i64 %v13 to i64*
  %v57 = load i64, i64* %lptr329
  %v58 = add i64 0, 62
  %v59 = call i64 @aial_rt_strcat(i64 %v57, i64 %v58)
  call void @aial_rt_println(i64 %v59)
  %v60 = add i64 0, 0
  %lptr333 = inttoptr i64 %v16 to i64*
  %v61 = load i64, i64* %lptr333
  %v62 = add i64 0, 63
  %v63 = call i64 @aial_rt_strcat(i64 %v61, i64 %v62)
  call void @aial_rt_println(i64 %v63)
  %v64 = add i64 0, 0
  %lptr337 = inttoptr i64 %v19 to i64*
  %v65 = load i64, i64* %lptr337
  %v66 = add i64 0, 64
  %v67 = call i64 @aial_rt_strcat(i64 %v65, i64 %v66)
  call void @aial_rt_println(i64 %v67)
  %v68 = add i64 0, 0
  %lptr341 = inttoptr i64 %v22 to i64*
  %v69 = load i64, i64* %lptr341
  %v70 = add i64 0, 65
  %v71 = call i64 @aial_rt_strcat(i64 %v69, i64 %v70)
  call void @aial_rt_println(i64 %v71)
  %v72 = add i64 0, 0
  %lptr345 = inttoptr i64 %v25 to i64*
  %v73 = load i64, i64* %lptr345
  %v74 = add i64 0, 66
  %v75 = call i64 @aial_rt_strcat(i64 %v73, i64 %v74)
  call void @aial_rt_println(i64 %v75)
  %v76 = add i64 0, 0
  %lptr349 = inttoptr i64 %v28 to i64*
  %v77 = load i64, i64* %lptr349
  %v78 = add i64 0, 67
  %v79 = call i64 @aial_rt_strcat(i64 %v77, i64 %v78)
  call void @aial_rt_println(i64 %v79)
  %v80 = add i64 0, 0
  %lptr353 = inttoptr i64 %v31 to i64*
  %v81 = load i64, i64* %lptr353
  %v82 = add i64 0, 68
  %v83 = call i64 @aial_rt_strcat(i64 %v81, i64 %v82)
  call void @aial_rt_println(i64 %v83)
  %v84 = add i64 0, 0
  %lptr357 = inttoptr i64 %v34 to i64*
  %v85 = load i64, i64* %lptr357
  %v86 = add i64 0, 69
  %v87 = call i64 @aial_rt_strcat(i64 %v85, i64 %v86)
  call void @aial_rt_println(i64 %v87)
  %v88 = add i64 0, 0
  %lptr361 = inttoptr i64 %v37 to i64*
  %v89 = load i64, i64* %lptr361
  %v90 = add i64 0, 70
  %v91 = call i64 @aial_rt_strcat(i64 %v89, i64 %v90)
  call void @aial_rt_println(i64 %v91)
  %v92 = add i64 0, 0
  %lptr365 = inttoptr i64 %v40 to i64*
  %v93 = load i64, i64* %lptr365
  %v94 = add i64 0, 71
  %lptr367 = inttoptr i64 %v1 to i64*
  %v95 = load i64, i64* %lptr367
  %v96 = call i64 @aial_rt_strcat(i64 %v94, i64 %v95)
  %v97 = call i64 @aial_rt_strcat(i64 %v93, i64 %v96)
  call void @aial_rt_println(i64 %v97)
  %v98 = add i64 0, 0
  %v99 = add i64 0, 18
  call void @aial_rt_println(i64 %v99)
  %v100 = add i64 0, 0
  %v101 = call i64 @color_ai()
  %v102 = add i64 0, 73
  %v103 = call i64 @aial_rt_strcat(i64 %v101, i64 %v102)
  %lptr376 = inttoptr i64 %v1 to i64*
  %v104 = load i64, i64* %lptr376
  %v105 = call i64 @aial_rt_strcat(i64 %v103, i64 %v104)
  call void @aial_rt_println(i64 %v105)
  %v106 = add i64 0, 0
  %v107 = add i64 0, 18
  call void @aial_rt_println(i64 %v107)
  %v108 = add i64 0, 0
  ret void
}

define void @bar_draw(i64 %arg0, i64 %arg1, i64 %arg2) {

b104:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %arg1_addr = alloca i64
  store i64 %arg1, i64* %arg1_addr
  %arg1_ptr = ptrtoint i64* %arg1_addr to i64
  %arg2_addr = alloca i64
  store i64 %arg2, i64* %arg2_addr
  %arg2_ptr = ptrtoint i64* %arg2_addr to i64
  %v0 = call i64 @color_bar()
  %aptr382 = alloca i64
  %v1 = ptrtoint i64* %aptr382 to i64
  %sptr383 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr383
  %v2 = add i64 0, 0
  %v3 = call i64 @color_bar_reset()
  %aptr385 = alloca i64
  %v4 = ptrtoint i64* %aptr385 to i64
  %sptr386 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr386
  %v5 = add i64 0, 0
  %v6 = add i64 0, 75
  call void @aial_rt_print(i64 %v6)
  %v7 = add i64 0, 0
  %v8 = add i64 0, 76
  call void @aial_rt_print(i64 %v8)
  %v9 = add i64 0, 0
  %lptr391 = inttoptr i64 %v1 to i64*
  %v10 = load i64, i64* %lptr391
  %v11 = add i64 0, 77
  %v12 = call i64 @aial_rt_strcat(i64 %v10, i64 %v11)
  call void @aial_rt_print(i64 %v12)
  %v13 = add i64 0, 0
  %lptr395 = inttoptr i64 %arg0_ptr to i64*
  %v14 = load i64, i64* %lptr395
  call void @aial_rt_print(i64 %v14)
  %v15 = add i64 0, 0
  %v16 = add i64 0, 78
  call void @aial_rt_print(i64 %v16)
  %v17 = add i64 0, 0
  %v18 = add i64 0, 79
  %lptr400 = inttoptr i64 %v4 to i64*
  %v19 = load i64, i64* %lptr400
  %v20 = call i64 @aial_rt_strcat(i64 %v18, i64 %v19)
  call void @aial_rt_print(i64 %v20)
  %v21 = add i64 0, 0
  %v22 = add i64 0, 80
  call void @aial_rt_print(i64 %v22)
  %v23 = add i64 0, 0
  ret void
}

define i64 @K_ENTER() {

b105:
  %v0 = add i64 0, 81
  ret i64 %v0
}

define i64 @K_BACKSPACE() {

b107:
  %v0 = add i64 0, 82
  ret i64 %v0
}

define i64 @K_ESC() {

b109:
  %v0 = add i64 0, 83
  ret i64 %v0
}

define i64 @K_UP() {

b111:
  %v0 = add i64 0, 84
  ret i64 %v0
}

define i64 @K_DOWN() {

b113:
  %v0 = add i64 0, 85
  ret i64 %v0
}

define i64 @K_LEFT() {

b115:
  %v0 = add i64 0, 86
  ret i64 %v0
}

define i64 @K_RIGHT() {

b117:
  %v0 = add i64 0, 87
  ret i64 %v0
}

define i64 @K_CTRL_Q() {

b119:
  %v0 = add i64 0, 88
  ret i64 %v0
}

define i64 @K_CTRL_L() {

b121:
  %v0 = add i64 0, 89
  ret i64 %v0
}

define i64 @K_CTRL_D() {

b123:
  %v0 = add i64 0, 90
  ret i64 %v0
}

define i32 @main() {

b125:
  %str_init_0 = getelementptr inbounds [5 x i8], [5 x i8]* @.str0, i32 0, i32 0
  call void @aial_rt_string_register(i64 0, i8* %str_init_0)
  %str_init_1 = getelementptr inbounds [6 x i8], [6 x i8]* @.str1, i32 0, i32 0
  call void @aial_rt_string_register(i64 1, i8* %str_init_1)
  %str_init_2 = getelementptr inbounds [6 x i8], [6 x i8]* @.str2, i32 0, i32 0
  call void @aial_rt_string_register(i64 2, i8* %str_init_2)
  %str_init_3 = getelementptr inbounds [6 x i8], [6 x i8]* @.str3, i32 0, i32 0
  call void @aial_rt_string_register(i64 3, i8* %str_init_3)
  %str_init_4 = getelementptr inbounds [6 x i8], [6 x i8]* @.str4, i32 0, i32 0
  call void @aial_rt_string_register(i64 4, i8* %str_init_4)
  %str_init_5 = getelementptr inbounds [6 x i8], [6 x i8]* @.str5, i32 0, i32 0
  call void @aial_rt_string_register(i64 5, i8* %str_init_5)
  %str_init_6 = getelementptr inbounds [5 x i8], [5 x i8]* @.str6, i32 0, i32 0
  call void @aial_rt_string_register(i64 6, i8* %str_init_6)
  %str_init_7 = getelementptr inbounds [9 x i8], [9 x i8]* @.str7, i32 0, i32 0
  call void @aial_rt_string_register(i64 7, i8* %str_init_7)
  %str_init_8 = getelementptr inbounds [4 x i8], [4 x i8]* @.str8, i32 0, i32 0
  call void @aial_rt_string_register(i64 8, i8* %str_init_8)
  %str_init_9 = getelementptr inbounds [4 x i8], [4 x i8]* @.str9, i32 0, i32 0
  call void @aial_rt_string_register(i64 9, i8* %str_init_9)
  %str_init_10 = getelementptr inbounds [4 x i8], [4 x i8]* @.str10, i32 0, i32 0
  call void @aial_rt_string_register(i64 10, i8* %str_init_10)
  %str_init_11 = getelementptr inbounds [4 x i8], [4 x i8]* @.str11, i32 0, i32 0
  call void @aial_rt_string_register(i64 11, i8* %str_init_11)
  %str_init_12 = getelementptr inbounds [4 x i8], [4 x i8]* @.str12, i32 0, i32 0
  call void @aial_rt_string_register(i64 12, i8* %str_init_12)
  %str_init_13 = getelementptr inbounds [4 x i8], [4 x i8]* @.str13, i32 0, i32 0
  call void @aial_rt_string_register(i64 13, i8* %str_init_13)
  %str_init_14 = getelementptr inbounds [55 x i8], [55 x i8]* @.str14, i32 0, i32 0
  call void @aial_rt_string_register(i64 14, i8* %str_init_14)
  %str_init_15 = getelementptr inbounds [12 x i8], [12 x i8]* @.str15, i32 0, i32 0
  call void @aial_rt_string_register(i64 15, i8* %str_init_15)
  %str_init_16 = getelementptr inbounds [221 x i8], [221 x i8]* @.str16, i32 0, i32 0
  call void @aial_rt_string_register(i64 16, i8* %str_init_16)
  %str_init_17 = getelementptr inbounds [42 x i8], [42 x i8]* @.str17, i32 0, i32 0
  call void @aial_rt_string_register(i64 17, i8* %str_init_17)
  %str_init_18 = getelementptr inbounds [1 x i8], [1 x i8]* @.str18, i32 0, i32 0
  call void @aial_rt_string_register(i64 18, i8* %str_init_18)
  %str_init_19 = getelementptr inbounds [6 x i8], [6 x i8]* @.str19, i32 0, i32 0
  call void @aial_rt_string_register(i64 19, i8* %str_init_19)
  %str_init_20 = getelementptr inbounds [95 x i8], [95 x i8]* @.str20, i32 0, i32 0
  call void @aial_rt_string_register(i64 20, i8* %str_init_20)
  %str_init_21 = getelementptr inbounds [5 x i8], [5 x i8]* @.str21, i32 0, i32 0
  call void @aial_rt_string_register(i64 21, i8* %str_init_21)
  %str_init_22 = getelementptr inbounds [186 x i8], [186 x i8]* @.str22, i32 0, i32 0
  call void @aial_rt_string_register(i64 22, i8* %str_init_22)
  %str_init_23 = getelementptr inbounds [7 x i8], [7 x i8]* @.str23, i32 0, i32 0
  call void @aial_rt_string_register(i64 23, i8* %str_init_23)
  %str_init_24 = getelementptr inbounds [98 x i8], [98 x i8]* @.str24, i32 0, i32 0
  call void @aial_rt_string_register(i64 24, i8* %str_init_24)
  %str_init_25 = getelementptr inbounds [11 x i8], [11 x i8]* @.str25, i32 0, i32 0
  call void @aial_rt_string_register(i64 25, i8* %str_init_25)
  %str_init_26 = getelementptr inbounds [219 x i8], [219 x i8]* @.str26, i32 0, i32 0
  call void @aial_rt_string_register(i64 26, i8* %str_init_26)
  %str_init_27 = getelementptr inbounds [8 x i8], [8 x i8]* @.str27, i32 0, i32 0
  call void @aial_rt_string_register(i64 27, i8* %str_init_27)
  %str_init_28 = getelementptr inbounds [72 x i8], [72 x i8]* @.str28, i32 0, i32 0
  call void @aial_rt_string_register(i64 28, i8* %str_init_28)
  %str_init_29 = getelementptr inbounds [241 x i8], [241 x i8]* @.str29, i32 0, i32 0
  call void @aial_rt_string_register(i64 29, i8* %str_init_29)
  %str_init_30 = getelementptr inbounds [65 x i8], [65 x i8]* @.str30, i32 0, i32 0
  call void @aial_rt_string_register(i64 30, i8* %str_init_30)
  %str_init_31 = getelementptr inbounds [241 x i8], [241 x i8]* @.str31, i32 0, i32 0
  call void @aial_rt_string_register(i64 31, i8* %str_init_31)
  %str_init_32 = getelementptr inbounds [2 x i8], [2 x i8]* @.str32, i32 0, i32 0
  call void @aial_rt_string_register(i64 32, i8* %str_init_32)
  %str_init_33 = getelementptr inbounds [5 x i8], [5 x i8]* @.str33, i32 0, i32 0
  call void @aial_rt_string_register(i64 33, i8* %str_init_33)
  %str_init_34 = getelementptr inbounds [241 x i8], [241 x i8]* @.str34, i32 0, i32 0
  call void @aial_rt_string_register(i64 34, i8* %str_init_34)
  %str_init_35 = getelementptr inbounds [1 x i8], [1 x i8]* @.str35, i32 0, i32 0
  call void @aial_rt_string_register(i64 35, i8* %str_init_35)
  %str_init_36 = getelementptr inbounds [1 x i8], [1 x i8]* @.str36, i32 0, i32 0
  call void @aial_rt_string_register(i64 36, i8* %str_init_36)
  %str_init_37 = getelementptr inbounds [1 x i8], [1 x i8]* @.str37, i32 0, i32 0
  call void @aial_rt_string_register(i64 37, i8* %str_init_37)
  %str_init_38 = getelementptr inbounds [1 x i8], [1 x i8]* @.str38, i32 0, i32 0
  call void @aial_rt_string_register(i64 38, i8* %str_init_38)
  %str_init_39 = getelementptr inbounds [11 x i8], [11 x i8]* @.str39, i32 0, i32 0
  call void @aial_rt_string_register(i64 39, i8* %str_init_39)
  %str_init_40 = getelementptr inbounds [10 x i8], [10 x i8]* @.str40, i32 0, i32 0
  call void @aial_rt_string_register(i64 40, i8* %str_init_40)
  %str_init_41 = getelementptr inbounds [10 x i8], [10 x i8]* @.str41, i32 0, i32 0
  call void @aial_rt_string_register(i64 41, i8* %str_init_41)
  %str_init_42 = getelementptr inbounds [6 x i8], [6 x i8]* @.str42, i32 0, i32 0
  call void @aial_rt_string_register(i64 42, i8* %str_init_42)
  %str_init_43 = getelementptr inbounds [14 x i8], [14 x i8]* @.str43, i32 0, i32 0
  call void @aial_rt_string_register(i64 43, i8* %str_init_43)
  %str_init_44 = getelementptr inbounds [11 x i8], [11 x i8]* @.str44, i32 0, i32 0
  call void @aial_rt_string_register(i64 44, i8* %str_init_44)
  %str_init_45 = getelementptr inbounds [19 x i8], [19 x i8]* @.str45, i32 0, i32 0
  call void @aial_rt_string_register(i64 45, i8* %str_init_45)
  %str_init_46 = getelementptr inbounds [19 x i8], [19 x i8]* @.str46, i32 0, i32 0
  call void @aial_rt_string_register(i64 46, i8* %str_init_46)
  %str_init_47 = getelementptr inbounds [19 x i8], [19 x i8]* @.str47, i32 0, i32 0
  call void @aial_rt_string_register(i64 47, i8* %str_init_47)
  %str_init_48 = getelementptr inbounds [19 x i8], [19 x i8]* @.str48, i32 0, i32 0
  call void @aial_rt_string_register(i64 48, i8* %str_init_48)
  %str_init_49 = getelementptr inbounds [19 x i8], [19 x i8]* @.str49, i32 0, i32 0
  call void @aial_rt_string_register(i64 49, i8* %str_init_49)
  %str_init_50 = getelementptr inbounds [19 x i8], [19 x i8]* @.str50, i32 0, i32 0
  call void @aial_rt_string_register(i64 50, i8* %str_init_50)
  %str_init_51 = getelementptr inbounds [19 x i8], [19 x i8]* @.str51, i32 0, i32 0
  call void @aial_rt_string_register(i64 51, i8* %str_init_51)
  %str_init_52 = getelementptr inbounds [19 x i8], [19 x i8]* @.str52, i32 0, i32 0
  call void @aial_rt_string_register(i64 52, i8* %str_init_52)
  %str_init_53 = getelementptr inbounds [18 x i8], [18 x i8]* @.str53, i32 0, i32 0
  call void @aial_rt_string_register(i64 53, i8* %str_init_53)
  %str_init_54 = getelementptr inbounds [19 x i8], [19 x i8]* @.str54, i32 0, i32 0
  call void @aial_rt_string_register(i64 54, i8* %str_init_54)
  %str_init_55 = getelementptr inbounds [19 x i8], [19 x i8]* @.str55, i32 0, i32 0
  call void @aial_rt_string_register(i64 55, i8* %str_init_55)
  %str_init_56 = getelementptr inbounds [18 x i8], [18 x i8]* @.str56, i32 0, i32 0
  call void @aial_rt_string_register(i64 56, i8* %str_init_56)
  %str_init_57 = getelementptr inbounds [18 x i8], [18 x i8]* @.str57, i32 0, i32 0
  call void @aial_rt_string_register(i64 57, i8* %str_init_57)
  %str_init_58 = getelementptr inbounds [18 x i8], [18 x i8]* @.str58, i32 0, i32 0
  call void @aial_rt_string_register(i64 58, i8* %str_init_58)
  %str_init_59 = getelementptr inbounds [154 x i8], [154 x i8]* @.str59, i32 0, i32 0
  call void @aial_rt_string_register(i64 59, i8* %str_init_59)
  %str_init_60 = getelementptr inbounds [158 x i8], [158 x i8]* @.str60, i32 0, i32 0
  call void @aial_rt_string_register(i64 60, i8* %str_init_60)
  %str_init_61 = getelementptr inbounds [134 x i8], [134 x i8]* @.str61, i32 0, i32 0
  call void @aial_rt_string_register(i64 61, i8* %str_init_61)
  %str_init_62 = getelementptr inbounds [132 x i8], [132 x i8]* @.str62, i32 0, i32 0
  call void @aial_rt_string_register(i64 62, i8* %str_init_62)
  %str_init_63 = getelementptr inbounds [142 x i8], [142 x i8]* @.str63, i32 0, i32 0
  call void @aial_rt_string_register(i64 63, i8* %str_init_63)
  %str_init_64 = getelementptr inbounds [136 x i8], [136 x i8]* @.str64, i32 0, i32 0
  call void @aial_rt_string_register(i64 64, i8* %str_init_64)
  %str_init_65 = getelementptr inbounds [58 x i8], [58 x i8]* @.str65, i32 0, i32 0
  call void @aial_rt_string_register(i64 65, i8* %str_init_65)
  %str_init_66 = getelementptr inbounds [163 x i8], [163 x i8]* @.str66, i32 0, i32 0
  call void @aial_rt_string_register(i64 66, i8* %str_init_66)
  %str_init_67 = getelementptr inbounds [175 x i8], [175 x i8]* @.str67, i32 0, i32 0
  call void @aial_rt_string_register(i64 67, i8* %str_init_67)
  %str_init_68 = getelementptr inbounds [163 x i8], [163 x i8]* @.str68, i32 0, i32 0
  call void @aial_rt_string_register(i64 68, i8* %str_init_68)
  %str_init_69 = getelementptr inbounds [165 x i8], [165 x i8]* @.str69, i32 0, i32 0
  call void @aial_rt_string_register(i64 69, i8* %str_init_69)
  %str_init_70 = getelementptr inbounds [170 x i8], [170 x i8]* @.str70, i32 0, i32 0
  call void @aial_rt_string_register(i64 70, i8* %str_init_70)
  %str_init_71 = getelementptr inbounds [162 x i8], [162 x i8]* @.str71, i32 0, i32 0
  call void @aial_rt_string_register(i64 71, i8* %str_init_71)
  %str_init_72 = getelementptr inbounds [1 x i8], [1 x i8]* @.str72, i32 0, i32 0
  call void @aial_rt_string_register(i64 72, i8* %str_init_72)
  %str_init_73 = getelementptr inbounds [81 x i8], [81 x i8]* @.str73, i32 0, i32 0
  call void @aial_rt_string_register(i64 73, i8* %str_init_73)
  %str_init_74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str74, i32 0, i32 0
  call void @aial_rt_string_register(i64 74, i8* %str_init_74)
  %str_init_75 = getelementptr inbounds [3 x i8], [3 x i8]* @.str75, i32 0, i32 0
  call void @aial_rt_string_register(i64 75, i8* %str_init_75)
  %str_init_76 = getelementptr inbounds [9 x i8], [9 x i8]* @.str76, i32 0, i32 0
  call void @aial_rt_string_register(i64 76, i8* %str_init_76)
  %str_init_77 = getelementptr inbounds [13 x i8], [13 x i8]* @.str77, i32 0, i32 0
  call void @aial_rt_string_register(i64 77, i8* %str_init_77)
  %str_init_78 = getelementptr inbounds [26 x i8], [26 x i8]* @.str78, i32 0, i32 0
  call void @aial_rt_string_register(i64 78, i8* %str_init_78)
  %str_init_79 = getelementptr inbounds [4 x i8], [4 x i8]* @.str79, i32 0, i32 0
  call void @aial_rt_string_register(i64 79, i8* %str_init_79)
  %str_init_80 = getelementptr inbounds [3 x i8], [3 x i8]* @.str80, i32 0, i32 0
  call void @aial_rt_string_register(i64 80, i8* %str_init_80)
  %str_init_81 = getelementptr inbounds [6 x i8], [6 x i8]* @.str81, i32 0, i32 0
  call void @aial_rt_string_register(i64 81, i8* %str_init_81)
  %str_init_82 = getelementptr inbounds [10 x i8], [10 x i8]* @.str82, i32 0, i32 0
  call void @aial_rt_string_register(i64 82, i8* %str_init_82)
  %str_init_83 = getelementptr inbounds [4 x i8], [4 x i8]* @.str83, i32 0, i32 0
  call void @aial_rt_string_register(i64 83, i8* %str_init_83)
  %str_init_84 = getelementptr inbounds [3 x i8], [3 x i8]* @.str84, i32 0, i32 0
  call void @aial_rt_string_register(i64 84, i8* %str_init_84)
  %str_init_85 = getelementptr inbounds [5 x i8], [5 x i8]* @.str85, i32 0, i32 0
  call void @aial_rt_string_register(i64 85, i8* %str_init_85)
  %str_init_86 = getelementptr inbounds [5 x i8], [5 x i8]* @.str86, i32 0, i32 0
  call void @aial_rt_string_register(i64 86, i8* %str_init_86)
  %str_init_87 = getelementptr inbounds [6 x i8], [6 x i8]* @.str87, i32 0, i32 0
  call void @aial_rt_string_register(i64 87, i8* %str_init_87)
  %str_init_88 = getelementptr inbounds [7 x i8], [7 x i8]* @.str88, i32 0, i32 0
  call void @aial_rt_string_register(i64 88, i8* %str_init_88)
  %str_init_89 = getelementptr inbounds [7 x i8], [7 x i8]* @.str89, i32 0, i32 0
  call void @aial_rt_string_register(i64 89, i8* %str_init_89)
  %str_init_90 = getelementptr inbounds [7 x i8], [7 x i8]* @.str90, i32 0, i32 0
  call void @aial_rt_string_register(i64 90, i8* %str_init_90)
  %str_init_91 = getelementptr inbounds [8 x i8], [8 x i8]* @.str91, i32 0, i32 0
  call void @aial_rt_string_register(i64 91, i8* %str_init_91)
  %str_init_92 = getelementptr inbounds [48 x i8], [48 x i8]* @.str92, i32 0, i32 0
  call void @aial_rt_string_register(i64 92, i8* %str_init_92)
  %str_init_93 = getelementptr inbounds [9 x i8], [9 x i8]* @.str93, i32 0, i32 0
  call void @aial_rt_string_register(i64 93, i8* %str_init_93)
  %str_init_94 = getelementptr inbounds [22 x i8], [22 x i8]* @.str94, i32 0, i32 0
  call void @aial_rt_string_register(i64 94, i8* %str_init_94)
  %str_init_95 = getelementptr inbounds [23 x i8], [23 x i8]* @.str95, i32 0, i32 0
  call void @aial_rt_string_register(i64 95, i8* %str_init_95)
  %str_init_96 = getelementptr inbounds [52 x i8], [52 x i8]* @.str96, i32 0, i32 0
  call void @aial_rt_string_register(i64 96, i8* %str_init_96)
  %str_init_97 = getelementptr inbounds [1 x i8], [1 x i8]* @.str97, i32 0, i32 0
  call void @aial_rt_string_register(i64 97, i8* %str_init_97)
  %str_init_98 = getelementptr inbounds [8 x i8], [8 x i8]* @.str98, i32 0, i32 0
  call void @aial_rt_string_register(i64 98, i8* %str_init_98)
  %str_init_99 = getelementptr inbounds [1 x i8], [1 x i8]* @.str99, i32 0, i32 0
  call void @aial_rt_string_register(i64 99, i8* %str_init_99)
  %str_init_100 = getelementptr inbounds [48 x i8], [48 x i8]* @.str100, i32 0, i32 0
  call void @aial_rt_string_register(i64 100, i8* %str_init_100)
  %str_init_101 = getelementptr inbounds [15 x i8], [15 x i8]* @.str101, i32 0, i32 0
  call void @aial_rt_string_register(i64 101, i8* %str_init_101)
  %str_init_102 = getelementptr inbounds [9 x i8], [9 x i8]* @.str102, i32 0, i32 0
  call void @aial_rt_string_register(i64 102, i8* %str_init_102)
  %str_init_103 = getelementptr inbounds [9 x i8], [9 x i8]* @.str103, i32 0, i32 0
  call void @aial_rt_string_register(i64 103, i8* %str_init_103)
  %str_init_104 = getelementptr inbounds [1 x i8], [1 x i8]* @.str104, i32 0, i32 0
  call void @aial_rt_string_register(i64 104, i8* %str_init_104)
  %str_init_105 = getelementptr inbounds [5 x i8], [5 x i8]* @.str105, i32 0, i32 0
  call void @aial_rt_string_register(i64 105, i8* %str_init_105)
  %str_init_106 = getelementptr inbounds [5 x i8], [5 x i8]* @.str106, i32 0, i32 0
  call void @aial_rt_string_register(i64 106, i8* %str_init_106)
  %str_init_107 = getelementptr inbounds [2 x i8], [2 x i8]* @.str107, i32 0, i32 0
  call void @aial_rt_string_register(i64 107, i8* %str_init_107)
  %str_init_108 = getelementptr inbounds [4 x i8], [4 x i8]* @.str108, i32 0, i32 0
  call void @aial_rt_string_register(i64 108, i8* %str_init_108)
  %str_init_109 = getelementptr inbounds [1 x i8], [1 x i8]* @.str109, i32 0, i32 0
  call void @aial_rt_string_register(i64 109, i8* %str_init_109)
  %str_init_110 = getelementptr inbounds [10 x i8], [10 x i8]* @.str110, i32 0, i32 0
  call void @aial_rt_string_register(i64 110, i8* %str_init_110)
  %str_init_111 = getelementptr inbounds [9 x i8], [9 x i8]* @.str111, i32 0, i32 0
  call void @aial_rt_string_register(i64 111, i8* %str_init_111)
  %str_init_112 = getelementptr inbounds [41 x i8], [41 x i8]* @.str112, i32 0, i32 0
  call void @aial_rt_string_register(i64 112, i8* %str_init_112)
  %str_init_113 = getelementptr inbounds [20 x i8], [20 x i8]* @.str113, i32 0, i32 0
  call void @aial_rt_string_register(i64 113, i8* %str_init_113)
  %str_init_114 = getelementptr inbounds [33 x i8], [33 x i8]* @.str114, i32 0, i32 0
  call void @aial_rt_string_register(i64 114, i8* %str_init_114)
  %str_init_115 = getelementptr inbounds [1 x i8], [1 x i8]* @.str115, i32 0, i32 0
  call void @aial_rt_string_register(i64 115, i8* %str_init_115)
  %str_init_116 = getelementptr inbounds [1 x i8], [1 x i8]* @.str116, i32 0, i32 0
  call void @aial_rt_string_register(i64 116, i8* %str_init_116)
  %str_init_117 = getelementptr inbounds [6 x i8], [6 x i8]* @.str117, i32 0, i32 0
  call void @aial_rt_string_register(i64 117, i8* %str_init_117)
  %str_init_118 = getelementptr inbounds [7 x i8], [7 x i8]* @.str118, i32 0, i32 0
  call void @aial_rt_string_register(i64 118, i8* %str_init_118)
  %str_init_119 = getelementptr inbounds [8 x i8], [8 x i8]* @.str119, i32 0, i32 0
  call void @aial_rt_string_register(i64 119, i8* %str_init_119)
  %str_init_120 = getelementptr inbounds [1 x i8], [1 x i8]* @.str120, i32 0, i32 0
  call void @aial_rt_string_register(i64 120, i8* %str_init_120)
  %str_init_121 = getelementptr inbounds [9 x i8], [9 x i8]* @.str121, i32 0, i32 0
  call void @aial_rt_string_register(i64 121, i8* %str_init_121)
  %str_init_122 = getelementptr inbounds [1 x i8], [1 x i8]* @.str122, i32 0, i32 0
  call void @aial_rt_string_register(i64 122, i8* %str_init_122)
  %str_init_123 = getelementptr inbounds [5 x i8], [5 x i8]* @.str123, i32 0, i32 0
  call void @aial_rt_string_register(i64 123, i8* %str_init_123)
  %str_init_124 = getelementptr inbounds [5 x i8], [5 x i8]* @.str124, i32 0, i32 0
  call void @aial_rt_string_register(i64 124, i8* %str_init_124)
  %str_init_125 = getelementptr inbounds [1 x i8], [1 x i8]* @.str125, i32 0, i32 0
  call void @aial_rt_string_register(i64 125, i8* %str_init_125)
  %str_init_126 = getelementptr inbounds [8 x i8], [8 x i8]* @.str126, i32 0, i32 0
  call void @aial_rt_string_register(i64 126, i8* %str_init_126)
  %str_init_127 = getelementptr inbounds [7 x i8], [7 x i8]* @.str127, i32 0, i32 0
  call void @aial_rt_string_register(i64 127, i8* %str_init_127)
  %str_init_128 = getelementptr inbounds [7 x i8], [7 x i8]* @.str128, i32 0, i32 0
  call void @aial_rt_string_register(i64 128, i8* %str_init_128)
  %str_init_129 = getelementptr inbounds [7 x i8], [7 x i8]* @.str129, i32 0, i32 0
  call void @aial_rt_string_register(i64 129, i8* %str_init_129)
  %str_init_130 = getelementptr inbounds [8 x i8], [8 x i8]* @.str130, i32 0, i32 0
  call void @aial_rt_string_register(i64 130, i8* %str_init_130)
  %str_init_131 = getelementptr inbounds [19 x i8], [19 x i8]* @.str131, i32 0, i32 0
  call void @aial_rt_string_register(i64 131, i8* %str_init_131)
  %str_init_132 = getelementptr inbounds [18 x i8], [18 x i8]* @.str132, i32 0, i32 0
  call void @aial_rt_string_register(i64 132, i8* %str_init_132)
  %str_init_133 = getelementptr inbounds [11 x i8], [11 x i8]* @.str133, i32 0, i32 0
  call void @aial_rt_string_register(i64 133, i8* %str_init_133)
  %str_init_134 = getelementptr inbounds [11 x i8], [11 x i8]* @.str134, i32 0, i32 0
  call void @aial_rt_string_register(i64 134, i8* %str_init_134)
  %str_init_135 = getelementptr inbounds [50 x i8], [50 x i8]* @.str135, i32 0, i32 0
  call void @aial_rt_string_register(i64 135, i8* %str_init_135)
  %str_init_136 = getelementptr inbounds [19 x i8], [19 x i8]* @.str136, i32 0, i32 0
  call void @aial_rt_string_register(i64 136, i8* %str_init_136)
  %str_init_137 = getelementptr inbounds [1 x i8], [1 x i8]* @.str137, i32 0, i32 0
  call void @aial_rt_string_register(i64 137, i8* %str_init_137)
  %str_init_138 = getelementptr inbounds [7 x i8], [7 x i8]* @.str138, i32 0, i32 0
  call void @aial_rt_string_register(i64 138, i8* %str_init_138)
  %str_init_139 = getelementptr inbounds [5 x i8], [5 x i8]* @.str139, i32 0, i32 0
  call void @aial_rt_string_register(i64 139, i8* %str_init_139)
  %str_init_140 = getelementptr inbounds [10 x i8], [10 x i8]* @.str140, i32 0, i32 0
  call void @aial_rt_string_register(i64 140, i8* %str_init_140)
  %str_init_141 = getelementptr inbounds [5 x i8], [5 x i8]* @.str141, i32 0, i32 0
  call void @aial_rt_string_register(i64 141, i8* %str_init_141)
  %str_init_142 = getelementptr inbounds [10 x i8], [10 x i8]* @.str142, i32 0, i32 0
  call void @aial_rt_string_register(i64 142, i8* %str_init_142)
  %str_init_143 = getelementptr inbounds [5 x i8], [5 x i8]* @.str143, i32 0, i32 0
  call void @aial_rt_string_register(i64 143, i8* %str_init_143)
  %str_init_144 = getelementptr inbounds [1 x i8], [1 x i8]* @.str144, i32 0, i32 0
  call void @aial_rt_string_register(i64 144, i8* %str_init_144)
  %str_init_145 = getelementptr inbounds [8 x i8], [8 x i8]* @.str145, i32 0, i32 0
  call void @aial_rt_string_register(i64 145, i8* %str_init_145)
  %str_init_146 = getelementptr inbounds [8 x i8], [8 x i8]* @.str146, i32 0, i32 0
  call void @aial_rt_string_register(i64 146, i8* %str_init_146)
  %str_init_147 = getelementptr inbounds [24 x i8], [24 x i8]* @.str147, i32 0, i32 0
  call void @aial_rt_string_register(i64 147, i8* %str_init_147)
  %str_init_148 = getelementptr inbounds [5 x i8], [5 x i8]* @.str148, i32 0, i32 0
  call void @aial_rt_string_register(i64 148, i8* %str_init_148)
  %v0 = call i64 @chat_context_new()
  %aptr416 = alloca i64
  %v1 = ptrtoint i64* %aptr416 to i64
  %sptr417 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr417
  %v2 = add i64 0, 0
  %v3 = call i64 @mem_open()
  %aptr419 = alloca i64
  %v4 = ptrtoint i64* %aptr419 to i64
  %sptr420 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr420
  %v5 = add i64 0, 0
  %v6 = call i64 @aial_rt_array_new()
  %aptr422 = alloca i64
  %v7 = ptrtoint i64* %aptr422 to i64
  %sptr423 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr423
  %v8 = add i64 0, 0
  %v9 = call i64 @cmd_cache_new()
  %aptr425 = alloca i64
  %v10 = ptrtoint i64* %aptr425 to i64
  %sptr426 = inttoptr i64 %v10 to i64*
  store i64 %v9, i64* %sptr426
  %v11 = add i64 0, 0
  %lptr427 = inttoptr i64 %v10 to i64*
  %v12 = load i64, i64* %lptr427
  %v13 = call i64 @cmd_cache_populate(i64 %v12)
  %v14 = add i64 0, 0
  %aptr430 = alloca i64
  %v15 = ptrtoint i64* %aptr430 to i64
  %sptr431 = inttoptr i64 %v15 to i64*
  store i64 %v14, i64* %sptr431
  %v16 = add i64 0, 0
  %v17 = add i64 0, 1
  call void @aial_rt_io_raw_mode(i64 %v17)
  %v18 = add i64 0, 0
  %v19 = add i64 0, 91
  call void @aial_rt_print(i64 %v19)
  %v20 = add i64 0, 0
  %v21 = call i64 @header_draw()
  %v22 = call i64 @chat_show_welcome()
  %v23 = add i64 0, 92
  %v24 = call i64 @chat_show_system(i64 %v23)
  %v25 = add i64 0, 93
  %v26 = call i64 @aial_rt_key_exists(i64 %v25)
  %v27 = add i64 0, 1
  %v28 = icmp eq i64 %v26, %v27
  br i1 %v28, label %b126, label %b127

b126:
  %v29 = add i64 0, 94
  %v30 = call i64 @chat_show_hint(i64 %v29)
  br label %b128

b127:
  %v31 = add i64 0, 95
  %v32 = call i64 @chat_show_error(i64 %v31)
  %v33 = add i64 0, 96
  %v34 = call i64 @chat_show_hint(i64 %v33)
  br label %b128

b128:
  %v35 = call i64 @input_draw_border()
  %v36 = add i64 0, 18
  %aptr452 = alloca i64
  %v37 = ptrtoint i64* %aptr452 to i64
  %sptr453 = inttoptr i64 %v37 to i64*
  store i64 %v36, i64* %sptr453
  %v38 = add i64 0, 0
  %v39 = add i64 0, 0
  %aptr455 = alloca i64
  %v40 = ptrtoint i64* %aptr455 to i64
  %sptr456 = inttoptr i64 %v40 to i64*
  store i64 %v39, i64* %sptr456
  %v41 = add i64 0, 0
  %v42 = add i64 0, 0
  %aptr458 = alloca i64
  %v43 = ptrtoint i64* %aptr458 to i64
  %sptr459 = inttoptr i64 %v43 to i64*
  store i64 %v42, i64* %sptr459
  %v44 = add i64 0, 0
  %v45 = add i64 0, 0
  %aptr461 = alloca i64
  %v46 = ptrtoint i64* %aptr461 to i64
  %sptr462 = inttoptr i64 %v46 to i64*
  store i64 %v45, i64* %sptr462
  %v47 = add i64 0, 0
  br label %b129

b129:
  %v48 = call i64 @aial_rt_io_readkey()
  %aptr464 = alloca i64
  %v49 = ptrtoint i64* %aptr464 to i64
  %sptr465 = inttoptr i64 %v49 to i64*
  store i64 %v48, i64* %sptr465
  %v50 = add i64 0, 0
  %lptr466 = inttoptr i64 %v49 to i64*
  %v51 = load i64, i64* %lptr466
  %v52 = call i64 @aial_rt_strlen(i64 %v51)
  %v53 = add i64 0, 0
  %v54 = icmp eq i64 %v52, %v53
  br i1 %v54, label %b131, label %b132

b131:
  %lptr470 = inttoptr i64 %v40 to i64*
  %v55 = load i64, i64* %lptr470
  %v56 = add i64 0, 1
  %v57 = add i64 %v55, %v56
  %sptr473 = inttoptr i64 %v40 to i64*
  store i64 %v57, i64* %sptr473
  %v58 = add i64 0, 0
  %lptr474 = inttoptr i64 %v40 to i64*
  %v59 = load i64, i64* %lptr474
  %v60 = add i64 0, 20
  %v61 = icmp sgt i64 %v59, %v60
  br i1 %v61, label %b134, label %b135

b136:
  %v62 = add i64 0, 50
  call void @aial_rt_time_sleep(i64 %v62)
  %v63 = add i64 0, 0
  br label %b129

b133:
  %v64 = add i64 0, 0
  %sptr480 = inttoptr i64 %v40 to i64*
  store i64 %v64, i64* %sptr480
  %v65 = add i64 0, 0
  %lptr481 = inttoptr i64 %v49 to i64*
  %v66 = load i64, i64* %lptr481
  %v67 = call i64 @K_CTRL_Q()
  %x68 = call i64 @aial_rt_str_eq(i64 %v66, i64 %v67)
  %v68 = trunc i64 %x68 to i1
  br i1 %v68, label %b139, label %b140

b141:
  %lptr484 = inttoptr i64 %v49 to i64*
  %v69 = load i64, i64* %lptr484
  %v70 = call i64 @K_CTRL_L()
  %x71 = call i64 @aial_rt_str_eq(i64 %v69, i64 %v70)
  %v71 = trunc i64 %x71 to i1
  br i1 %v71, label %b143, label %b144

b143:
  %v72 = add i64 0, 91
  call void @aial_rt_print(i64 %v72)
  %v73 = add i64 0, 0
  %v74 = call i64 @header_draw()
  %v75 = call i64 @input_draw_border()
  %v76 = add i64 0, 18
  %sptr492 = inttoptr i64 %v37 to i64*
  store i64 %v76, i64* %sptr492
  %v77 = add i64 0, 0
  br label %b129

b145:
  %lptr493 = inttoptr i64 %v49 to i64*
  %v78 = load i64, i64* %lptr493
  %v79 = call i64 @K_CTRL_D()
  %x80 = call i64 @aial_rt_str_eq(i64 %v78, i64 %v79)
  %v80 = trunc i64 %x80 to i1
  br i1 %v80, label %b147, label %b148

b147:
  %v81 = add i64 0, 1
  %lptr497 = inttoptr i64 %v43 to i64*
  %v82 = load i64, i64* %lptr497
  %v83 = sub i64 %v81, %v82
  %sptr499 = inttoptr i64 %v43 to i64*
  store i64 %v83, i64* %sptr499
  %v84 = add i64 0, 0
  %lptr500 = inttoptr i64 %v43 to i64*
  %v85 = load i64, i64* %lptr500
  %v86 = add i64 0, 1
  %v87 = icmp eq i64 %v85, %v86
  br i1 %v87, label %b150, label %b151

b150:
  %v88 = add i64 0, 100
  %v89 = call i64 @chat_show_system(i64 %v88)
  br label %b152

b152:
  %lptr505 = inttoptr i64 %v43 to i64*
  %v90 = load i64, i64* %lptr505
  %v91 = add i64 0, 0
  %v92 = icmp eq i64 %v90, %v91
  br i1 %v92, label %b153, label %b154

b153:
  %v93 = add i64 0, 101
  %v94 = call i64 @chat_show_system(i64 %v93)
  br label %b155

b149:
  %lptr510 = inttoptr i64 %v49 to i64*
  %v95 = load i64, i64* %lptr510
  %v96 = call i64 @K_UP()
  %x97 = call i64 @aial_rt_str_eq(i64 %v95, i64 %v96)
  %v97 = trunc i64 %x97 to i1
  br i1 %v97, label %b157, label %b158

b157:
  %lptr513 = inttoptr i64 %v7 to i64*
  %v98 = load i64, i64* %lptr513
  %v99 = call i64 @history_size(i64 %v98)
  %aptr515 = alloca i64
  %v100 = ptrtoint i64* %aptr515 to i64
  %sptr516 = inttoptr i64 %v100 to i64*
  store i64 %v99, i64* %sptr516
  %v101 = add i64 0, 0
  %lptr517 = inttoptr i64 %v100 to i64*
  %v102 = load i64, i64* %lptr517
  %v103 = add i64 0, 0
  %v104 = icmp sgt i64 %v102, %v103
  %lptr520 = inttoptr i64 %v15 to i64*
  %v105 = load i64, i64* %lptr520
  %lptr521 = inttoptr i64 %v100 to i64*
  %v106 = load i64, i64* %lptr521
  %v107 = icmp slt i64 %v105, %v106
  %v108 = and i1 %v104, %v107
  br i1 %v108, label %b160, label %b161

b160:
  %lptr524 = inttoptr i64 %v15 to i64*
  %v109 = load i64, i64* %lptr524
  %v110 = add i64 0, 1
  %v111 = add i64 %v109, %v110
  %sptr527 = inttoptr i64 %v15 to i64*
  store i64 %v111, i64* %sptr527
  %v112 = add i64 0, 0
  %lptr528 = inttoptr i64 %v7 to i64*
  %v113 = load i64, i64* %lptr528
  %lptr529 = inttoptr i64 %v100 to i64*
  %v114 = load i64, i64* %lptr529
  %lptr530 = inttoptr i64 %v15 to i64*
  %v115 = load i64, i64* %lptr530
  %v116 = sub i64 %v114, %v115
  %v117 = call i64 @history_recall(i64 %v113, i64 %v116)
  %aptr533 = alloca i64
  %v118 = ptrtoint i64* %aptr533 to i64
  %sptr534 = inttoptr i64 %v118 to i64*
  store i64 %v117, i64* %sptr534
  %v119 = add i64 0, 0
  %v120 = add i64 0, 102
  %v121 = call i64 @color_input()
  %lptr537 = inttoptr i64 %v118 to i64*
  %v122 = load i64, i64* %lptr537
  %v123 = call i64 @aial_rt_strcat(i64 %v121, i64 %v122)
  %v124 = call i64 @aial_rt_strcat(i64 %v120, i64 %v123)
  call void @aial_rt_print(i64 %v124)
  %v125 = add i64 0, 0
  %lptr541 = inttoptr i64 %v118 to i64*
  %v126 = load i64, i64* %lptr541
  %sptr542 = inttoptr i64 %v37 to i64*
  store i64 %v126, i64* %sptr542
  %v127 = add i64 0, 0
  br label %b162

b159:
  %lptr543 = inttoptr i64 %v49 to i64*
  %v128 = load i64, i64* %lptr543
  %v129 = call i64 @K_DOWN()
  %x130 = call i64 @aial_rt_str_eq(i64 %v128, i64 %v129)
  %v130 = trunc i64 %x130 to i1
  br i1 %v130, label %b164, label %b165

b164:
  %lptr546 = inttoptr i64 %v15 to i64*
  %v131 = load i64, i64* %lptr546
  %v132 = add i64 0, 1
  %v133 = icmp sgt i64 %v131, %v132
  br i1 %v133, label %b167, label %b168

b167:
  %lptr549 = inttoptr i64 %v15 to i64*
  %v134 = load i64, i64* %lptr549
  %v135 = add i64 0, 1
  %v136 = sub i64 %v134, %v135
  %sptr552 = inttoptr i64 %v15 to i64*
  store i64 %v136, i64* %sptr552
  %v137 = add i64 0, 0
  %lptr553 = inttoptr i64 %v7 to i64*
  %v138 = load i64, i64* %lptr553
  %v139 = call i64 @history_size(i64 %v138)
  %aptr555 = alloca i64
  %v140 = ptrtoint i64* %aptr555 to i64
  %sptr556 = inttoptr i64 %v140 to i64*
  store i64 %v139, i64* %sptr556
  %v141 = add i64 0, 0
  %lptr557 = inttoptr i64 %v7 to i64*
  %v142 = load i64, i64* %lptr557
  %lptr558 = inttoptr i64 %v140 to i64*
  %v143 = load i64, i64* %lptr558
  %lptr559 = inttoptr i64 %v15 to i64*
  %v144 = load i64, i64* %lptr559
  %v145 = sub i64 %v143, %v144
  %v146 = call i64 @history_recall(i64 %v142, i64 %v145)
  %aptr562 = alloca i64
  %v147 = ptrtoint i64* %aptr562 to i64
  %sptr563 = inttoptr i64 %v147 to i64*
  store i64 %v146, i64* %sptr563
  %v148 = add i64 0, 0
  %v149 = add i64 0, 102
  %v150 = call i64 @color_input()
  %lptr566 = inttoptr i64 %v147 to i64*
  %v151 = load i64, i64* %lptr566
  %v152 = call i64 @aial_rt_strcat(i64 %v150, i64 %v151)
  %v153 = call i64 @aial_rt_strcat(i64 %v149, i64 %v152)
  call void @aial_rt_print(i64 %v153)
  %v154 = add i64 0, 0
  %lptr570 = inttoptr i64 %v147 to i64*
  %v155 = load i64, i64* %lptr570
  %sptr571 = inttoptr i64 %v37 to i64*
  store i64 %v155, i64* %sptr571
  %v156 = add i64 0, 0
  br label %b169

b168:
  %lptr572 = inttoptr i64 %v15 to i64*
  %v157 = load i64, i64* %lptr572
  %v158 = add i64 0, 1
  %v159 = icmp eq i64 %v157, %v158
  br i1 %v159, label %b170, label %b171

b170:
  %v160 = add i64 0, 0
  %sptr576 = inttoptr i64 %v15 to i64*
  store i64 %v160, i64* %sptr576
  %v161 = add i64 0, 0
  %v162 = add i64 0, 18
  %sptr578 = inttoptr i64 %v37 to i64*
  store i64 %v162, i64* %sptr578
  %v163 = add i64 0, 0
  %v164 = add i64 0, 105
  %v165 = call i64 @color_input()
  %v166 = add i64 0, 33
  %v167 = call i64 @aial_rt_strcat(i64 %v165, i64 %v166)
  %v168 = call i64 @aial_rt_strcat(i64 %v164, i64 %v167)
  call void @aial_rt_print(i64 %v168)
  %v169 = add i64 0, 0
  br label %b172

b166:
  %lptr585 = inttoptr i64 %v49 to i64*
  %v170 = load i64, i64* %lptr585
  %v171 = call i64 @K_LEFT()
  %x172 = call i64 @aial_rt_str_eq(i64 %v170, i64 %v171)
  %v172 = trunc i64 %x172 to i1
  %lptr588 = inttoptr i64 %v49 to i64*
  %v173 = load i64, i64* %lptr588
  %v174 = call i64 @K_RIGHT()
  %x175 = call i64 @aial_rt_str_eq(i64 %v173, i64 %v174)
  %v175 = trunc i64 %x175 to i1
  %v176 = or i1 %v172, %v175
  br i1 %v176, label %b174, label %b175

b176:
  %lptr592 = inttoptr i64 %v49 to i64*
  %v177 = load i64, i64* %lptr592
  %v178 = call i64 @K_ESC()
  %x179 = call i64 @aial_rt_str_eq(i64 %v177, i64 %v178)
  %v179 = trunc i64 %x179 to i1
  br i1 %v179, label %b178, label %b179

b180:
  %lptr595 = inttoptr i64 %v49 to i64*
  %v180 = load i64, i64* %lptr595
  %v181 = call i64 @K_ENTER()
  %x182 = call i64 @aial_rt_str_eq(i64 %v180, i64 %v181)
  %v182 = trunc i64 %x182 to i1
  br i1 %v182, label %b182, label %b183

b182:
  %lptr598 = inttoptr i64 %v43 to i64*
  %v183 = load i64, i64* %lptr598
  %v184 = add i64 0, 1
  %v185 = icmp eq i64 %v183, %v184
  br i1 %v185, label %b185, label %b186

b185:
  %lptr601 = inttoptr i64 %v37 to i64*
  %v186 = load i64, i64* %lptr601
  %v187 = add i64 0, 32
  %v188 = call i64 @aial_rt_strcat(i64 %v186, i64 %v187)
  %sptr604 = inttoptr i64 %v37 to i64*
  store i64 %v188, i64* %sptr604
  %v189 = add i64 0, 0
  %v190 = add i64 0, 108
  call void @aial_rt_print(i64 %v190)
  %v191 = add i64 0, 0
  br label %b129

b187:
  %lptr607 = inttoptr i64 %v37 to i64*
  %v192 = load i64, i64* %lptr607
  %v193 = call i64 @aial_rt_strlen(i64 %v192)
  %v194 = add i64 0, 0
  %v195 = icmp eq i64 %v193, %v194
  br i1 %v195, label %b189, label %b190

b191:
  %v196 = add i64 0, 18
  call void @aial_rt_println(i64 %v196)
  %v197 = add i64 0, 0
  %lptr613 = inttoptr i64 %v37 to i64*
  %v198 = load i64, i64* %lptr613
  %v199 = add i64 0, 110
  %x200 = call i64 @aial_rt_starts_with(i64 %v198, i64 %v199)
  %v200 = trunc i64 %x200 to i1
  br i1 %v200, label %b193, label %b194

b193:
  %lptr616 = inttoptr i64 %v37 to i64*
  %v201 = load i64, i64* %lptr616
  %v202 = add i64 0, 9
  %lptr618 = inttoptr i64 %v37 to i64*
  %v203 = load i64, i64* %lptr618
  %v204 = call i64 @aial_rt_strlen(i64 %v203)
  %v205 = add i64 0, 9
  %v206 = sub i64 %v204, %v205
  %v207 = call i64 @aial_rt_strslice(i64 %v201, i64 %v202, i64 %v206)
  %aptr623 = alloca i64
  %v208 = ptrtoint i64* %aptr623 to i64
  %sptr624 = inttoptr i64 %v208 to i64*
  store i64 %v207, i64* %sptr624
  %v209 = add i64 0, 0
  %lptr625 = inttoptr i64 %v208 to i64*
  %v210 = load i64, i64* %lptr625
  %v211 = call i64 @aial_rt_strlen(i64 %v210)
  %v212 = add i64 0, 0
  %v213 = icmp sgt i64 %v211, %v212
  br i1 %v213, label %b196, label %b197

b196:
  %v214 = add i64 0, 93
  %lptr630 = inttoptr i64 %v208 to i64*
  %v215 = load i64, i64* %lptr630
  %v216 = call i64 @aial_rt_key_set(i64 %v214, i64 %v215)
  %aptr632 = alloca i64
  %v217 = ptrtoint i64* %aptr632 to i64
  %sptr633 = inttoptr i64 %v217 to i64*
  store i64 %v216, i64* %sptr633
  %v218 = add i64 0, 0
  %lptr634 = inttoptr i64 %v217 to i64*
  %v219 = load i64, i64* %lptr634
  %v220 = add i64 0, 1
  %v221 = icmp eq i64 %v219, %v220
  br i1 %v221, label %b199, label %b200

b199:
  %v222 = add i64 0, 112
  %v223 = call i64 @chat_show_system(i64 %v222)
  br label %b201

b200:
  %v224 = add i64 0, 113
  %v225 = call i64 @chat_show_error(i64 %v224)
  br label %b201

b197:
  %v226 = add i64 0, 114
  %v227 = call i64 @chat_show_error(i64 %v226)
  br label %b198

b198:
  %v228 = call i64 @input_draw_border()
  %v229 = add i64 0, 18
  %sptr645 = inttoptr i64 %v37 to i64*
  store i64 %v229, i64* %sptr645
  %v230 = add i64 0, 0
  br label %b129

b195:
  %lptr646 = inttoptr i64 %v10 to i64*
  %v231 = load i64, i64* %lptr646
  %lptr647 = inttoptr i64 %v37 to i64*
  %v232 = load i64, i64* %lptr647
  %v233 = call i64 @cmd_cache_get(i64 %v231, i64 %v232)
  %aptr649 = alloca i64
  %v234 = ptrtoint i64* %aptr649 to i64
  %sptr650 = inttoptr i64 %v234 to i64*
  store i64 %v233, i64* %sptr650
  %v235 = add i64 0, 0
  %lptr651 = inttoptr i64 %v234 to i64*
  %v236 = load i64, i64* %lptr651
  %v237 = call i64 @aial_rt_strlen(i64 %v236)
  %v238 = add i64 0, 0
  %v239 = icmp sgt i64 %v237, %v238
  br i1 %v239, label %b203, label %b204

b203:
  %lptr655 = inttoptr i64 %v234 to i64*
  %v240 = load i64, i64* %lptr655
  %v241 = call i64 @chat_show_system(i64 %v240)
  %v242 = call i64 @input_draw_border()
  %v243 = add i64 0, 18
  %sptr659 = inttoptr i64 %v37 to i64*
  store i64 %v243, i64* %sptr659
  %v244 = add i64 0, 0
  br label %b129

b205:
  %lptr660 = inttoptr i64 %v37 to i64*
  %v245 = load i64, i64* %lptr660
  %v246 = add i64 0, 117
  %x247 = call i64 @aial_rt_str_eq(i64 %v245, i64 %v246)
  %v247 = trunc i64 %x247 to i1
  br i1 %v247, label %b207, label %b208

b209:
  %lptr663 = inttoptr i64 %v37 to i64*
  %v248 = load i64, i64* %lptr663
  %v249 = add i64 0, 118
  %x250 = call i64 @aial_rt_str_eq(i64 %v248, i64 %v249)
  %v250 = trunc i64 %x250 to i1
  br i1 %v250, label %b211, label %b212

b211:
  %v251 = add i64 0, 91
  call void @aial_rt_print(i64 %v251)
  %v252 = add i64 0, 0
  %v253 = call i64 @header_draw()
  %v254 = call i64 @input_draw_border()
  %v255 = add i64 0, 18
  %sptr671 = inttoptr i64 %v37 to i64*
  store i64 %v255, i64* %sptr671
  %v256 = add i64 0, 0
  br label %b129

b213:
  %lptr672 = inttoptr i64 %v7 to i64*
  %v257 = load i64, i64* %lptr672
  %lptr673 = inttoptr i64 %v37 to i64*
  %v258 = load i64, i64* %lptr673
  %v259 = call i64 @history_push(i64 %v257, i64 %v258)
  %v260 = add i64 0, 0
  %sptr676 = inttoptr i64 %v15 to i64*
  store i64 %v260, i64* %sptr676
  %v261 = add i64 0, 0
  %lptr677 = inttoptr i64 %v46 to i64*
  %v262 = load i64, i64* %lptr677
  %v263 = add i64 0, 1
  %v264 = add i64 %v262, %v263
  %sptr680 = inttoptr i64 %v46 to i64*
  store i64 %v264, i64* %sptr680
  %v265 = add i64 0, 0
  %v266 = add i64 0, 121
  call void @aial_rt_print(i64 %v266)
  %v267 = add i64 0, 0
  %v268 = add i64 0, 18
  call void @aial_rt_println(i64 %v268)
  %v269 = add i64 0, 0
  %lptr685 = inttoptr i64 %v37 to i64*
  %v270 = load i64, i64* %lptr685
  %v271 = call i64 @chat_show_user(i64 %v270)
  %lptr687 = inttoptr i64 %v4 to i64*
  %v272 = load i64, i64* %lptr687
  %v273 = add i64 0, 123
  %v274 = add i64 0, 124
  %lptr690 = inttoptr i64 %v37 to i64*
  %v275 = load i64, i64* %lptr690
  %v276 = call i64 @mem_save(i64 %v272, i64 %v273, i64 %v274, i64 %v275)
  %v277 = call i64 @chat_show_ai_prefix()
  %lptr693 = inttoptr i64 %v1 to i64*
  %v278 = load i64, i64* %lptr693
  %lptr694 = inttoptr i64 %v37 to i64*
  %v279 = load i64, i64* %lptr694
  %v280 = call i64 @chat_send(i64 %v278, i64 %v279)
  %aptr696 = alloca i64
  %v281 = ptrtoint i64* %aptr696 to i64
  %sptr697 = inttoptr i64 %v281 to i64*
  store i64 %v280, i64* %sptr697
  %v282 = add i64 0, 0
  %v283 = add i64 0, 18
  %aptr699 = alloca i64
  %v284 = ptrtoint i64* %aptr699 to i64
  %sptr700 = inttoptr i64 %v284 to i64*
  store i64 %v283, i64* %sptr700
  %v285 = add i64 0, 0
  %v286 = add i64 0, 1
  %aptr702 = alloca i64
  %v287 = ptrtoint i64* %aptr702 to i64
  %sptr703 = inttoptr i64 %v287 to i64*
  store i64 %v286, i64* %sptr703
  %v288 = add i64 0, 0
  %v289 = add i64 0, 0
  %aptr705 = alloca i64
  %v290 = ptrtoint i64* %aptr705 to i64
  %sptr706 = inttoptr i64 %v290 to i64*
  store i64 %v289, i64* %sptr706
  %v291 = add i64 0, 0
  %v292 = add i64 0, 0
  %aptr708 = alloca i64
  %v293 = ptrtoint i64* %aptr708 to i64
  %sptr709 = inttoptr i64 %v293 to i64*
  store i64 %v292, i64* %sptr709
  %v294 = add i64 0, 0
  br label %b215

b215:
  %lptr710 = inttoptr i64 %v281 to i64*
  %v295 = load i64, i64* %lptr710
  %v296 = call i64 @chat_read_token(i64 %v295)
  %aptr712 = alloca i64
  %v297 = ptrtoint i64* %aptr712 to i64
  %sptr713 = inttoptr i64 %v297 to i64*
  store i64 %v296, i64* %sptr713
  %v298 = add i64 0, 0
  %lptr714 = inttoptr i64 %v297 to i64*
  %v299 = load i64, i64* %lptr714
  %v300 = call i64 @aial_rt_strlen(i64 %v299)
  %v301 = add i64 0, 0
  %v302 = icmp eq i64 %v300, %v301
  br i1 %v302, label %b217, label %b218

b217:
  %lptr718 = inttoptr i64 %v290 to i64*
  %v303 = load i64, i64* %lptr718
  %v304 = add i64 0, 0
  %v305 = icmp eq i64 %v303, %v304
  %lptr721 = inttoptr i64 %v293 to i64*
  %v306 = load i64, i64* %lptr721
  %v307 = add i64 0, 50
  %v308 = icmp slt i64 %v306, %v307
  %v309 = and i1 %v305, %v308
  br i1 %v309, label %b220, label %b221

b220:
  %lptr725 = inttoptr i64 %v293 to i64*
  %v310 = load i64, i64* %lptr725
  %v311 = add i64 0, 1
  %v312 = add i64 %v310, %v311
  %sptr728 = inttoptr i64 %v293 to i64*
  store i64 %v312, i64* %sptr728
  %v313 = add i64 0, 0
  %v314 = add i64 0, 100
  call void @aial_rt_time_sleep(i64 %v314)
  %v315 = add i64 0, 0
  br label %b215

b219:
  %v316 = add i64 0, 1
  %sptr732 = inttoptr i64 %v290 to i64*
  store i64 %v316, i64* %sptr732
  %v317 = add i64 0, 0
  %lptr733 = inttoptr i64 %v287 to i64*
  %v318 = load i64, i64* %lptr733
  %v319 = add i64 0, 1
  %v320 = icmp eq i64 %v318, %v319
  br i1 %v320, label %b225, label %b226

b225:
  %v321 = add i64 0, 0
  %sptr737 = inttoptr i64 %v287 to i64*
  store i64 %v321, i64* %sptr737
  %v322 = add i64 0, 0
  %lptr738 = inttoptr i64 %v297 to i64*
  %v323 = load i64, i64* %lptr738
  %v324 = add i64 0, 126
  %x325 = call i64 @aial_rt_starts_with(i64 %v323, i64 %v324)
  %v325 = trunc i64 %x325 to i1
  %lptr741 = inttoptr i64 %v297 to i64*
  %v326 = load i64, i64* %lptr741
  %v327 = add i64 0, 127
  %x328 = call i64 @aial_rt_starts_with(i64 %v326, i64 %v327)
  %v328 = trunc i64 %x328 to i1
  %v329 = or i1 %v325, %v328
  %lptr745 = inttoptr i64 %v297 to i64*
  %v330 = load i64, i64* %lptr745
  %v331 = add i64 0, 128
  %x332 = call i64 @aial_rt_starts_with(i64 %v330, i64 %v331)
  %v332 = trunc i64 %x332 to i1
  %v333 = or i1 %v329, %v332
  %lptr749 = inttoptr i64 %v297 to i64*
  %v334 = load i64, i64* %lptr749
  %v335 = add i64 0, 129
  %x336 = call i64 @aial_rt_starts_with(i64 %v334, i64 %v335)
  %v336 = trunc i64 %x336 to i1
  %v337 = or i1 %v333, %v336
  %lptr753 = inttoptr i64 %v297 to i64*
  %v338 = load i64, i64* %lptr753
  %v339 = add i64 0, 130
  %x340 = call i64 @aial_rt_starts_with(i64 %v338, i64 %v339)
  %v340 = trunc i64 %x340 to i1
  %v341 = or i1 %v337, %v340
  br i1 %v341, label %b228, label %b229

b228:
  %lptr757 = inttoptr i64 %v297 to i64*
  %v342 = load i64, i64* %lptr757
  %v343 = call i64 @chat_show_error(i64 %v342)
  %v344 = call i64 @aial_rt_ctx_last_error()
  %aptr760 = alloca i64
  %v345 = ptrtoint i64* %aptr760 to i64
  %sptr761 = inttoptr i64 %v345 to i64*
  store i64 %v344, i64* %sptr761
  %v346 = add i64 0, 0
  %lptr762 = inttoptr i64 %v345 to i64*
  %v347 = load i64, i64* %lptr762
  %v348 = call i64 @aial_rt_strlen(i64 %v347)
  %v349 = add i64 0, 0
  %v350 = icmp sgt i64 %v348, %v349
  br i1 %v350, label %b231, label %b232

b231:
  %lptr766 = inttoptr i64 %v345 to i64*
  %v351 = load i64, i64* %lptr766
  %v352 = call i64 @chat_show_error(i64 %v351)
  br label %b233

b233:
  %lptr768 = inttoptr i64 %v297 to i64*
  %v353 = load i64, i64* %lptr768
  %v354 = add i64 0, 131
  %x355 = call i64 @aial_rt_starts_with(i64 %v353, i64 %v354)
  %v355 = trunc i64 %x355 to i1
  %lptr771 = inttoptr i64 %v297 to i64*
  %v356 = load i64, i64* %lptr771
  %v357 = add i64 0, 132
  %x358 = call i64 @aial_rt_starts_with(i64 %v356, i64 %v357)
  %v358 = trunc i64 %x358 to i1
  %v359 = or i1 %v355, %v358
  %lptr775 = inttoptr i64 %v345 to i64*
  %v360 = load i64, i64* %lptr775
  %v361 = add i64 0, 133
  %x362 = call i64 @aial_rt_starts_with(i64 %v360, i64 %v361)
  %v362 = trunc i64 %x362 to i1
  %v363 = or i1 %v359, %v362
  br i1 %v363, label %b234, label %b235

b234:
  %lptr779 = inttoptr i64 %v10 to i64*
  %v364 = load i64, i64* %lptr779
  %v365 = add i64 0, 25
  %v366 = call i64 @cmd_cache_get(i64 %v364, i64 %v365)
  %v367 = call i64 @chat_show_system(i64 %v366)
  br label %b236

b236:
  %lptr783 = inttoptr i64 %v297 to i64*
  %v368 = load i64, i64* %lptr783
  %sptr784 = inttoptr i64 %v284 to i64*
  store i64 %v368, i64* %sptr784
  %v369 = add i64 0, 0
  br label %b216

b227:
  %lptr785 = inttoptr i64 %v297 to i64*
  %v370 = load i64, i64* %lptr785
  call void @aial_rt_print(i64 %v370)
  %v371 = add i64 0, 0
  %lptr787 = inttoptr i64 %v284 to i64*
  %v372 = load i64, i64* %lptr787
  %lptr788 = inttoptr i64 %v297 to i64*
  %v373 = load i64, i64* %lptr788
  %v374 = call i64 @aial_rt_strcat(i64 %v372, i64 %v373)
  %sptr790 = inttoptr i64 %v284 to i64*
  store i64 %v374, i64* %sptr790
  %v375 = add i64 0, 0
  %v376 = add i64 0, 0
  %v377 = call i64 @aial_rt_io_readkey_timeout(i64 %v376)
  %aptr793 = alloca i64
  %v378 = ptrtoint i64* %aptr793 to i64
  %sptr794 = inttoptr i64 %v378 to i64*
  store i64 %v377, i64* %sptr794
  %v379 = add i64 0, 0
  %lptr795 = inttoptr i64 %v378 to i64*
  %v380 = load i64, i64* %lptr795
  %v381 = call i64 @aial_rt_strlen(i64 %v380)
  %v382 = add i64 0, 0
  %v383 = icmp sgt i64 %v381, %v382
  br i1 %v383, label %b238, label %b239

b238:
  %lptr799 = inttoptr i64 %v378 to i64*
  %v384 = load i64, i64* %lptr799
  %v385 = call i64 @K_CTRL_Q()
  %x386 = call i64 @aial_rt_str_eq(i64 %v384, i64 %v385)
  %v386 = trunc i64 %x386 to i1
  br i1 %v386, label %b241, label %b242

b216:
  %lptr802 = inttoptr i64 %v290 to i64*
  %v387 = load i64, i64* %lptr802
  %v388 = add i64 0, 0
  %v389 = icmp eq i64 %v387, %v388
  br i1 %v389, label %b245, label %b246

b245:
  %v390 = add i64 0, 135
  %v391 = call i64 @chat_show_error(i64 %v390)
  %v392 = call i64 @aial_rt_ctx_last_error()
  %aptr808 = alloca i64
  %v393 = ptrtoint i64* %aptr808 to i64
  %sptr809 = inttoptr i64 %v393 to i64*
  store i64 %v392, i64* %sptr809
  %v394 = add i64 0, 0
  %lptr810 = inttoptr i64 %v393 to i64*
  %v395 = load i64, i64* %lptr810
  %v396 = call i64 @aial_rt_strlen(i64 %v395)
  %v397 = add i64 0, 0
  %v398 = icmp sgt i64 %v396, %v397
  br i1 %v398, label %b248, label %b249

b248:
  %lptr814 = inttoptr i64 %v393 to i64*
  %v399 = load i64, i64* %lptr814
  %v400 = call i64 @chat_show_error(i64 %v399)
  br label %b250

b250:
  %v401 = add i64 0, 136
  %sptr817 = inttoptr i64 %v284 to i64*
  store i64 %v401, i64* %sptr817
  %v402 = add i64 0, 0
  br label %b247

b247:
  %v403 = add i64 0, 18
  call void @aial_rt_println(i64 %v403)
  %v404 = add i64 0, 0
  %lptr820 = inttoptr i64 %v284 to i64*
  %v405 = load i64, i64* %lptr820
  %v406 = add i64 0, 128
  %x407 = call i64 @aial_rt_starts_with(i64 %v405, i64 %v406)
  %v407 = trunc i64 %x407 to i1
  %v408 = xor i1 %v407, true
  br i1 %v408, label %b251, label %b252

b251:
  %lptr824 = inttoptr i64 %v4 to i64*
  %v409 = load i64, i64* %lptr824
  %v410 = add i64 0, 123
  %v411 = add i64 0, 140
  %lptr827 = inttoptr i64 %v284 to i64*
  %v412 = load i64, i64* %lptr827
  %v413 = call i64 @mem_save(i64 %v409, i64 %v410, i64 %v411, i64 %v412)
  %lptr829 = inttoptr i64 %v1 to i64*
  %v414 = load i64, i64* %lptr829
  %v415 = add i64 0, 124
  %lptr831 = inttoptr i64 %v37 to i64*
  %v416 = load i64, i64* %lptr831
  %v417 = call i64 @aial_rt_ctx_add_message(i64 %v414, i64 %v415, i64 %v416)
  %sptr833 = inttoptr i64 %v1 to i64*
  store i64 %v417, i64* %sptr833
  %v418 = add i64 0, 0
  %lptr834 = inttoptr i64 %v1 to i64*
  %v419 = load i64, i64* %lptr834
  %v420 = add i64 0, 140
  %lptr836 = inttoptr i64 %v284 to i64*
  %v421 = load i64, i64* %lptr836
  %v422 = call i64 @aial_rt_ctx_add_message(i64 %v419, i64 %v420, i64 %v421)
  %sptr838 = inttoptr i64 %v1 to i64*
  store i64 %v422, i64* %sptr838
  %v423 = add i64 0, 0
  br label %b253

b253:
  %lptr839 = inttoptr i64 %v46 to i64*
  %v424 = load i64, i64* %lptr839
  %v425 = add i64 0, 1
  %v426 = add i64 %v424, %v425
  %sptr842 = inttoptr i64 %v46 to i64*
  store i64 %v426, i64* %sptr842
  %v427 = add i64 0, 0
  %v428 = add i64 0, 123
  %lptr844 = inttoptr i64 %v46 to i64*
  %v429 = load i64, i64* %lptr844
  %lptr845 = inttoptr i64 %v284 to i64*
  %v430 = load i64, i64* %lptr845
  %v431 = call i64 @aial_rt_token_estimate(i64 %v430)
  %v432 = call i64 @bar_draw(i64 %v428, i64 %v429, i64 %v431)
  %v433 = add i64 0, 18
  %sptr849 = inttoptr i64 %v37 to i64*
  store i64 %v433, i64* %sptr849
  %v434 = add i64 0, 0
  %v435 = call i64 @input_draw_border()
  br label %b129

b184:
  %lptr851 = inttoptr i64 %v49 to i64*
  %v436 = load i64, i64* %lptr851
  %v437 = call i64 @K_BACKSPACE()
  %x438 = call i64 @aial_rt_str_eq(i64 %v436, i64 %v437)
  %v438 = trunc i64 %x438 to i1
  br i1 %v438, label %b255, label %b256

b255:
  %lptr854 = inttoptr i64 %v37 to i64*
  %v439 = load i64, i64* %lptr854
  %v440 = call i64 @aial_rt_strlen(i64 %v439)
  %v441 = add i64 0, 0
  %v442 = icmp sgt i64 %v440, %v441
  br i1 %v442, label %b258, label %b259

b258:
  %lptr858 = inttoptr i64 %v37 to i64*
  %v443 = load i64, i64* %lptr858
  %v444 = add i64 0, 0
  %lptr860 = inttoptr i64 %v37 to i64*
  %v445 = load i64, i64* %lptr860
  %v446 = call i64 @aial_rt_strlen(i64 %v445)
  %v447 = add i64 0, 1
  %v448 = sub i64 %v446, %v447
  %v449 = call i64 @aial_rt_strslice(i64 %v443, i64 %v444, i64 %v448)
  %sptr865 = inttoptr i64 %v37 to i64*
  store i64 %v449, i64* %sptr865
  %v450 = add i64 0, 0
  %v451 = add i64 0, 145
  call void @aial_rt_print(i64 %v451)
  %v452 = add i64 0, 0
  br label %b260

b257:
  %lptr868 = inttoptr i64 %v49 to i64*
  %v453 = load i64, i64* %lptr868
  %v454 = call i64 @aial_rt_strlen(i64 %v453)
  %v455 = add i64 0, 1
  %v456 = icmp sge i64 %v454, %v455
  br i1 %v456, label %b262, label %b263

b262:
  %lptr872 = inttoptr i64 %v49 to i64*
  %v457 = load i64, i64* %lptr872
  %v458 = add i64 0, 0
  %v459 = call i64 @aial_rt_strchr(i64 %v457, i64 %v458)
  %aptr875 = alloca i64
  %v460 = ptrtoint i64* %aptr875 to i64
  %sptr876 = inttoptr i64 %v460 to i64*
  store i64 %v459, i64* %sptr876
  %v461 = add i64 0, 0
  %lptr877 = inttoptr i64 %v460 to i64*
  %v462 = load i64, i64* %lptr877
  %v463 = add i64 0, 32
  %v464 = icmp sge i64 %v462, %v463
  %lptr880 = inttoptr i64 %v460 to i64*
  %v465 = load i64, i64* %lptr880
  %v466 = add i64 0, 128
  %v467 = icmp sge i64 %v465, %v466
  %v468 = or i1 %v464, %v467
  br i1 %v468, label %b265, label %b266

b265:
  %lptr884 = inttoptr i64 %v49 to i64*
  %v469 = load i64, i64* %lptr884
  call void @aial_rt_print(i64 %v469)
  %v470 = add i64 0, 0
  %lptr886 = inttoptr i64 %v37 to i64*
  %v471 = load i64, i64* %lptr886
  %lptr887 = inttoptr i64 %v49 to i64*
  %v472 = load i64, i64* %lptr887
  %v473 = call i64 @aial_rt_strcat(i64 %v471, i64 %v472)
  %sptr889 = inttoptr i64 %v37 to i64*
  store i64 %v473, i64* %sptr889
  %v474 = add i64 0, 0
  br label %b267

b130:
  %v475 = add i64 0, 0
  call void @aial_rt_io_raw_mode(i64 %v475)
  %v476 = add i64 0, 0
  %v477 = add i64 0, 91
  call void @aial_rt_print(i64 %v477)
  %v478 = add i64 0, 0
  %v479 = call i64 @header_draw()
  %v480 = add i64 0, 147
  %v481 = call i64 @chat_show_system(i64 %v480)
  %v482 = call i64 @input_draw_bottom()
  %v483 = add i64 0, 123
  %lptr899 = inttoptr i64 %v46 to i64*
  %v484 = load i64, i64* %lptr899
  %v485 = add i64 0, 0
  %v486 = call i64 @bar_draw(i64 %v483, i64 %v484, i64 %v485)
  %lptr902 = inttoptr i64 %v4 to i64*
  %v487 = load i64, i64* %lptr902
  %v488 = call i64 @mem_close(i64 %v487)
  ret i32 0

b132:
  br label %b133

b134:
  br label %b130

b135:
  br label %b136

b137:
  br label %b136

b138:
  br label %b133

b139:
  br label %b130

b140:
  br label %b141

b142:
  br label %b141

b144:
  br label %b145

b146:
  br label %b145

b148:
  br label %b149

b151:
  br label %b152

b154:
  br label %b155

b155:
  br label %b129

b156:
  br label %b149

b158:
  br label %b159

b161:
  br label %b162

b162:
  br label %b129

b163:
  br label %b159

b165:
  br label %b166

b169:
  br label %b129

b171:
  br label %b172

b172:
  br label %b169

b173:
  br label %b166

b174:
  br label %b129

b175:
  br label %b176

b177:
  br label %b176

b178:
  br label %b129

b179:
  br label %b180

b181:
  br label %b180

b183:
  br label %b184

b186:
  br label %b187

b188:
  br label %b187

b189:
  br label %b129

b190:
  br label %b191

b192:
  br label %b191

b194:
  br label %b195

b201:
  br label %b198

b202:
  br label %b195

b204:
  br label %b205

b206:
  br label %b205

b207:
  br label %b130

b208:
  br label %b209

b210:
  br label %b209

b212:
  br label %b213

b214:
  br label %b213

b218:
  br label %b219

b221:
  br label %b222

b222:
  br label %b216

b223:
  br label %b222

b224:
  br label %b219

b226:
  br label %b227

b229:
  br label %b230

b230:
  br label %b227

b232:
  br label %b233

b235:
  br label %b236

b237:
  br label %b230

b239:
  br label %b240

b240:
  br label %b215

b241:
  br label %b216

b242:
  br label %b243

b243:
  br label %b240

b244:
  br label %b243

b246:
  br label %b247

b249:
  br label %b250

b252:
  br label %b253

b254:
  br label %b184

b256:
  br label %b257

b259:
  br label %b260

b260:
  br label %b129

b261:
  br label %b257

b263:
  br label %b264

b264:
  br label %b129

b266:
  br label %b267

b267:
  br label %b264
}

