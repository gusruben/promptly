<script lang="ts">
	import SendButton from "$lib/components/SendButton.svelte";
	import Task from "$lib/components/Task.svelte";
	import * as TaskList from "$lib/components/ui/tasklist";
	import "$lib/styles/carta.scss";
	import { code } from "@cartamd/plugin-code";
	import "@cartamd/plugin-code/default.css";
	import "@fontsource/space-mono";
	import { invoke } from "@tauri-apps/api";
	import { listen } from "@tauri-apps/api/event";
	import { Carta, MarkdownEditor } from "carta-md";
	import DOMPurify from "isomorphic-dompurify";
	import { onMount } from "svelte";

	let currentPrompt = "";
	let currentResponse = "";
	let unlisten: Function;

	async function submitPrompt() {
		if (!currentPrompt.trim()) return;

		unlisten = await listen("response", e => {
			console.log(e.payload);
			currentResponse += e.payload;
		})

		await invoke("prompt_claude", { prompt: currentPrompt })
		console.log("Prompt submitted:", currentPrompt);
	}

	onMount(() => {
		listen("response-end", () => {
			console.log("Claude response finished");
			if (unlisten) unlisten();
		})
	})

	function handlePromptInput(e: KeyboardEvent) {
		if (e.target && (e.target as Element).nodeName == "TEXTAREA") {
			if (e.key == "Enter" && !e.shiftKey) {
				e.preventDefault();
				submitPrompt();
			}
		}
	}

	const carta = new Carta({
		theme: "github-dark",
		sanitizer: DOMPurify.sanitize,
		disableIcons: true,
		extensions: [
			code({ theme: "github-dark" }),
			{
				components: [
					{
						component: SendButton,
						parent: "input",
						props: {
							callback: submitPrompt,
						},
					},
				],
			},
		],
	});

	// mouse trailer logic
	let mouseTrailer: HTMLDivElement;

	function updateMouseTrailer(e: MouseEvent) {
		mouseTrailer.style.left = e.clientX - mouseTrailer.offsetWidth / 2 + "px";
		mouseTrailer.style.top = e.clientY - mouseTrailer.offsetHeight / 2 + "px";
	}
</script>

<div class="flex h-full w-full flex-row items-stretch p-6">
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="flex h-full flex-grow flex-col gap-6" on:keydown={handlePromptInput}>
		<div class="flex-grow rounded-lg bg-background-2">
			<p class="p-5 text-lg font-bold">Tasks</p>

			<TaskList.Root class="border-t-2">
				<Task status="good" title="Task 1">Task 1 content</Task>
				<Task status="good" title="Task 2">Task 2 content</Task>
				<Task status="bad" title="Task 3">Task 3 content</Task>
				<Task status="working" title="Task 4">Task 4 content here, in progress</Task>
			</TaskList.Root>
		</div>

		<MarkdownEditor
			placeholder="Enter a prompt..."
			mode="tabs"
			disableToolbar={true}
			{carta}
			bind:value={currentPrompt}
		/>
	</div>
</div>

<!-- TODO with trailer: make it an overlay on the entire page, and move the radial gradient position -->
<!-- radial-gradient( 600px at var(--cursor-x) var(--cursor-y), rgba(255, 254, 241, 0.05), transparent 60% ) -->
<!-- <svelte:body on:mousemove={updateMouseTrailer} />
<div
	bind:this={mouseTrailer}
	class="fixed left-0 top-0 z-50 h-[32rem] w-[32rem] bg-[radial-gradient(circle_at_center,#E8EAEE10,#E8EAEE00_60%)]"
/> -->

<style lang="postcss">
	:global(body),
	:global(html) {
		@apply h-full w-full;
	}

	:global(.carta-font-code) {
		font-family: monospace;
		font-size: 1.1rem;
	}
</style>
