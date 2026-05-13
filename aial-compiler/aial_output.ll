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
declare i64 @aial_rt_strchr(i64, i64)
declare i64 @aial_rt_starts_with(i64, i64)
declare i64 @aial_rt_strslice(i64, i64, i64)
declare i64 @aial_rt_key_set(i64, i64)
declare i64 @aial_rt_str_eq(i64, i64)
declare i64 @aial_rt_ctx_last_error()
declare i64 @aial_rt_io_readkey_timeout(i64)
declare i64 @aial_rt_token_estimate(i64)
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
@.str73 = private unnamed_addr constant [84 x i8] c"     DEEP TUI  —  AIAL Terminal AI Chat   |   v0.3   |   github.com/gordonlu/aial\00", align 1
@.str74 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str75 = private unnamed_addr constant [13 x i8] c" Deep TUI | \00", align 1
@.str76 = private unnamed_addr constant [11 x i8] c" | msgs: ~\00", align 1
@.str77 = private unnamed_addr constant [2 x i8] c"N\00", align 1
@.str78 = private unnamed_addr constant [13 x i8] c" | tokens: ~\00", align 1
@.str79 = private unnamed_addr constant [2 x i8] c"N\00", align 1
@.str80 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str81 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str82 = private unnamed_addr constant [48 x i8] c"Run /key for API setup, /help for all commands.\00", align 1
@.str83 = private unnamed_addr constant [9 x i8] c"deepseek\00", align 1
@.str84 = private unnamed_addr constant [22 x i8] c"API key found. Ready!\00", align 1
@.str85 = private unnamed_addr constant [23 x i8] c"No API key configured.\00", align 1
@.str86 = private unnamed_addr constant [52 x i8] c"Type /key set YOUR_KEY  or  export DEEPSEEK_API_KEY\00", align 1
@.str87 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str88 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str89 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str90 = private unnamed_addr constant [48 x i8] c"multi-line ON — Enter for newline, ^D to send\00", align 1
@.str91 = private unnamed_addr constant [15 x i8] c"multi-line OFF\00", align 1
@.str92 = private unnamed_addr constant [9 x i8] c"\0D[K  > \00", align 1
@.str93 = private unnamed_addr constant [9 x i8] c"\0D[K  > \00", align 1
@.str94 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str95 = private unnamed_addr constant [5 x i8] c"\0D[K\00", align 1
@.str96 = private unnamed_addr constant [5 x i8] c"  > \00", align 1
@.str97 = private unnamed_addr constant [2 x i8] c"\0A\00", align 1
@.str98 = private unnamed_addr constant [4 x i8] c"\0A+ \00", align 1
@.str99 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str100 = private unnamed_addr constant [10 x i8] c"/key set \00", align 1
@.str101 = private unnamed_addr constant [9 x i8] c"deepseek\00", align 1
@.str102 = private unnamed_addr constant [41 x i8] c"Key saved securely. Restart recommended.\00", align 1
@.str103 = private unnamed_addr constant [20 x i8] c"Failed to save key.\00", align 1
@.str104 = private unnamed_addr constant [33 x i8] c"Usage: /key set sk-your-key-here\00", align 1
@.str105 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str106 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str107 = private unnamed_addr constant [6 x i8] c"/quit\00", align 1
@.str108 = private unnamed_addr constant [7 x i8] c"/clear\00", align 1
@.str109 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str110 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str111 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str112 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str113 = private unnamed_addr constant [5 x i8] c"user\00", align 1
@.str114 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str115 = private unnamed_addr constant [7 x i8] c"Error:\00", align 1
@.str116 = private unnamed_addr constant [7 x i8] c"error:\00", align 1
@.str117 = private unnamed_addr constant [18 x i8] c"Error: No API key\00", align 1
@.str118 = private unnamed_addr constant [11 x i8] c"No API key\00", align 1
@.str119 = private unnamed_addr constant [11 x i8] c"no_api_key\00", align 1
@.str120 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str121 = private unnamed_addr constant [7 x i8] c"Error:\00", align 1
@.str122 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str123 = private unnamed_addr constant [10 x i8] c"assistant\00", align 1
@.str124 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str125 = private unnamed_addr constant [5 x i8] c"main\00", align 1
@.str126 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str127 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str128 = private unnamed_addr constant [8 x i8] c"[D [D\00", align 1
@.str129 = private unnamed_addr constant [8 x i8] c"[2J[H\00", align 1
@.str130 = private unnamed_addr constant [24 x i8] c"Session saved. Goodbye!\00", align 1
@.str131 = private unnamed_addr constant [5 x i8] c"main\00", align 1

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
  %v3 = add i64 0, 0
  %v4 = add i64 0, 0
  %v5 = add i64 0, 0
  %v6 = add i64 0, 0
  %lptr103 = inttoptr i64 %arg0_ptr to i64*
  %v7 = load i64, i64* %lptr103
  %lptr104 = inttoptr i64 %arg1_ptr to i64*
  %v8 = load i64, i64* %lptr104
  %v9 = add i1 0, 1
  %v10 = add i64 0, 1024
  %v11 = call i64 @aial_rt_ai_stream_start(i64 %v6, i64 %v7, i64 %v8, i64 %v3, i64 %v10, i64 %v5)
  ret i64 %v11
}

define i64 @chat_read_token(i64 %arg0) {

b64:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %lptr108 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr108
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
  %lptr112 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr112
  %lptr113 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr113
  %x2 = call i64 @aial_rt_map_has(i64 %v0, i64 %v1)
  %v2 = trunc i64 %x2 to i1
  br i1 %v2, label %b71, label %b72

b71:
  %lptr115 = inttoptr i64 %arg0_ptr to i64*
  %v3 = load i64, i64* %lptr115
  %lptr116 = inttoptr i64 %arg1_ptr to i64*
  %v4 = load i64, i64* %lptr116
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
  %lptr119 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr119
  %lptr120 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr120
  %lptr121 = inttoptr i64 %arg2_ptr to i64*
  %v2 = load i64, i64* %lptr121
  call void @aial_rt_map_set(i64 %v0, i64 %v1, i64 %v2)
  %v3 = add i64 0, 0
  ret void
}

define void @cmd_cache_populate(i64 %arg0) {

b77:
  %arg0_addr = alloca i64
  store i64 %arg0, i64* %arg0_addr
  %arg0_ptr = ptrtoint i64* %arg0_addr to i64
  %lptr123 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr123
  %v1 = add i64 0, 19
  %v2 = add i64 0, 20
  call void @aial_rt_map_set(i64 %v0, i64 %v1, i64 %v2)
  %v3 = add i64 0, 0
  %lptr127 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr127
  %v5 = add i64 0, 21
  %v6 = add i64 0, 22
  call void @aial_rt_map_set(i64 %v4, i64 %v5, i64 %v6)
  %v7 = add i64 0, 0
  %lptr131 = inttoptr i64 %arg0_ptr to i64*
  %v8 = load i64, i64* %lptr131
  %v9 = add i64 0, 23
  %v10 = add i64 0, 24
  call void @aial_rt_map_set(i64 %v8, i64 %v9, i64 %v10)
  %v11 = add i64 0, 0
  %lptr135 = inttoptr i64 %arg0_ptr to i64*
  %v12 = load i64, i64* %lptr135
  %v13 = add i64 0, 25
  %v14 = add i64 0, 26
  call void @aial_rt_map_set(i64 %v12, i64 %v13, i64 %v14)
  %v15 = add i64 0, 0
  %lptr139 = inttoptr i64 %arg0_ptr to i64*
  %v16 = load i64, i64* %lptr139
  %v17 = add i64 0, 27
  %v18 = add i64 0, 28
  call void @aial_rt_map_set(i64 %v16, i64 %v17, i64 %v18)
  %v19 = add i64 0, 0
  ret void
}

define void @header_draw() {

b78:
  %v0 = call i64 @theme_dim()
  %aptr144 = alloca i64
  %v1 = ptrtoint i64* %aptr144 to i64
  %sptr145 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr145
  %v2 = add i64 0, 0
  %v3 = call i64 @theme_reset()
  %aptr147 = alloca i64
  %v4 = ptrtoint i64* %aptr147 to i64
  %sptr148 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr148
  %v5 = add i64 0, 0
  %v6 = add i64 0, 29
  %aptr150 = alloca i64
  %v7 = ptrtoint i64* %aptr150 to i64
  %sptr151 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr151
  %v8 = add i64 0, 0
  %lptr152 = inttoptr i64 %v1 to i64*
  %v9 = load i64, i64* %lptr152
  %lptr153 = inttoptr i64 %v7 to i64*
  %v10 = load i64, i64* %lptr153
  %v11 = call i64 @aial_rt_strcat(i64 %v9, i64 %v10)
  %lptr155 = inttoptr i64 %v4 to i64*
  %v12 = load i64, i64* %lptr155
  %v13 = call i64 @aial_rt_strcat(i64 %v11, i64 %v12)
  call void @aial_rt_println(i64 %v13)
  %v14 = add i64 0, 0
  %v15 = add i64 0, 30
  call void @aial_rt_println(i64 %v15)
  %v16 = add i64 0, 0
  %lptr160 = inttoptr i64 %v1 to i64*
  %v17 = load i64, i64* %lptr160
  %lptr161 = inttoptr i64 %v7 to i64*
  %v18 = load i64, i64* %lptr161
  %v19 = call i64 @aial_rt_strcat(i64 %v17, i64 %v18)
  %lptr163 = inttoptr i64 %v4 to i64*
  %v20 = load i64, i64* %lptr163
  %v21 = call i64 @aial_rt_strcat(i64 %v19, i64 %v20)
  call void @aial_rt_println(i64 %v21)
  %v22 = add i64 0, 0
  ret void
}

define void @input_draw_border() {

b79:
  %v0 = call i64 @theme_dim()
  %aptr167 = alloca i64
  %v1 = ptrtoint i64* %aptr167 to i64
  %sptr168 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr168
  %v2 = add i64 0, 0
  %v3 = call i64 @color_input()
  %aptr170 = alloca i64
  %v4 = ptrtoint i64* %aptr170 to i64
  %sptr171 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr171
  %v5 = add i64 0, 0
  %v6 = call i64 @theme_reset()
  %aptr173 = alloca i64
  %v7 = ptrtoint i64* %aptr173 to i64
  %sptr174 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr174
  %v8 = add i64 0, 0
  %v9 = add i64 0, 29
  %aptr176 = alloca i64
  %v10 = ptrtoint i64* %aptr176 to i64
  %sptr177 = inttoptr i64 %v10 to i64*
  store i64 %v9, i64* %sptr177
  %v11 = add i64 0, 0
  %v12 = add i64 0, 32
  %lptr179 = inttoptr i64 %v1 to i64*
  %v13 = load i64, i64* %lptr179
  %v14 = call i64 @aial_rt_strcat(i64 %v12, i64 %v13)
  %lptr181 = inttoptr i64 %v10 to i64*
  %v15 = load i64, i64* %lptr181
  %v16 = call i64 @aial_rt_strcat(i64 %v14, i64 %v15)
  %lptr183 = inttoptr i64 %v7 to i64*
  %v17 = load i64, i64* %lptr183
  %v18 = call i64 @aial_rt_strcat(i64 %v16, i64 %v17)
  call void @aial_rt_println(i64 %v18)
  %v19 = add i64 0, 0
  %lptr186 = inttoptr i64 %v4 to i64*
  %v20 = load i64, i64* %lptr186
  %v21 = add i64 0, 33
  %v22 = call i64 @aial_rt_strcat(i64 %v20, i64 %v21)
  %lptr189 = inttoptr i64 %v7 to i64*
  %v23 = load i64, i64* %lptr189
  %v24 = call i64 @aial_rt_strcat(i64 %v22, i64 %v23)
  call void @aial_rt_print(i64 %v24)
  %v25 = add i64 0, 0
  ret void
}

define void @input_draw_bottom() {

b80:
  %v0 = call i64 @theme_dim()
  %aptr193 = alloca i64
  %v1 = ptrtoint i64* %aptr193 to i64
  %sptr194 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr194
  %v2 = add i64 0, 0
  %v3 = call i64 @theme_reset()
  %aptr196 = alloca i64
  %v4 = ptrtoint i64* %aptr196 to i64
  %sptr197 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr197
  %v5 = add i64 0, 0
  %v6 = add i64 0, 29
  %aptr199 = alloca i64
  %v7 = ptrtoint i64* %aptr199 to i64
  %sptr200 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr200
  %v8 = add i64 0, 0
  %v9 = add i64 0, 18
  call void @aial_rt_println(i64 %v9)
  %v10 = add i64 0, 0
  %lptr203 = inttoptr i64 %v1 to i64*
  %v11 = load i64, i64* %lptr203
  %lptr204 = inttoptr i64 %v7 to i64*
  %v12 = load i64, i64* %lptr204
  %v13 = call i64 @aial_rt_strcat(i64 %v11, i64 %v12)
  %lptr206 = inttoptr i64 %v4 to i64*
  %v14 = load i64, i64* %lptr206
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
  %lptr209 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr209
  %lptr210 = inttoptr i64 %arg1_ptr to i64*
  %v1 = load i64, i64* %lptr210
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
  %lptr212 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr212
  %v1 = call i64 @aial_rt_array_len(i64 %v0)
  %aptr214 = alloca i64
  %v2 = ptrtoint i64* %aptr214 to i64
  %sptr215 = inttoptr i64 %v2 to i64*
  store i64 %v1, i64* %sptr215
  %v3 = add i64 0, 0
  %lptr216 = inttoptr i64 %v2 to i64*
  %v4 = load i64, i64* %lptr216
  %v5 = add i64 0, 0
  %v6 = icmp eq i64 %v4, %v5
  br i1 %v6, label %b83, label %b84

b83:
  %v7 = add i64 0, 18
  ret i64 %v7

b85:
  %lptr220 = inttoptr i64 %arg1_ptr to i64*
  %v8 = load i64, i64* %lptr220
  %v9 = add i64 0, 0
  %v10 = icmp slt i64 %v8, %v9
  br i1 %v10, label %b87, label %b88

b87:
  %v11 = add i64 0, 18
  ret i64 %v11

b89:
  %lptr224 = inttoptr i64 %arg1_ptr to i64*
  %v12 = load i64, i64* %lptr224
  %lptr225 = inttoptr i64 %v2 to i64*
  %v13 = load i64, i64* %lptr225
  %v14 = icmp sge i64 %v12, %v13
  br i1 %v14, label %b91, label %b92

b91:
  %v15 = add i64 0, 18
  ret i64 %v15

b93:
  %lptr228 = inttoptr i64 %arg0_ptr to i64*
  %v16 = load i64, i64* %lptr228
  %lptr229 = inttoptr i64 %arg1_ptr to i64*
  %v17 = load i64, i64* %lptr229
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
  %lptr231 = inttoptr i64 %arg0_ptr to i64*
  %v0 = load i64, i64* %lptr231
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
  %lptr239 = inttoptr i64 %arg0_ptr to i64*
  %v6 = load i64, i64* %lptr239
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
  %lptr251 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr251
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
  %lptr259 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr259
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
  %lptr267 = inttoptr i64 %arg0_ptr to i64*
  %v4 = load i64, i64* %lptr267
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
  %aptr272 = alloca i64
  %v1 = ptrtoint i64* %aptr272 to i64
  %sptr273 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr273
  %v2 = add i64 0, 0
  %v3 = add i64 0, 45
  %aptr275 = alloca i64
  %v4 = ptrtoint i64* %aptr275 to i64
  %sptr276 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr276
  %v5 = add i64 0, 0
  %v6 = add i64 0, 46
  %aptr278 = alloca i64
  %v7 = ptrtoint i64* %aptr278 to i64
  %sptr279 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr279
  %v8 = add i64 0, 0
  %v9 = add i64 0, 47
  %aptr281 = alloca i64
  %v10 = ptrtoint i64* %aptr281 to i64
  %sptr282 = inttoptr i64 %v10 to i64*
  store i64 %v9, i64* %sptr282
  %v11 = add i64 0, 0
  %v12 = add i64 0, 48
  %aptr284 = alloca i64
  %v13 = ptrtoint i64* %aptr284 to i64
  %sptr285 = inttoptr i64 %v13 to i64*
  store i64 %v12, i64* %sptr285
  %v14 = add i64 0, 0
  %v15 = add i64 0, 49
  %aptr287 = alloca i64
  %v16 = ptrtoint i64* %aptr287 to i64
  %sptr288 = inttoptr i64 %v16 to i64*
  store i64 %v15, i64* %sptr288
  %v17 = add i64 0, 0
  %v18 = add i64 0, 50
  %aptr290 = alloca i64
  %v19 = ptrtoint i64* %aptr290 to i64
  %sptr291 = inttoptr i64 %v19 to i64*
  store i64 %v18, i64* %sptr291
  %v20 = add i64 0, 0
  %v21 = add i64 0, 51
  %aptr293 = alloca i64
  %v22 = ptrtoint i64* %aptr293 to i64
  %sptr294 = inttoptr i64 %v22 to i64*
  store i64 %v21, i64* %sptr294
  %v23 = add i64 0, 0
  %v24 = add i64 0, 52
  %aptr296 = alloca i64
  %v25 = ptrtoint i64* %aptr296 to i64
  %sptr297 = inttoptr i64 %v25 to i64*
  store i64 %v24, i64* %sptr297
  %v26 = add i64 0, 0
  %v27 = add i64 0, 53
  %aptr299 = alloca i64
  %v28 = ptrtoint i64* %aptr299 to i64
  %sptr300 = inttoptr i64 %v28 to i64*
  store i64 %v27, i64* %sptr300
  %v29 = add i64 0, 0
  %v30 = add i64 0, 54
  %aptr302 = alloca i64
  %v31 = ptrtoint i64* %aptr302 to i64
  %sptr303 = inttoptr i64 %v31 to i64*
  store i64 %v30, i64* %sptr303
  %v32 = add i64 0, 0
  %v33 = add i64 0, 55
  %aptr305 = alloca i64
  %v34 = ptrtoint i64* %aptr305 to i64
  %sptr306 = inttoptr i64 %v34 to i64*
  store i64 %v33, i64* %sptr306
  %v35 = add i64 0, 0
  %v36 = add i64 0, 56
  %aptr308 = alloca i64
  %v37 = ptrtoint i64* %aptr308 to i64
  %sptr309 = inttoptr i64 %v37 to i64*
  store i64 %v36, i64* %sptr309
  %v38 = add i64 0, 0
  %v39 = add i64 0, 57
  %aptr311 = alloca i64
  %v40 = ptrtoint i64* %aptr311 to i64
  %sptr312 = inttoptr i64 %v40 to i64*
  store i64 %v39, i64* %sptr312
  %v41 = add i64 0, 0
  %v42 = add i64 0, 57
  %aptr314 = alloca i64
  %v43 = ptrtoint i64* %aptr314 to i64
  %sptr315 = inttoptr i64 %v43 to i64*
  store i64 %v42, i64* %sptr315
  %v44 = add i64 0, 0
  %lptr316 = inttoptr i64 %v4 to i64*
  %v45 = load i64, i64* %lptr316
  %v46 = add i64 0, 59
  %v47 = call i64 @aial_rt_strcat(i64 %v45, i64 %v46)
  call void @aial_rt_println(i64 %v47)
  %v48 = add i64 0, 0
  %lptr320 = inttoptr i64 %v7 to i64*
  %v49 = load i64, i64* %lptr320
  %v50 = add i64 0, 60
  %v51 = call i64 @aial_rt_strcat(i64 %v49, i64 %v50)
  call void @aial_rt_println(i64 %v51)
  %v52 = add i64 0, 0
  %lptr324 = inttoptr i64 %v10 to i64*
  %v53 = load i64, i64* %lptr324
  %v54 = add i64 0, 61
  %v55 = call i64 @aial_rt_strcat(i64 %v53, i64 %v54)
  call void @aial_rt_println(i64 %v55)
  %v56 = add i64 0, 0
  %lptr328 = inttoptr i64 %v13 to i64*
  %v57 = load i64, i64* %lptr328
  %v58 = add i64 0, 62
  %v59 = call i64 @aial_rt_strcat(i64 %v57, i64 %v58)
  call void @aial_rt_println(i64 %v59)
  %v60 = add i64 0, 0
  %lptr332 = inttoptr i64 %v16 to i64*
  %v61 = load i64, i64* %lptr332
  %v62 = add i64 0, 63
  %v63 = call i64 @aial_rt_strcat(i64 %v61, i64 %v62)
  call void @aial_rt_println(i64 %v63)
  %v64 = add i64 0, 0
  %lptr336 = inttoptr i64 %v19 to i64*
  %v65 = load i64, i64* %lptr336
  %v66 = add i64 0, 64
  %v67 = call i64 @aial_rt_strcat(i64 %v65, i64 %v66)
  call void @aial_rt_println(i64 %v67)
  %v68 = add i64 0, 0
  %lptr340 = inttoptr i64 %v22 to i64*
  %v69 = load i64, i64* %lptr340
  %v70 = add i64 0, 65
  %v71 = call i64 @aial_rt_strcat(i64 %v69, i64 %v70)
  call void @aial_rt_println(i64 %v71)
  %v72 = add i64 0, 0
  %lptr344 = inttoptr i64 %v25 to i64*
  %v73 = load i64, i64* %lptr344
  %v74 = add i64 0, 66
  %v75 = call i64 @aial_rt_strcat(i64 %v73, i64 %v74)
  call void @aial_rt_println(i64 %v75)
  %v76 = add i64 0, 0
  %lptr348 = inttoptr i64 %v28 to i64*
  %v77 = load i64, i64* %lptr348
  %v78 = add i64 0, 67
  %v79 = call i64 @aial_rt_strcat(i64 %v77, i64 %v78)
  call void @aial_rt_println(i64 %v79)
  %v80 = add i64 0, 0
  %lptr352 = inttoptr i64 %v31 to i64*
  %v81 = load i64, i64* %lptr352
  %v82 = add i64 0, 68
  %v83 = call i64 @aial_rt_strcat(i64 %v81, i64 %v82)
  call void @aial_rt_println(i64 %v83)
  %v84 = add i64 0, 0
  %lptr356 = inttoptr i64 %v34 to i64*
  %v85 = load i64, i64* %lptr356
  %v86 = add i64 0, 69
  %v87 = call i64 @aial_rt_strcat(i64 %v85, i64 %v86)
  call void @aial_rt_println(i64 %v87)
  %v88 = add i64 0, 0
  %lptr360 = inttoptr i64 %v37 to i64*
  %v89 = load i64, i64* %lptr360
  %v90 = add i64 0, 70
  %v91 = call i64 @aial_rt_strcat(i64 %v89, i64 %v90)
  call void @aial_rt_println(i64 %v91)
  %v92 = add i64 0, 0
  %lptr364 = inttoptr i64 %v40 to i64*
  %v93 = load i64, i64* %lptr364
  %v94 = add i64 0, 71
  %lptr366 = inttoptr i64 %v1 to i64*
  %v95 = load i64, i64* %lptr366
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
  %lptr375 = inttoptr i64 %v1 to i64*
  %v104 = load i64, i64* %lptr375
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
  %aptr381 = alloca i64
  %v1 = ptrtoint i64* %aptr381 to i64
  %sptr382 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr382
  %v2 = add i64 0, 0
  %v3 = call i64 @color_bar_reset()
  %aptr384 = alloca i64
  %v4 = ptrtoint i64* %aptr384 to i64
  %sptr385 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr385
  %v5 = add i64 0, 0
  %lptr386 = inttoptr i64 %v1 to i64*
  %v6 = load i64, i64* %lptr386
  %v7 = add i64 0, 75
  %v8 = call i64 @aial_rt_strcat(i64 %v6, i64 %v7)
  call void @aial_rt_print(i64 %v8)
  %v9 = add i64 0, 0
  %lptr390 = inttoptr i64 %arg0_ptr to i64*
  %v10 = load i64, i64* %lptr390
  call void @aial_rt_print(i64 %v10)
  %v11 = add i64 0, 0
  %v12 = add i64 0, 76
  call void @aial_rt_print(i64 %v12)
  %v13 = add i64 0, 0
  %v14 = add i64 0, 77
  call void @aial_rt_print(i64 %v14)
  %v15 = add i64 0, 0
  %v16 = add i64 0, 78
  call void @aial_rt_print(i64 %v16)
  %v17 = add i64 0, 0
  %v18 = add i64 0, 77
  call void @aial_rt_print(i64 %v18)
  %v19 = add i64 0, 0
  %v20 = add i64 0, 80
  %lptr401 = inttoptr i64 %v4 to i64*
  %v21 = load i64, i64* %lptr401
  %v22 = call i64 @aial_rt_strcat(i64 %v20, i64 %v21)
  call void @aial_rt_println(i64 %v22)
  %v23 = add i64 0, 0
  ret void
}

define i32 @main() {

b105:
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
  %str_init_73 = getelementptr inbounds [84 x i8], [84 x i8]* @.str73, i32 0, i32 0
  call void @aial_rt_string_register(i64 73, i8* %str_init_73)
  %str_init_74 = getelementptr inbounds [1 x i8], [1 x i8]* @.str74, i32 0, i32 0
  call void @aial_rt_string_register(i64 74, i8* %str_init_74)
  %str_init_75 = getelementptr inbounds [13 x i8], [13 x i8]* @.str75, i32 0, i32 0
  call void @aial_rt_string_register(i64 75, i8* %str_init_75)
  %str_init_76 = getelementptr inbounds [11 x i8], [11 x i8]* @.str76, i32 0, i32 0
  call void @aial_rt_string_register(i64 76, i8* %str_init_76)
  %str_init_77 = getelementptr inbounds [2 x i8], [2 x i8]* @.str77, i32 0, i32 0
  call void @aial_rt_string_register(i64 77, i8* %str_init_77)
  %str_init_78 = getelementptr inbounds [13 x i8], [13 x i8]* @.str78, i32 0, i32 0
  call void @aial_rt_string_register(i64 78, i8* %str_init_78)
  %str_init_79 = getelementptr inbounds [2 x i8], [2 x i8]* @.str79, i32 0, i32 0
  call void @aial_rt_string_register(i64 79, i8* %str_init_79)
  %str_init_80 = getelementptr inbounds [2 x i8], [2 x i8]* @.str80, i32 0, i32 0
  call void @aial_rt_string_register(i64 80, i8* %str_init_80)
  %str_init_81 = getelementptr inbounds [8 x i8], [8 x i8]* @.str81, i32 0, i32 0
  call void @aial_rt_string_register(i64 81, i8* %str_init_81)
  %str_init_82 = getelementptr inbounds [48 x i8], [48 x i8]* @.str82, i32 0, i32 0
  call void @aial_rt_string_register(i64 82, i8* %str_init_82)
  %str_init_83 = getelementptr inbounds [9 x i8], [9 x i8]* @.str83, i32 0, i32 0
  call void @aial_rt_string_register(i64 83, i8* %str_init_83)
  %str_init_84 = getelementptr inbounds [22 x i8], [22 x i8]* @.str84, i32 0, i32 0
  call void @aial_rt_string_register(i64 84, i8* %str_init_84)
  %str_init_85 = getelementptr inbounds [23 x i8], [23 x i8]* @.str85, i32 0, i32 0
  call void @aial_rt_string_register(i64 85, i8* %str_init_85)
  %str_init_86 = getelementptr inbounds [52 x i8], [52 x i8]* @.str86, i32 0, i32 0
  call void @aial_rt_string_register(i64 86, i8* %str_init_86)
  %str_init_87 = getelementptr inbounds [1 x i8], [1 x i8]* @.str87, i32 0, i32 0
  call void @aial_rt_string_register(i64 87, i8* %str_init_87)
  %str_init_88 = getelementptr inbounds [8 x i8], [8 x i8]* @.str88, i32 0, i32 0
  call void @aial_rt_string_register(i64 88, i8* %str_init_88)
  %str_init_89 = getelementptr inbounds [1 x i8], [1 x i8]* @.str89, i32 0, i32 0
  call void @aial_rt_string_register(i64 89, i8* %str_init_89)
  %str_init_90 = getelementptr inbounds [48 x i8], [48 x i8]* @.str90, i32 0, i32 0
  call void @aial_rt_string_register(i64 90, i8* %str_init_90)
  %str_init_91 = getelementptr inbounds [15 x i8], [15 x i8]* @.str91, i32 0, i32 0
  call void @aial_rt_string_register(i64 91, i8* %str_init_91)
  %str_init_92 = getelementptr inbounds [9 x i8], [9 x i8]* @.str92, i32 0, i32 0
  call void @aial_rt_string_register(i64 92, i8* %str_init_92)
  %str_init_93 = getelementptr inbounds [9 x i8], [9 x i8]* @.str93, i32 0, i32 0
  call void @aial_rt_string_register(i64 93, i8* %str_init_93)
  %str_init_94 = getelementptr inbounds [1 x i8], [1 x i8]* @.str94, i32 0, i32 0
  call void @aial_rt_string_register(i64 94, i8* %str_init_94)
  %str_init_95 = getelementptr inbounds [5 x i8], [5 x i8]* @.str95, i32 0, i32 0
  call void @aial_rt_string_register(i64 95, i8* %str_init_95)
  %str_init_96 = getelementptr inbounds [5 x i8], [5 x i8]* @.str96, i32 0, i32 0
  call void @aial_rt_string_register(i64 96, i8* %str_init_96)
  %str_init_97 = getelementptr inbounds [2 x i8], [2 x i8]* @.str97, i32 0, i32 0
  call void @aial_rt_string_register(i64 97, i8* %str_init_97)
  %str_init_98 = getelementptr inbounds [4 x i8], [4 x i8]* @.str98, i32 0, i32 0
  call void @aial_rt_string_register(i64 98, i8* %str_init_98)
  %str_init_99 = getelementptr inbounds [1 x i8], [1 x i8]* @.str99, i32 0, i32 0
  call void @aial_rt_string_register(i64 99, i8* %str_init_99)
  %str_init_100 = getelementptr inbounds [10 x i8], [10 x i8]* @.str100, i32 0, i32 0
  call void @aial_rt_string_register(i64 100, i8* %str_init_100)
  %str_init_101 = getelementptr inbounds [9 x i8], [9 x i8]* @.str101, i32 0, i32 0
  call void @aial_rt_string_register(i64 101, i8* %str_init_101)
  %str_init_102 = getelementptr inbounds [41 x i8], [41 x i8]* @.str102, i32 0, i32 0
  call void @aial_rt_string_register(i64 102, i8* %str_init_102)
  %str_init_103 = getelementptr inbounds [20 x i8], [20 x i8]* @.str103, i32 0, i32 0
  call void @aial_rt_string_register(i64 103, i8* %str_init_103)
  %str_init_104 = getelementptr inbounds [33 x i8], [33 x i8]* @.str104, i32 0, i32 0
  call void @aial_rt_string_register(i64 104, i8* %str_init_104)
  %str_init_105 = getelementptr inbounds [1 x i8], [1 x i8]* @.str105, i32 0, i32 0
  call void @aial_rt_string_register(i64 105, i8* %str_init_105)
  %str_init_106 = getelementptr inbounds [1 x i8], [1 x i8]* @.str106, i32 0, i32 0
  call void @aial_rt_string_register(i64 106, i8* %str_init_106)
  %str_init_107 = getelementptr inbounds [6 x i8], [6 x i8]* @.str107, i32 0, i32 0
  call void @aial_rt_string_register(i64 107, i8* %str_init_107)
  %str_init_108 = getelementptr inbounds [7 x i8], [7 x i8]* @.str108, i32 0, i32 0
  call void @aial_rt_string_register(i64 108, i8* %str_init_108)
  %str_init_109 = getelementptr inbounds [8 x i8], [8 x i8]* @.str109, i32 0, i32 0
  call void @aial_rt_string_register(i64 109, i8* %str_init_109)
  %str_init_110 = getelementptr inbounds [1 x i8], [1 x i8]* @.str110, i32 0, i32 0
  call void @aial_rt_string_register(i64 110, i8* %str_init_110)
  %str_init_111 = getelementptr inbounds [1 x i8], [1 x i8]* @.str111, i32 0, i32 0
  call void @aial_rt_string_register(i64 111, i8* %str_init_111)
  %str_init_112 = getelementptr inbounds [5 x i8], [5 x i8]* @.str112, i32 0, i32 0
  call void @aial_rt_string_register(i64 112, i8* %str_init_112)
  %str_init_113 = getelementptr inbounds [5 x i8], [5 x i8]* @.str113, i32 0, i32 0
  call void @aial_rt_string_register(i64 113, i8* %str_init_113)
  %str_init_114 = getelementptr inbounds [1 x i8], [1 x i8]* @.str114, i32 0, i32 0
  call void @aial_rt_string_register(i64 114, i8* %str_init_114)
  %str_init_115 = getelementptr inbounds [7 x i8], [7 x i8]* @.str115, i32 0, i32 0
  call void @aial_rt_string_register(i64 115, i8* %str_init_115)
  %str_init_116 = getelementptr inbounds [7 x i8], [7 x i8]* @.str116, i32 0, i32 0
  call void @aial_rt_string_register(i64 116, i8* %str_init_116)
  %str_init_117 = getelementptr inbounds [18 x i8], [18 x i8]* @.str117, i32 0, i32 0
  call void @aial_rt_string_register(i64 117, i8* %str_init_117)
  %str_init_118 = getelementptr inbounds [11 x i8], [11 x i8]* @.str118, i32 0, i32 0
  call void @aial_rt_string_register(i64 118, i8* %str_init_118)
  %str_init_119 = getelementptr inbounds [11 x i8], [11 x i8]* @.str119, i32 0, i32 0
  call void @aial_rt_string_register(i64 119, i8* %str_init_119)
  %str_init_120 = getelementptr inbounds [1 x i8], [1 x i8]* @.str120, i32 0, i32 0
  call void @aial_rt_string_register(i64 120, i8* %str_init_120)
  %str_init_121 = getelementptr inbounds [7 x i8], [7 x i8]* @.str121, i32 0, i32 0
  call void @aial_rt_string_register(i64 121, i8* %str_init_121)
  %str_init_122 = getelementptr inbounds [5 x i8], [5 x i8]* @.str122, i32 0, i32 0
  call void @aial_rt_string_register(i64 122, i8* %str_init_122)
  %str_init_123 = getelementptr inbounds [10 x i8], [10 x i8]* @.str123, i32 0, i32 0
  call void @aial_rt_string_register(i64 123, i8* %str_init_123)
  %str_init_124 = getelementptr inbounds [1 x i8], [1 x i8]* @.str124, i32 0, i32 0
  call void @aial_rt_string_register(i64 124, i8* %str_init_124)
  %str_init_125 = getelementptr inbounds [5 x i8], [5 x i8]* @.str125, i32 0, i32 0
  call void @aial_rt_string_register(i64 125, i8* %str_init_125)
  %str_init_126 = getelementptr inbounds [1 x i8], [1 x i8]* @.str126, i32 0, i32 0
  call void @aial_rt_string_register(i64 126, i8* %str_init_126)
  %str_init_127 = getelementptr inbounds [1 x i8], [1 x i8]* @.str127, i32 0, i32 0
  call void @aial_rt_string_register(i64 127, i8* %str_init_127)
  %str_init_128 = getelementptr inbounds [8 x i8], [8 x i8]* @.str128, i32 0, i32 0
  call void @aial_rt_string_register(i64 128, i8* %str_init_128)
  %str_init_129 = getelementptr inbounds [8 x i8], [8 x i8]* @.str129, i32 0, i32 0
  call void @aial_rt_string_register(i64 129, i8* %str_init_129)
  %str_init_130 = getelementptr inbounds [24 x i8], [24 x i8]* @.str130, i32 0, i32 0
  call void @aial_rt_string_register(i64 130, i8* %str_init_130)
  %str_init_131 = getelementptr inbounds [5 x i8], [5 x i8]* @.str131, i32 0, i32 0
  call void @aial_rt_string_register(i64 131, i8* %str_init_131)
  %v0 = call i64 @chat_context_new()
  %aptr405 = alloca i64
  %v1 = ptrtoint i64* %aptr405 to i64
  %sptr406 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr406
  %v2 = add i64 0, 0
  %v3 = call i64 @mem_open()
  %aptr408 = alloca i64
  %v4 = ptrtoint i64* %aptr408 to i64
  %sptr409 = inttoptr i64 %v4 to i64*
  store i64 %v3, i64* %sptr409
  %v5 = add i64 0, 0
  %v6 = call i64 @aial_rt_array_new()
  %aptr411 = alloca i64
  %v7 = ptrtoint i64* %aptr411 to i64
  %sptr412 = inttoptr i64 %v7 to i64*
  store i64 %v6, i64* %sptr412
  %v8 = add i64 0, 0
  %v9 = call i64 @cmd_cache_new()
  %aptr414 = alloca i64
  %v10 = ptrtoint i64* %aptr414 to i64
  %sptr415 = inttoptr i64 %v10 to i64*
  store i64 %v9, i64* %sptr415
  %v11 = add i64 0, 0
  %lptr416 = inttoptr i64 %v10 to i64*
  %v12 = load i64, i64* %lptr416
  %v13 = call i64 @cmd_cache_populate(i64 %v12)
  %v14 = add i64 0, 0
  %aptr419 = alloca i64
  %v15 = ptrtoint i64* %aptr419 to i64
  %sptr420 = inttoptr i64 %v15 to i64*
  store i64 %v14, i64* %sptr420
  %v16 = add i64 0, 0
  %v17 = add i64 0, 1
  call void @aial_rt_io_raw_mode(i64 %v17)
  %v18 = add i64 0, 0
  %v19 = add i64 0, 81
  call void @aial_rt_print(i64 %v19)
  %v20 = add i64 0, 0
  %v21 = call i64 @header_draw()
  %v22 = call i64 @chat_show_welcome()
  %v23 = add i64 0, 82
  %v24 = call i64 @chat_show_system(i64 %v23)
  %v25 = add i64 0, 83
  %v26 = call i64 @aial_rt_key_exists(i64 %v25)
  %v27 = add i64 0, 1
  %v28 = icmp eq i64 %v26, %v27
  br i1 %v28, label %b106, label %b107

b106:
  %v29 = add i64 0, 84
  %v30 = call i64 @chat_show_hint(i64 %v29)
  br label %b108

b107:
  %v31 = add i64 0, 85
  %v32 = call i64 @chat_show_error(i64 %v31)
  %v33 = add i64 0, 86
  %v34 = call i64 @chat_show_hint(i64 %v33)
  br label %b108

b108:
  %v35 = call i64 @input_draw_border()
  %v36 = add i64 0, 18
  %aptr441 = alloca i64
  %v37 = ptrtoint i64* %aptr441 to i64
  %sptr442 = inttoptr i64 %v37 to i64*
  store i64 %v36, i64* %sptr442
  %v38 = add i64 0, 0
  %v39 = add i64 0, 0
  %aptr444 = alloca i64
  %v40 = ptrtoint i64* %aptr444 to i64
  %sptr445 = inttoptr i64 %v40 to i64*
  store i64 %v39, i64* %sptr445
  %v41 = add i64 0, 0
  %v42 = add i64 0, 0
  %aptr447 = alloca i64
  %v43 = ptrtoint i64* %aptr447 to i64
  %sptr448 = inttoptr i64 %v43 to i64*
  store i64 %v42, i64* %sptr448
  %v44 = add i64 0, 0
  %v45 = add i64 0, 0
  %aptr450 = alloca i64
  %v46 = ptrtoint i64* %aptr450 to i64
  %sptr451 = inttoptr i64 %v46 to i64*
  store i64 %v45, i64* %sptr451
  %v47 = add i64 0, 0
  br label %b109

b109:
  %v48 = call i64 @aial_rt_io_readkey()
  %aptr453 = alloca i64
  %v49 = ptrtoint i64* %aptr453 to i64
  %sptr454 = inttoptr i64 %v49 to i64*
  store i64 %v48, i64* %sptr454
  %v50 = add i64 0, 0
  %lptr455 = inttoptr i64 %v49 to i64*
  %v51 = load i64, i64* %lptr455
  %v52 = call i64 @aial_rt_strlen(i64 %v51)
  %aptr457 = alloca i64
  %v53 = ptrtoint i64* %aptr457 to i64
  %sptr458 = inttoptr i64 %v53 to i64*
  store i64 %v52, i64* %sptr458
  %v54 = add i64 0, 0
  %lptr459 = inttoptr i64 %v53 to i64*
  %v55 = load i64, i64* %lptr459
  %v56 = add i64 0, 0
  %v57 = icmp eq i64 %v55, %v56
  br i1 %v57, label %b111, label %b112

b111:
  %lptr462 = inttoptr i64 %v40 to i64*
  %v58 = load i64, i64* %lptr462
  %v59 = add i64 0, 1
  %v60 = add i64 %v58, %v59
  %sptr465 = inttoptr i64 %v40 to i64*
  store i64 %v60, i64* %sptr465
  %v61 = add i64 0, 0
  %lptr466 = inttoptr i64 %v40 to i64*
  %v62 = load i64, i64* %lptr466
  %v63 = add i64 0, 20
  %v64 = icmp sgt i64 %v62, %v63
  br i1 %v64, label %b114, label %b115

b116:
  %v65 = add i64 0, 50
  call void @aial_rt_time_sleep(i64 %v65)
  %v66 = add i64 0, 0
  br label %b109

b113:
  %v67 = add i64 0, 0
  %sptr472 = inttoptr i64 %v40 to i64*
  store i64 %v67, i64* %sptr472
  %v68 = add i64 0, 0
  %lptr473 = inttoptr i64 %v49 to i64*
  %v69 = load i64, i64* %lptr473
  %v70 = add i64 0, 0
  %v71 = call i64 @aial_rt_strchr(i64 %v69, i64 %v70)
  %aptr476 = alloca i64
  %v72 = ptrtoint i64* %aptr476 to i64
  %sptr477 = inttoptr i64 %v72 to i64*
  store i64 %v71, i64* %sptr477
  %v73 = add i64 0, 0
  %lptr478 = inttoptr i64 %v72 to i64*
  %v74 = load i64, i64* %lptr478
  %v75 = add i64 0, 17
  %v76 = icmp eq i64 %v74, %v75
  br i1 %v76, label %b119, label %b120

b121:
  %lptr481 = inttoptr i64 %v72 to i64*
  %v77 = load i64, i64* %lptr481
  %v78 = add i64 0, 12
  %v79 = icmp eq i64 %v77, %v78
  br i1 %v79, label %b123, label %b124

b123:
  %v80 = add i64 0, 81
  call void @aial_rt_print(i64 %v80)
  %v81 = add i64 0, 0
  %v82 = call i64 @header_draw()
  %v83 = call i64 @input_draw_border()
  %v84 = add i64 0, 18
  %sptr489 = inttoptr i64 %v37 to i64*
  store i64 %v84, i64* %sptr489
  %v85 = add i64 0, 0
  br label %b109

b125:
  %lptr490 = inttoptr i64 %v72 to i64*
  %v86 = load i64, i64* %lptr490
  %v87 = add i64 0, 4
  %v88 = icmp eq i64 %v86, %v87
  br i1 %v88, label %b127, label %b128

b127:
  %v89 = add i64 0, 1
  %lptr494 = inttoptr i64 %v43 to i64*
  %v90 = load i64, i64* %lptr494
  %v91 = sub i64 %v89, %v90
  %sptr496 = inttoptr i64 %v43 to i64*
  store i64 %v91, i64* %sptr496
  %v92 = add i64 0, 0
  %lptr497 = inttoptr i64 %v43 to i64*
  %v93 = load i64, i64* %lptr497
  %v94 = add i64 0, 1
  %v95 = icmp eq i64 %v93, %v94
  br i1 %v95, label %b130, label %b131

b130:
  %v96 = add i64 0, 90
  %v97 = call i64 @chat_show_system(i64 %v96)
  br label %b132

b132:
  %lptr502 = inttoptr i64 %v43 to i64*
  %v98 = load i64, i64* %lptr502
  %v99 = add i64 0, 0
  %v100 = icmp eq i64 %v98, %v99
  br i1 %v100, label %b133, label %b134

b133:
  %v101 = add i64 0, 91
  %v102 = call i64 @chat_show_system(i64 %v101)
  br label %b135

b129:
  %lptr507 = inttoptr i64 %v72 to i64*
  %v103 = load i64, i64* %lptr507
  %v104 = add i64 0, 27
  %v105 = icmp eq i64 %v103, %v104
  %lptr510 = inttoptr i64 %v49 to i64*
  %v106 = load i64, i64* %lptr510
  %v107 = call i64 @aial_rt_strlen(i64 %v106)
  %v108 = add i64 0, 3
  %v109 = icmp sge i64 %v107, %v108
  %v110 = and i1 %v105, %v109
  br i1 %v110, label %b137, label %b138

b137:
  %lptr515 = inttoptr i64 %v49 to i64*
  %v111 = load i64, i64* %lptr515
  %v112 = add i64 0, 2
  %v113 = call i64 @aial_rt_strchr(i64 %v111, i64 %v112)
  %aptr518 = alloca i64
  %v114 = ptrtoint i64* %aptr518 to i64
  %sptr519 = inttoptr i64 %v114 to i64*
  store i64 %v113, i64* %sptr519
  %v115 = add i64 0, 0
  %lptr520 = inttoptr i64 %v114 to i64*
  %v116 = load i64, i64* %lptr520
  %v117 = add i64 0, 65
  %v118 = icmp eq i64 %v116, %v117
  br i1 %v118, label %b140, label %b141

b140:
  %lptr523 = inttoptr i64 %v7 to i64*
  %v119 = load i64, i64* %lptr523
  %v120 = call i64 @history_size(i64 %v119)
  %aptr525 = alloca i64
  %v121 = ptrtoint i64* %aptr525 to i64
  %sptr526 = inttoptr i64 %v121 to i64*
  store i64 %v120, i64* %sptr526
  %v122 = add i64 0, 0
  %lptr527 = inttoptr i64 %v121 to i64*
  %v123 = load i64, i64* %lptr527
  %v124 = add i64 0, 0
  %v125 = icmp sgt i64 %v123, %v124
  %lptr530 = inttoptr i64 %v15 to i64*
  %v126 = load i64, i64* %lptr530
  %lptr531 = inttoptr i64 %v121 to i64*
  %v127 = load i64, i64* %lptr531
  %v128 = icmp slt i64 %v126, %v127
  %v129 = and i1 %v125, %v128
  br i1 %v129, label %b143, label %b144

b143:
  %lptr534 = inttoptr i64 %v15 to i64*
  %v130 = load i64, i64* %lptr534
  %v131 = add i64 0, 1
  %v132 = add i64 %v130, %v131
  %sptr537 = inttoptr i64 %v15 to i64*
  store i64 %v132, i64* %sptr537
  %v133 = add i64 0, 0
  %lptr538 = inttoptr i64 %v7 to i64*
  %v134 = load i64, i64* %lptr538
  %lptr539 = inttoptr i64 %v121 to i64*
  %v135 = load i64, i64* %lptr539
  %lptr540 = inttoptr i64 %v15 to i64*
  %v136 = load i64, i64* %lptr540
  %v137 = sub i64 %v135, %v136
  %v138 = call i64 @history_recall(i64 %v134, i64 %v137)
  %aptr543 = alloca i64
  %v139 = ptrtoint i64* %aptr543 to i64
  %sptr544 = inttoptr i64 %v139 to i64*
  store i64 %v138, i64* %sptr544
  %v140 = add i64 0, 0
  %v141 = add i64 0, 92
  %v142 = call i64 @color_input()
  %lptr547 = inttoptr i64 %v139 to i64*
  %v143 = load i64, i64* %lptr547
  %v144 = call i64 @aial_rt_strcat(i64 %v142, i64 %v143)
  %v145 = call i64 @aial_rt_strcat(i64 %v141, i64 %v144)
  call void @aial_rt_print(i64 %v145)
  %v146 = add i64 0, 0
  %lptr551 = inttoptr i64 %v139 to i64*
  %v147 = load i64, i64* %lptr551
  %sptr552 = inttoptr i64 %v37 to i64*
  store i64 %v147, i64* %sptr552
  %v148 = add i64 0, 0
  br label %b145

b142:
  %lptr553 = inttoptr i64 %v114 to i64*
  %v149 = load i64, i64* %lptr553
  %v150 = add i64 0, 66
  %v151 = icmp eq i64 %v149, %v150
  br i1 %v151, label %b146, label %b147

b146:
  %lptr556 = inttoptr i64 %v15 to i64*
  %v152 = load i64, i64* %lptr556
  %v153 = add i64 0, 1
  %v154 = icmp sgt i64 %v152, %v153
  br i1 %v154, label %b149, label %b150

b149:
  %lptr559 = inttoptr i64 %v15 to i64*
  %v155 = load i64, i64* %lptr559
  %v156 = add i64 0, 1
  %v157 = sub i64 %v155, %v156
  %sptr562 = inttoptr i64 %v15 to i64*
  store i64 %v157, i64* %sptr562
  %v158 = add i64 0, 0
  %lptr563 = inttoptr i64 %v7 to i64*
  %v159 = load i64, i64* %lptr563
  %v160 = call i64 @history_size(i64 %v159)
  %aptr565 = alloca i64
  %v161 = ptrtoint i64* %aptr565 to i64
  %sptr566 = inttoptr i64 %v161 to i64*
  store i64 %v160, i64* %sptr566
  %v162 = add i64 0, 0
  %lptr567 = inttoptr i64 %v7 to i64*
  %v163 = load i64, i64* %lptr567
  %lptr568 = inttoptr i64 %v161 to i64*
  %v164 = load i64, i64* %lptr568
  %lptr569 = inttoptr i64 %v15 to i64*
  %v165 = load i64, i64* %lptr569
  %v166 = sub i64 %v164, %v165
  %v167 = call i64 @history_recall(i64 %v163, i64 %v166)
  %aptr572 = alloca i64
  %v168 = ptrtoint i64* %aptr572 to i64
  %sptr573 = inttoptr i64 %v168 to i64*
  store i64 %v167, i64* %sptr573
  %v169 = add i64 0, 0
  %v170 = add i64 0, 92
  %v171 = call i64 @color_input()
  %lptr576 = inttoptr i64 %v168 to i64*
  %v172 = load i64, i64* %lptr576
  %v173 = call i64 @aial_rt_strcat(i64 %v171, i64 %v172)
  %v174 = call i64 @aial_rt_strcat(i64 %v170, i64 %v173)
  call void @aial_rt_print(i64 %v174)
  %v175 = add i64 0, 0
  %lptr580 = inttoptr i64 %v168 to i64*
  %v176 = load i64, i64* %lptr580
  %sptr581 = inttoptr i64 %v37 to i64*
  store i64 %v176, i64* %sptr581
  %v177 = add i64 0, 0
  br label %b151

b150:
  %lptr582 = inttoptr i64 %v15 to i64*
  %v178 = load i64, i64* %lptr582
  %v179 = add i64 0, 1
  %v180 = icmp eq i64 %v178, %v179
  br i1 %v180, label %b152, label %b153

b152:
  %v181 = add i64 0, 0
  %sptr586 = inttoptr i64 %v15 to i64*
  store i64 %v181, i64* %sptr586
  %v182 = add i64 0, 0
  %v183 = add i64 0, 18
  %sptr588 = inttoptr i64 %v37 to i64*
  store i64 %v183, i64* %sptr588
  %v184 = add i64 0, 0
  %v185 = add i64 0, 95
  %v186 = call i64 @color_input()
  %v187 = add i64 0, 33
  %v188 = call i64 @aial_rt_strcat(i64 %v186, i64 %v187)
  %v189 = call i64 @aial_rt_strcat(i64 %v185, i64 %v188)
  call void @aial_rt_print(i64 %v189)
  %v190 = add i64 0, 0
  br label %b154

b139:
  %lptr595 = inttoptr i64 %v72 to i64*
  %v191 = load i64, i64* %lptr595
  %v192 = add i64 0, 27
  %v193 = icmp eq i64 %v191, %v192
  br i1 %v193, label %b156, label %b157

b158:
  %lptr598 = inttoptr i64 %v72 to i64*
  %v194 = load i64, i64* %lptr598
  %v195 = add i64 0, 13
  %v196 = icmp eq i64 %v194, %v195
  %lptr601 = inttoptr i64 %v72 to i64*
  %v197 = load i64, i64* %lptr601
  %v198 = add i64 0, 10
  %v199 = icmp eq i64 %v197, %v198
  %v200 = or i1 %v196, %v199
  br i1 %v200, label %b160, label %b161

b160:
  %lptr605 = inttoptr i64 %v43 to i64*
  %v201 = load i64, i64* %lptr605
  %v202 = add i64 0, 1
  %v203 = icmp eq i64 %v201, %v202
  br i1 %v203, label %b163, label %b164

b163:
  %lptr608 = inttoptr i64 %v37 to i64*
  %v204 = load i64, i64* %lptr608
  %v205 = add i64 0, 32
  %v206 = call i64 @aial_rt_strcat(i64 %v204, i64 %v205)
  %sptr611 = inttoptr i64 %v37 to i64*
  store i64 %v206, i64* %sptr611
  %v207 = add i64 0, 0
  %v208 = add i64 0, 98
  call void @aial_rt_print(i64 %v208)
  %v209 = add i64 0, 0
  br label %b109

b165:
  %v210 = add i64 0, 18
  call void @aial_rt_println(i64 %v210)
  %v211 = add i64 0, 0
  %lptr616 = inttoptr i64 %v37 to i64*
  %v212 = load i64, i64* %lptr616
  %v213 = call i64 @aial_rt_strlen(i64 %v212)
  %v214 = add i64 0, 0
  %v215 = icmp eq i64 %v213, %v214
  br i1 %v215, label %b167, label %b168

b167:
  %v216 = call i64 @input_draw_border()
  br label %b109

b169:
  %lptr621 = inttoptr i64 %v37 to i64*
  %v217 = load i64, i64* %lptr621
  %v218 = add i64 0, 100
  %x219 = call i64 @aial_rt_starts_with(i64 %v217, i64 %v218)
  %v219 = trunc i64 %x219 to i1
  br i1 %v219, label %b171, label %b172

b171:
  %lptr624 = inttoptr i64 %v37 to i64*
  %v220 = load i64, i64* %lptr624
  %v221 = add i64 0, 9
  %lptr626 = inttoptr i64 %v37 to i64*
  %v222 = load i64, i64* %lptr626
  %v223 = call i64 @aial_rt_strlen(i64 %v222)
  %v224 = add i64 0, 9
  %v225 = sub i64 %v223, %v224
  %v226 = call i64 @aial_rt_strslice(i64 %v220, i64 %v221, i64 %v225)
  %aptr631 = alloca i64
  %v227 = ptrtoint i64* %aptr631 to i64
  %sptr632 = inttoptr i64 %v227 to i64*
  store i64 %v226, i64* %sptr632
  %v228 = add i64 0, 0
  %lptr633 = inttoptr i64 %v227 to i64*
  %v229 = load i64, i64* %lptr633
  %v230 = call i64 @aial_rt_strlen(i64 %v229)
  %v231 = add i64 0, 0
  %v232 = icmp sgt i64 %v230, %v231
  br i1 %v232, label %b174, label %b175

b174:
  %v233 = add i64 0, 83
  %lptr638 = inttoptr i64 %v227 to i64*
  %v234 = load i64, i64* %lptr638
  %v235 = call i64 @aial_rt_key_set(i64 %v233, i64 %v234)
  %aptr640 = alloca i64
  %v236 = ptrtoint i64* %aptr640 to i64
  %sptr641 = inttoptr i64 %v236 to i64*
  store i64 %v235, i64* %sptr641
  %v237 = add i64 0, 0
  %lptr642 = inttoptr i64 %v236 to i64*
  %v238 = load i64, i64* %lptr642
  %v239 = add i64 0, 1
  %v240 = icmp eq i64 %v238, %v239
  br i1 %v240, label %b177, label %b178

b177:
  %v241 = add i64 0, 102
  %v242 = call i64 @chat_show_system(i64 %v241)
  br label %b179

b178:
  %v243 = add i64 0, 103
  %v244 = call i64 @chat_show_error(i64 %v243)
  br label %b179

b175:
  %v245 = add i64 0, 104
  %v246 = call i64 @chat_show_error(i64 %v245)
  br label %b176

b176:
  %v247 = call i64 @input_draw_border()
  %v248 = add i64 0, 18
  %sptr653 = inttoptr i64 %v37 to i64*
  store i64 %v248, i64* %sptr653
  %v249 = add i64 0, 0
  br label %b109

b173:
  %lptr654 = inttoptr i64 %v10 to i64*
  %v250 = load i64, i64* %lptr654
  %lptr655 = inttoptr i64 %v37 to i64*
  %v251 = load i64, i64* %lptr655
  %v252 = call i64 @cmd_cache_get(i64 %v250, i64 %v251)
  %aptr657 = alloca i64
  %v253 = ptrtoint i64* %aptr657 to i64
  %sptr658 = inttoptr i64 %v253 to i64*
  store i64 %v252, i64* %sptr658
  %v254 = add i64 0, 0
  %lptr659 = inttoptr i64 %v253 to i64*
  %v255 = load i64, i64* %lptr659
  %v256 = call i64 @aial_rt_strlen(i64 %v255)
  %v257 = add i64 0, 0
  %v258 = icmp sgt i64 %v256, %v257
  br i1 %v258, label %b181, label %b182

b181:
  %lptr663 = inttoptr i64 %v253 to i64*
  %v259 = load i64, i64* %lptr663
  %v260 = call i64 @chat_show_system(i64 %v259)
  %v261 = call i64 @input_draw_border()
  %v262 = add i64 0, 18
  %sptr667 = inttoptr i64 %v37 to i64*
  store i64 %v262, i64* %sptr667
  %v263 = add i64 0, 0
  br label %b109

b183:
  %lptr668 = inttoptr i64 %v37 to i64*
  %v264 = load i64, i64* %lptr668
  %v265 = add i64 0, 107
  %x266 = call i64 @aial_rt_str_eq(i64 %v264, i64 %v265)
  %v266 = trunc i64 %x266 to i1
  br i1 %v266, label %b185, label %b186

b187:
  %lptr671 = inttoptr i64 %v37 to i64*
  %v267 = load i64, i64* %lptr671
  %v268 = add i64 0, 108
  %x269 = call i64 @aial_rt_str_eq(i64 %v267, i64 %v268)
  %v269 = trunc i64 %x269 to i1
  br i1 %v269, label %b189, label %b190

b189:
  %v270 = add i64 0, 81
  call void @aial_rt_print(i64 %v270)
  %v271 = add i64 0, 0
  %v272 = call i64 @header_draw()
  %v273 = call i64 @input_draw_border()
  %v274 = add i64 0, 18
  %sptr679 = inttoptr i64 %v37 to i64*
  store i64 %v274, i64* %sptr679
  %v275 = add i64 0, 0
  br label %b109

b191:
  %lptr680 = inttoptr i64 %v7 to i64*
  %v276 = load i64, i64* %lptr680
  %lptr681 = inttoptr i64 %v37 to i64*
  %v277 = load i64, i64* %lptr681
  %v278 = call i64 @history_push(i64 %v276, i64 %v277)
  %v279 = add i64 0, 0
  %sptr684 = inttoptr i64 %v15 to i64*
  store i64 %v279, i64* %sptr684
  %v280 = add i64 0, 0
  %lptr685 = inttoptr i64 %v46 to i64*
  %v281 = load i64, i64* %lptr685
  %v282 = add i64 0, 1
  %v283 = add i64 %v281, %v282
  %sptr688 = inttoptr i64 %v46 to i64*
  store i64 %v283, i64* %sptr688
  %v284 = add i64 0, 0
  %v285 = add i64 0, 18
  call void @aial_rt_println(i64 %v285)
  %v286 = add i64 0, 0
  %lptr691 = inttoptr i64 %v37 to i64*
  %v287 = load i64, i64* %lptr691
  %v288 = call i64 @chat_show_user(i64 %v287)
  %lptr693 = inttoptr i64 %v4 to i64*
  %v289 = load i64, i64* %lptr693
  %v290 = add i64 0, 112
  %v291 = add i64 0, 113
  %lptr696 = inttoptr i64 %v37 to i64*
  %v292 = load i64, i64* %lptr696
  %v293 = call i64 @mem_save(i64 %v289, i64 %v290, i64 %v291, i64 %v292)
  %v294 = call i64 @chat_show_ai_prefix()
  %lptr699 = inttoptr i64 %v1 to i64*
  %v295 = load i64, i64* %lptr699
  %lptr700 = inttoptr i64 %v37 to i64*
  %v296 = load i64, i64* %lptr700
  %v297 = call i64 @chat_send(i64 %v295, i64 %v296)
  %aptr702 = alloca i64
  %v298 = ptrtoint i64* %aptr702 to i64
  %sptr703 = inttoptr i64 %v298 to i64*
  store i64 %v297, i64* %sptr703
  %v299 = add i64 0, 0
  %v300 = add i64 0, 18
  %aptr705 = alloca i64
  %v301 = ptrtoint i64* %aptr705 to i64
  %sptr706 = inttoptr i64 %v301 to i64*
  store i64 %v300, i64* %sptr706
  %v302 = add i64 0, 0
  %v303 = add i64 0, 1
  %aptr708 = alloca i64
  %v304 = ptrtoint i64* %aptr708 to i64
  %sptr709 = inttoptr i64 %v304 to i64*
  store i64 %v303, i64* %sptr709
  %v305 = add i64 0, 0
  br label %b193

b193:
  %lptr710 = inttoptr i64 %v298 to i64*
  %v306 = load i64, i64* %lptr710
  %v307 = call i64 @chat_read_token(i64 %v306)
  %aptr712 = alloca i64
  %v308 = ptrtoint i64* %aptr712 to i64
  %sptr713 = inttoptr i64 %v308 to i64*
  store i64 %v307, i64* %sptr713
  %v309 = add i64 0, 0
  %lptr714 = inttoptr i64 %v308 to i64*
  %v310 = load i64, i64* %lptr714
  %v311 = call i64 @aial_rt_strlen(i64 %v310)
  %v312 = add i64 0, 0
  %v313 = icmp eq i64 %v311, %v312
  br i1 %v313, label %b195, label %b196

b197:
  %lptr718 = inttoptr i64 %v304 to i64*
  %v314 = load i64, i64* %lptr718
  %v315 = add i64 0, 1
  %v316 = icmp eq i64 %v314, %v315
  br i1 %v316, label %b199, label %b200

b199:
  %v317 = add i64 0, 0
  %sptr722 = inttoptr i64 %v304 to i64*
  store i64 %v317, i64* %sptr722
  %v318 = add i64 0, 0
  %lptr723 = inttoptr i64 %v308 to i64*
  %v319 = load i64, i64* %lptr723
  %v320 = add i64 0, 115
  %x321 = call i64 @aial_rt_starts_with(i64 %v319, i64 %v320)
  %v321 = trunc i64 %x321 to i1
  %lptr726 = inttoptr i64 %v308 to i64*
  %v322 = load i64, i64* %lptr726
  %v323 = add i64 0, 116
  %x324 = call i64 @aial_rt_starts_with(i64 %v322, i64 %v323)
  %v324 = trunc i64 %x324 to i1
  %v325 = or i1 %v321, %v324
  br i1 %v325, label %b202, label %b203

b202:
  %lptr730 = inttoptr i64 %v308 to i64*
  %v326 = load i64, i64* %lptr730
  %v327 = call i64 @chat_show_error(i64 %v326)
  %v328 = call i64 @aial_rt_ctx_last_error()
  %aptr733 = alloca i64
  %v329 = ptrtoint i64* %aptr733 to i64
  %sptr734 = inttoptr i64 %v329 to i64*
  store i64 %v328, i64* %sptr734
  %v330 = add i64 0, 0
  %lptr735 = inttoptr i64 %v329 to i64*
  %v331 = load i64, i64* %lptr735
  %v332 = call i64 @aial_rt_strlen(i64 %v331)
  %v333 = add i64 0, 0
  %v334 = icmp sgt i64 %v332, %v333
  br i1 %v334, label %b205, label %b206

b205:
  %lptr739 = inttoptr i64 %v329 to i64*
  %v335 = load i64, i64* %lptr739
  %v336 = call i64 @chat_show_error(i64 %v335)
  br label %b207

b207:
  %lptr741 = inttoptr i64 %v308 to i64*
  %v337 = load i64, i64* %lptr741
  %v338 = add i64 0, 117
  %x339 = call i64 @aial_rt_starts_with(i64 %v337, i64 %v338)
  %v339 = trunc i64 %x339 to i1
  %lptr744 = inttoptr i64 %v329 to i64*
  %v340 = load i64, i64* %lptr744
  %v341 = add i64 0, 118
  %x342 = call i64 @aial_rt_starts_with(i64 %v340, i64 %v341)
  %v342 = trunc i64 %x342 to i1
  %v343 = or i1 %v339, %v342
  br i1 %v343, label %b208, label %b209

b208:
  %lptr748 = inttoptr i64 %v10 to i64*
  %v344 = load i64, i64* %lptr748
  %v345 = add i64 0, 25
  %v346 = call i64 @cmd_cache_get(i64 %v344, i64 %v345)
  %aptr751 = alloca i64
  %v347 = ptrtoint i64* %aptr751 to i64
  %sptr752 = inttoptr i64 %v347 to i64*
  store i64 %v346, i64* %sptr752
  %v348 = add i64 0, 0
  %lptr753 = inttoptr i64 %v347 to i64*
  %v349 = load i64, i64* %lptr753
  %v350 = call i64 @chat_show_system(i64 %v349)
  br label %b210

b210:
  %lptr755 = inttoptr i64 %v308 to i64*
  %v351 = load i64, i64* %lptr755
  %sptr756 = inttoptr i64 %v301 to i64*
  store i64 %v351, i64* %sptr756
  %v352 = add i64 0, 0
  br label %b194

b201:
  %lptr757 = inttoptr i64 %v308 to i64*
  %v353 = load i64, i64* %lptr757
  call void @aial_rt_print(i64 %v353)
  %v354 = add i64 0, 0
  %lptr759 = inttoptr i64 %v301 to i64*
  %v355 = load i64, i64* %lptr759
  %lptr760 = inttoptr i64 %v308 to i64*
  %v356 = load i64, i64* %lptr760
  %v357 = call i64 @aial_rt_strcat(i64 %v355, i64 %v356)
  %sptr762 = inttoptr i64 %v301 to i64*
  store i64 %v357, i64* %sptr762
  %v358 = add i64 0, 0
  %v359 = add i64 0, 0
  %v360 = call i64 @aial_rt_io_readkey_timeout(i64 %v359)
  %aptr765 = alloca i64
  %v361 = ptrtoint i64* %aptr765 to i64
  %sptr766 = inttoptr i64 %v361 to i64*
  store i64 %v360, i64* %sptr766
  %v362 = add i64 0, 0
  %lptr767 = inttoptr i64 %v361 to i64*
  %v363 = load i64, i64* %lptr767
  %v364 = call i64 @aial_rt_strlen(i64 %v363)
  %v365 = add i64 0, 0
  %v366 = icmp sgt i64 %v364, %v365
  br i1 %v366, label %b212, label %b213

b212:
  %lptr771 = inttoptr i64 %v361 to i64*
  %v367 = load i64, i64* %lptr771
  %v368 = add i64 0, 0
  %v369 = call i64 @aial_rt_strchr(i64 %v367, i64 %v368)
  %v370 = add i64 0, 17
  %v371 = icmp eq i64 %v369, %v370
  br i1 %v371, label %b215, label %b216

b194:
  %v372 = add i64 0, 18
  call void @aial_rt_println(i64 %v372)
  %v373 = add i64 0, 0
  %lptr778 = inttoptr i64 %v301 to i64*
  %v374 = load i64, i64* %lptr778
  %v375 = add i64 0, 115
  %x376 = call i64 @aial_rt_starts_with(i64 %v374, i64 %v375)
  %v376 = trunc i64 %x376 to i1
  %v377 = xor i1 %v376, true
  br i1 %v377, label %b219, label %b220

b219:
  %lptr782 = inttoptr i64 %v4 to i64*
  %v378 = load i64, i64* %lptr782
  %v379 = add i64 0, 112
  %v380 = add i64 0, 123
  %lptr785 = inttoptr i64 %v301 to i64*
  %v381 = load i64, i64* %lptr785
  %v382 = call i64 @mem_save(i64 %v378, i64 %v379, i64 %v380, i64 %v381)
  br label %b221

b221:
  %lptr787 = inttoptr i64 %v46 to i64*
  %v383 = load i64, i64* %lptr787
  %v384 = add i64 0, 1
  %v385 = add i64 %v383, %v384
  %sptr790 = inttoptr i64 %v46 to i64*
  store i64 %v385, i64* %sptr790
  %v386 = add i64 0, 0
  %v387 = add i64 0, 18
  call void @aial_rt_println(i64 %v387)
  %v388 = add i64 0, 0
  %v389 = add i64 0, 112
  %lptr794 = inttoptr i64 %v46 to i64*
  %v390 = load i64, i64* %lptr794
  %lptr795 = inttoptr i64 %v301 to i64*
  %v391 = load i64, i64* %lptr795
  %v392 = call i64 @aial_rt_token_estimate(i64 %v391)
  %v393 = call i64 @bar_draw(i64 %v389, i64 %v390, i64 %v392)
  %v394 = add i64 0, 18
  call void @aial_rt_println(i64 %v394)
  %v395 = add i64 0, 0
  %v396 = add i64 0, 18
  %sptr801 = inttoptr i64 %v37 to i64*
  store i64 %v396, i64* %sptr801
  %v397 = add i64 0, 0
  %v398 = call i64 @input_draw_border()
  br label %b109

b162:
  %lptr803 = inttoptr i64 %v72 to i64*
  %v399 = load i64, i64* %lptr803
  %v400 = add i64 0, 127
  %v401 = icmp eq i64 %v399, %v400
  %lptr806 = inttoptr i64 %v72 to i64*
  %v402 = load i64, i64* %lptr806
  %v403 = add i64 0, 8
  %v404 = icmp eq i64 %v402, %v403
  %v405 = or i1 %v401, %v404
  br i1 %v405, label %b223, label %b224

b223:
  %lptr810 = inttoptr i64 %v37 to i64*
  %v406 = load i64, i64* %lptr810
  %v407 = call i64 @aial_rt_strlen(i64 %v406)
  %v408 = add i64 0, 0
  %v409 = icmp sgt i64 %v407, %v408
  br i1 %v409, label %b226, label %b227

b226:
  %lptr814 = inttoptr i64 %v37 to i64*
  %v410 = load i64, i64* %lptr814
  %v411 = add i64 0, 0
  %lptr816 = inttoptr i64 %v37 to i64*
  %v412 = load i64, i64* %lptr816
  %v413 = call i64 @aial_rt_strlen(i64 %v412)
  %v414 = add i64 0, 1
  %v415 = sub i64 %v413, %v414
  %v416 = call i64 @aial_rt_strslice(i64 %v410, i64 %v411, i64 %v415)
  %sptr821 = inttoptr i64 %v37 to i64*
  store i64 %v416, i64* %sptr821
  %v417 = add i64 0, 0
  %v418 = add i64 0, 128
  call void @aial_rt_print(i64 %v418)
  %v419 = add i64 0, 0
  br label %b228

b225:
  %lptr824 = inttoptr i64 %v72 to i64*
  %v420 = load i64, i64* %lptr824
  %v421 = add i64 0, 32
  %v422 = icmp sge i64 %v420, %v421
  br i1 %v422, label %b230, label %b231

b230:
  %lptr827 = inttoptr i64 %v49 to i64*
  %v423 = load i64, i64* %lptr827
  call void @aial_rt_print(i64 %v423)
  %v424 = add i64 0, 0
  %lptr829 = inttoptr i64 %v37 to i64*
  %v425 = load i64, i64* %lptr829
  %lptr830 = inttoptr i64 %v49 to i64*
  %v426 = load i64, i64* %lptr830
  %v427 = call i64 @aial_rt_strcat(i64 %v425, i64 %v426)
  %sptr832 = inttoptr i64 %v37 to i64*
  store i64 %v427, i64* %sptr832
  %v428 = add i64 0, 0
  br label %b232

b110:
  %v429 = add i64 0, 0
  call void @aial_rt_io_raw_mode(i64 %v429)
  %v430 = add i64 0, 0
  %v431 = add i64 0, 81
  call void @aial_rt_print(i64 %v431)
  %v432 = add i64 0, 0
  %v433 = call i64 @header_draw()
  %v434 = add i64 0, 130
  %v435 = call i64 @chat_show_system(i64 %v434)
  %v436 = call i64 @input_draw_bottom()
  %v437 = add i64 0, 112
  %lptr842 = inttoptr i64 %v46 to i64*
  %v438 = load i64, i64* %lptr842
  %v439 = add i64 0, 0
  %v440 = call i64 @bar_draw(i64 %v437, i64 %v438, i64 %v439)
  %lptr845 = inttoptr i64 %v4 to i64*
  %v441 = load i64, i64* %lptr845
  %v442 = call i64 @mem_close(i64 %v441)
  ret i32 0

b112:
  br label %b113

b114:
  br label %b110

b115:
  br label %b116

b117:
  br label %b116

b118:
  br label %b113

b119:
  br label %b110

b120:
  br label %b121

b122:
  br label %b121

b124:
  br label %b125

b126:
  br label %b125

b128:
  br label %b129

b131:
  br label %b132

b134:
  br label %b135

b135:
  br label %b109

b136:
  br label %b129

b138:
  br label %b139

b141:
  br label %b142

b144:
  br label %b145

b145:
  br label %b142

b147:
  br label %b148

b148:
  br label %b109

b151:
  br label %b148

b153:
  br label %b154

b154:
  br label %b151

b155:
  br label %b139

b156:
  br label %b109

b157:
  br label %b158

b159:
  br label %b158

b161:
  br label %b162

b164:
  br label %b165

b166:
  br label %b165

b168:
  br label %b169

b170:
  br label %b169

b172:
  br label %b173

b179:
  br label %b176

b180:
  br label %b173

b182:
  br label %b183

b184:
  br label %b183

b185:
  br label %b110

b186:
  br label %b187

b188:
  br label %b187

b190:
  br label %b191

b192:
  br label %b191

b195:
  br label %b194

b196:
  br label %b197

b198:
  br label %b197

b200:
  br label %b201

b203:
  br label %b204

b204:
  br label %b201

b206:
  br label %b207

b209:
  br label %b210

b211:
  br label %b204

b213:
  br label %b214

b214:
  br label %b193

b215:
  br label %b194

b216:
  br label %b217

b217:
  br label %b214

b218:
  br label %b217

b220:
  br label %b221

b222:
  br label %b162

b224:
  br label %b225

b227:
  br label %b228

b228:
  br label %b109

b229:
  br label %b225

b231:
  br label %b232

b232:
  br label %b109
}

