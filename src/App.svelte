<script lang="ts">
  import { onMount } from "svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import SearchBar from "./lib/pages/SearchBar.svelte";
  import SaveSnippet from "./lib/pages/SaveSnippet.svelte";

  let page = "FIND";

  interface Payload {
    screen: string;
  }

  onMount(async () => {
    await appWindow.listen("channel", ({ event, payload }) => {
      page = (payload as Payload).screen;
      appWindow.maximize();
    });
  });
</script>

{#if page === "FIND"}
  <SearchBar />
{:else}
  <SaveSnippet />
{/if}
