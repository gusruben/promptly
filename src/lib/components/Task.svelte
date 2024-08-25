<script lang="ts">
	import * as TaskList from "$lib/components/ui/tasklist";

	export let status: "good" | "bad" | "working";
	export let title: string;
	export let value: string | undefined = undefined;
</script>

<TaskList.Item value={value || title} {status}>
	<TaskList.Trigger {status}>{title}</TaskList.Trigger>
	<TaskList.Content>
		<slot />
	</TaskList.Content>
</TaskList.Item>
