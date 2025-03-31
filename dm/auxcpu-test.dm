//#define AUXTOOLS
#ifdef AUXTOOLS
#include "auxtools.dm"
#else
#include "byondapi.dm"
#endif

/world/New()
	if(!setup())
		del(src)
		return
	//reset_cpu_table()
	//world.log << "cpu table reset"
	spawn start_wasting_time()
	return ..()

/world/Del()
	cleanup()
	return ..()

var/static/_i = 1
/proc/start_wasting_time()
	while(TRUE)
		for(var/i = 1 to 10)
			_i += rand()
		sleep(5)

var/static/ticks = 1
/world/Tick()
	world.log << "=== Tick [ticks++] ==="
	world.log << "world.cpu = [world.cpu]"
	world.log << "current_true_cpu() = [current_true_cpu()]"
	world.log << "current_cpu_index() = [current_cpu_index()]"
	world.log << "cpu_values() = [json_encode(cpu_values())]"
	world.log << ""

	if(ticks > 20)
		world.log << "Rebooting now\n"
		world.Reboot()
		Reboot()
		return