<script>
  import { Trash2, Volume2 } from "lucide-svelte"
  import Button from "./ui/Button.svelte"
  import { devices } from "../lib/stores.svelte"
</script>

<div>
  <p class="text-sm text-neutral-200">Track 1</p>
  <div class="mt-2 flex flex-row items-center justify-between gap-x-12">
    <select
      name=""
      id=""
      class="form-select w-full rounded-lg bg-neutral-700 px-2 py-1"
    >
      {#each $devices.filter(({ kind }) => kind === "audioinput") as device (`${device.groupId}:${device.deviceId}`)}
        <option value="asdfasf" class="">{device.label}</option>
      {/each}
    </select>
    <div class="flex flex-row items-center gap-x-3">
      <!-- svelte-ignore a11y_consider_explicit_label -->
      <Button class="p-2">
        <Volume2 class="size-4" />
      </Button>
      <Button class="bg-red-300 p-2">
        <Trash2 class="size-4 stroke-red-900" />
      </Button>
    </div>
  </div>
</div>
