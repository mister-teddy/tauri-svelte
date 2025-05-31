<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  interface NetworkHealth {
    status: string;
    latency: number;
    timestamp: string;
  }

  interface GeoLocation {
    ip: string;
    city: string;
    region: string;
    country: string;
    org: string;
    timezone: string;
  }

  interface SystemStatus {
    processes: number;
    uptime: string;
    load: string;
    timestamp: string;
  }

  // Network Health Monitor
  let networkStatus = $state({
    status: "disconnected",
    latency: 0,
    timestamp: "",
    history: [] as { status: string; latency: number; timestamp: string }[],
    error: "",
    isRunning: true,
  });
  let networkInterval: number;

  // IP Geolocation
  let geoLocation = $state({
    ip: "",
    city: "",
    region: "",
    country: "",
    org: "",
    timezone: "",
    timestamp: "",
    error: "",
    isRunning: true,
  });
  let geoInterval: number;

  // System Status
  let systemStatus = $state({
    processes: 0,
    uptime: "",
    load: "",
    timestamp: "",
    history: [] as { processes: number; timestamp: string }[],
    error: "",
    isRunning: true,
  });
  let systemInterval: number;

  // Theme management
  let isDark = $state(false);

  function toggleTheme() {
    isDark = !isDark;
    if (isDark) {
      document.documentElement.classList.add("dark");
      localStorage.setItem("theme", "dark");
    } else {
      document.documentElement.classList.remove("dark");
      localStorage.setItem("theme", "light");
    }
  }

  // Initialize theme
  onMount(() => {
    // Check localStorage first
    const savedTheme = localStorage.getItem("theme");
    if (savedTheme) {
      isDark = savedTheme === "dark";
      if (isDark) document.documentElement.classList.add("dark");
    } else {
      // Check system preference
      isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      if (isDark) document.documentElement.classList.add("dark");
    }
  });

  async function checkNetwork() {
    try {
      const result = await invoke<NetworkHealth>("check_network_health");
      networkStatus.status = result.status;
      networkStatus.latency = result.latency;
      networkStatus.timestamp = result.timestamp;
      networkStatus.error = "";

      networkStatus.history = [
        {
          status: result.status,
          latency: result.latency,
          timestamp: result.timestamp,
        },
        ...networkStatus.history.slice(0, 4),
      ];
    } catch (e: unknown) {
      networkStatus.error = e instanceof Error ? e.message : String(e);
    }
  }

  async function checkGeoLocation() {
    try {
      const result = await invoke<GeoLocation>("get_geo_location");
      Object.assign(geoLocation, {
        ...result,
        error: "",
        timestamp: new Date().toISOString(),
      });
    } catch (e: unknown) {
      geoLocation.error = e instanceof Error ? e.message : String(e);
    }
  }

  async function checkSystemStatus() {
    try {
      const result = await invoke<SystemStatus>("get_system_status");
      systemStatus.processes = result.processes;
      systemStatus.uptime = result.uptime;
      systemStatus.load = result.load;
      systemStatus.timestamp = result.timestamp;
      systemStatus.error = "";

      systemStatus.history = [
        { processes: result.processes, timestamp: result.timestamp },
        ...systemStatus.history.slice(0, 4),
      ];
    } catch (e: unknown) {
      systemStatus.error = e instanceof Error ? e.message : String(e);
    }
  }

  function toggleNetworkMonitor() {
    if (networkStatus.isRunning) {
      clearInterval(networkInterval);
    } else {
      checkNetwork();
      networkInterval = setInterval(checkNetwork, 3000);
    }
    networkStatus.isRunning = !networkStatus.isRunning;
  }

  function toggleGeoLocation() {
    if (geoLocation.isRunning) {
      clearInterval(geoInterval);
    } else {
      checkGeoLocation();
      geoInterval = setInterval(checkGeoLocation, 5000);
    }
    geoLocation.isRunning = !geoLocation.isRunning;
  }

  function toggleSystemStatus() {
    if (systemStatus.isRunning) {
      clearInterval(systemInterval);
    } else {
      checkSystemStatus();
      systemInterval = setInterval(checkSystemStatus, 10000);
    }
    systemStatus.isRunning = !systemStatus.isRunning;
  }

  onMount(() => {
    // Initial checks and start intervals
    checkNetwork();
    checkGeoLocation();
    checkSystemStatus();

    networkInterval = setInterval(checkNetwork, 3000);
    geoInterval = setInterval(checkGeoLocation, 5000);
    systemInterval = setInterval(checkSystemStatus, 10000);
  });

  onDestroy(() => {
    clearInterval(networkInterval);
    clearInterval(geoInterval);
    clearInterval(systemInterval);
  });
</script>

<div
  class="fixed inset-0 font-mono bg-background text-primary text-sm p-5 flex flex-col"
>
  <div class="border-2 border-primary border-double p-4 mb-5">
    <div class="flex justify-between items-center">
      <h1 class="text-lg font-mono">System Dashboard</h1>
      <button
        class="bg-transparent border border-primary text-primary cursor-pointer font-mono px-2 py-1 hover:bg-gray-100 dark:hover:bg-gray-800"
        onclick={toggleTheme}
        >{isDark ? "☀️ Light Mode" : "🌙 Dark Mode"}</button
      >
    </div>
  </div>

  <div class="flex flex-col md:flex-row gap-5 flex-1">
    <div class="flex-1 border-2 border-primary border-double p-4 flex flex-col">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-base font-mono">Network Health Monitor</h2>
      </div>
      <div class="space-y-2 flex-1">
        <div class="font-mono">Status : {networkStatus.status}</div>
        <div class="font-mono">Latency : {networkStatus.latency} ms</div>
        <div class="font-mono">
          Last Check : {networkStatus.timestamp.slice(0, 19)}
        </div>
        <div class="font-mono">Error : {networkStatus.error || "None"}</div>
        <div class="border-t border-primary my-2"></div>
        <div class="font-mono">Last 5 Results:</div>
        {#each networkStatus.history as result, i}
          <div class="font-mono">
            {i + 1}. {result.status === "connected" ? "✓" : "×"}
            {result.latency}ms at {result.timestamp.slice(11, 19)}
          </div>
        {/each}
      </div>
      <div class="border-t border-primary my-2"></div>
      <button
        class="bg-transparent border-none text-primary cursor-pointer font-mono p-0 hover:underline w-full text-left"
        onclick={toggleNetworkMonitor}
        >{networkStatus.isRunning ? "[Pause]" : "[Start]"}</button
      >
    </div>

    <div class="flex-1 border-2 border-primary border-double p-4 flex flex-col">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-base font-mono">IP Geolocation Tracker</h2>
      </div>
      <div class="space-y-2 flex-1">
        <div class="font-mono">IP : {geoLocation.ip}</div>
        <div class="font-mono">City : {geoLocation.city}</div>
        <div class="font-mono">Region : {geoLocation.region}</div>
        <div class="font-mono">Country : {geoLocation.country}</div>
        <div class="font-mono">Org : {geoLocation.org}</div>
        <div class="font-mono">Timezone : {geoLocation.timezone}</div>
        <div class="font-mono">
          Last Check: {geoLocation.timestamp.slice(0, 19)}
        </div>
        <div class="font-mono">Error : {geoLocation.error || "None"}</div>
      </div>
      <div class="border-t border-primary my-2"></div>
      <button
        class="bg-transparent border-none text-primary cursor-pointer font-mono p-0 hover:underline w-full text-left"
        onclick={toggleGeoLocation}
        >{geoLocation.isRunning ? "[Pause]" : "[Start]"}</button
      >
    </div>

    <div class="flex-1 border-2 border-primary border-double p-4 flex flex-col">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-base font-mono">System Status Checker</h2>
      </div>
      <div class="space-y-2 flex-1">
        <div class="font-mono">Processes : {systemStatus.processes}</div>
        <div class="font-mono">Uptime : {systemStatus.uptime}</div>
        <div class="font-mono">Load : {systemStatus.load}</div>
        <div class="font-mono">
          Last Check: {systemStatus.timestamp.slice(0, 19)}
        </div>
        <div class="font-mono">Error : {systemStatus.error || "None"}</div>
        <div class="border-t border-primary my-2"></div>
        <div class="font-mono">Last 5 Results:</div>
        {#each systemStatus.history as result, i}
          <div class="font-mono">
            {i + 1}. {result.processes} processes at {result.timestamp.slice(
              11,
              19
            )}
          </div>
        {/each}
      </div>
      <div class="border-t border-primary my-2"></div>
      <button
        class="bg-transparent border-none text-primary cursor-pointer font-mono p-0 hover:underline w-full text-left"
        onclick={toggleSystemStatus}
        >{systemStatus.isRunning ? "[Pause]" : "[Start]"}</button
      >
    </div>
  </div>
</div>
