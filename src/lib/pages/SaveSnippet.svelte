<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";

    let title = "";
    let snippet = "";

    const save = async () => {
        await invoke("save_snippet", { title, snippet });
        appWindow.hide();
    };
</script>

<div class="container">
    <h1>New Snippet</h1>
    <input type="text" bind:value={title} placeholder="Title your snippet" autofocus/>
    <textarea
        rows="4"
        cols="50"
        bind:value={snippet}
        placeholder="Some text to save"
    />
    <button type="button" on:click={save}>Save</button>
</div>

<style lang="scss">
    $light-gray: rgba(255, 250, 250, 0.9);
    $dark-gray: rgba(169, 169, 169, 0.2);

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        row-gap: 10px;
        box-sizing: border-box;

        padding: 0 35%;

        width: 100%;
        height: 100%;

        h1 {
            color: $light-gray;
        }

        input {
            height: 20px;
            padding: 10px;

            font-size: 20px;
            background-color: $dark-gray;
            color: $light-gray;
        }

        textarea {
            padding: 10px;
            background-color: $dark-gray;
            font-size: 20px;
            height: 300px;

            color: $light-gray;
        }

        button {
            background-color: $light-gray;
            height: 40px;
            font-size: 20px;
            font-weight: bold;
        }
    }
</style>
