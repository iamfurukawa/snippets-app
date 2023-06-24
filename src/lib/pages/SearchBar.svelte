<script lang="ts">
    import { onMount } from "svelte";
    import { copyText } from "svelte-copy";
    import { invoke } from "@tauri-apps/api/tauri";
    import { appWindow } from "@tauri-apps/api/window";

    let query = "";
    let result = [];
    let selectedItemIndex = -1;
    let timeoutId;

    const find = () => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            if (query.length === 0) {
                result = [];
                return;
            }
            invoke("find_snippet", { query });
        }, 500);
    };

    const selectItem = (index) => {
        if (index >= 0 && index < result.length) {
            selectedItemIndex = index;
        }
    };

    interface Snippet {
        title: string;
        content: string;
    }

    interface QueryResult {
        snippets: Snippet[];
    }

    onMount(async () => {
        await appWindow.listen("find_result", ({ event, payload }) => {
            result = (payload as QueryResult).snippets;
        });

        window.addEventListener("keydown", handleKeyDown);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    });

    const copyToClipboard = async () => {
        const selectedItem = result[selectedItemIndex];
        copyText(selectedItem.content);
        console.log("Copied!");
        await appWindow.hide();
    };

    const handleKeyDown = (event) => {
        switch (event.key) {
            case "ArrowUp":
                selectItem(selectedItemIndex - 1);
                break;
            case "ArrowDown":
                selectItem(selectedItemIndex + 1);
                break;
            case "Enter":
                copyToClipboard();
                break;
        }
    };
</script>

<div class="container">
    <input
        type="text"
        bind:value={query}
        placeholder={"Spotlight Search..."}
        on:input={find}
        autofocus
    />
    <ul class="search-results">
        {#each result as item, index}
            <li class:selected={index === selectedItemIndex}>
                {`${item.title} - ${item.content}`}
            </li>
        {/each}
    </ul>
</div>

<style lang="scss">
    $light-gray: rgba(255, 250, 250, 0.9);
    $dark-gray: rgba(169, 169, 169, 0.2);

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        width: 100%;
        height: 100%;

        row-gap: 10px;

        input {
            height: 60px;
            width: 60%;

            padding: 10px;
            background-color: $dark-gray;

            border: 3px solid $light-gray;
            border-radius: 15px;

            color: $light-gray;
            font-weight: 500;
            font-size: 40px;
        }

        .search-results {
            height: 50%;
            width: 60%;
            padding: 0;

            li {
                padding: 10px;
                height: 30px;
                list-style-type: none;

                font-size: 20px;
                font-weight: 400;
                color: white;

                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
        }

        .selected {
            background-color: $dark-gray;

            height: auto;
            padding: 10px;
        }
    }
</style>
