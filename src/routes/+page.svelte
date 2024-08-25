<script>
	import * as TaskList from "$lib/components/ui/tasklist";
</script>

<div class="flex h-full w-full flex-row items-stretch p-6">
	<div class="flex h-full flex-grow flex-col gap-6">
		<div class="flex-grow rounded-lg bg-background-2">
			<p class="p-5 text-lg font-bold">Tasks</p>
			<!-- <div class="flex-col w-full border-t-2"></div> -->
			<TaskList.Root class="border-t-2">
				<TaskList.Item value="task-1" status="good">
					<TaskList.Trigger>
                        Task 1
                    </TaskList.Trigger>
					<TaskList.Content>Task 1 content</TaskList.Content>
				</TaskList.Item>
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
