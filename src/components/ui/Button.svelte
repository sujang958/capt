<script lang="ts">
  import { Button } from "bits-ui"
  import { twMerge } from "tailwind-merge"

  let { class: className = "", children = null } = $props()
</script>

<Button.Root
  class={twMerge(
    "flex cursor-pointer flex-row items-center justify-center gap-x-2 rounded-lg bg-neutral-700 px-3 py-1 text-white transition duration-200 hover:opacity-80",
    className,
  )}
>
  {@render children()}
</Button.Root>
