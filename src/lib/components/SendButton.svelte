<script lang="ts">
    export let callback: Function;
</script>

<button
	class="mgc_send_line cursor-pointer text-2xl transition-all
            before:!text-muted-foreground hover:brightness-125"
    on:click={callback()}
/>
