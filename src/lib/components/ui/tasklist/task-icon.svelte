<script lang="ts">
	export let status: "good" | "bad" | "working";
</script>

{#if status == "good"}
	<span
		class="mgc_check_circle_fill text-lg before:bg-gradient-to-t
            before:from-[#1BC249] before:to-[#34D460] before:bg-clip-text before:!text-transparent"
	/>
{:else if status == "bad"}
	<span
		class="mgc_close_circle_fill text-lg before:bg-gradient-to-t
            before:from-[#CC2525] before:to-[#E04545] before:bg-clip-text before:!text-transparent"
	/>
{:else}
	<span class="mgc_loading_3_line text-lg w-[1em] h-[1em] animate-spin">
        <span class="path1 before:!text-white"></span>
        <span class="path2 absolute before:!text-white"></span>
    </span>
{/if}
