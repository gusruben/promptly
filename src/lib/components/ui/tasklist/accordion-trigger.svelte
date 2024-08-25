<script lang="ts">
	import { Accordion as AccordionPrimitive } from "bits-ui";
	import ChevronDown from "lucide-svelte/icons/chevron-down";
	import { cn } from "$lib/utils.js";
	import TaskIcon from "./task-icon.svelte";

	type $$Props = AccordionPrimitive.TriggerProps & { status: "good" | "bad" | "working" };
	type $$Events = AccordionPrimitive.TriggerEvents;

	let className: $$Props["class"] = undefined;
	export let level: AccordionPrimitive.HeaderProps["level"] = 3;
	export { className as class };

	export let status: $$Props["status"] = "working";
</script>

<AccordionPrimitive.Header {level} class="flex">
	<AccordionPrimitive.Trigger
		class={cn(
			"group flex flex-1 items-center gap-3 py-4 font-medium transition-all [&[data-state=open]>svg]:rotate-180",
			className
		)}
		{...$$restProps}
		on:click
	>
		<TaskIcon {status} />
		<div class="group-hover:underline">
			<slot />
		</div>

		<!-- spacer so the chevron is at the end -->
		<div class="flex-grow"></div>

		<ChevronDown class="h-4 w-4 transition-transform duration-200" />
	</AccordionPrimitive.Trigger>
</AccordionPrimitive.Header>
