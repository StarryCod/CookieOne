<!-- src/App.svelte -->
<script>
  import { Router, goto } from "@roxi/routify";
  import { routes } from "../.routify/routes";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import { SvelteUIProvider } from '@svelteuidev/core';

  import Events from "./Events.svelte";

  /** START LISTENING **/
  import { startListening } from "./functions";

  let isOnboardingCompleted = false;
  let checkingOnboarding = true;

  onMount(async () => {
    try {
      isOnboardingCompleted = await invoke("get_is_onboarding_completed");
      
      if (!isOnboardingCompleted) {
        // User needs onboarding, navigate to /onboarding
        goto("/onboarding");
      } else {
        // Start listening if onboarding is completed
        startListening();
      }
    } catch (error) {
      console.error("Error checking onboarding status:", error);
      // On error, assume onboarding is needed
      goto("/onboarding");
    } finally {
      checkingOnboarding = false;
    }
  });
</script>

{#if !checkingOnboarding}
  <SvelteUIProvider themeObserver='dark' withNormalizeCSS withGlobalStyles>
    <Router {routes} />
  </SvelteUIProvider>

  <Events />
{:else}
  <div style="display: flex; align-items: center; justify-content: center; height: 100vh;">
    <p>Загрузка...</p>
  </div>
{/if}