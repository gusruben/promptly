<script lang="ts">
	import { Accordion as AccordionPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	type $$Props = AccordionPrimitive.ItemProps & { status: "good" | "bad" | "working" };

	let className: $$Props["class"] = undefined;
	export let value: $$Props["value"];
	export { className as class };

	// code for task status
	export let status: $$Props["status"] = "working";

	const styling = {
		good: "from-[#24C95119] to-[#12632800] border-l-good border-b-[#3F4741]",
		bad: "from-[#D7363619] to-[#7C161600] border-l-bad border-b-[#423535]",
		working: "from-[#CDCDCD19] to-[#6A6A6A00] border-l-working border-b-[#404349]",
	}[status];
</script>

<AccordionPrimitive.Item
	{value}
	class={cn(
		`border-b-2 border-l-2 bg-gradient-to-r px-5 ${styling}`,
		className
	)}
	{...$$restProps}
>
	<slot />
</AccordionPrimitive.Item>
