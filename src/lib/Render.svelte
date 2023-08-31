<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import PdfViewer from 'svelte-pdf';

    let name = "";
    let chemfig = ""
    let error = false;
    let waiting = false;

    async function greet(){
        waiting = true;
        chemfig = await invoke("name_to_pdf", { name })
        waiting = false;
        error = chemfig.startsWith("Err");
    }
</script>

<div>
    <form class="row" on:submit|preventDefault={greet}>
        <input id="greet-input" bind:value={name} placeholder="ein Alkan"/>
        <button type="submit">Render</button>
    </form>
    {#if waiting}
        <p>Waiting</p>
    {:else}
        {#if error}
            <p style="color: red;">{chemfig}</p>
        {:else}
            <div class="pdf">
            <PdfViewer url={chemfig} showButtons=[] showBorder=false/>
            </div>
        {/if}
    {/if}
</div>

<style>
    .pdf {
        padding: 5%;
    }
</style>
