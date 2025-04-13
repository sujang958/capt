<script lang="ts">
  import { Button } from "bits-ui"
  import { twMerge } from "tailwind-merge"

  let { class: className = "", children } = $props()
</script>

<Button.Root
  class={twMerge(
    "cursor-pointer rounded-lg bg-neutral-700 px-3 py-1 text-white transition duration-200 hover:opacity-80",
    className,
  )}
>
  {@render children()}
</Button.Root>
