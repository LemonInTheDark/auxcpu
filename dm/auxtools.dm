#define AUXCPU_DLL (world.GetConfig("env", "AUXCPU_DLL") || (world.system_type == MS_WINDOWS ? "../target/i686-pc-windows-msvc/release/auxcpu_auxtools.dll" : "../target/i686-pc-windows-msvc/release/libauxcpu_auxtools.so"))

/proc/current_true_cpu()
	CRASH()

/proc/current_cpu_index()
	CRASH()

/proc/true_cpu_at_index(index)
	CRASH()

/proc/cpu_values()
	CRASH()

/* don't use this for now
/proc/reset_cpu_table()
	CRASH()
*/

var/static/did_auxtools_init = FALSE

/proc/setup()
	var/init_result = call_ext(AUXCPU_DLL, "auxtools_init")()
	if(init_result != "SUCCESS")
		world.log << "auxtools failed to init: [init_result]"
		return FALSE
	world.log << "auxcpu initialized"
	return TRUE

/proc/cleanup()
	if(did_auxtools_init)
		call_ext(AUXCPU_DLL, "auxtools_shutdown")()
		did_auxtools_init = FALSE
