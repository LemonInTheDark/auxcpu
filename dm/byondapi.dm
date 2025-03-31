#define AUXCPU_DLL (world.GetConfig("env", "AUXCPU_DLL") || (world.system_type == MS_WINDOWS ? "../target/i686-pc-windows-msvc/release/auxcpu_byondapi.dll" : "../target/i686-pc-windows-msvc/release/libauxcpu_byondapi.so"))

#if DM_VERSION >= 516
var/static/__current_true_cpu
#define current_true_cpu(...) call_ext(__current_true_cpu ||= load_ext(AUXCPU_DLL, "byond:current_true_cpu"))()

var/static/__current_cpu_index
#define current_cpu_index(...) call_ext(__current_cpu_index ||= load_ext(AUXCPU_DLL, "byond:current_cpu_index"))()

var/global/__true_cpu_at_index
#define true_cpu_at_index(index) call_ext(__true_cpu_at_index ||= load_ext(AUXCPU_DLL, "byond:true_cpu_at_index"))(index)

var/global/__cpu_values
#define cpu_values(...) call_ext(__cpu_values ||= load_ext(AUXCPU_DLL, "byond:cpu_values"))()

// don't use this for now
/*
var/global/__reset_cpu_table
#define reset_cpu_table(...) call_ext(__reset_cpu_table ||= load_ext(AUXCPU_DLL, "byond:reset_cpu_table"))()
*/

#else

#define current_true_cpu(...) call_ext(AUXCPU_DLL, "byond:current_true_cpu")()
#define current_cpu_index(...) call_ext(AUXCPU_DLL, "byond:current_cpu_index")()
#define true_cpu_at_index(index) call_ext(AUXCPU_DLL, "byond:true_cpu_at_index")(index)
#define cpu_values(...) call_ext(AUXCPU_DLL, "byond:cpu_values")()

// don't use this for now
/* #define reset_cpu_table(...) call_ext(AUXCPU_DLL, "byond:reset_cpu_table")() */
#endif

/proc/setup()
	if(!call_ext(AUXCPU_DLL, "byond:find_signatures")())
		world.log << "auxcpu failed to find signatures"
		return FALSE
	world.log << "signatures found"
	return TRUE

/proc/cleanup()
	return

/proc/meowtonin_stack_trace(message, source, line, full_info)
    var/list/info = list("[message || "N/A"]")
    if(istext(source))
        info += "\tsource: [source]"
        if(line)
            info += "\tline: [line]"
    if(full_info)
        world.log << "\n=== (panic start) ===\n[full_info]\n=== (panic end) ===\n"
    CRASH(jointext(info, "\n"))
