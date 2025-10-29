<script>
  import { invoke } from "@tauri-apps/api/core";

  // let name = $state("");
  // let greetMsg = $state("");

  /**
   * @param {{ preventDefault: () => void; }} event
   */
  // async function greet(event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main class="container">
  <h1>test</h1>
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 13px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

/* .container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
} */

h1 {
  text-align: center;
}

</style>
