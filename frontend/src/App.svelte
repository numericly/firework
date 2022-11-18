<script>
  import { onMount } from "svelte";
  import { players, world, server } from "./store";
  import healthIcon from "./assets/health.png";
  import hungerIcon from "./assets/hunger.png";

  let socket;

  onMount(async () => {
    socket = new WebSocket("ws://127.0.0.1:8080/ws");
    window.setInterval(async () => {
      socket.send("ping");
    }, 1000);
  });

  
</script>

<main>
  <h1>{$world.name}</h1>
  <div id="content">
    <div id="left">
      <div id="world">
        <h1>TODO</h1>
      </div>
      <div id="console">
        <!-- {#each $server.console as line}
            <code>{line}<br/></code>
            {/each} -->
        <div />
        <input type="text" placeholder="Enter a command" />
      </div>
    </div>
    <div id="right">
      <div>
        <h2>World Info</h2>
        <h5>Time: {$world.time}</h5>
        <h5>Weather: {$world.weather}</h5>
        <h5>Difficulty: {$world.difficulty}</h5>
      </div>
      <div>
        <h2>Players</h2>
        {#each $players as player}
          <div class="player">
            <img
              class="skin"
              src="https://crafatar.com/avatars/{player.uuid}?size=24&overlay"
              alt="Player Skin"
            />
            <h3 class="name">{player.name}</h3>
            <br />
            <img src={healthIcon} alt="Health" />
            <p class="health">{player.health}</p>
            <img src={hungerIcon} alt="Hunger" />
            <p class="hunger">{player.food}</p>
          </div>
        {/each}
      </div>
      <div>
        <h2>Server Info</h2>
        <h5>Version: {$server.version}</h5>
        <h5>Render Distance: {$server.render_distance}</h5>
        <h5>
          Ram Usage: {$server.ram < 1000000000
            ? Math.round($server.ram / 100000) / 10 + " MB"
            : Math.round($server.ram / 100000000) / 10 + " GB"}
        </h5>
        <h5>TPS: {$server.tps}</h5>
        <h5>Cpu usage: {$server.cpu}</h5>
      </div>
    </div>
  </div>
</main>

<style>
</style>
