<script>
	import Task from "$lib/components/Task.svelte";
	import * as TaskList from "$lib/components/ui/tasklist";
</script>

<div class="flex h-full w-full flex-row items-stretch p-6">
	<div class="flex h-full flex-grow flex-col gap-6">
		<div class="flex-grow rounded-lg bg-background-2">
			<p class="p-5 text-lg font-bold">Tasks</p>

			<TaskList.Root class="border-t-2">
				<Task status="good" title="Task 1">Task 1 content</Task>
				<Task status="good" title="Task 2">Task 2 content</Task>
				<Task status="bad" title="Task 3">Task 3 content</Task>
				<Task status="working" title="Task 4">Task 4 content here, in progress</Task>
			</TaskList.Root>
		</div>
	</div>
</div>

<style lang="postcss">
	:global(body),
	:global(html) {
		@apply h-full w-full;
	}
</style>
