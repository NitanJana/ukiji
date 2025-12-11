<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { onDestroy, onMount } from "svelte";

    let displayState = "Waiting...";
    let unlisten: () => void;

    onMount(async () => {
        unlisten = await listen<string>("global-keypress", (event) => {
            displayState = event.payload;
        });
    });

    onDestroy(() => {
        if (unlisten) unlisten();
    });
</script>

<main
    class="h-screen flex justify-center items-center text-white rounded-lg bg-neutral-400 opacity-50 select-none text-4xl font-bold"
>
    {displayState}
</main>
