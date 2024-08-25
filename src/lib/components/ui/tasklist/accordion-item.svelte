<script lang="ts">
	import { Accordion as AccordionPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	type $$Props = AccordionPrimitive.ItemProps & {status: "good" | "bad" | "working"};

	let className: $$Props["class"] = undefined;
	export let value: $$Props["value"];
	export { className as class };

	// code for task status
	export let status: $$Props["status"] = "working";
</script>

<AccordionPrimitive.Item {value} class={cn("border-b-2 px-5 border-l-2 border-l-" + status, className)} {...$$restProps}>
	<slot />
</AccordionPrimitive.Item>
